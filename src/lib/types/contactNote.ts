import type { NoteType } from "./enums";

export interface ContactNote {
  id: number;
  contactId: number;
  noteType?: NoteType;
  content?: string;
  createdAt: string;
  updatedAt: string;
}
