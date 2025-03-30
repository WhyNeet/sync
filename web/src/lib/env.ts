const projectEnvVariables: ImportMetaEnv = {
  ...import.meta.env,
  VITE_APP_URI: "SYNC_APP_URI",
}

export function env() {
  return {
    VITE_APP_URI: import.meta.env.MODE === "production" ? projectEnvVariables.VITE_APP_URI : import.meta.env.VITE_APP_URI,
  }
}