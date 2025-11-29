import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import type { JobListing } from "$lib/types/jobListing";
import type { BackendResponse } from "$lib/types/backendResponse";

/**
 * ---------------------------------------------------------------------
 * Reactive job listings store
 * ---------------------------------------------------------------------
 */
export const jobListings = writable<JobListing[]>([]);

/**
 * ---------------------------------------------------------------------
 * Load all job listings
 * ---------------------------------------------------------------------
 */
export async function loadJobListings() {
  const raw = await invoke<string>("handle_job_listing_command", {
    command: { action: "ListAll" },
  });

  const res = JSON.parse(raw) as BackendResponse<JobListing[]>;

  if (res.status === "success" && res.data) {
    jobListings.set(res.data);
  } else {
    console.error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Create a new job listing
 * ---------------------------------------------------------------------
 */
export async function createJobListing(
  payload: Omit<JobListing, "id" | "createdAt" | "updatedAt">,
): Promise<JobListing> {
  const raw = await invoke<string>("handle_job_listing_command", {
    command: { action: "Create", payload },
  });

  const res = JSON.parse(raw) as BackendResponse<JobListing>;

  if (res.status === "success" && res.data) {
    jobListings.update((list) => [...list, res.data!]);
    return res.data;
  } else {
    console.error(res.message);
    throw new Error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Update an existing job listing
 * ---------------------------------------------------------------------
 */
export async function updateJobListing(
  id: number,
  updates: Partial<JobListing>,
) {
  const raw = await invoke<string>("handle_job_listing_command", {
    command: { action: "Update", payload: { id, ...updates } },
  });

  const res = JSON.parse(raw) as BackendResponse<JobListing>;

  if (res.status === "success" && res.data) {
    jobListings.update((list) =>
      list.map((l) => (l.id === id ? res.data! : l)),
    );
  } else {
    console.error(res.message);
  }
}

/**
 * ---------------------------------------------------------------------
 * Delete a job listing
 * ---------------------------------------------------------------------
 */
export async function deleteJobListing(id: number) {
  const raw = await invoke<string>("handle_job_listing_command", {
    command: { action: "Delete", payload: { id } },
  });

  const res = JSON.parse(raw) as BackendResponse<null>;

  if (res.status === "success") {
    jobListings.update((list) => list.filter((l) => l.id !== id));
  } else {
    console.error(res.message);
  }
}
