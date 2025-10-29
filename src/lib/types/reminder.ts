import type { BaseEntity } from "./baseType";

export interface Reminder extends BaseEntity {
  title: string;
  message?: string;
  reminderDate: string;
  isCompleted: boolean;
  applicationId?: number;
  jobListingId?: number;
  interactionId?: number;
  noteId?: number;
  companyId?: number;
  personId?: number;
}
