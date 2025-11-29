import { writable } from "svelte/store";

export const newlyCreatedCompanyId = writable<number | undefined>(undefined);
export const newlyCreatedPersonId = writable<number | undefined>(undefined);
export const newlyCreatedNoteId = writable<number | undefined>(undefined);
export const newlyCreatedApplicationId = writable<number | undefined>(
  undefined,
);
export const newlyCreatedJoblistingId = writable<number | undefined>(undefined);
export const newlyCreatedInteractionId = writable<number | undefined>(
  undefined,
);
export const newlyCreatedReminderId = writable<number | undefined>(undefined);
