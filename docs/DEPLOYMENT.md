# Deployment — Google Cloud Run

The backend is a single stateless container that serves HTTP on `$BIND_ADDR`
(default `0.0.0.0:8080`). Cloud Run is a natural fit. Gemini is Google's own API,
so keys and quotas live in the same cloud.

> **State note:** the MVP uses embedded SQLite. On Cloud Run's ephemeral
> filesystem this means data does not persist across revisions/instances — fine
> for a demo. For real persistence, either mount a volume (single instance) or
> switch to **Cloud SQL (Postgres)** per the audit. Both options are described below.

## 0. Prerequisites

```bash
gcloud auth login
gcloud config set project YOUR_PROJECT_ID
gcloud services enable run.googleapis.com artifactregistry.googleapis.com
```

## 1. Build & push the image

Using Artifact Registry:

```bash
REGION=us-central1
REPO=tricoach
gcloud artifacts repositories create $REPO --repository-format=docker --location=$REGION

IMAGE=$REGION-docker.pkg.dev/$(gcloud config get-value project)/$REPO/api:latest
gcloud builds submit ./backend --tag $IMAGE
```

## 2. Store secrets

```bash
echo -n "$(openssl rand -hex 32)" | gcloud secrets create jwt-secret --data-file=-
echo -n "YOUR_GEMINI_KEY"        | gcloud secrets create gemini-api-key --data-file=-
```

## 3. Deploy

### Option A — demo (SQLite, single instance)

```bash
gcloud run deploy tricoach-api \
  --image $IMAGE --region $REGION --allow-unauthenticated \
  --max-instances 1 \
  --set-env-vars "BIND_ADDR=0.0.0.0:8080,DATABASE_URL=sqlite:///data/tricoach.db,RUST_LOG=info" \
  --set-secrets "JWT_SECRET=jwt-secret:latest,GEMINI_API_KEY=gemini-api-key:latest"
```

`--max-instances 1` keeps a single SQLite writer. Add a mounted volume
(Cloud Run + a Filestore/GCS‑FUSE volume) if data must survive restarts.

### Option B — production (Cloud SQL Postgres)

1. Create a Postgres instance and database, then point `DATABASE_URL` at it via the
   Cloud SQL connector.
2. Change the backend's SQLx driver to Postgres (contained to `db.rs`,
   `repositories/`, and the enum `Type/Encode/Decode` impls — see the audit).
3. Deploy with the Cloud SQL connection attached:

```bash
gcloud run deploy tricoach-api \
  --image $IMAGE --region $REGION --allow-unauthenticated \
  --add-cloudsql-instances YOUR_PROJECT:REGION:INSTANCE \
  --set-env-vars "BIND_ADDR=0.0.0.0:8080,DATABASE_URL=postgres://…,RUST_LOG=info" \
  --set-secrets "JWT_SECRET=jwt-secret:latest,GEMINI_API_KEY=gemini-api-key:latest"
```

## 4. Point the mobile app at the deployed API

In `mobile/app.json`, set `expo.extra.apiUrl` to the Cloud Run URL, then build with
[EAS](https://docs.expo.dev/eas/):

```bash
cd mobile && eas build --platform ios   # or android
```

## 5. Scheduled push reminders (optional)

Add a small authenticated endpoint that sends "today's workout" notifications, and
trigger it with **Cloud Scheduler** (e.g. daily at 06:00). Device tokens are already
captured via `POST /api/v1/devices` and delivered through Expo Push.

## AWS equivalent

The same container runs on **AWS App Runner** or **ECS Fargate**; use **RDS Postgres**
for Option B and **Secrets Manager** for `JWT_SECRET` / `GEMINI_API_KEY`. Nothing in
the app is cloud‑specific.
