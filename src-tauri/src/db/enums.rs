use sqlx::Type;

#[derive(Type, Debug, Clone, PartialEq, Eq)]
#[sqlx(type_name = "TEXT")]
pub enum WorkType {
    #[sqlx(rename = "full_time")]
    FullTime,
    #[sqlx(rename = "part_time")]
    PartTime,
    #[sqlx(rename = "internship")]
    Internship,
    #[sqlx(rename = "contract")]
    Contract,
    #[sqlx(rename = "freelance")]
    Freelance,
    #[sqlx(rename = "other")]
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
            WorkType::Other => "other",
        }
    }
}

#[derive(Type, Debug, Clone, PartialEq, Eq)]
#[sqlx(type_name = "TEXT")]
pub enum SeniorityLevel {
    #[sqlx(rename = "junior")]
    Junior,
    #[sqlx(rename = "mid")]
    Mid,
    #[sqlx(rename = "senior")]
    Senior,
    #[sqlx(rename = "lead")]
    Lead,
    #[sqlx(rename = "manager")]
    Manager,
    #[sqlx(rename = "other")]
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
#[derive(Type, Debug, Clone, PartialEq, Eq)]
#[sqlx(type_name = "TEXT")]
pub enum Currency {
    #[sqlx(rename = "USD")]
    USD,
    #[sqlx(rename = "EUR")]
    EUR,
    #[sqlx(rename = "GBP")]
    GBP,
    #[sqlx(rename = "DKK")]
    DKK,
    #[sqlx(rename = "other")]
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

#[derive(Type, Debug, Clone, PartialEq, Eq)]
#[sqlx(type_name = "TEXT")]
pub enum Role {
    #[sqlx(rename = "recruiter")]
    Recruiter,
    #[sqlx(rename = "hr")]
    HR,
    #[sqlx(rename = "manager")]
    Manager,
    #[sqlx(rename = "other")]
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
#[derive(Type, Debug, Clone, PartialEq, Eq)]
#[sqlx(type_name = "TEXT")]
pub enum Status {
    #[sqlx(rename = "applied")]
    Applied,
    #[sqlx(rename = "interviewing")]
    Interviewing,
    #[sqlx(rename = "offered")]
    Offered,
    #[sqlx(rename = "rejected")]
    Rejected,
    #[sqlx(rename = "withdrawn")]
    Withdrawn,
    #[sqlx(rename = "other")]
    Other,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Applied => "applied",
            Status::Interviewing => "interviewing",
            Status::Offered => "offered",
            Status::Rejected => "rejected",
            Status::Withdrawn => "withdrawn",
            Status::Other => "other",
        }
    }
}
#[derive(Type, Debug, Clone, PartialEq, Eq)]
#[sqlx(type_name = "TEXT")]
pub enum ContactType {
    #[sqlx(rename = "phone")]
    Phone,
    #[sqlx(rename = "email")]
    Email,
    #[sqlx(rename = "in_person")]
    InPerson,
    #[sqlx(rename = "other")]
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
#[derive(Type, Debug, Clone, PartialEq, Eq)]
#[sqlx(type_name = "TEXT")]
pub enum NoteType {
    #[sqlx(rename = "before")]
    Before,
    #[sqlx(rename = "during")]
    During,
    #[sqlx(rename = "after")]
    After,
    #[sqlx(rename = "other")]
    Other,
}

impl NoteType {
    pub fn as_str(&self) -> &'static str {
        match self {
            NoteType::Before => "before",
            NoteType::During => "during",
            NoteType::After => "after",
            NoteType::Other => "other",
        }
    }
}
