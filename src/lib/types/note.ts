import type { BaseEntity } from "./baseType";
import type { NoteType } from "./enums";

export interface Note extends BaseEntity {
  interactionId?: number;
  jobListingId?: number;
  applicationId?: number;
  personId?: number;
  companyId?: number;
  noteType?: NoteType;
  title?: string;
  content?: string;
}
