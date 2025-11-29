// ======================================================
// ENUMS + DISPLAY MAPS
// ======================================================

export const WorkType = {
  FullTime: "full_time",
  PartTime: "part_time",
  Internship: "internship",
  Contract: "contract",
  Freelance: "freelance",
  Remote: "remote",
  InOffice: "in_office",
  Hybrid: "hybrid",
  Other: "other",
} as const;
export type WorkType = (typeof WorkType)[keyof typeof WorkType];

export const WorkTypeDisplay: Record<WorkType, string> = {
  full_time: "Full time",
  part_time: "Part time",
  internship: "Internship",
  contract: "Contract",
  freelance: "Freelance",
  remote: "Remote",
  in_office: "In office",
  hybrid: "Hybrid",
  other: "Other",
};

// ======================================================

export const SeniorityLevel = {
  Junior: "junior",
  Mid: "mid",
  Senior: "senior",
  Lead: "lead",
  Manager: "manager",
  Other: "other",
} as const;
export type SeniorityLevel =
  (typeof SeniorityLevel)[keyof typeof SeniorityLevel];

export const SeniorityLevelDisplay: Record<SeniorityLevel, string> = {
  junior: "Junior",
  mid: "Mid level",
  senior: "Senior",
  lead: "Lead",
  manager: "Manager",
  other: "Other",
};

// ======================================================

export const Currency = {
  USD: "USD",
  EUR: "EUR",
  GBP: "GBP",
  DKK: "DKK",
  Other: "other",
} as const;
export type Currency = (typeof Currency)[keyof typeof Currency];

export const CurrencyDisplay: Record<Currency, string> = {
  USD: "USD ($)",
  EUR: "EUR (€)",
  GBP: "GBP (£)",
  DKK: "DKK (kr)",
  other: "Other",
};

// ======================================================

export const Role = {
  Recruiter: "recruiter",
  HiringManager: "hiring_manager",
  TeamLead: "team_lead",
  HR: "hr",
  Founder: "founder",
  Developer: "developer",
  Other: "other",
} as const;
export type Role = (typeof Role)[keyof typeof Role];

export const RoleDisplay: Record<Role, string> = {
  recruiter: "Recruiter",
  hiring_manager: "Hiring manager",
  team_lead: "Team lead",
  hr: "HR",
  founder: "Founder",
  developer: "Developer",
  other: "Other",
};

// ======================================================

export const Stage = {
  Applied: "applied",
  Screening: "screening",
  Assessment: "assessment",
  Interviewing: "interviewing",
  Offered: "offered",
  Negotiation: "negotiation",
  Accepted: "accepted",
  Rejected: "rejected",
  Withdrawn: "withdrawn",
  OnHold: "on_hold",
  Other: "other",
} as const;
export type Stage = (typeof Stage)[keyof typeof Stage];

export const StageDisplay: Record<Stage, string> = {
  applied: "Applied",
  screening: "Screening",
  assessment: "Assessment",
  interviewing: "Interviewing",
  offered: "Offered",
  negotiation: "Negotiation",
  accepted: "Accepted",
  rejected: "Rejected",
  withdrawn: "Withdrawn",
  on_hold: "On hold",
  other: "Other",
};

// ======================================================

export const InteractionType = {
  Email: "email",
  Phone: "phone",
  Interview: "interview",
  Meeting: "meeting",
  FollowUp: "follow_up",
  OfferDiscussion: "offer_discussion",
  Other: "other",
} as const;
export type InteractionType =
  (typeof InteractionType)[keyof typeof InteractionType];

export const InteractionTypeDisplay: Record<InteractionType, string> = {
  email: "Email",
  phone: "Phone",
  interview: "Interview",
  meeting: "Meeting",
  follow_up: "Follow up",
  offer_discussion: "Offer discussion",
  other: "Other",
};

// ======================================================

export const NoteType = {
  General: "general",
  Feedback: "feedback",
  Reminder: "reminder",
  Summary: "summary",
  Other: "other",
} as const;
export type NoteType = (typeof NoteType)[keyof typeof NoteType];

export const NoteTypeDisplay: Record<NoteType, string> = {
  general: "General",
  feedback: "Feedback",
  reminder: "Reminder",
  summary: "Summary",
  other: "Other",
};
