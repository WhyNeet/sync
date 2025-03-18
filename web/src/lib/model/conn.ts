import { Message } from "./message";

export type WsMessage = { kind: "auth", data: string } | { kind: "message", data: Message };