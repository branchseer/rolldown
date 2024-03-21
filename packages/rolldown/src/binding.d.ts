/* auto-generated by NAPI-RS */
/* eslint-disable */

export class Bundler {
  constructor(
    inputOptions: BindingInputOptions,
    outputOptions: BindingOutputOptions,
  )
  write(): Promise<BindingOutputs>
  generate(): Promise<BindingOutputs>
  scan(): Promise<void>
}

export interface BindingInputItem {
  name?: string
  import: string
}

export interface BindingInputOptions {
  external?:
    | undefined
    | ((
        source: string,
        importer: string | undefined,
        isResolved: boolean,
      ) => boolean)
  input: Array<BindingInputItem>
  plugins: Array<PluginOptions>
  resolve?: BindingResolveOptions
  cwd: string
}

export interface BindingOutputAsset {
  fileName: string
  source: string
}

export interface BindingOutputChunk {
  isEntry: boolean
  isDynamicEntry: boolean
  facadeModuleId?: string
  moduleIds: Array<string>
  exports: Array<string>
  fileName: string
  modules: Record<string, BindingRenderedModule>
  code: string
  map?: string
  sourcemapFileName?: string
}

export interface BindingOutputOptions {
  entryFileNames?: string
  chunkFileNames?: string
  dir?: string
  exports?: 'default' | 'named' | 'none' | 'auto'
  format?: 'es' | 'cjs'
  plugins: Array<PluginOptions>
  sourcemap?: 'file' | 'inline' | 'hidden'
}

export interface BindingOutputs {
  chunks: Array<BindingOutputChunk>
  assets: Array<BindingOutputAsset>
}

export interface BindingRenderedModule {
  code?: string
}

export interface BindingResolveOptions {
  alias?: Record<string, Array<string>>
  aliasFields?: Array<Array<string>>
  conditionNames?: Array<string>
  exportsFields?: Array<Array<string>>
  extensions?: Array<string>
  mainFields?: Array<string>
  mainFiles?: Array<string>
  modules?: Array<string>
  symlinks?: boolean
}

export interface HookRenderChunkOutput {
  code: string
}

export interface HookResolveIdArgsOptions {
  isEntry: boolean
  kind: string
}

export interface PluginOptions {
  name: string
  buildStart?: () => Promise<void>
  resolveId?: (
    specifier: string,
    importer?: string,
    options?: HookResolveIdArgsOptions,
  ) => Promise<undefined | ResolveIdResult>
  load?: (id: string) => Promise<undefined | SourceResult>
  transform?: (id: string, code: string) => Promise<undefined | SourceResult>
  buildEnd?: (error?: string) => Promise<void>
  renderChunk?: (
    code: string,
    chunk: RenderedChunk,
  ) => Promise<undefined | HookRenderChunkOutput>
  generateBundle?: (bundle: Outputs, isWrite: boolean) => Promise<void>
  writeBundle?: (bundle: Outputs) => Promise<void>
}

export interface PreRenderedChunk {
  isEntry: boolean
  isDynamicEntry: boolean
  facadeModuleId?: string
  moduleIds: Array<string>
  exports: Array<string>
}

export interface RenderedChunk {
  isEntry: boolean
  isDynamicEntry: boolean
  facadeModuleId?: string
  moduleIds: Array<string>
  exports: Array<string>
  fileName: string
  modules: Record<string, BindingRenderedModule>
}

export interface ResolveIdResult {
  id: string
  external?: boolean
}

export interface SourceResult {
  code: string
  map?: string
}
