import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import type { JobListingContact } from "$lib/types/jobListingContact";
import type { BackendResponse } from "$lib/types/backendResponse";

/**
 * ---------------------------------------------------------------------
 * Reactive store for job listing contacts
 * ---------------------------------------------------------------------
 */
export const jobListingContacts = writable<JobListingContact[]>([]);

/**
 * ---------------------------------------------------------------------
 * Load all job listing contacts
 * ---------------------------------------------------------------------
 */
export async function loadJobListingContacts() {
  const raw = await invoke<string>("get_all_job_listing_contacts_command");
  const res = JSON.parse(raw) as BackendResponse<JobListingContact[]>;

  if (res.status === "success" && res.data) {
    jobListingContacts.set(res.data);
  } else {
    console.error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Create a new job listing contact
 * ---------------------------------------------------------------------
 */
export async function createJobListingContact(
  payload: Omit<JobListingContact, "id" | "createdAt" | "updatedAt">,
) {
  const raw = await invoke<string>(
    "create_job_listing_contact_command",
    payload,
  );
  const res = JSON.parse(raw) as BackendResponse<JobListingContact>;

  if (res.status === "success" && res.data) {
    jobListingContacts.update((list) => [...list, res.data!]);
  } else {
    console.error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Update an existing job listing contact
 * ---------------------------------------------------------------------
 */
export async function updateJobListingContact(
  id: number,
  updates: Partial<JobListingContact>,
) {
  const raw = await invoke<string>("update_job_listing_contact_command", {
    id,
    ...updates,
  });
  const res = JSON.parse(raw) as BackendResponse<JobListingContact>;

  if (res.status === "success" && res.data) {
    jobListingContacts.update((list) =>
      list.map((c) => (c.id === id ? res.data! : c)),
    );
  } else {
    console.error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Delete a job listing contact
 * ---------------------------------------------------------------------
 */
export async function deleteJobListingContact(id: number) {
  const raw = await invoke<string>("delete_job_listing_contact_command", {
    id,
  });
  const res = JSON.parse(raw) as BackendResponse<null>;

  if (res.status === "success") {
    jobListingContacts.update((list) => list.filter((c) => c.id !== id));
  } else {
    console.error(res.message);
  }
}
