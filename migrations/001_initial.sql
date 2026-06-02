-- Initial schema

CREATE TABLE IF NOT EXISTS profile (
    id           TEXT PRIMARY KEY,
    name         TEXT NOT NULL DEFAULT '',
    title        TEXT NOT NULL DEFAULT '',
    bio          TEXT NOT NULL DEFAULT '',
    summary      TEXT NOT NULL DEFAULT '',
    email        TEXT NOT NULL DEFAULT '',
    github       TEXT NOT NULL DEFAULT '',
    linkedin     TEXT NOT NULL DEFAULT '',
    twitter      TEXT NOT NULL DEFAULT '',
    avatar_url   TEXT NOT NULL DEFAULT '',
    location     TEXT NOT NULL DEFAULT '',
    resume_url   TEXT NOT NULL DEFAULT '',
    theme        TEXT NOT NULL DEFAULT 'dark-emerald',
    section_order TEXT NOT NULL DEFAULT 'experience,projects,skills,education,certifications'
);

CREATE TABLE IF NOT EXISTS experiences (
    id           TEXT PRIMARY KEY,
    company      TEXT NOT NULL,
    role         TEXT NOT NULL,
    start_date   TEXT NOT NULL,
    end_date     TEXT,
    current      INTEGER NOT NULL DEFAULT 0,
    description  TEXT NOT NULL DEFAULT '[]',
    technologies TEXT NOT NULL DEFAULT '[]',
    order_index  INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS educations (
    id          TEXT PRIMARY KEY,
    institution TEXT NOT NULL,
    degree      TEXT NOT NULL,
    field       TEXT NOT NULL,
    start_year  TEXT NOT NULL,
    end_year    TEXT,
    current     INTEGER NOT NULL DEFAULT 0,
    description TEXT NOT NULL DEFAULT '',
    gpa         TEXT,
    order_index INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS projects (
    id               TEXT PRIMARY KEY,
    title            TEXT NOT NULL,
    description      TEXT NOT NULL DEFAULT '',
    long_description TEXT NOT NULL DEFAULT '',
    technologies     TEXT NOT NULL DEFAULT '[]',
    github_url       TEXT,
    live_url         TEXT,
    image_url        TEXT,
    featured         INTEGER NOT NULL DEFAULT 0,
    order_index      INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS skills (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL,
    category    TEXT NOT NULL DEFAULT 'Languages',
    level       INTEGER NOT NULL DEFAULT 3,
    order_index INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS certifications (
    id             TEXT PRIMARY KEY,
    name           TEXT NOT NULL,
    issuer         TEXT NOT NULL,
    date           TEXT NOT NULL,
    expiry_date    TEXT,
    credential_url TEXT,
    order_index    INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS admin (
    id            INTEGER PRIMARY KEY,
    username      TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
);
