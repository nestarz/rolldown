/* eslint-disable */
/* prettier-ignore */

/* auto-generated by NAPI-RS */

const __nodeFs = require('node:fs')
const __nodePath = require('node:path')
const { WASI: __nodeWASI } = require('node:wasi')
const { Worker } = require('node:worker_threads')

const {
  instantiateNapiModuleSync: __emnapiInstantiateNapiModuleSync,
  getDefaultContext: __emnapiGetDefaultContext,
  createOnMessage: __wasmCreateOnMessageForFsProxy,
} = require('@napi-rs/wasm-runtime')

const __rootDir = __nodePath.parse(process.cwd()).root

const __wasi = new __nodeWASI({
  version: 'preview1',
  env: process.env,
  preopens: {
    [__rootDir]: __rootDir,
  }
})

const __emnapiContext = __emnapiGetDefaultContext()

const __sharedMemory = new WebAssembly.Memory({
  initial: 16384,
  maximum: 65536,
  shared: true,
})

let __wasmFilePath = __nodePath.join(__dirname, 'rolldown-binding.wasm32-wasi.wasm')
const __wasmDebugFilePath = __nodePath.join(__dirname, 'rolldown-binding.wasm32-wasi.debug.wasm')

if (__nodeFs.existsSync(__wasmDebugFilePath)) {
  __wasmFilePath = __wasmDebugFilePath
} else if (!__nodeFs.existsSync(__wasmFilePath)) {
  try {
    __wasmFilePath = __nodePath.resolve('@rolldown/binding-wasm32-wasi')
  } catch {
    throw new Error('Cannot find rolldown-binding.wasm32-wasi.wasm file, and @rolldown/binding-wasm32-wasi package is not installed.')
  }
}

const { instance: __napiInstance, module: __wasiModule, napiModule: __napiModule } = __emnapiInstantiateNapiModuleSync(__nodeFs.readFileSync(__wasmFilePath), {
  context: __emnapiContext,
  asyncWorkPoolSize: (function() {
    const threadsSizeFromEnv = Number(process.env.NAPI_RS_ASYNC_WORK_POOL_SIZE ?? process.env.UV_THREADPOOL_SIZE)
    // NaN > 0 is false
    if (threadsSizeFromEnv > 0) {
      return threadsSizeFromEnv
    } else {
      return 4
    }
  })(),
  wasi: __wasi,
  onCreateWorker() {
    const worker = new Worker(__nodePath.join(__dirname, 'wasi-worker.mjs'), {
      env: process.env,
      execArgv: ['--experimental-wasi-unstable-preview1'],
    })
    worker.onmessage = ({ data }) => {
      __wasmCreateOnMessageForFsProxy(__nodeFs)(data)
    }
    return worker
  },
  overwriteImports(importObject) {
    importObject.env = {
      ...importObject.env,
      ...importObject.napi,
      ...importObject.emnapi,
      memory: __sharedMemory,
    }
    return importObject
  },
  beforeInit({ instance }) {
    __napi_rs_initialize_modules(instance)
  }
})

