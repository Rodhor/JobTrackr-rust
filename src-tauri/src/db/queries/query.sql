-- ===============================
-- USERS TABLE QUERIES
-- ===============================
-- name: CreateUser :one
INSERT INTO user (
        first_name,
        last_name,
        email,
        phone_number,
        street_address,
        zip_code,
        city,
        country
    )
VALUES (?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;
-- name: GetUserByID :one
SELECT *
FROM user
WHERE id = ?;
-- name: GetAllUsers :many
SELECT *
FROM user;
-- name: UpdateUser :one
UPDATE user
SET first_name = ?,
    last_name = ?,
    email = ?,
    phone_number = ?,
    street_address = ?,
    zip_code = ?,
    city = ?,
    country = ?
WHERE id = ?
RETURNING *;
-- name: DeleteUser :one
DELETE FROM user
WHERE id = ?
RETURNING id;
-- ===============================
-- COMPANIES TABLE QUERIES
-- ===============================
-- name: CreateCompany :one
INSERT INTO company (
        name,
        street_address,
        zip_code,
        city,
        country,
        default_work_type,
        industry,
        website,
        phone_number
    )
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;
-- name: GetCompanyByID :one
SELECT *
FROM company
WHERE id = ?;
-- name: GetAllCompanies :many
SELECT *
FROM company;
-- name: UpdateCompany :one
UPDATE company
SET name = ?,
    street_address = ?,
    zip_code = ?,
    city = ?,
    country = ?,
    default_work_type = ?,
    industry = ?,
    website = ?,
    phone_number = ?
WHERE id = ?
RETURNING *;
-- name: DeleteCompany :one
DELETE FROM company
WHERE id = ?
RETURNING id;
-- ===============================
-- JOB LISTINGS TABLE QUERIES
-- ===============================
-- name: CreateJobListing :one
INSERT INTO job_listing (
        company_id,
        title,
        work_type,
        category,
        seniority_level,
        salary_min,
        salary_max,
        currency,
        description,
        url
    )
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;
-- name: GetJobListingByID :one
SELECT *
FROM job_listing
WHERE id = ?;
-- name: GetAllJobListings :many
SELECT *
FROM job_listing;
-- name: GetJobListingsByCompanyID :many
SELECT *
FROM job_listing
WHERE company_id = ?;
-- name: UpdateJobListing :one
UPDATE job_listing
SET company_id = ?,
    title = ?,
    work_type = ?,
    category = ?,
    seniority_level = ?,
    salary_min = ?,
    salary_max = ?,
    currency = ?,
    description = ?,
    url = ?
WHERE id = ?
RETURNING *;
-- name: DeleteJobListing :one
DELETE FROM job_listing
WHERE id = ?
RETURNING id;
-- ===============================
-- PERSON TABLE QUERIES
-- ===============================
-- name: CreatePerson :one
INSERT INTO person (
        first_name,
        last_name,
        email,
        phone_number,
        role,
        linkedin_url
    )
VALUES (?, ?, ?, ?, ?, ?)
RETURNING *;
-- name: GetPersonByID :one
SELECT *
FROM person
WHERE id = ?;
-- name: GetAllPersons :many
SELECT *
FROM person;
-- name: UpdatePerson :one
UPDATE person
SET first_name = ?,
    last_name = ?,
    email = ?,
    phone_number = ?,
    role = ?,
    linkedin_url = ?
WHERE id = ?
RETURNING *;
-- name: DeletePerson :one
DELETE FROM person
WHERE id = ?
RETURNING id;
-- ===============================
-- JOB LISTING CONTACT TABLE QUERIES
-- ===============================
-- name: CreateJobListingContact :one
INSERT INTO job_listing_contact (joblisting_id, person_id, role)
VALUES (?, ?, ?)
RETURNING *;
-- name: GetJobListingContactByID :one
SELECT *
FROM job_listing_contact
WHERE id = ?;
-- name: GetAllJobListingContacts :many
SELECT *
FROM job_listing_contact;
-- name: GetContactsByJobListingID :many
SELECT *
FROM job_listing_contact
WHERE joblisting_id = ?;
-- name: UpdateJobListingContact :one
UPDATE job_listing_contact
SET joblisting_id = ?,
    person_id = ?,
    role = ?
WHERE id = ?
RETURNING *;
-- name: DeleteJobListingContact :one
DELETE FROM job_listing_contact
WHERE id = ?
RETURNING id;
-- ===============================
-- APPLICATION TABLE QUERIES
-- ===============================
-- name: CreateApplication :one
INSERT INTO application (
        user_id,
        joblisting_id,
        status,
        applied_date,
        cv_file_path,
        cover_letter_file_path,
        application_notes
    )
VALUES (?, ?, ?, ?, ?, ?, ?)
RETURNING *;
-- name: GetApplicationByID :one
SELECT *
FROM application
WHERE id = ?;
-- name: GetAllApplications :many
SELECT *
FROM application;
-- name: GetApplicationsByUserID :many
SELECT *
FROM application
WHERE user_id = ?;
-- name: GetApplicationsByJobListingID :many
SELECT *
FROM application
WHERE joblisting_id = ?;
-- name: UpdateApplication :one
UPDATE application
SET user_id = ?,
    joblisting_id = ?,
    status = ?,
    applied_date = ?,
    cv_file_path = ?,
    cover_letter_file_path = ?,
    application_notes = ?
WHERE id = ?
RETURNING *;
-- name: DeleteApplication :one
DELETE FROM application
WHERE id = ?
RETURNING id;
-- ===============================
-- CONTACT TABLE QUERIES
-- ===============================
-- name: CreateContact :one
INSERT INTO contact (
        contact_type,
        contact_date,
        location,
        user_id,
        person_id
    )
VALUES (?, ?, ?, ?, ?)
RETURNING *;
-- name: GetContactByID :one
SELECT *
FROM contact
WHERE id = ?;
-- name: GetAllContacts :many
SELECT *
FROM contact;
-- name: GetContactsByUserID :many
SELECT *
FROM contact
WHERE user_id = ?;
-- name: GetContactsByPersonID :many
SELECT *
FROM contact
WHERE person_id = ?;
-- name: UpdateContact :one
UPDATE contact
SET contact_type = ?,
    contact_date = ?,
    location = ?,
    user_id = ?,
    person_id = ?
WHERE id = ?
RETURNING *;
-- name: DeleteContact :one
DELETE FROM contact
WHERE id = ?
RETURNING id;
-- ===============================
-- CONTACT NOTE TABLE QUERIES
-- ===============================
-- name: CreateContactNote :one
INSERT INTO contact_note (contact_id, note_type, content)
VALUES (?, ?, ?)
RETURNING *;
-- name: GetContactNoteByID :one
SELECT *
FROM contact_note
WHERE id = ?;
-- name: GetAllContactNotes :many
SELECT *
FROM contact_note;
-- name: GetContactNotesByContactID :many
SELECT *
FROM contact_note
WHERE contact_id = ?;
-- name: UpdateContactNote :one
UPDATE contact_note
SET contact_id = ?,
    note_type = ?,
    content = ?
WHERE id = ?
RETURNING *;
-- name: DeleteContactNote :one
DELETE FROM contact_note
WHERE id = ?
RETURNING id;