# Deployment

## Current deployment — `internal-one`

The app is deployed on the `internal-one` server:

- **URL:** https://coaching-app.getprixite.com (nginx + Let's Encrypt, cert auto-renews)
- **Service:** systemd unit `coaching-app` running as the `coaching-app` system user;
  native binary `/opt/coaching-app/bin/tricoach` listening on `127.0.0.1:3010`
- **Database:** one PostgreSQL database `coaching-app` on the same host
- **Config/secrets:** `/etc/coaching-app/coaching-app.env` (root:coaching-app, 640)
- **Source + deploy script:** `/opt/coaching-app/src` and `/opt/coaching-app/deploy.sh`

Redeploy the latest `main`:

```bash
git push origin main
ssh internal-one /opt/coaching-app/deploy.sh
```

`deploy.sh` pulls, builds the release binary with Docker (no Rust toolchain on the
host), installs it, and restarts the service. Migrations run on startup. To switch
the coach from deterministic to Gemini, add `GEMINI_API_KEY=…` to the env file and
`systemctl restart coaching-app`.

Handy operations:

```bash
systemctl status coaching-app          # service state
journalctl -u coaching-app -f          # live logs
```

---

## Alternative — Google Cloud Run

The same container also runs as a stateless service on Cloud Run (or AWS App
Runner / ECS). Gemini is Google's own API, so keys and quotas live in the same cloud.

> **Database:** run a managed PostgreSQL instance (Cloud SQL / RDS) and point
> `DATABASE_URL` at it. The API container is stateless, so it scales horizontally
> behind one database.

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

## 3. Deploy (Cloud SQL Postgres)

1. Create a Postgres instance and database, then reference it via the Cloud SQL
   connector. Store the connection string as a secret:

```bash
echo -n "postgres://USER:PASS@/tricoach?host=/cloudsql/PROJECT:REGION:INSTANCE" \
  | gcloud secrets create database-url --data-file=-
```

2. Deploy with the Cloud SQL connection attached (migrations run on startup):

```bash
gcloud run deploy tricoach-api \
  --image $IMAGE --region $REGION --allow-unauthenticated \
  --add-cloudsql-instances YOUR_PROJECT:REGION:INSTANCE \
  --set-env-vars "BIND_ADDR=0.0.0.0:8080,RUST_LOG=info" \
  --set-secrets "DATABASE_URL=database-url:latest,JWT_SECRET=jwt-secret:latest,GEMINI_API_KEY=gemini-api-key:latest"
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
for the database and **Secrets Manager** for `DATABASE_URL` / `JWT_SECRET` /
`GEMINI_API_KEY`. Nothing in the app is cloud‑specific.
