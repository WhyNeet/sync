import { useEffect, useState } from 'react';
import { $messages, send } from '../lib/state/messages';
import { useStore } from '@rpm-state/react';
import { connect } from '../lib/state/ws';
import { Message } from '../components/message';
import { Button, Flex, Stack, Textarea } from '@chakra-ui/react';


export function Messages() {
  const messages = useStore($messages);
  const [message, setMessage] = useState("");

  useEffect(() => {
    connect(`ws://${import.meta.env.VITE_DEV_APP_SERVER_URI}/chat`);
  }, [])

  return <Stack h="full">
    <Stack h="full" overflow="scroll" gap="1" py="1">
      {messages.messages.map((msg, idx) => <Message key={idx} text={msg.content} />)}
    </Stack>
    <Flex gap="1" pb="1">
      <Textarea resize="none" placeholder='Type a message...' borderRadius="lg" value={message} onChange={e => setMessage(e.currentTarget.value)} />
      <Button onClick={() => { send(message); setMessage(""); }} borderRadius="lg">Send</Button>
    </Flex>
  </Stack>
}
