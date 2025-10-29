// ======================================================
// ENUMS + DISPLAY MAPS
// ======================================================

export enum WorkType {
  FullTime = "full_time",
  PartTime = "part_time",
  Internship = "internship",
  Contract = "contract",
  Freelance = "freelance",
  Remote = "remote",
  InOffice = "in_office",
  Hybrid = "hybrid",
  Other = "other",
}

export const WorkTypeDisplay: Record<WorkType, string> = {
  [WorkType.FullTime]: "Full time",
  [WorkType.PartTime]: "Part time",
  [WorkType.Internship]: "Internship",
  [WorkType.Contract]: "Contract",
  [WorkType.Freelance]: "Freelance",
  [WorkType.Remote]: "Remote",
  [WorkType.InOffice]: "In office",
  [WorkType.Hybrid]: "Hybrid",
  [WorkType.Other]: "Other",
};

// ======================================================

export enum SeniorityLevel {
  Junior = "junior",
  Mid = "mid",
  Senior = "senior",
  Lead = "lead",
  Manager = "manager",
  Other = "other",
}

export const SeniorityLevelDisplay: Record<SeniorityLevel, string> = {
  [SeniorityLevel.Junior]: "Junior",
  [SeniorityLevel.Mid]: "Mid level",
  [SeniorityLevel.Senior]: "Senior",
  [SeniorityLevel.Lead]: "Lead",
  [SeniorityLevel.Manager]: "Manager",
  [SeniorityLevel.Other]: "Other",
};

// ======================================================

export enum Currency {
  USD = "USD",
  EUR = "EUR",
  GBP = "GBP",
  DKK = "DKK",
  Other = "other",
}

export const CurrencyDisplay: Record<Currency, string> = {
  [Currency.USD]: "USD ($)",
  [Currency.EUR]: "EUR (€)",
  [Currency.GBP]: "GBP (£)",
  [Currency.DKK]: "DKK (kr)",
  [Currency.Other]: "Other",
};

// ======================================================

export enum Role {
  Recruiter = "recruiter",
  HiringManager = "hiring_manager",
  TeamLead = "team_lead",
  HR = "hr",
  Founder = "founder",
  Developer = "developer",
  Other = "other",
}

export const RoleDisplay: Record<Role, string> = {
  [Role.Recruiter]: "Recruiter",
  [Role.HiringManager]: "Hiring manager",
  [Role.TeamLead]: "Team lead",
  [Role.HR]: "HR",
  [Role.Founder]: "Founder",
  [Role.Developer]: "Developer",
  [Role.Other]: "Other",
};

// ======================================================

export enum Stage {
  Applied = "applied",
  Screening = "screening",
  Assessment = "assessment",
  Interviewing = "interviewing",
  Offered = "offered",
  Negotiation = "negotiation",
  Accepted = "accepted",
  Rejected = "rejected",
  Withdrawn = "withdrawn",
  OnHold = "on_hold",
  Other = "other",
}

export const StageDisplay: Record<Stage, string> = {
  [Stage.Applied]: "Applied",
  [Stage.Screening]: "Screening",
  [Stage.Assessment]: "Assessment",
  [Stage.Interviewing]: "Interviewing",
  [Stage.Offered]: "Offered",
  [Stage.Negotiation]: "Negotiation",
  [Stage.Accepted]: "Accepted",
  [Stage.Rejected]: "Rejected",
  [Stage.Withdrawn]: "Withdrawn",
  [Stage.OnHold]: "On hold",
  [Stage.Other]: "Other",
};

// ======================================================

export enum InteractionType {
  Email = "email",
  Phone = "phone",
  Interview = "interview",
  Meeting = "meeting",
  FollowUp = "follow_up",
  OfferDiscussion = "offer_discussion",
  Other = "other",
}

export const InteractionTypeDisplay: Record<InteractionType, string> = {
  [InteractionType.Email]: "Email",
  [InteractionType.Phone]: "Phone",
  [InteractionType.Interview]: "Interview",
  [InteractionType.Meeting]: "Meeting",
  [InteractionType.FollowUp]: "Follow up",
  [InteractionType.OfferDiscussion]: "Offer discussion",
  [InteractionType.Other]: "Other",
};

// ======================================================

export enum NoteType {
  General = "general",
  Feedback = "feedback",
  Reminder = "reminder",
  Summary = "summary",
  Other = "other",
}

export const NoteTypeDisplay: Record<NoteType, string> = {
  [NoteType.General]: "General",
  [NoteType.Feedback]: "Feedback",
  [NoteType.Reminder]: "Reminder",
  [NoteType.Summary]: "Summary",
  [NoteType.Other]: "Other",
};
