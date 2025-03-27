import { pipeline, store } from "@rpm-state/core";
import { WsMessage } from "../model/conn";

const $ws = store<WebSocket | null>(null);

export const connectWs = pipeline<string>();
export const closeWs = pipeline();
export const sendWs = pipeline<string>();
export const receiveWs = pipeline<WsMessage>();

$ws.on(connectWs, (state, uri) => {
  if (state) {
    console.warn("[ws] cannot open connection: already established.");
    return state;
  }
  const ws = new WebSocket(uri);

  ws.addEventListener("message", ev => {
    const data = JSON.parse(ev.data);
    receiveWs(data);
  });

  return ws;
});

$ws.on(closeWs, (ws) => {
  if (!ws) throw new Error("[ws] cannot close connection: not established.");
  ws.close();
  return null;
})

$ws.on(sendWs, (ws, message) => {
  if (!ws) throw new Error("[ws] cannot send over connection: not established.");
  ws.send(message);
  return ws;
})