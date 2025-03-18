import { createFileRoute } from '@tanstack/react-router'
import { useEffect, useState } from 'react';
import { $messages, send } from '../lib/state/messages';
import { $user } from '../lib/state/user';
import { useStore } from '@rpm-state/react';
import { connect } from '../lib/state/ws';

export const Route = createFileRoute('/')({
  component: RouteComponent,
})

function RouteComponent() {
  const messages = useStore($messages);
  const user = useStore($user);
  const [message, setMessage] = useState("");

  useEffect(() => {
    connect(`ws://${import.meta.env.VITE_DEV_APP_SERVER_URI}/chat`);
  }, [])

  return <div>
    {messages.messages.map((msg, idx) => <div key={idx}>{msg.content}</div>)}
    <input type="text" placeholder="Enter message..." value={message} onChange={e => setMessage(e.currentTarget.value)} />
    <button onClick={() => {
      send(message)
      setMessage("");
    }}>Send</button>
    <br />
    <div>uuid: {user.uuid}</div>
  </div>
}
