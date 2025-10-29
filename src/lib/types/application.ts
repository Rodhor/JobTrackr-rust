import type { BaseEntity } from "./baseType";
import type { Stage } from "./enums";

export interface Application extends BaseEntity {
  jobListingId?: number;
  stage: Stage;
  appliedDate: string;
  applicationNotes?: string;
}
