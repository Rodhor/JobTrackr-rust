import type { Role } from "./enums";

export interface Person {
  id: number;
  firstName: string;
  lastName: string;
  email?: string;
  phoneNumber?: string;
  role?: Role;
  linkedinUrl?: string;
  companyId?: number;
  createdAt: string;
  updatedAt: string;
}
