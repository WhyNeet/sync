import { Paper, Stack, Typography } from "@mui/material";
import { Message as ChatMessage } from "../lib/model/message";

export function Message({ message, isCurrentUser }: { message: ChatMessage, isCurrentUser: boolean }) {
  return <Paper sx={{ px: "1rem", py: "0.6rem", bgcolor: isCurrentUser ? "Highlight" : "Background", width: "fit", alignSelf: isCurrentUser ? "end" : "start" }} elevation={3}>
    <Stack>
      <Typography variant="body1">{message.content}</Typography>
      <Typography variant="caption" sx={{ alignSelf: "end" }}>{new Date(message.id).getHours()}:{new Date(message.id).getMinutes()}</Typography>
    </Stack>
  </Paper>
}