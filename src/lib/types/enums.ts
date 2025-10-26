export type DefaultWorkType =
  | "full_time"
  | "part_time"
  | "internship"
  | "contract"
  | "freelance"
  | "remote"
  | "in_office"
  | "hybrid"
  | "other";

export type SeniorityLevel =
  | "junior"
  | "mid"
  | "senior"
  | "lead"
  | "manager"
  | "other";

export type Currency = "USD" | "EUR" | "GBP" | "DKK" | "other";

export type Role = "recruiter" | "hr" | "manager" | "other";

export type Stage =
  | "applied"
  | "screening"
  | "interviewing"
  | "offered"
  | "rejected"
  | "withdrawn"
  | "other";

export type ContactType = "phone" | "email" | "in_person" | "other";

export type NoteType = "general" | "feedback" | "reminder" | "other";
