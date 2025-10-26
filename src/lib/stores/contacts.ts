import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import type { Contact } from "$lib/types/contact";
import type { BackendResponse } from "$lib/types/backendResponse";

/**
 * ---------------------------------------------------------------------
 * Reactive contact list store
 * ---------------------------------------------------------------------
 */
export const contacts = writable<Contact[]>([]);

/**
 * ---------------------------------------------------------------------
 * Load all contacts from backend
 * ---------------------------------------------------------------------
 */
export async function loadContacts() {
  const raw = await invoke<string>("get_all_contacts_command");
  const res = JSON.parse(raw) as BackendResponse<Contact[]>;

  if (res.status === "success" && res.data) {
    contacts.set(res.data);
  } else {
    console.error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Create a new contact
 * ---------------------------------------------------------------------
 */
export async function createContact(
  payload: Omit<Contact, "id" | "createdAt" | "updatedAt">,
) {
  const raw = await invoke<string>("create_contact_command", payload);
  const res = JSON.parse(raw) as BackendResponse<Contact>;

  if (res.status === "success" && res.data) {
    contacts.update((list) => [...list, res.data!]);
  } else {
    console.error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Update existing contact
 * ---------------------------------------------------------------------
 */
export async function updateContact(id: number, updates: Partial<Contact>) {
  const raw = await invoke<string>("update_contact_command", {
    id,
    ...updates,
  });
  const res = JSON.parse(raw) as BackendResponse<Contact>;

  if (res.status === "success" && res.data) {
    contacts.update((list) => list.map((c) => (c.id === id ? res.data! : c)));
  } else {
    console.error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Delete contact
 * ---------------------------------------------------------------------
 */
export async function deleteContact(id: number) {
  const raw = await invoke<string>("delete_contact_command", { id });
  const res = JSON.parse(raw) as BackendResponse<null>;

  if (res.status === "success") {
    contacts.update((list) => list.filter((c) => c.id !== id));
  } else {
    console.error(res.message);
  }
}
