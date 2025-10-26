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
  const raw = await invoke<string>("get_all_companies_command");
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
) {
  const raw = await invoke<string>("create_company_command", payload);
  const res = JSON.parse(raw) as BackendResponse<Company>;

  if (res.status === "success" && res.data) {
    companies.update((list) => [...list, res.data!]);
  } else {
    console.error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Update existing company
 * ---------------------------------------------------------------------
 */
export async function updateCompany(id: number, updates: Partial<Company>) {
  const raw = await invoke<string>("update_company_command", {
    id,
    ...updates,
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
  const raw = await invoke<string>("delete_company_command", { id });
  const res = JSON.parse(raw) as BackendResponse<null>;

  if (res.status === "success") {
    companies.update((list) => list.filter((c) => c.id !== id));
  } else {
    console.error(res.message);
  }
}
