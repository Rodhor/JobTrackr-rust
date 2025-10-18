-- ======================================================
-- DATABASE SCHEMA: Job Tracking / Application Manager
-- ======================================================

-- ======================================================
-- 1. Users Table
-- ======================================================
CREATE TABLE IF NOT EXISTS user (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    phone_number VARCHAR(50),
    street_address VARCHAR(255),
    zip_code VARCHAR(20),
    city VARCHAR(255),
    country VARCHAR(255),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- ======================================================
-- 2. Companies Table
-- ======================================================
CREATE TABLE IF NOT EXISTS company (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(255) NOT NULL,
    street_address VARCHAR(255),
    zip_code VARCHAR(20),
    city VARCHAR(255),
    country VARCHAR(255),
    default_work_type VARCHAR(50),
    industry VARCHAR(100),
    website TEXT,
    phone_number VARCHAR(50) UNIQUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
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
    title VARCHAR(255) NOT NULL,
    work_type VARCHAR(50),
    category VARCHAR(50),
    seniority_level VARCHAR(100),
    salary_min INTEGER,
    salary_max INTEGER,
    currency VARCHAR(10),
    description TEXT,
    url TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
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
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    email VARCHAR(255) UNIQUE,
    phone_number VARCHAR(50),
    role VARCHAR(100),
    linkedin_url TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- ======================================================
-- 5. Job Listing Contacts Table
-- ======================================================
CREATE TABLE IF NOT EXISTS job_listing_contact (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    joblisting_id INTEGER NOT NULL REFERENCES job_listing(id) ON DELETE CASCADE,
    person_id INTEGER NOT NULL REFERENCES person(id) ON DELETE CASCADE,
    role VARCHAR(50),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CHECK (role IN ('recruiter', 'hr', 'manager', 'other'))
);

-- ======================================================
-- 6. Applications Table
-- ======================================================
CREATE TABLE IF NOT EXISTS application (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES user(id) ON DELETE CASCADE,
    joblisting_id INTEGER REFERENCES job_listing(id) ON DELETE SET NULL,
    status VARCHAR(50),
    applied_date DATETIME,
    cv_file_path TEXT,
    cover_letter_file_path TEXT,
    application_notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
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
    contact_type VARCHAR(50) NOT NULL,
    contact_date DATETIME NOT NULL,
    location VARCHAR(255),
    user_id INTEGER REFERENCES user(id) ON DELETE CASCADE,
    person_id INTEGER REFERENCES person(id) ON DELETE CASCADE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
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
    note_type VARCHAR(50),
    content TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CHECK (
        note_type IN ('before', 'during', 'after', 'other')
    )
);

-- ======================================================
-- Foreign Key Enforcement
-- ======================================================
PRAGMA foreign_keys = ON;
