import { pipeline, store } from "@rpm-state/core";
import { WsMessage } from "../model/conn";

const $ws = store<WebSocket | null>(null);

export const connect = pipeline<string>();
export const close = pipeline();
export const send = pipeline<string>();
export const receive = pipeline<WsMessage>();

$ws.on(connect, (state, uri) => {
  if (state) {
    console.warn("[ws] cannot open connection: already established.");
    return state;
  }
  const ws = new WebSocket(uri);

  ws.addEventListener("message", ev => {
    const data = JSON.parse(ev.data);
    receive(data);
  });

  return ws;
});

$ws.on(close, (ws) => {
  if (!ws) throw new Error("[ws] cannot close connection: not established.");
  ws.close();
  return null;
})

$ws.on(send, (ws, message) => {
  if (!ws) throw new Error("[ws] cannot send over connection: not established.");
  ws.send(message);
  return ws;
})