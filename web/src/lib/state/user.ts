import { User } from "../model/user";
import { pipeline, store } from "@rpm-state/core";

export const $user = store<User | null>(null);

export const authorized = pipeline<User | null>();

$user.on(authorized, (_, user) => user)
