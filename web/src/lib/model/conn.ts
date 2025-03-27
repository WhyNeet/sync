import { Message } from "./message";

export type WsMessage = { kind: "auth" | "error", data: string } | { kind: "message", data: Message } | { kind: "messages", data: Message[] };