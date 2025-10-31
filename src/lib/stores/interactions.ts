import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import type { Interaction } from "$lib/types/interaction";
import type { BackendResponse } from "$lib/types/backendResponse";

/**
 * ---------------------------------------------------------------------
 * Reactive interaction list store
 * ---------------------------------------------------------------------
 */
export const interactions = writable<Interaction[]>([]);

/**
 * ---------------------------------------------------------------------
 * Load all interactions from backend
 * ---------------------------------------------------------------------
 */
export async function loadInteractions() {
  const raw = await invoke<string>("get_all_interactions_command");
  const res = JSON.parse(raw) as BackendResponse<Interaction[]>;

  if (res.status === "success" && res.data) {
    interactions.set(res.data);
  } else {
    console.error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Create a new interaction
 * ---------------------------------------------------------------------
 */
export async function createInteraction(
  payload: Omit<Interaction, "id" | "createdAt" | "updatedAt">,
) {
  const raw = await invoke<string>("create_interaction_command", payload);
  const res = JSON.parse(raw) as BackendResponse<Interaction>;

  if (res.status === "success" && res.data) {
    interactions.update((list) => [...list, res.data!]);
  } else {
    console.error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Update existing interaction
 * ---------------------------------------------------------------------
 */
export async function updateInteraction(
  id: number,
  updates: Partial<Interaction>,
) {
  const raw = await invoke<string>("update_interaction_command", {
    id,
    ...updates,
  });
  const res = JSON.parse(raw) as BackendResponse<Interaction>;

  if (res.status === "success" && res.data) {
    interactions.update((list) =>
      list.map((i) => (i.id === id ? res.data! : i)),
    );
  } else {
    console.error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Delete interaction
 * ---------------------------------------------------------------------
 */
export async function deleteInteraction(id: number) {
  const raw = await invoke<string>("delete_interaction_command", { id });
  const res = JSON.parse(raw) as BackendResponse<null>;

  if (res.status === "success") {
    interactions.update((list) => list.filter((i) => i.id !== id));
  } else {
    console.error(res.message);
  }
}
