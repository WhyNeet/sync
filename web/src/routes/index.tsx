import { useEffect, useState } from 'react';
import { $messages, send } from '../lib/state/messages';
import { useStore } from '@rpm-state/react';
import { connectWs } from '../lib/state/ws';
import { Message } from '../components/message';
import { Button, Stack, TextField } from '@mui/material';
import { env } from '../lib/env';

const VITE_APP_URI = env().VITE_APP_URI;

export function Messages() {
  const messages = useStore($messages);
  const [message, setMessage] = useState("");

  useEffect(() => {
    connectWs(`ws://${VITE_APP_URI}/messaging/chat`);
  }, [])

  return <Stack sx={{ height: "100%" }}>
    <Stack spacing="1rem" sx={{ height: "100%", p: "1rem", overflowY: "scroll", overflowX: "visible" }}>
      {messages.messages.map((msg) => <Message key={msg.id} message={msg} />)}
    </Stack>
    <Stack direction="row" gap="1" pb="1">
      <TextField multiline placeholder='Type a message...' value={message} onChange={e => setMessage(e.currentTarget.value)} fullWidth />
      <Button onClick={() => { send(message); setMessage(""); }}>Send</Button>
    </Stack>
  </Stack>
}
