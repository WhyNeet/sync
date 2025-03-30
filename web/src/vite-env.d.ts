/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_DEV_APP_SERVER_URI: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}