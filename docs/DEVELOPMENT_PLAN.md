# Software Development Plan

A milestone plan for taking the audited prototype to a launchable MVP and beyond.
Milestones 0–4 are **implemented in this repository**; 5–7 are the proposed path
to production.

## Guiding principles

- **Correctness before cleverness** — deterministic scheduling first, AI polish second.
- **Thin client, well‑specified API** — the mobile app should never encode business rules.
- **Ship vertically** — each milestone is a working slice, demoable end‑to‑end.

---

## Milestone 0 — Foundations ✅

**Deliverables:** repo layout, Rust + Axum skeleton, SQLite schema & migrations,
config/telemetry, error model, CI‑ready `cargo test`.
**Acceptance:** `cargo run` serves `/health`; migrations apply on boot.

## Milestone 1 — Auth & athlete profile ✅

**Deliverables:** register/login with JWT + Argon2; `AuthUser` extractor;
athlete‑profile upsert/read.
**Acceptance:** protected routes reject missing/invalid tokens (401); profile round‑trips.

## Milestone 2 — Plan generation (the core) ✅

**Deliverables:** deterministic periodisation engine (base → build → peak → taper,
recovery weeks, fixed weekly microcycle, HR‑zone intensities); `AiCoach` trait;
plan + workouts persisted atomically.
**Acceptance:** unit tests for timing (one rest day/week, rest on Monday, taper < peak,
nothing before the start date, brick for Olympic+); a 12‑week Olympic plan generates ~68 training sessions with correct dates.

## Milestone 3 — Workout feedback & AI voice ✅

**Deliverables:** feedback submission (duration/distance/HR/RPE/notes); concise AI
feedback via Gemini with deterministic fallback; workout status transitions.
**Acceptance:** feedback returns ≤3 sentences grounded in planned‑vs‑actual + HR zones;
workout flips to `completed`.

## Milestone 4 — Mobile app & scheduling ✅

**Deliverables:** Expo/TS app (auth, onboarding, Today, Plan, Workout detail,
Schedule); busy‑block scheduling; push‑token registration; push on plan creation.
**Acceptance:** full journey works end‑to‑end (verified in‑browser, no console errors).

---

## Milestone 5 — Production hardening (proposed)

**Deliverables:** rate limiting, refresh tokens/rotation, request‑id tracing,
structured audit logs, OpenAPI spec, integration test suite against a temp DB,
GitHub Actions CI (fmt/clippy/test), error monitoring (Sentry).
**Acceptance:** CI green on PRs; load test meets a target p95 latency.

## Milestone 6 — Data & integrations (proposed)

**Deliverables:** Postgres (Cloud SQL) behind the existing repository layer;
device/watch ingestion (Apple Health / Garmin / Strava) to auto‑populate feedback;
calendar sync (Google/Microsoft) to auto‑create busy blocks.
**Acceptance:** a completed watch activity appears as workout feedback without manual entry.

## Milestone 7 — Launch & scale (proposed)

**Deliverables:** Cloud Run deploy with autoscaling, secrets in Secret Manager,
scheduled push reminders (Cloud Scheduler), analytics, app‑store builds via EAS.
**Acceptance:** blue/green deploy; reminders fire on schedule; crash‑free sessions ≥ target.

---

## Cross‑cutting workstreams

| Workstream | Approach |
| --- | --- |
| **Testing** | Pure logic unit‑tested (done); API integration tests and a mobile e2e (Detox/Maestro) in M5. |
| **Security** | Argon2id, JWT, ownership in SQL, no secret logging (done); rate limiting + token rotation in M5. |
| **Observability** | `tracing` structured logs (done); request ids, metrics, error monitoring in M5. |
| **Delivery** | Docker + compose (done); CI + Cloud Run in M5–M7. |

## Rough sequencing

M5 and M6 can partly run in parallel (hardening vs integrations). M7 depends on both.
A realistic solo cadence is one milestone per 1–2 week iteration, demoable at each step.
