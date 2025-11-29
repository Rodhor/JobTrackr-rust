import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import type { Note } from "$lib/types/note";
import type { BackendResponse } from "$lib/types/backendResponse";

/**
 * ---------------------------------------------------------------------
 * Reactive notes store
 * ---------------------------------------------------------------------
 */
export const notes = writable<Note[]>([]);

/**
 * ---------------------------------------------------------------------
 * Load all notes
 * ---------------------------------------------------------------------
 */
export async function loadNotes() {
  const raw = await invoke<string>("handle_note_command", {
    command: { action: "ListAll" },
  });

  const res = JSON.parse(raw) as BackendResponse<Note[]>;

  if (res.status === "success" && res.data) {
    notes.set(res.data);
  } else {
    console.error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Create a new note
 * ---------------------------------------------------------------------
 */
export async function createNote(
  payload: Omit<Note, "id" | "createdAt" | "updatedAt">,
): Promise<Note> {
  const raw = await invoke<string>("handle_note_command", {
    command: { action: "Create", payload },
  });

  const res = JSON.parse(raw) as BackendResponse<Note>;

  if (res.status === "success" && res.data) {
    notes.update((list) => [...list, res.data!]);
    return res.data;
  } else {
    console.error(res.message);
    throw new Error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Update an existing note
 * ---------------------------------------------------------------------
 */
export async function updateNote(id: number, updates: Partial<Note>) {
  const raw = await invoke<string>("handle_note_command", {
    command: { action: "Update", payload: { id, ...updates } },
  });

  const res = JSON.parse(raw) as BackendResponse<Note>;

  if (res.status === "success" && res.data) {
    notes.update((list) => list.map((l) => (l.id === id ? res.data! : l)));
  } else {
    console.error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Delete a note
 * ---------------------------------------------------------------------
 */
export async function deleteNote(id: number) {
  const raw = await invoke<string>("handle_note_command", {
    command: { action: "Delete", payload: { id } },
  });

  const res = JSON.parse(raw) as BackendResponse<null>;

  if (res.status === "success") {
    notes.update((list) => list.filter((l) => l.id !== id));
  } else {
    console.error(res.message);
  }
}
