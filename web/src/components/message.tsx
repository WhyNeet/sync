import { Paper, Stack, Typography } from "@mui/material";
import { Message as ChatMessage } from "../lib/model/message";

export function Message({ message }: { message: ChatMessage }) {
  return <Paper sx={{ px: "1rem", py: "0.6rem" }} elevation={3}>
    <Stack direction="row" justifyContent="space-between">
      <Typography variant="body1">{message.content}</Typography>
      <Typography variant="caption">{new Date(message.id).toString()}</Typography>
    </Stack>
  </Paper>
}