use serde::{Deserialize, Serialize};
use sqlx::Type;

// ======================================================
// Work Type
// ======================================================
#[derive(Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "TEXT")]
pub enum WorkType {
    #[sqlx(rename = "full_time")]
    #[serde(rename = "full_time")]
    FullTime,
    #[sqlx(rename = "part_time")]
    #[serde(rename = "part_time")]
    PartTime,
    #[sqlx(rename = "internship")]
    #[serde(rename = "internship")]
    Internship,
    #[sqlx(rename = "contract")]
    #[serde(rename = "contract")]
    Contract,
    #[sqlx(rename = "freelance")]
    #[serde(rename = "freelance")]
    Freelance,
    #[sqlx(rename = "remote")]
    #[serde(rename = "remote")]
    Remote,
    #[sqlx(rename = "in_office")]
    #[serde(rename = "in_office")]
    InOffice,
    #[sqlx(rename = "hybrid")]
    #[serde(rename = "hybrid")]
    Hybrid,
    #[sqlx(rename = "other")]
    #[serde(rename = "other")]
    Other,
}

impl WorkType {
    pub fn as_str(&self) -> &'static str {
        match self {
            WorkType::FullTime => "full_time",
            WorkType::PartTime => "part_time",
            WorkType::Internship => "internship",
            WorkType::Contract => "contract",
            WorkType::Freelance => "freelance",
            WorkType::Remote => "remote",
            WorkType::InOffice => "in_office",
            WorkType::Hybrid => "hybrid",
            WorkType::Other => "other",
        }
    }
}

// ======================================================
// Seniority Level
// ======================================================
#[derive(Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "TEXT")]
pub enum SeniorityLevel {
    #[sqlx(rename = "junior")]
    #[serde(rename = "junior")]
    Junior,
    #[sqlx(rename = "mid")]
    #[serde(rename = "mid")]
    Mid,
    #[sqlx(rename = "senior")]
    #[serde(rename = "senior")]
    Senior,
    #[sqlx(rename = "lead")]
    #[serde(rename = "lead")]
    Lead,
    #[sqlx(rename = "manager")]
    #[serde(rename = "manager")]
    Manager,
    #[sqlx(rename = "other")]
    #[serde(rename = "other")]
    Other,
}

impl SeniorityLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            SeniorityLevel::Junior => "junior",
            SeniorityLevel::Mid => "mid",
            SeniorityLevel::Senior => "senior",
            SeniorityLevel::Lead => "lead",
            SeniorityLevel::Manager => "manager",
            SeniorityLevel::Other => "other",
        }
    }
}

// ======================================================
// Currency
// ======================================================
#[derive(Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "TEXT")]
pub enum Currency {
    #[sqlx(rename = "USD")]
    #[serde(rename = "USD")]
    USD,
    #[sqlx(rename = "EUR")]
    #[serde(rename = "EUR")]
    EUR,
    #[sqlx(rename = "GBP")]
    #[serde(rename = "GBP")]
    GBP,
    #[sqlx(rename = "DKK")]
    #[serde(rename = "DKK")]
    DKK,
    #[sqlx(rename = "other")]
    #[serde(rename = "other")]
    Other,
}

impl Currency {
    pub fn as_str(&self) -> &'static str {
        match self {
            Currency::USD => "USD",
            Currency::EUR => "EUR",
            Currency::GBP => "GBP",
            Currency::DKK => "DKK",
            Currency::Other => "other",
        }
    }
}

// ======================================================
// Role (Person)
// ======================================================
#[derive(Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "TEXT")]
pub enum Role {
    #[sqlx(rename = "recruiter")]
    #[serde(rename = "recruiter")]
    Recruiter,
    #[sqlx(rename = "hiring_manager")]
    #[serde(rename = "hiring_manager")]
    HiringManager,
    #[sqlx(rename = "team_lead")]
    #[serde(rename = "team_lead")]
    TeamLead,
    #[sqlx(rename = "hr")]
    #[serde(rename = "hr")]
    HR,
    #[sqlx(rename = "founder")]
    #[serde(rename = "founder")]
    Founder,
    #[sqlx(rename = "developer")]
    #[serde(rename = "developer")]
    Developer,
    #[sqlx(rename = "other")]
    #[serde(rename = "other")]
    Other,
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Recruiter => "recruiter",
            Role::HiringManager => "hiring_manager",
            Role::TeamLead => "team_lead",
            Role::HR => "hr",
            Role::Founder => "founder",
            Role::Developer => "developer",
            Role::Other => "other",
        }
    }
}

