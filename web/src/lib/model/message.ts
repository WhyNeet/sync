export interface Message {
  content: string,
  user_id: string
}

export interface CreateMessagePayload {
  content: string
}