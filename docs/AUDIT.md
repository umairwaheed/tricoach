# Prototype Audit & Technical Recommendation

*Context: the brief describes an existing AI triathlon‑coaching prototype, tested
privately with friends and a small beta group, that should be rebuilt as a
scalable MVP. This document is the audit and the recommended technical path — the
kind of assessment I'd produce before writing code. The findings below are framed
against the issues named in the brief and common failure modes of early AI‑app
prototypes.*

## 1. Method

An audit like this looks at five axes: **functional correctness**, **UX**,
**AI/product behaviour**, **architecture/scalability**, and **operational
readiness**. For each I note the likely issue, its impact, and the fix adopted in
this rebuild.

## 2. Findings

### 2.1 Functional

| # | Finding | Impact | Fix in rebuild |
| --- | --- | --- | --- |
| F1 | **Incorrect day / workout timing** (named in the brief). LLM‑generated schedules drift: wrong weekdays, sessions before the start date, inconsistent week structure. | Users lose trust immediately; a coaching app that mis‑dates workouts is unusable. | Scheduling is now a **pure deterministic engine** anchored to the week's Monday, with a fixed microcycle. The LLM never touches dates. Covered by unit tests (rest‑day placement, taper, no pre‑start sessions). |
| F2 | No enforcement that plans respect the athlete's real constraints (weekly hours, experience). | Plans feel generic or unrealistic. | Volume scales from `weekly_hours_available`; intensity/labelling scales from `experience_level`. |
| F3 | Weak input validation. | Bad data reaches the model/DB. | DTO‑level validation (email, password length, RPE range, race date after start). |

### 2.2 UX

| # | Finding | Impact | Fix in rebuild |
| --- | --- | --- | --- |
| U1 | **Chat input spacing / cramped forms** (named in the brief). | Feels unfinished. | Consistent spacing scale and a shared component library (`Button`, `Card`, `TextField`, `SegmentedControl`). |
| U2 | Unclear plan structure; hard to see "what do I do today". | Users can't act. | A **Today** screen surfaces today + next‑up; a **Plan** screen shows week‑by‑week phases and planned hours. |
| U3 | No empty/loading/error states. | Confusing first run. | Explicit empty states drive onboarding → plan generation; query loading/refetch handled throughout. |

### 2.3 AI / product

| # | Finding | Impact | Fix in rebuild |
| --- | --- | --- | --- |
| A1 | **Overly long AI responses** (named in the brief). | Users skim past coaching. | Prompts constrain feedback to 2–3 sentences / ~55 words; the deterministic fallback is likewise capped. |
| A2 | Direct, unguarded LLM dependency — an outage or bad key breaks the app. | Fragile. | `AiCoach` trait with a **deterministic fallback**; failures degrade gracefully and are logged. |
| A3 | Coaching not grounded in the athlete's actual numbers. | Generic advice. | Feedback prompt includes planned vs actual duration/distance, HR vs the athlete's max‑HR zones, and RPE. |

### 2.4 Architecture / scalability

| # | Finding | Impact | Fix in rebuild |
| --- | --- | --- | --- |
| S1 | Prototype logic likely tangled (UI ↔ data ↔ AI). | Hard to change safely. | Clear layering: `routes → services → repositories`; AI behind a trait. |
| S2 | No test coverage on the part that matters most (scheduling). | Regressions ship. | The periodisation engine is pure and unit‑tested. |
| S3 | Authorisation easy to get wrong per‑endpoint. | Data leaks. | Ownership folded into SQL joins; a single `AuthUser` extractor guards protected routes. |

### 2.5 Operational

| # | Finding | Impact | Fix in rebuild |
| --- | --- | --- | --- |
| O1 | No reproducible build / deploy story. | "Works on my machine". | Multi‑stage Dockerfile + compose; Cloud Run guide in `DEPLOYMENT.md`. |
| O2 | Secrets/config unclear. | Risky. | All config via env with safe defaults; secrets never logged; JWT secret required in prod. |

## 3. Recommended technical path

1. **Keep the data model boring and relational.** The domain (users, profiles,
   plans, workouts, feedback, schedule blocks) is naturally relational.
2. **Make scheduling deterministic; make prose AI.** This is the core insight —
   it fixes timing and length issues at the root.
3. **Isolate persistence.** Repository functions per table. This MVP ships on
   **SQLite** for zero‑setup; the same interface backs a Postgres move when
   concurrency demands it (see §4).
4. **Ship a thin, typed mobile client** over a well‑specified API, with caching
   (TanStack Query) and secure token storage.

## 4. When to graduate from SQLite → Postgres

SQLite is the right call for an MVP demo: no server, fast, transactional. Move to
Postgres (e.g. Cloud SQL) when any of these appear:

- Concurrent writers beyond a single instance (SQLite serialises writers).
- Horizontal scaling of the API (multiple stateless instances need a shared DB).
- Analytics / heavier querying, connection pooling at scale.

Because SQL lives only in `repositories/`, this is a contained migration: swap the
SQLx driver/types and the connection setup; the services and routes are unchanged.

## 5. Out of scope for this MVP (deliberate)

Real device‑sync (Garmin/Apple Health) ingestion, multi‑coach/team features, and
payment. These are noted as follow‑on work, consistent with the brief's phased
intent.
