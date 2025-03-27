import { CreateMessagePayload, Message } from "../model/message";
import { sendWs, receiveWs } from "./ws"
import { effect, Pipeline, pipeline, store } from "@rpm-state/core";

export const $messages = store({
  messages: [] as Message[]
})

// bulk messages, from most recent to latest
export const receiveBulk: Pipeline<Message[]> = receiveWs.filter(message => message.kind === "messages").map(messages => messages.data as Message[]);
export const receive: Pipeline<Message> = receiveWs.filter(message => message.kind === "message").map(message => message.data as Message);
export const send = pipeline<string>();
const sendMessage: Pipeline<CreateMessagePayload> = send.map(msg => msg.trim()).filter(msg => msg.length > 0).map(content => ({ content }));

const publishMessage = effect<CreateMessagePayload>((msg) => sendWs(JSON.stringify(msg)));
publishMessage(sendMessage);

$messages.on(receive, (state, message) => ({ ...state, messages: [...state.messages, message] }));
$messages.on(receiveBulk, (state, messages) => ({ ...state, messages: [...messages.reverse(), ...state.messages] }));