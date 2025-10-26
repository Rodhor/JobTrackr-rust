import type { NoteType } from "./enums";

export interface Note {
  id: number;
  contactId?: number;
  jobListingId?: number;
  applicationId?: number;
  personId?: number;
  companyId?: number;
  noteType?: NoteType;
  content?: string;
  createAt: string;
  updatedAt: string;
}
