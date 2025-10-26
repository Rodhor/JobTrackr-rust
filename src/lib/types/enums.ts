export type DefaultWorkType =
  | "full_time"
  | "part_time"
  | "internship"
  | "contract"
  | "freelance"
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

export type Status =
  | "applied"
  | "interviewing"
  | "offered"
  | "rejected"
  | "withdrawn"
  | "other";

export type ContactType = "phone" | "email" | "in_person" | "other";

export type NoteType = "before" | "during" | "after" | "other";
