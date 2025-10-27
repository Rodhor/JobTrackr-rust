import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import type { Reminder } from "$lib/types/reminder";
import type { BackendResponse } from "$lib/types/backendResponse";

/**
 * ---------------------------------------------------------------------
 * Reactive reminder store
 * ---------------------------------------------------------------------
 */
export const reminders = writable<Reminder[]>([]);

/**
 * ---------------------------------------------------------------------
 * Load all reminders
 * ---------------------------------------------------------------------
 */
export async function loadReminders() {
  const raw = await invoke<string>("get_all_reminders_command");
  const res = JSON.parse(raw) as BackendResponse<Reminder[]>;

  if (res.status === "success" && res.data) {
    reminders.set(res.data);
  } else {
    console.error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Create a new reminder
 * ---------------------------------------------------------------------
 */
export async function createReminder(
  payload: Omit<Reminder, "id" | "createdAt" | "updatedAt">,
) {
  const raw = await invoke<string>("create_reminder_command", payload);
  const res = JSON.parse(raw) as BackendResponse<Reminder>;

  if (res.status === "success" && res.data) {
    reminders.update((list) => [...list, res.data!]);
  } else {
    console.error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Update an existing reminder
 * ---------------------------------------------------------------------
 */
export async function updateReminder(id: number, updates: Partial<Reminder>) {
  const raw = await invoke<string>("update_note_command", {
    id,
    ...updates,
  });
  const res = JSON.parse(raw) as BackendResponse<Reminder>;

  if (res.status === "success" && res.data) {
    reminders.update((list) => list.map((l) => (l.id === id ? res.data! : l)));
  } else {
    console.error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Delete a reminder
 * ---------------------------------------------------------------------
 */
export async function deleteReminder(id: number) {
  const raw = await invoke<string>("delete_reminder_command", { id });
  const res = JSON.parse(raw) as BackendResponse<null>;

  if (res.status === "success") {
    reminders.update((list) => list.filter((l) => l.id !== id));
  } else {
    console.error(res.message);
  }
}
