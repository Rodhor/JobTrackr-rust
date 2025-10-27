import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import type { Application } from "$lib/types/application";
import type { BackendResponse } from "$lib/types/backendResponse";

export const applications = writable<Application[]>([]);

export async function loadApplications() {
  const raw = await invoke<string>("get_all_applications_command");
  const res = JSON.parse(raw) as BackendResponse<Application[]>;

  if (res.status === "success" && res.data) {
    applications.set(res.data);
  } else {
    console.error(res.message);
  }
}

export async function loadApplicationById(id: number) {
  const raw = await invoke<string>("get_application_by_id_command", { id });
  const res = JSON.parse(raw) as BackendResponse<Application>;
  return res.status === "success" ? (res.data ?? null) : null;
}

export async function createApplication(
  payload: Omit<Application, "id" | "createdAt" | "updatedAt">,
) {
  const raw = await invoke<string>("create_application_command", payload);
  const res = JSON.parse(raw) as BackendResponse<Application>;

  if (res.status === "success" && res.data) {
    applications.update((list) => [...list, res.data!]);
  } else {
    console.error(res.message);
  }
}

export async function updateApplication(
  id: number,
  updates: Partial<Application>,
) {
  const raw = await invoke<string>("update_application_command", {
    id,
    ...updates,
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

export async function deleteApplication(id: number) {
  const raw = await invoke<string>("delete_application_command", { id });
  const res = JSON.parse(raw) as BackendResponse<null>;

  if (res.status === "success") {
    applications.update((list) => list.filter((a) => a.id !== id));
  } else {
    console.error(res.message);
  }
}
