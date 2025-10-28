-- ======================================================
-- DATABASE SCHEMA: Job Tracking / Application Manager
-- ======================================================

PRAGMA foreign_keys = ON;

-- ======================================================
-- Companies
-- ======================================================
CREATE TABLE IF NOT EXISTS company (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    street_address TEXT,
    zip_code TEXT,
    city TEXT,
    country TEXT,
    default_work_type TEXT,
    industry TEXT,
    website TEXT,
    phone_number TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (
        default_work_type IN (
            'full_time', 'part_time', 'internship', 'contract',
            'freelance', 'remote', 'in_office', 'hybrid', 'other'
        )
    )
);

-- ======================================================
-- Job Listings
-- ======================================================
CREATE TABLE IF NOT EXISTS job_listing (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    company_id INTEGER NOT NULL REFERENCES company(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    work_type TEXT,
    category TEXT,
    seniority_level TEXT,
    salary_min INTEGER,
    salary_max INTEGER,
    currency TEXT,
    description TEXT,
    url TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (
        work_type IN (
            'full_time', 'part_time', 'internship', 'contract',
            'freelance', 'remote', 'in_office', 'hybrid', 'other'
        )
    ),
    CHECK (
        seniority_level IN (
            'junior', 'mid', 'senior', 'lead', 'manager', 'other'
        )
    ),
    CHECK (currency IN ('USD', 'EUR', 'GBP', 'DKK', 'other')),
    CHECK (salary_min IS NULL OR salary_max IS NULL OR salary_min <= salary_max)
);

-- ======================================================
-- Persons
-- ======================================================
CREATE TABLE IF NOT EXISTS person (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT,
    phone_number TEXT,
    role TEXT,
    linkedin_url TEXT,
    company_id INTEGER REFERENCES company(id) ON DELETE SET NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (
        role IN (
            'recruiter', 'hiring_manager', 'team_lead',
            'hr', 'founder', 'developer', 'other'
        )
    )
);

-- ======================================================
-- Applications
-- ======================================================
CREATE TABLE IF NOT EXISTS application (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    job_listing_id INTEGER REFERENCES job_listing(id) ON DELETE SET NULL,
    stage TEXT NOT NULL DEFAULT 'applied',
    applied_date DATE NOT NULL DEFAULT (date('now')),
    application_notes TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (
        stage IN (
            'applied', 'screening', 'assessment', 'interviewing',
            'offered', 'negotiation', 'accepted', 'rejected',
            'withdrawn', 'on_hold', 'other'
        )
    )
);

-- ======================================================
-- Interactions
-- ======================================================
CREATE TABLE IF NOT EXISTS interaction (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    interaction_type TEXT NOT NULL, -- 'email', 'phone', 'meeting', etc.
    interaction_date TEXT NOT NULL DEFAULT (date('now')),
    subject TEXT,
    summary TEXT,
    medium TEXT, -- optional: 'video', 'in_person', etc.
    application_id INTEGER REFERENCES application(id) ON DELETE SET NULL,
    person_id INTEGER REFERENCES person(id) ON DELETE SET NULL,
    company_id INTEGER REFERENCES company(id) ON DELETE SET NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (
        interaction_type IN (
            'email', 'phone', 'interview', 'meeting',
            'follow_up', 'offer_discussion', 'other'
        )
    )
);


-- ======================================================
-- Notes
-- ======================================================
CREATE TABLE IF NOT EXISTS note (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    interaction_id INTEGER REFERENCES interaction(id) ON DELETE SET NULL,
    job_listing_id INTEGER REFERENCES job_listing(id) ON DELETE SET NULL,
    application_id INTEGER REFERENCES application(id) ON DELETE SET NULL,
    person_id INTEGER REFERENCES person(id) ON DELETE SET NULL,
    company_id INTEGER REFERENCES company(id) ON DELETE SET NULL,
    note_type TEXT DEFAULT 'general',
    title TEXT,
    content TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (note_type IN ('general', 'feedback', 'reminder', 'summary', 'other'))
);


-- ======================================================
-- Reminders
-- ======================================================
CREATE TABLE IF NOT EXISTS reminder (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    message TEXT,
    reminder_date TEXT NOT NULL,
    is_completed INTEGER NOT NULL DEFAULT 0,
    application_id INTEGER REFERENCES application(id) ON DELETE SET NULL,
    job_listing_id INTEGER REFERENCES job_listing(id) ON DELETE SET NULL,
    interaction_id INTEGER REFERENCES interaction(id) ON DELETE SET NULL,
    note_id INTEGER REFERENCES note(id) ON DELETE SET NULL,
    company_id INTEGER REFERENCES company(id) ON DELETE SET NULL,
    person_id INTEGER REFERENCES person(id) ON DELETE SET NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (is_completed IN (0, 1))
);

-- ======================================================
-- Indexes
-- ======================================================

-- Company
CREATE INDEX IF NOT EXISTS idx_company_name ON company(name);

-- Person
CREATE INDEX IF NOT EXISTS idx_person_company_id ON person(company_id);

-- Job Listing
CREATE INDEX IF NOT EXISTS idx_job_listing_company_id ON job_listing(company_id);

-- Application
CREATE INDEX IF NOT EXISTS idx_application_job_listing_id ON application(job_listing_id);

-- Interaction
CREATE INDEX IF NOT EXISTS idx_interaction_person_id ON interaction(person_id);
CREATE INDEX IF NOT EXISTS idx_interaction_application_id ON interaction(application_id);
CREATE INDEX IF NOT EXISTS idx_interaction_date ON interaction(interaction_date);
CREATE INDEX IF NOT EXISTS idx_interaction_company_id ON interaction(company_id);

-- Note
CREATE INDEX IF NOT EXISTS idx_note_interaction_id ON note(interaction_id);
CREATE INDEX IF NOT EXISTS idx_note_job_listing_id ON note(job_listing_id);
CREATE INDEX IF NOT EXISTS idx_note_person_id ON note(person_id);
CREATE INDEX IF NOT EXISTS idx_note_company_id ON note(company_id);
CREATE INDEX IF NOT EXISTS idx_note_application_id ON note(application_id);

-- Reminder
CREATE INDEX IF NOT EXISTS idx_reminder_job_listing_id ON reminder(job_listing_id);
CREATE INDEX IF NOT EXISTS idx_reminder_date ON reminder(reminder_date);
CREATE INDEX IF NOT EXISTS idx_reminder_completed ON reminder(is_completed);
CREATE INDEX IF NOT EXISTS idx_reminder_app_id ON reminder(application_id);
CREATE INDEX IF NOT EXISTS idx_reminder_interaction_id ON reminder(interaction_id);
CREATE INDEX IF NOT EXISTS idx_reminder_note_id ON reminder(note_id);
CREATE INDEX IF NOT EXISTS idx_reminder_company_id ON reminder(company_id);
CREATE INDEX IF NOT EXISTS idx_reminder_person_id ON reminder(person_id);
