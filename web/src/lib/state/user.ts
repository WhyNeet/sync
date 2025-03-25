import { receive as receiveWs } from "./ws";
import { store } from "@rpm-state/core";

export const $user = store<{ uuid: string | null }>({
  uuid: null
});

export const authorized = receiveWs.filter(msg => msg.kind === "auth").map(msg => msg.data as string);

$user.on(authorized, (_, uuid) => ({ uuid }))
