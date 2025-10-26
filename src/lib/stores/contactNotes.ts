import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import type { BackendResponse } from "$lib/types/backendResponse";
import type { ContactNote } from "$lib/types/contactNote";

/**
 * ---------------------------------------------------------------------
 * Reactive store for contact notes
 * ---------------------------------------------------------------------
 */
export const contactNotes = writable<ContactNote[]>([]);

/**
 * ---------------------------------------------------------------------
 * Load all contact notes
 * ---------------------------------------------------------------------
 */
export async function loadContactNotes() {
  const raw = await invoke<string>("get_all_contact_notes_command");
  const res = JSON.parse(raw) as BackendResponse<ContactNote[]>;

  if (res.status === "success" && res.data) {
    contactNotes.set(res.data);
  } else {
    console.error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Create a new contact note
 * ---------------------------------------------------------------------
 */
export async function createContactNote(
  payload: Omit<ContactNote, "id" | "createdAt" | "updatedAt">,
) {
  const raw = await invoke<string>("create_contact_note_command", payload);
  const res = JSON.parse(raw) as BackendResponse<ContactNote>;

  if (res.status === "success" && res.data) {
    contactNotes.update((list) => [...list, res.data!]);
  } else {
    console.error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Update an existing contact note
 * ---------------------------------------------------------------------
 */
export async function updateContactNote(
  id: number,
  updates: Partial<ContactNote>,
) {
  const raw = await invoke<string>("update_contact_note_command", {
    id,
    ...updates,
  });
  const res = JSON.parse(raw) as BackendResponse<ContactNote>;

  if (res.status === "success" && res.data) {
    contactNotes.update((list) =>
      list.map((n) => (n.id === id ? res.data! : n)),
    );
  } else {
    console.error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Delete contact note
 * ---------------------------------------------------------------------
 */
export async function deleteContactNote(id: number) {
  const raw = await invoke<string>("delete_contact_note_command", { id });
  const res = JSON.parse(raw) as BackendResponse<null>;

  if (res.status === "success") {
    contactNotes.update((list) => list.filter((n) => n.id !== id));
  } else {
    console.error(res.message);
  }
}
