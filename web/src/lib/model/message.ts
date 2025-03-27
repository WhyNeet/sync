export interface Message {
  content: string,
  user_id: string,
  id: string,
  chat_id: string
}

export interface CreateMessagePayload {
  content: string
}