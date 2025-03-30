/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_APP_URI: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}