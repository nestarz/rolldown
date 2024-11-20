use super::stages::{
  link_stage::{LinkStage, LinkStageOutput},
  scan_stage::ScanStageOutput,
};
use crate::{
  bundler_builder::BundlerBuilder,
  stages::{generate_stage::GenerateStage, scan_stage::ScanStage},
  types::bundle_output::BundleOutput,
  watcher::watcher::{wait_for_change, Watcher},
  BundlerOptions, SharedOptions, SharedResolver,
};
use anyhow::Result;

use rolldown_common::{NormalizedBundlerOptions, SharedFileEmitter};
use rolldown_error::{BuildDiagnostic, BuildResult};
use rolldown_fs::{FileSystem, OsFileSystem};
use rolldown_plugin::{
  HookBuildEndArgs, HookRenderErrorArgs, SharedPluginDriver, __inner::SharedPluginable,
};
use rolldown_std_utils::OptionExt;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing_chrome::FlushGuard;

pub struct Bundler {
  pub closed: bool,
  pub(crate) fs: OsFileSystem,
  pub(crate) options: SharedOptions,
  pub(crate) resolver: SharedResolver,
  pub(crate) file_emitter: SharedFileEmitter,
  pub(crate) plugin_driver: SharedPluginDriver,
  pub(crate) warnings: Vec<BuildDiagnostic>,
  pub(crate) _log_guard: Option<FlushGuard>,
}

impl Bundler {
  pub fn new(options: BundlerOptions) -> Self {
    BundlerBuilder::default().with_options(options).build()
  }

  pub fn with_plugins(options: BundlerOptions, plugins: Vec<SharedPluginable>) -> Self {
    BundlerBuilder::default().with_options(options).with_plugins(plugins).build()
  }
}

impl Bundler {
  #[tracing::instrument(level = "debug", skip_all)]
  pub async fn write(&mut self) -> BuildResult<BundleOutput> {
    let dir = self.options.cwd.join(&self.options.dir);

    let mut output = self.bundle_up(/* is_write */ true).await?;

    self.fs.create_dir_all(&dir).map_err(|err| {
      anyhow::anyhow!("Could not create directory for output chunks: {:?}", dir).context(err)
    })?;

    for chunk in &output.assets {
      let dest = dir.join(chunk.filename());
      if let Some(p) = dest.parent() {
        if !self.fs.exists(p) {
          self.fs.create_dir_all(p).unwrap();
        }
      };
      self
        .fs
        .write(&dest, chunk.content_as_bytes())
        .map_err(|err| anyhow::anyhow!("Failed to write file in {:?}", dest).context(err))?;
    }

    self.plugin_driver.write_bundle(&mut output.assets).await?;

    output.warnings.append(&mut self.warnings);

    Ok(output)
  }

  #[tracing::instrument(level = "debug", skip_all)]
  pub async fn generate(&mut self) -> BuildResult<BundleOutput> {
    self.bundle_up(/* is_write */ false).await.map(|mut output| {
      output.warnings.append(&mut self.warnings);
      output
    })
  }

  #[tracing::instrument(level = "debug", skip_all)]
  pub async fn close(&mut self) -> Result<()> {
    if self.closed {
      return Ok(());
    }

    self.closed = true;
    self.plugin_driver.close_bundle().await?;

    Ok(())
  }

  pub async fn scan(&mut self) -> BuildResult<ScanStageOutput> {
    self.plugin_driver.build_start().await?;

    let mut error_for_build_end_hook = None;

    let scan_stage_output = match ScanStage::new(
      Arc::clone(&self.options),
      Arc::clone(&self.plugin_driver),
      self.fs,
      Arc::clone(&self.resolver),
    )
    .scan()
    .await
    {
      Ok(v) => v,
      Err(errs) => {
        // TODO: So far we even call build end hooks on unhandleable errors . But should we call build end hook even for unhandleable errors?
        error_for_build_end_hook = Some(errs.first().unpack_ref().to_string());
        self
          .plugin_driver
          .build_end(error_for_build_end_hook.map(|error| HookBuildEndArgs { error }).as_ref())
          .await?;
        self.plugin_driver.close_bundle().await?;
        return Err(errs);
      }
    };

    self
      .plugin_driver
      .build_end(error_for_build_end_hook.map(|error| HookBuildEndArgs { error }).as_ref())
      .await?;

    Ok(scan_stage_output)
  }

  async fn try_build(&mut self) -> BuildResult<LinkStageOutput> {
    let build_info = self.scan().await?;
    Ok(LinkStage::new(build_info, &self.options).link())
  }

  #[allow(clippy::missing_transmute_annotations)]
  async fn bundle_up(&mut self, is_write: bool) -> BuildResult<BundleOutput> {
    if self.closed {
      return Err(
        anyhow::anyhow!(
          "Bundle is already closed, no more calls to 'generate' or 'write' are allowed."
        )
        .into(),
      );
    }

    let mut link_stage_output = self.try_build().await?;

    self.plugin_driver.render_start().await?;

    let bundle_output =
      GenerateStage::new(&mut link_stage_output, &self.options, &self.plugin_driver)
        .generate()
        .await; // Notice we don't use `?` to break the control flow here.

    if let Err(errs) = &bundle_output {
      self
        .plugin_driver
        .render_error(&HookRenderErrorArgs { error: errs.first().unpack_ref().to_string() })
        .await?;
    }

    let mut output = bundle_output?;

    // Add additional files from build plugins.
    self.file_emitter.add_additional_files(&mut output.assets);

    self.plugin_driver.generate_bundle(&mut output.assets, is_write).await?;

    output.watch_files = self.plugin_driver.watch_files.iter().map(|f| f.clone()).collect();

    Ok(output)
  }

  pub fn options(&self) -> &NormalizedBundlerOptions {
    &self.options
  }

  pub fn watch(bundler: Arc<Mutex<Bundler>>) -> Result<Arc<Watcher>> {
    let watcher = Arc::new(Watcher::new(bundler)?);

    wait_for_change(Arc::clone(&watcher));

    Ok(watcher)
  }
}

fn _test_bundler() {
  #[allow(clippy::needless_pass_by_value)]
  fn _assert_send(_foo: impl Send) {}
  let mut bundler = Bundler::new(BundlerOptions::default());
  let write_fut = bundler.write();
  _assert_send(write_fut);
  let mut bundler = Bundler::new(BundlerOptions::default());
  let generate_fut = bundler.generate();
  _assert_send(generate_fut);
}
