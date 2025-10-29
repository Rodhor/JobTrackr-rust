import type { Stage } from "./enums";

export interface Application {
  id: number;
  jobListingId?: number;
  stage: Stage;
  appliedDate: string;
  applicationNotes?: string;
  createdAt: string;
  updatedAt: string;
}
