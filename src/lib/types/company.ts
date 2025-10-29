import type { BaseEntity } from "./baseType";
import type { WorkType } from "./enums";

export interface Company extends BaseEntity {
  name: string;
  streetAddress?: string;
  zipCode?: string;
  city?: string;
  country?: string;
  defaultWorkType?: WorkType;
  industry?: string;
  website?: string;
  phoneNumber?: string;
}
