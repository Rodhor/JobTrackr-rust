import type { Role } from "./enums";

export interface JobListingContact {
  id: number;
  jobListingId: number;
  personId: number;
  role: Role;
  createdAt: string;
  updatedAt: string;
}
