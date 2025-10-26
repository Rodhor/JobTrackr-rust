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
    #[sqlx(rename = "hr")]
    #[serde(rename = "hr")]
    HR,
    #[sqlx(rename = "manager")]
    #[serde(rename = "manager")]
    Manager,
    #[sqlx(rename = "other")]
    #[serde(rename = "other")]
    Other,
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Recruiter => "recruiter",
            Role::HR => "hr",
            Role::Manager => "manager",
            Role::Other => "other",
        }
    }
}

// ======================================================
// Application Stage (formerly Status)
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
    #[sqlx(rename = "interviewing")]
    #[serde(rename = "interviewing")]
    Interviewing,
    #[sqlx(rename = "offered")]
    #[serde(rename = "offered")]
    Offered,
    #[sqlx(rename = "rejected")]
    #[serde(rename = "rejected")]
    Rejected,
    #[sqlx(rename = "withdrawn")]
    #[serde(rename = "withdrawn")]
    Withdrawn,
    #[sqlx(rename = "other")]
    #[serde(rename = "other")]
    Other,
}

impl Stage {
    pub fn as_str(&self) -> &'static str {
        match self {
            Stage::Applied => "applied",
            Stage::Screening => "screening",
            Stage::Interviewing => "interviewing",
            Stage::Offered => "offered",
            Stage::Rejected => "rejected",
            Stage::Withdrawn => "withdrawn",
            Stage::Other => "other",
        }
    }
}

// ======================================================
// Contact Type
// ======================================================
#[derive(Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "TEXT")]
pub enum ContactType {
    #[sqlx(rename = "phone")]
    #[serde(rename = "phone")]
    Phone,
    #[sqlx(rename = "email")]
    #[serde(rename = "email")]
    Email,
    #[sqlx(rename = "in_person")]
    #[serde(rename = "in_person")]
    InPerson,
    #[sqlx(rename = "other")]
    #[serde(rename = "other")]
    Other,
}

impl ContactType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContactType::Phone => "phone",
            ContactType::Email => "email",
            ContactType::InPerson => "in_person",
            ContactType::Other => "other",
        }
    }
}

// ======================================================
// Note Type (aligned with schema: general, feedback, reminder, other)
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
