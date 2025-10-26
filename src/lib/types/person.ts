import type { Role } from "$lib/types/enums";

export interface Person {
  id: number;
  firstName: string;
  lastName: string;
  email?: string;
  phoneNumber?: string;
  role?: Role;
  linkedinUrl?: string;
  createdAt: string;
  updatedAt: string;
}
