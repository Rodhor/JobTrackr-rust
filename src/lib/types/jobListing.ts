import type { Currency, DefaultWorkType, SeniorityLevel } from "./enums";

export interface JobListing {
  id: number;
  companyId: number;
  title: string;
  workType?: DefaultWorkType;
  category?: string;
  seniorityLevel?: SeniorityLevel;
  salaryMin?: number;
  salaryMax?: number;
  currency?: Currency;
  description?: string;
  url?: string;
  createdAt: string;
  updatedAt: string;
}
