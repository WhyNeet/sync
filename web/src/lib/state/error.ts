import { pipeline } from "@rpm-state/core";
import { receiveWs } from "./ws";

export const clientSideError = pipeline<string>()
export const error = receiveWs.filter(msg => msg.kind === "error").map(msg => msg.data as string).merge(clientSideError);

error.subscribe(error => {
  console.error(error);
  // toaster.error({ description: error });
});