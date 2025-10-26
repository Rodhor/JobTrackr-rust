import type { ContactType } from "./enums";

export interface Contact {
  id: number;
  contactType: ContactType;
  contactDate: string;
  location?: string;
  userId: number;
  personId?: number;
  createdAt: string;
  updatedAt: string;
}
