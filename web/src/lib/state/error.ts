import { pipeline } from "@rpm-state/core";
import { toaster } from "../../components/ui/toaster";
import { receive } from "./ws";

export const clientSideError = pipeline<string>()
export const error = receive.filter(msg => msg.kind === "error").map(msg => msg.data as string).merge(clientSideError);

error.subscribe(error => {
  toaster.error({ description: error });
});