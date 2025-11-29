import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import type { Company } from "$lib/types/company";
import type { BackendResponse } from "$lib/types/backendResponse";

/**
 * ---------------------------------------------------------------------
 * Reactive company list store
 * ---------------------------------------------------------------------
 */
export const companies = writable<Company[]>([]);

/**
 * ---------------------------------------------------------------------
 * Load all companies from backend
 * ---------------------------------------------------------------------
 */
export async function loadCompanies() {
  const raw = await invoke<string>("handle_company_command", {
    command: { action: "ListAll" },
  });
  const res = JSON.parse(raw) as BackendResponse<Company[]>;

  if (res.status === "success" && res.data) {
    companies.set(res.data);
  } else {
    console.error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Create a new company
 * ---------------------------------------------------------------------
 */
export async function createCompany(
  payload: Omit<Company, "id" | "createdAt" | "updatedAt">,
): Promise<Company> {
  const raw = await invoke<string>("handle_company_command", {
    command: { action: "Create", payload },
  });
  const res = JSON.parse(raw) as BackendResponse<Company>;

  if (res.status === "success" && res.data) {
    companies.update((list) => [...list, res.data!]);
    return res.data; // Add this line
  } else {
    console.error(res.message);
    throw new Error(res.message); // Also throw on error
  }
}

/**
 * ---------------------------------------------------------------------
 * Update existing company
 * ---------------------------------------------------------------------
 */
export async function updateCompany(id: number, updates: Partial<Company>) {
  const raw = await invoke<string>("handle_company_command", {
    command: { action: "Update", payload: { id, ...updates } },
  });
  const res = JSON.parse(raw) as BackendResponse<Company>;

  if (res.status === "success" && res.data) {
    companies.update((list) => list.map((c) => (c.id === id ? res.data! : c)));
  } else {
    console.error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Delete company
 * ---------------------------------------------------------------------
 */
export async function deleteCompany(id: number) {
  const raw = await invoke<string>("handle_company_command", {
    command: { action: "Delete", payload: { id } },
  });
  const res = JSON.parse(raw) as BackendResponse<null>;

  if (res.status === "success") {
    companies.update((list) => list.filter((c) => c.id !== id));
  } else {
    console.error(res.message);
  }
}
