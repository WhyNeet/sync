import { receive as receiveWs } from "./ws";
import { store } from "@rpm-state/core";

export const $user = store<{ uuid: string | null }>({
  uuid: null
});

$user.subscribe(console.log)

receiveWs.subscribe(console.log)

export const authorized = receiveWs.filter(msg => msg.kind === "auth").map(msg => msg.data as string);

$user.on(authorized, (_, uuid) => ({ uuid }))


// sample({
//   clock: receiveWs,
//   fn: clk => {
//     console.log("recv: ", clk)

//     if (clk.kind != "auth") return;

//     const uuid = clk.data;
//     authorized(uuid);
//   }
// })