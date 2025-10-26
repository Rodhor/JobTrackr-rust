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
            'freelance', 'remote', 'in_office', 'other'
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
            'freelance', 'remote', 'in_office','hybrid', 'other'
        )
    ),
    CHECK (
        seniority_level IN (
            'junior', 'mid', 'senior', 'lead', 'manager', 'other'
        )
    ),
    CHECK (currency IN ('USD', 'EUR', 'GBP', 'DKK', 'other'))
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
    applied_date DATE NOT NULL DEFAULT CURRENT_TIMESTAMP,
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
    contact_date DATE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    location TEXT,
    person_id INTEGER REFERENCES person(id) ON DELETE SET NULL,
    application_id INTEGER REFERENCES application(id) ON DELETE SET NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (
        contact_type IN ('phone', 'email', 'in_person', 'other')
    )
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
    CHECK (
        note_type IN ('general', 'feedback', 'reminder', 'other')
    )
);

-- ======================================================
-- Reminders
-- ======================================================
CREATE TABLE IF NOT EXISTS reminder (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    -- Optional links so a reminder can attach to any context
    application_id INTEGER REFERENCES application(id) ON DELETE SET NULL,
    contact_id INTEGER REFERENCES contact(id) ON DELETE SET NULL,
    note_id INTEGER REFERENCES note(id) ON DELETE SET NULL,
    job_listing_id INTEGER REFERENCES job_listing(id) ON DELETE SET NULL,
    company_id INTEGER REFERENCES company(id) ON DELETE SET NULL,
    person_id INTEGER REFERENCES person(id) ON DELETE SET NULL,
    reminder_date DATE NOT NULL,
    title TEXT,
    message TEXT,
    is_completed INTEGER NOT NULL DEFAULT 0, -- 0=false, 1=true
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- ======================================================
-- Indexes
-- ======================================================
CREATE INDEX IF NOT EXISTS idx_job_listing_company_id ON job_listing(company_id);
CREATE INDEX IF NOT EXISTS idx_application_job_listing_id ON application(job_listing_id);
CREATE INDEX IF NOT EXISTS idx_contact_person_id ON contact(person_id);
CREATE INDEX IF NOT EXISTS idx_note_application_id ON note(application_id);
CREATE INDEX IF NOT EXISTS idx_reminder_date ON reminder(reminder_date);
CREATE INDEX IF NOT EXISTS idx_reminder_completed ON reminder(is_completed);


-- ======================================================
-- Triggers to auto-update updated_at
-- ======================================================

-- Company
CREATE TRIGGER IF NOT EXISTS trg_company_updated_at
AFTER UPDATE ON company
FOR EACH ROW
BEGIN
    UPDATE company
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = OLD.id;
END;

-- Person
CREATE TRIGGER IF NOT EXISTS trg_person_updated_at
AFTER UPDATE ON person
FOR EACH ROW
BEGIN
    UPDATE person
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = OLD.id;
END;

-- Job Listing
CREATE TRIGGER IF NOT EXISTS trg_job_listing_updated_at
AFTER UPDATE ON job_listing
FOR EACH ROW
BEGIN
    UPDATE job_listing
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = OLD.id;
END;

-- Application
CREATE TRIGGER IF NOT EXISTS trg_application_updated_at
AFTER UPDATE ON application
FOR EACH ROW
BEGIN
    UPDATE application
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = OLD.id;
END;

-- Contact
CREATE TRIGGER IF NOT EXISTS trg_contact_updated_at
AFTER UPDATE ON contact
FOR EACH ROW
BEGIN
    UPDATE contact
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = OLD.id;
END;

-- Note
CREATE TRIGGER IF NOT EXISTS trg_note_updated_at
AFTER UPDATE ON note
FOR EACH ROW
BEGIN
    UPDATE note
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = OLD.id;
END;

-- Reminder
CREATE TRIGGER IF NOT EXISTS trg_reminder_updated_at
AFTER UPDATE ON reminder
FOR EACH ROW
BEGIN
    UPDATE reminder
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = OLD.id;
END;
