import { Paper } from "@mui/material";

export function Message({ text }: { text: string }) {
  return <Paper>
    {text}
  </Paper>
}