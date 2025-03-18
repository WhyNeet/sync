/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_DEV_APP_SERVER_URI: string
  // more env variables...
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}