import { useStore } from "@rpm-state/react";
import { Logo } from "./logo";
import { $user } from "../lib/state/user";

export function Header() {
  const { uuid: userId } = useStore($user);

  return <header className="h-12 flex items-center border-b-indigo-50 border-b px-4 fixed top-0 inset-x-0 bg-indigo-50/40 backdrop-blur-md">
    <div className="flex-1 translate-y-0.5">
      <Logo />
    </div>
    <div className="text-sm flex items-center bg-slate-400/10 rounded-lg font-medium">
      <button className="rounded-lg px-2 py-1 text-center w-20 bg-white shadow-md">
        Chats
      </button>
      <button className="rounded-lg px-2 py-1.5 text-center w-20">
        Calls
      </button>
      <button className="rounded-lg px-2 py-1.5 text-center w-20">
        Settings
      </button>
    </div>
    <div className="flex-1 flex justify-end opacity-30">{userId}</div>
  </header>
}