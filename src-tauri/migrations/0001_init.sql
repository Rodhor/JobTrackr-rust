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
    CHECK (role IN ('recruiter', 'hr', 'manager', 'other'))
);

-- ======================================================
-- Applications
-- ======================================================
CREATE TABLE IF NOT EXISTS application (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    job_listing_id INTEGER REFERENCES job_listing(id) ON DELETE SET NULL,
    stage TEXT,
    applied_date TEXT NOT NULL DEFAULT (date('now')),
    cv_file_path TEXT,
    cover_letter_file_path TEXT,
    application_notes TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (
        stage IN (
            'applied', 'screening', 'interviewing', 'offered',
            'rejected', 'withdrawn', 'other'
        )
    )
);

-- ======================================================
-- Contacts
-- ======================================================
CREATE TABLE IF NOT EXISTS contact (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    contact_type TEXT NOT NULL,
    contact_date TEXT NOT NULL DEFAULT (date('now')),
    location TEXT,
    person_id INTEGER REFERENCES person(id) ON DELETE SET NULL,
    application_id INTEGER REFERENCES application(id) ON DELETE SET NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (contact_type IN ('phone', 'email', 'in_person', 'other'))
);

-- ======================================================
-- Notes
-- ======================================================
CREATE TABLE IF NOT EXISTS note (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    contact_id INTEGER REFERENCES contact(id) ON DELETE SET NULL,
    job_listing_id INTEGER REFERENCES job_listing(id) ON DELETE SET NULL,
    application_id INTEGER REFERENCES application(id) ON DELETE SET NULL,
    person_id INTEGER REFERENCES person(id) ON DELETE SET NULL,
    company_id INTEGER REFERENCES company(id) ON DELETE SET NULL,
    note_type TEXT DEFAULT 'general',
    content TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (note_type IN ('general', 'feedback', 'reminder', 'other'))
);

-- ======================================================
-- Reminders
-- ======================================================
CREATE TABLE IF NOT EXISTS reminder (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    application_id INTEGER REFERENCES application(id) ON DELETE SET NULL,
    contact_id INTEGER REFERENCES contact(id) ON DELETE SET NULL,
    note_id INTEGER REFERENCES note(id) ON DELETE SET NULL,
    job_listing_id INTEGER REFERENCES job_listing(id) ON DELETE SET NULL,
    company_id INTEGER REFERENCES company(id) ON DELETE SET NULL,
    person_id INTEGER REFERENCES person(id) ON DELETE SET NULL,
    reminder_date TEXT NOT NULL,
    title TEXT,
    message TEXT,
    is_completed INTEGER NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (is_completed IN (0,1))
);

-- ======================================================
-- Indexes
-- ======================================================
CREATE INDEX IF NOT EXISTS idx_company_name ON company(name);
CREATE INDEX IF NOT EXISTS idx_person_company_id ON person(company_id);

CREATE INDEX IF NOT EXISTS idx_job_listing_company_id ON job_listing(company_id);

CREATE INDEX IF NOT EXISTS idx_application_job_listing_id ON application(job_listing_id);

CREATE INDEX IF NOT EXISTS idx_contact_person_id ON contact(person_id);
CREATE INDEX IF NOT EXISTS idx_contact_application_id ON contact(application_id);
CREATE INDEX IF NOT EXISTS idx_contact_date ON contact(contact_date);

CREATE INDEX IF NOT EXISTS idx_note_contact_id ON note(contact_id);
CREATE INDEX IF NOT EXISTS idx_note_job_listing_id ON note(job_listing_id);
CREATE INDEX IF NOT EXISTS idx_note_person_id ON note(person_id);
CREATE INDEX IF NOT EXISTS idx_note_company_id ON note(company_id);
CREATE INDEX IF NOT EXISTS idx_note_application_id ON note(application_id);

CREATE INDEX IF NOT EXISTS idx_reminder_date ON reminder(reminder_date);
CREATE INDEX IF NOT EXISTS idx_reminder_completed ON reminder(is_completed);
CREATE INDEX IF NOT EXISTS idx_reminder_app_id ON reminder(application_id);
CREATE INDEX IF NOT EXISTS idx_reminder_contact_id ON reminder(contact_id);
CREATE INDEX IF NOT EXISTS idx_reminder_note_id ON reminder(note_id);
CREATE INDEX IF NOT EXISTS idx_reminder_job_listing_id ON reminder(job_listing_id);
CREATE INDEX IF NOT EXISTS idx_reminder_company_id ON reminder(company_id);
CREATE INDEX IF NOT EXISTS idx_reminder_person_id ON reminder(person_id);
