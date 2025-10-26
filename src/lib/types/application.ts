import type { Stage } from "./enums";

export interface Application {
  id: number;
  jobListingId?: number;
  stage: Stage;
  appliedDate: string;
  cvFilePath?: string;
  coverLetterFilePath?: string;
  applicationNotes?: string;
  createdAt: string;
  updatedAt: string;
}
