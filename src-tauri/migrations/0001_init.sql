-- ======================================================
-- DATABASE SCHEMA: Job Tracking / Application Manager
-- ======================================================

-- Enable Foreign Keys
PRAGMA foreign_keys = ON;

-- ======================================================
-- 1. Users Table
-- ======================================================
CREATE TABLE IF NOT EXISTS user (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    phone_number TEXT,
    street_address TEXT,
    zip_code TEXT,
    city TEXT,
    country TEXT,
    created_at TEXT NOT NULL DEFAULT current_timestamp,
    updated_at TEXT NOT NULL DEFAULT current_timestamp
);

-- ======================================================
-- 2. Companies Table
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
    created_at TEXT NOT NULL DEFAULT current_timestamp,
    updated_at TEXT NOT NULL DEFAULT current_timestamp,
    CHECK (
        default_work_type IN (
            'full_time',
            'part_time',
            'internship',
            'contract',
            'freelance',
            'other'
        )
    )
);

-- ======================================================
-- 3. Job Listings Table
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
    created_at TEXT NOT NULL DEFAULT current_timestamp,
    updated_at TEXT NOT NULL DEFAULT current_timestamp,
    CHECK (
        work_type IN (
            'full_time',
            'part_time',
            'internship',
            'contract',
            'freelance',
            'other'
        )
    ),
    CHECK (
        seniority_level IN (
            'junior',
            'mid',
            'senior',
            'lead',
            'manager',
            'other'
        )
    ),
    CHECK (
        currency IN ('USD', 'EUR', 'GBP', 'DKK', 'other')
    )
);

-- ======================================================
-- 4. Persons Table
-- ======================================================
CREATE TABLE IF NOT EXISTS person (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT,
    phone_number TEXT,
    role TEXT,
    linkedin_url TEXT,
    created_at TEXT NOT NULL DEFAULT current_timestamp,
    updated_at TEXT NOT NULL DEFAULT current_timestamp
);

-- ======================================================
-- 5. Job Listing Contacts Table
-- ======================================================
CREATE TABLE IF NOT EXISTS job_listing_contact (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    job_listing_id INTEGER NOT NULL REFERENCES job_listing(id) ON DELETE CASCADE,
    person_id INTEGER NOT NULL REFERENCES person(id) ON DELETE CASCADE,
    role TEXT,
    created_at TEXT NOT NULL DEFAULT current_timestamp,
    updated_at TEXT NOT NULL DEFAULT current_timestamp,
    CHECK (role IN ('recruiter', 'hr', 'manager', 'other'))
);

-- ======================================================
-- 6. Applications Table
-- ======================================================
CREATE TABLE IF NOT EXISTS application (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES user(id) ON DELETE CASCADE,
    job_listing_id INTEGER NOT NULL REFERENCES job_listing(id) ON DELETE CASCADE,
    status TEXT,
    applied_date TEXT NOT NULL DEFAULT current_timestamp,
    cv_file_path TEXT,
    cover_letter_file_path TEXT,
    application_notes TEXT,
    created_at TEXT NOT NULL DEFAULT current_timestamp,
    updated_at TEXT NOT NULL DEFAULT current_timestamp,
    CHECK (
        status IN (
            'applied',
            'interviewing',
            'offered',
            'rejected',
            'withdrawn',
            'other'
        )
    )
);

-- ======================================================
-- 7. Contacts Table
-- ======================================================
CREATE TABLE IF NOT EXISTS contact (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    contact_type TEXT NOT NULL,
    contact_date TEXT NOT NULL DEFAULT current_timestamp,
    location TEXT,
    user_id INTEGER NOT NULL REFERENCES user(id) ON DELETE CASCADE,
    person_id INTEGER REFERENCES person(id) ON DELETE CASCADE,
    created_at TEXT NOT NULL DEFAULT current_timestamp,
    updated_at TEXT NOT NULL DEFAULT current_timestamp,
    CHECK (
        contact_type IN ('phone', 'email', 'in_person', 'other')
    )
);

-- ======================================================
-- 8. Contact Notes Table
-- ======================================================
CREATE TABLE IF NOT EXISTS contact_note (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    contact_id INTEGER NOT NULL REFERENCES contact(id) ON DELETE CASCADE,
    note_type TEXT,
    content TEXT,
    created_at TEXT NOT NULL DEFAULT current_timestamp,
    updated_at TEXT NOT NULL DEFAULT current_timestamp,
    CHECK (
        note_type IN ('before', 'during', 'after', 'other')
    )
);
