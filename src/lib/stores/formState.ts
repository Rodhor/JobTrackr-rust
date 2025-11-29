import { writable } from "svelte/store";

export const newlyCreatedCompanyId = writable<number | undefined>(undefined);
