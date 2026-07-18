-- TriCoach initial schema (PostgreSQL).
-- UUID primary keys and timestamps are supplied by the application layer.
-- Enum-like columns are stored as TEXT and validated in the Rust domain layer.

CREATE TABLE users (
    id            UUID PRIMARY KEY,
    email         TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL
);

CREATE TABLE athlete_profiles (
    user_id                 UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    display_name            TEXT NOT NULL,
    age                     INT  NOT NULL,
    weight_kg               DOUBLE PRECISION NOT NULL,
    experience_level        TEXT NOT NULL,   -- beginner | intermediate | advanced
    weekly_hours_available  DOUBLE PRECISION NOT NULL,
    resting_hr              INT,
    max_hr                  INT,
    updated_at              TIMESTAMPTZ NOT NULL
);

CREATE TABLE training_plans (
    id             UUID PRIMARY KEY,
    user_id        UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    race_distance  TEXT NOT NULL,            -- sprint | olympic | half_ironman | ironman
    race_date      DATE NOT NULL,
    start_date     DATE NOT NULL,
    total_weeks    INT  NOT NULL,
    status         TEXT NOT NULL DEFAULT 'active',  -- active | archived
    generated_by   TEXT NOT NULL,            -- gemini | rule_based
    summary        TEXT NOT NULL DEFAULT '',
    created_at     TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_training_plans_user ON training_plans(user_id);

CREATE TABLE workouts (
    id                   UUID PRIMARY KEY,
    plan_id              UUID NOT NULL REFERENCES training_plans(id) ON DELETE CASCADE,
    week_number          INT  NOT NULL,
    scheduled_date       DATE NOT NULL,
    discipline           TEXT NOT NULL,      -- swim | bike | run | brick | strength | rest
    title                TEXT NOT NULL,
    description          TEXT NOT NULL,
    planned_duration_min INT  NOT NULL,
    planned_distance_km  DOUBLE PRECISION,
    intensity            TEXT NOT NULL,      -- recovery | endurance | tempo | threshold | vo2max
    status               TEXT NOT NULL DEFAULT 'scheduled', -- scheduled | completed | skipped
    created_at           TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_workouts_plan ON workouts(plan_id);
CREATE INDEX idx_workouts_date ON workouts(scheduled_date);

CREATE TABLE workout_feedback (
    id                  UUID PRIMARY KEY,
    workout_id          UUID NOT NULL UNIQUE REFERENCES workouts(id) ON DELETE CASCADE,
    actual_duration_min INT,
    actual_distance_km  DOUBLE PRECISION,
    avg_hr              INT,
    max_hr              INT,
    perceived_effort    INT,                 -- RPE 1-10
    notes               TEXT NOT NULL DEFAULT '',
    ai_feedback         TEXT NOT NULL DEFAULT '',
    created_at          TIMESTAMPTZ NOT NULL
);

CREATE TABLE device_tokens (
    id          UUID PRIMARY KEY,
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token       TEXT NOT NULL,
    platform    TEXT NOT NULL,               -- ios | android
    created_at  TIMESTAMPTZ NOT NULL,
    UNIQUE (user_id, token)
);

-- Busy calendar blocks used to schedule workouts around work/personal commitments.
CREATE TABLE schedule_blocks (
    id          UUID PRIMARY KEY,
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title       TEXT NOT NULL,
    starts_at   TIMESTAMPTZ NOT NULL,
    ends_at     TIMESTAMPTZ NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_schedule_blocks_user ON schedule_blocks(user_id);
