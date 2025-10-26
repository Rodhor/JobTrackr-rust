import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import type { Person } from "$lib/types/person";
import type { BackendResponse } from "$lib/types/backendResponse";

/**
 * ---------------------------------------------------------------------
 * Reactive store for people (persons)
 * ---------------------------------------------------------------------
 */
export const people = writable<Person[]>([]);

/**
 * ---------------------------------------------------------------------
 * Load all people
 * ---------------------------------------------------------------------
 */
export async function loadPeople() {
  const raw = await invoke<string>("get_all_persons_command");
  const res = JSON.parse(raw) as BackendResponse<Person[]>;

  if (res.status === "success" && res.data) {
    people.set(res.data);
  } else {
    console.error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Create a new person
 * ---------------------------------------------------------------------
 */
export async function createPerson(
  payload: Omit<Person, "id" | "createdAt" | "updatedAt">,
) {
  const raw = await invoke<string>("create_person_command", payload);
  const res = JSON.parse(raw) as BackendResponse<Person>;

  if (res.status === "success" && res.data) {
    people.update((list) => [...list, res.data!]);
  } else {
    console.error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Update existing person
 * ---------------------------------------------------------------------
 */
export async function updatePerson(id: number, updates: Partial<Person>) {
  const raw = await invoke<string>("update_person_command", { id, ...updates });
  const res = JSON.parse(raw) as BackendResponse<Person>;

  if (res.status === "success" && res.data) {
    people.update((list) => list.map((p) => (p.id === id ? res.data! : p)));
  } else {
    console.error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Delete person
 * ---------------------------------------------------------------------
 */
export async function deletePerson(id: number) {
  const raw = await invoke<string>("delete_person_command", { id });
  const res = JSON.parse(raw) as BackendResponse<null>;

  if (res.status === "success") {
    people.update((list) => list.filter((p) => p.id !== id));
  } else {
    console.error(res.message);
  }
}
