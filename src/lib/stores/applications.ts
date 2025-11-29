import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import type { Application } from "$lib/types/application";
import type { BackendResponse } from "$lib/types/backendResponse";

export const applications = writable<Application[]>([]);

/**
 * ---------------------------------------------------------------------
 * Load all applications
 * ---------------------------------------------------------------------
 */
export async function loadApplications() {
  const raw = await invoke<string>("handle_application_command", {
    command: { action: "ListAll" },
  });

  const res = JSON.parse(raw) as BackendResponse<Application[]>;

  if (res.status === "success" && res.data) {
    applications.set(res.data);
  } else {
    console.error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Load single application by ID
 * ---------------------------------------------------------------------
 */
export async function loadApplicationById(id: number) {
  const raw = await invoke<string>("handle_application_command", {
    command: { action: "GetById", payload: { id } },
  });

  const res = JSON.parse(raw) as BackendResponse<Application>;
  return res.status === "success" ? (res.data ?? null) : null;
}

/**
 * ---------------------------------------------------------------------
 * Create new application
 * ---------------------------------------------------------------------
 */
export async function createApplication(
  payload: Omit<Application, "id" | "createdAt" | "updatedAt">,
): Promise<Application> {
  const raw = await invoke<string>("handle_application_command", {
    command: { action: "Create", payload },
  });

  const res = JSON.parse(raw) as BackendResponse<Application>;

  if (res.status === "success" && res.data) {
    applications.update((list) => [...list, res.data!]);
    return res.data;
  } else {
    console.error(res.message);
    throw new Error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Update existing application
 * ---------------------------------------------------------------------
 */
export async function updateApplication(
  id: number,
  updates: Partial<Application>,
) {
  const raw = await invoke<string>("handle_application_command", {
    command: { action: "Update", payload: { id, ...updates } },
  });

  const res = JSON.parse(raw) as BackendResponse<Application>;

  if (res.status === "success" && res.data) {
    applications.update((list) =>
      list.map((a) => (a.id === id ? res.data! : a)),
    );
  } else {
    console.error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Delete application
 * ---------------------------------------------------------------------
 */
export async function deleteApplication(id: number) {
  const raw = await invoke<string>("handle_application_command", {
    command: { action: "Delete", payload: { id } },
  });

  const res = JSON.parse(raw) as BackendResponse<null>;

  if (res.status === "success") {
    applications.update((list) => list.filter((a) => a.id !== id));
  } else {
    console.error(res.message);
  }
}
