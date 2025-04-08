import { env } from "../env"
import { clear } from "../state/messages";
import { $user, authorized } from "../state/user";
import { closeWs } from "../state/ws";

const { VITE_APP_URI } = env();

export async function checkAuth() {
  if ($user.get() != null) return;

  const response = await fetch(`https://${VITE_APP_URI}/users/me`, { credentials: "include" }).catch(() => authorized(null));
  if (!response || !response.ok) return;

  authorized((await response.json()).data);
}

export async function signIn(data: { username: string, password: string }) {
  return await fetch(`https://${VITE_APP_URI}/identity/session/create`, { method: "POST", credentials: "include", headers: { "Content-Type": "application/json" }, body: JSON.stringify(data) }).then(() => true).catch(e => {
    console.log("error", e);
    return false;
  })
}

export async function signUp(data: { username: string, password: string, display_name: string, email: string }) {
  return await fetch(`https://${VITE_APP_URI}/users/register`, { method: "POST", credentials: "include", headers: { "Content-Type": "application/json" }, body: JSON.stringify(data) }).then(() => true).catch(e => {
    console.log("error", e);
    return false;
  })
}

export async function signOut() {
  await fetch(`https://${VITE_APP_URI}/identity/session`, { method: "DELETE", credentials: "include" }).then(() => {
    closeWs(null);
    clear(null);
    authorized(null);
  });
}