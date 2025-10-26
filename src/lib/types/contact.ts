import type { ContactType } from "./enums";

export interface Contact {
  id: number;
  contactType: ContactType;
  contactDate: string;
  location?: string;
  personId?: number;
  applicationId?: number;
  createdAt: string;
  updatedAt: string;
}
