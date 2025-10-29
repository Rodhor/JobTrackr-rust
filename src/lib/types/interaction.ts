import type { BaseEntity } from "./baseType";
import type { InteractionType } from "./enums";

export interface Interaction extends BaseEntity {
  interactionType: InteractionType;
  interactionDate: string;
  subject?: string;
  summary?: string;
  medium?: string;
  applicationId?: number;
  personId?: number;
  companyId?: number;
}