// ======================================================
// Application Stage
// ======================================================
#[derive(Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "TEXT")]
pub enum Stage {
    #[sqlx(rename = "applied")]
    #[serde(rename = "applied")]
    Applied,
    #[sqlx(rename = "screening")]
    #[serde(rename = "screening")]
    Screening,
    #[sqlx(rename = "assessment")]
    #[serde(rename = "assessment")]
    Assessment,
    #[sqlx(rename = "interviewing")]
    #[serde(rename = "interviewing")]
    Interviewing,
    #[sqlx(rename = "offered")]
    #[serde(rename = "offered")]
    Offered,
    #[sqlx(rename = "negotiation")]
    #[serde(rename = "negotiation")]
    Negotiation,
    #[sqlx(rename = "accepted")]
    #[serde(rename = "accepted")]
    Accepted,
    #[sqlx(rename = "rejected")]
    #[serde(rename = "rejected")]
    Rejected,
    #[sqlx(rename = "withdrawn")]
    #[serde(rename = "withdrawn")]
    Withdrawn,
    #[sqlx(rename = "on_hold")]
    #[serde(rename = "on_hold")]
    OnHold,
    #[sqlx(rename = "other")]
    #[serde(rename = "other")]
    Other,
}

impl Stage {
    pub fn as_str(&self) -> &'static str {
        match self {
            Stage::Applied => "applied",
            Stage::Screening => "screening",
            Stage::Assessment => "assessment",
            Stage::Interviewing => "interviewing",
            Stage::Offered => "offered",
            Stage::Negotiation => "negotiation",
            Stage::Accepted => "accepted",
            Stage::Rejected => "rejected",
            Stage::Withdrawn => "withdrawn",
            Stage::OnHold => "on_hold",
            Stage::Other => "other",
        }
    }
}

// ======================================================
// Interaction Type
// ======================================================
#[derive(Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "TEXT")]
pub enum InteractionType {
    #[sqlx(rename = "email")]
    #[serde(rename = "email")]
    Email,
    #[sqlx(rename = "phone")]
    #[serde(rename = "phone")]
    Phone,
    #[sqlx(rename = "interview")]
    #[serde(rename = "interview")]
    Interview,
    #[sqlx(rename = "meeting")]
    #[serde(rename = "meeting")]
    Meeting,
    #[sqlx(rename = "follow_up")]
    #[serde(rename = "follow_up")]
    FollowUp,
    #[sqlx(rename = "offer_discussion")]
    #[serde(rename = "offer_discussion")]
    OfferDiscussion,
    #[sqlx(rename = "other")]
    #[serde(rename = "other")]
    Other,
}

impl InteractionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            InteractionType::Email => "email",
            InteractionType::Phone => "phone",
            InteractionType::Interview => "interview",
            InteractionType::Meeting => "meeting",
            InteractionType::FollowUp => "follow_up",
            InteractionType::OfferDiscussion => "offer_discussion",
            InteractionType::Other => "other",
        }
    }
}

// ======================================================
// Note Type
// ======================================================
#[derive(Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "TEXT")]
pub enum NoteType {
    #[sqlx(rename = "general")]
    #[serde(rename = "general")]
    General,
    #[sqlx(rename = "feedback")]
    #[serde(rename = "feedback")]
    Feedback,
    #[sqlx(rename = "reminder")]
    #[serde(rename = "reminder")]
    Reminder,
    #[sqlx(rename = "summary")]
    #[serde(rename = "summary")]
    Summary,
    #[sqlx(rename = "other")]
    #[serde(rename = "other")]
    Other,
}

impl NoteType {
    pub fn as_str(&self) -> &'static str {
        match self {
            NoteType::General => "general",
            NoteType::Feedback => "feedback",
            NoteType::Reminder => "reminder",
            NoteType::Summary => "summary",
            NoteType::Other => "other",
        }
    }
}

// ======================================================
// Reminder Completion Status
// ======================================================
#[derive(Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "INTEGER")]
pub enum ReminderStatus {
    #[sqlx(rename = "0")]
    #[serde(rename = "0")]
    Pending,
    #[sqlx(rename = "1")]
    #[serde(rename = "1")]
    Completed,
}

impl ReminderStatus {
    pub fn as_int(&self) -> i32 {
        match self {
            ReminderStatus::Pending => 0,
            ReminderStatus::Completed => 1,
        }
    }
}
