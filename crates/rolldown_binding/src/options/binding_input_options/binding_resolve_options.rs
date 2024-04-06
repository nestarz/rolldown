use std::collections::HashMap;

use serde::Deserialize;

#[napi_derive::napi(object)]
#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct BindingResolveOptions {
  // FIXME: Using `HashMap<String, Vec<String>>` is wrong here. Rust's `HashMap` is not ordered, while JavaScript's `Object` is ordered.
  // We should use tuple array instead of object.
  pub alias: Option<HashMap<String, Vec<String>>>,
  pub alias_fields: Option<Vec<Vec<String>>>,
  pub condition_names: Option<Vec<String>>,
  pub exports_fields: Option<Vec<Vec<String>>>,
  pub extensions: Option<Vec<String>>,
  pub main_fields: Option<Vec<String>>,
  pub main_files: Option<Vec<String>>,
  pub modules: Option<Vec<String>>,
  pub symlinks: Option<bool>,
  pub tsconfig_filename: Option<String>,
}

impl From<BindingResolveOptions> for rolldown::ResolveOptions {
  fn from(value: BindingResolveOptions) -> Self {
    Self {
      alias: value.alias.map(|alias| alias.into_iter().collect::<Vec<_>>()),
      alias_fields: value.alias_fields,
      condition_names: value.condition_names,
      exports_fields: value.exports_fields,
      extensions: value.extensions,
      main_fields: value.main_fields,
      main_files: value.main_files,
      modules: value.modules,
      symlinks: value.symlinks,
      tsconfig_filename: value.tsconfig_filename,
    }
  }
}
