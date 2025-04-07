import { useEffect, useRef, useState } from 'react';
import { $messages, send } from '../lib/state/messages';
import { useStore } from '@rpm-state/react';
import { connectWs } from '../lib/state/ws';
import { Message } from '../components/message';
import { AppBar, Avatar, Box, Button, IconButton, Menu, MenuItem, Stack, TextField, Toolbar, Tooltip, Typography } from '@mui/material';
import { env } from '../lib/env';
import { $user } from '../lib/state/user';
import { Link } from 'react-router';
import { checkAuth, signOut } from '../lib/cases/auth';

const { VITE_APP_URI } = env();

export function Messages() {
  const user = useStore($user);
  const messages = useStore($messages);
  const [message, setMessage] = useState("");

  useEffect(() => {
    if (!user) checkAuth();
  }, [])

  useEffect(() => {
    if (user) connectWs(`https://${VITE_APP_URI}/messaging/chat`);
  }, [user])

  const userMenuAnchor = useRef(null);
  const [userMenuOpen, setUserMenuOpen] = useState(false);

  return <>
    <AppBar position="fixed">
      <Toolbar variant="regular">
        <Typography variant="h6" color="inherit" component="div">
          Sync
        </Typography>
        <Box sx={{ flexGrow: "1" }}></Box>
        {user ? <>
          <Tooltip title="Open settings">
            <IconButton ref={userMenuAnchor} onClick={() => setUserMenuOpen(true)}>
              <Avatar>
                {user.display_name.charAt(0)}
              </Avatar>
            </IconButton>
          </Tooltip>
          <Menu anchorEl={userMenuAnchor.current} open={userMenuOpen} onClose={() => setUserMenuOpen(false)}>
            <MenuItem onClick={() => {
              setUserMenuOpen(false);
              signOut();
            }}>Sign Out</MenuItem>
          </Menu>
        </> : <></>}
      </Toolbar>
    </AppBar>
    <Stack sx={{ height: "100%" }}>
      {user ? <>
        <Stack spacing="1rem" sx={{ height: "100%", p: "1rem", overflowY: "scroll", overflowX: "visible", pt: "5rem" }}>
          {messages.messages.map((msg) => <Message key={msg.id} message={msg} isCurrentUser={msg.user_id === user.id} />)}
        </Stack>
        <Stack direction="row" gap="1" pb={1}>
          <TextField multiline placeholder='Type a message...' value={message} onChange={e => setMessage(e.currentTarget.value)} fullWidth />
          <Button onClick={() => { send(message); setMessage(""); }}>Send</Button>
        </Stack>
      </> : <>
        <Stack sx={{ height: "100%" }} alignItems="center" justifyContent="center">
          <Typography variant="h3" component="h1">You are not signed in.</Typography>
          <Typography variant="subtitle1" mb={4}>Sign in to access Sync.</Typography>
          <Link to="/sign-in"><Button variant="contained">Sign In</Button></Link>
        </Stack>
      </>}
    </Stack>
  </>
}
