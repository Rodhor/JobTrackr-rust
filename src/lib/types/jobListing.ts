import type { BaseEntity } from "./baseType";
import type { WorkType, SeniorityLevel, Currency } from "./enums";

export interface JobListing extends BaseEntity {
  companyId: number | undefined;
  title: string;
  workType?: WorkType;
  category?: string;
  seniorityLevel?: SeniorityLevel;
  salaryMin?: number;
  salaryMax?: number;
  currency?: Currency;
  description?: string;
  url?: string;
}
