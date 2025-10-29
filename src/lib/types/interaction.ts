import type { InteractionType } from "./enums";

export interface Interaction {
  id: number;
  interactionType: InteractionType;
  interactionDate: string;
  subject?: string;
  summary?: string;
  medium?: string;
  applicationId?: number;
  personId?: number;
  companyId?: number;
  createdAt: string;
  updatedAt: string;
}
