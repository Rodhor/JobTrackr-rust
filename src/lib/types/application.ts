import type { Status } from "./enums";

export interface Application {
  id: number;
  userId: number;
  jobListingId: number;
  status: Status;
  appliedDate: string;
  cvFilePath?: string;
  coverLetterFilePath?: string;
  applicationNotes?: string;
  createdAt: string;
  updatedAt: string;
}
