import type { WorkType } from "./enums";

export interface Company {
  id: number;
  name: string;
  streetAddress?: string;
  zipCode?: string;
  city?: string;
  country?: string;
  defaultWorkType?: WorkType;
  industry?: string;
  website?: string;
  phoneNumber?: string;
  createdAt: string;
  updatedAt: string;
}
