// src/lib/stores/companies.mock.ts
import { writable } from "svelte/store";
import type { Company } from "$lib/types/company";

/**
 * ---------------------------------------------------------------------
 * Mock company list store (no backend)
 * ---------------------------------------------------------------------
 */
export const companies = writable<Company[]>([]);
// export const companies = writable<Company[]>([
//   {
//     id: 1,
//     name: "OpenAI GmbH",
//     streetAddress: "Lange Straße 10",
//     zipCode: "10115",
//     city: "Berlin",
//     country: "Germany",
//     defaultWorkType: "remote",
//     industry: "Artificial Intelligence",
//     website: "https://openai.com",
//     phoneNumber: "+49 30 12345678",
//     createdAt: new Date().toISOString(),
//     updatedAt: new Date().toISOString(),
//   },
//   {
//     id: 2,
//     name: "TechNova Solutions",
//     streetAddress: "Innovation Park 5",
//     zipCode: "80331",
//     city: "Munich",
//     country: "Germany",
//     defaultWorkType: "hybrid",
//     industry: "Software Development",
//     website: "https://technova.example.com",
//     phoneNumber: "+49 89 9876543",
//     createdAt: new Date().toISOString(),
//     updatedAt: new Date().toISOString(),
//   },
//   {
//     id: 3,
//     name: "GreenFuture AG",
//     streetAddress: "Solarallee 12",
//     zipCode: "50667",
//     city: "Cologne",
//     country: "Germany",
//     defaultWorkType: "remote",
//     industry: "Renewable Energy",
//     website: "https://greenfuture.example.com",
//     phoneNumber: "+49 221 445566",
//     createdAt: new Date().toISOString(),
//     updatedAt: new Date().toISOString(),
//   },
// ]);

/**
 * ---------------------------------------------------------------------
 * Stubbed async loader (simulates backend latency)
 * ---------------------------------------------------------------------
 */
export async function loadCompanies() {
  await new Promise((r) => setTimeout(r, 300));
  return true;
}

/**
 * ---------------------------------------------------------------------
 * Stubbed CRUD (operate purely on in-memory list)
 * ---------------------------------------------------------------------
 */
export async function createCompany(
  payload: Omit<Company, "id" | "createdAt" | "updatedAt">,
) {
  companies.update((list) => {
    const id = list.length ? Math.max(...list.map((c) => c.id)) + 1 : 1;
    const now = new Date().toISOString();
    const newCompany: Company = {
      id,
      ...payload,
      createdAt: now,
      updatedAt: now,
    };
    return [...list, newCompany];
  });
}

export async function updateCompany(id: number, updates: Partial<Company>) {
  companies.update((list) =>
    list.map((c) =>
      c.id === id
        ? { ...c, ...updates, updatedAt: new Date().toISOString() }
        : c,
    ),
  );
}

export async function deleteCompany(id: number) {
  companies.update((list) => list.filter((c) => c.id !== id));
}
