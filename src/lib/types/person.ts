import type { BaseEntity } from "./baseType";
import type { Role } from "./enums";

export interface Person extends BaseEntity {
  firstName: string;
  lastName: string;
  email?: string;
  phoneNumber?: string;
  role?: Role;
  linkedinUrl?: string;
  companyId?: number;
}
