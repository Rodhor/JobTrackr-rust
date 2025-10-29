import type { NoteType } from "./enums";

export interface Note {
  id: number;
  interactionId?: number;
  jobListingId?: number;
  applicationId?: number;
  personId?: number;
  companyId?: number;
  noteType?: NoteType;
  title?: string;
  content?: string;
  createdAt: string;
  updatedAt: string;
}
