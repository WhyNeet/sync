import { CreateMessagePayload, Message } from "../model/message";
import { send as sendWs, receive as wsReceive } from "./ws"
import { effect, Pipeline, pipeline, store } from "@rpm-state/core";

export const $messages = store({
  messages: [] as Message[]
})

export const receive: Pipeline<Message> = wsReceive.filter(message => message.kind === "message").map(message => (message.data as Message));
export const send = pipeline<string>();
const sendMessage: Pipeline<CreateMessagePayload> = send.map(content => ({ content }))

const publishMessage = effect<CreateMessagePayload>((msg) => sendWs(JSON.stringify(msg)));
publishMessage(sendMessage);

$messages.on(receive, (state, message) => ({ ...state, messages: [...state.messages, message] }));