function __napi_rs_initialize_modules(__napiInstance) {
  __napiInstance.exports['__napi_register__SourceMap_struct_0']?.()
  __napiInstance.exports['__napi_register__IsolatedDeclarationsResult_struct_1']?.()
  __napiInstance.exports['__napi_register__IsolatedDeclarationsOptions_struct_2']?.()
  __napiInstance.exports['__napi_register__TransformResult_struct_3']?.()
  __napiInstance.exports['__napi_register__TransformOptions_struct_4']?.()
  __napiInstance.exports['__napi_register__TypeScriptOptions_struct_5']?.()
  __napiInstance.exports['__napi_register__JsxOptions_struct_6']?.()
  __napiInstance.exports['__napi_register__ReactRefreshOptions_struct_7']?.()
  __napiInstance.exports['__napi_register__ArrowFunctionsOptions_struct_8']?.()
  __napiInstance.exports['__napi_register__Es2015Options_struct_9']?.()
  __napiInstance.exports['__napi_register__isolated_declaration_0']?.()
  __napiInstance.exports['__napi_register__transform_1']?.()
  __napiInstance.exports['__napi_register__Bundler_struct_0']?.()
  __napiInstance.exports['__napi_register__Bundler_impl_7']?.()
  __napiInstance.exports['__napi_register__BindingExperimentalOptions_struct_8']?.()
  __napiInstance.exports['__napi_register__BindingInjectImportNamed_struct_9']?.()
  __napiInstance.exports['__napi_register__BindingInjectImportNamespace_struct_10']?.()
  __napiInstance.exports['__napi_register__BindingInputItem_struct_11']?.()
  __napiInstance.exports['__napi_register__BindingWatchOption_struct_12']?.()
  __napiInstance.exports['__napi_register__BindingNotifyOption_struct_13']?.()
  __napiInstance.exports['__napi_register__BindingResolveOptions_struct_14']?.()
  __napiInstance.exports['__napi_register__BindingTreeshake_struct_15']?.()
  __napiInstance.exports['__napi_register__BindingInputOptions_struct_16']?.()
  __napiInstance.exports['__napi_register__BindingAdvancedChunksOptions_struct_17']?.()
  __napiInstance.exports['__napi_register__BindingMatchGroup_struct_18']?.()
  __napiInstance.exports['__napi_register__BindingOutputOptions_struct_19']?.()
  __napiInstance.exports['__napi_register__BindingPluginContext_struct_20']?.()
  __napiInstance.exports['__napi_register__BindingPluginContext_impl_27']?.()
  __napiInstance.exports['__napi_register__BindingPluginContextResolvedId_struct_28']?.()
  __napiInstance.exports['__napi_register__BindingPluginOptions_struct_29']?.()
  __napiInstance.exports['__napi_register__BindingPluginWithIndex_struct_30']?.()
  __napiInstance.exports['__napi_register__BindingTransformPluginContext_struct_31']?.()
  __napiInstance.exports['__napi_register__BindingTransformPluginContext_impl_33']?.()
  __napiInstance.exports['__napi_register__BindingAssetSource_struct_34']?.()
  __napiInstance.exports['__napi_register__BindingEmittedAsset_struct_35']?.()
  __napiInstance.exports['__napi_register__BindingGeneralHookFilter_struct_36']?.()
  __napiInstance.exports['__napi_register__BindingTransformHookFilter_struct_37']?.()
  __napiInstance.exports['__napi_register__BindingHookLoadOutput_struct_38']?.()
  __napiInstance.exports['__napi_register__BindingHookRenderChunkOutput_struct_39']?.()
  __napiInstance.exports['__napi_register__BindingHookResolveIdExtraArgs_struct_40']?.()
  __napiInstance.exports['__napi_register__BindingHookResolveIdOutput_struct_41']?.()
  __napiInstance.exports['__napi_register__BindingHookSideEffects_42']?.()
  __napiInstance.exports['__napi_register__BindingHookTransformOutput_struct_43']?.()
  __napiInstance.exports['__napi_register__BindingPluginContextResolveOptions_struct_44']?.()
  __napiInstance.exports['__napi_register__BindingTransformHookExtraArgs_struct_45']?.()
  __napiInstance.exports['__napi_register__BindingBuiltinPlugin_struct_46']?.()
  __napiInstance.exports['__napi_register__BindingBuiltinPluginName_47']?.()
  __napiInstance.exports['__napi_register__BindingGlobImportPluginConfig_struct_48']?.()
  __napiInstance.exports['__napi_register__BindingManifestPluginConfig_struct_49']?.()
  __napiInstance.exports['__napi_register__BindingModulePreloadPolyfillPluginConfig_struct_50']?.()
  __napiInstance.exports['__napi_register__BindingJsonPluginConfig_struct_51']?.()
  __napiInstance.exports['__napi_register__BindingTransformPluginConfig_struct_52']?.()
  __napiInstance.exports['__napi_register__BindingAliasPluginConfig_struct_53']?.()
  __napiInstance.exports['__napi_register__BindingAliasPluginAlias_struct_54']?.()
  __napiInstance.exports['__napi_register__BindingBuildImportAnalysisPluginConfig_struct_55']?.()
  __napiInstance.exports['__napi_register__BindingReplacePluginConfig_struct_56']?.()
  __napiInstance.exports['__napi_register__BindingPluginOrder_57']?.()
  __napiInstance.exports['__napi_register__BindingPluginHookMeta_struct_58']?.()
  __napiInstance.exports['__napi_register__ParallelJsPluginRegistry_struct_59']?.()
  __napiInstance.exports['__napi_register__ParallelJsPluginRegistry_impl_61']?.()
  __napiInstance.exports['__napi_register__register_plugins_62']?.()
  __napiInstance.exports['__napi_register__BindingLog_struct_63']?.()
  __napiInstance.exports['__napi_register__BindingLogLevel_64']?.()
  __napiInstance.exports['__napi_register__BindingModuleInfo_struct_65']?.()
  __napiInstance.exports['__napi_register__BindingModuleInfo_impl_67']?.()
  __napiInstance.exports['__napi_register__BindingOutputAsset_struct_68']?.()
  __napiInstance.exports['__napi_register__BindingOutputAsset_impl_73']?.()
  __napiInstance.exports['__napi_register__JsOutputAsset_struct_74']?.()
  __napiInstance.exports['__napi_register__BindingOutputChunk_struct_75']?.()
  __napiInstance.exports['__napi_register__BindingOutputChunk_impl_90']?.()
  __napiInstance.exports['__napi_register__JsOutputChunk_struct_91']?.()
  __napiInstance.exports['__napi_register__BindingOutputs_struct_92']?.()
  __napiInstance.exports['__napi_register__BindingOutputs_impl_95']?.()
  __napiInstance.exports['__napi_register__JsChangedOutputs_struct_96']?.()
  __napiInstance.exports['__napi_register__PreRenderedChunk_struct_97']?.()
  __napiInstance.exports['__napi_register__RenderedChunk_struct_98']?.()
  __napiInstance.exports['__napi_register__BindingRenderedModule_struct_99']?.()
  __napiInstance.exports['__napi_register__AliasItem_struct_100']?.()
  __napiInstance.exports['__napi_register__ExtensionAliasItem_struct_101']?.()
  __napiInstance.exports['__napi_register__BindingSourcemap_struct_102']?.()
  __napiInstance.exports['__napi_register__BindingJsonSourcemap_struct_103']?.()
  __napiInstance.exports['__napi_register__BindingWatcher_struct_104']?.()
  __napiInstance.exports['__napi_register__BindingWatcher_impl_107']?.()
  __napiInstance.exports['__napi_register__BindingWatcherEvent_108']?.()
}
module.exports.BindingLog = __napiModule.exports.BindingLog
module.exports.BindingModuleInfo = __napiModule.exports.BindingModuleInfo
module.exports.BindingOutputAsset = __napiModule.exports.BindingOutputAsset
module.exports.BindingOutputChunk = __napiModule.exports.BindingOutputChunk
module.exports.BindingOutputs = __napiModule.exports.BindingOutputs
module.exports.BindingPluginContext = __napiModule.exports.BindingPluginContext
module.exports.BindingTransformPluginContext = __napiModule.exports.BindingTransformPluginContext
module.exports.BindingWatcher = __napiModule.exports.BindingWatcher
module.exports.Bundler = __napiModule.exports.Bundler
module.exports.ParallelJsPluginRegistry = __napiModule.exports.ParallelJsPluginRegistry
module.exports.BindingBuiltinPluginName = __napiModule.exports.BindingBuiltinPluginName
module.exports.BindingHookSideEffects = __napiModule.exports.BindingHookSideEffects
module.exports.BindingLogLevel = __napiModule.exports.BindingLogLevel
module.exports.BindingPluginOrder = __napiModule.exports.BindingPluginOrder
module.exports.BindingWatcherEvent = __napiModule.exports.BindingWatcherEvent
module.exports.isolatedDeclaration = __napiModule.exports.isolatedDeclaration
module.exports.registerPlugins = __napiModule.exports.registerPlugins
module.exports.transform = __napiModule.exports.transform
