//! Push notifications via the Expo push service.
//!
//! Expo exposes a simple HTTP endpoint that relays to APNs/FCM, which keeps the
//! backend free of platform-specific credentials — a pragmatic choice for an MVP.
//! Failures are logged, never fatal: a missed notification must not break a request.

use serde::Serialize;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::repositories::device_repo;

const EXPO_PUSH_URL: &str = "https://exp.host/--/api/v2/push/send";

#[derive(Clone)]
pub struct PushSender {
    http: reqwest::Client,
    enabled: bool,
}

#[derive(Serialize)]
struct ExpoMessage<'a> {
    to: &'a str,
    title: &'a str,
    body: &'a str,
    sound: &'a str,
}

impl PushSender {
    /// `enabled = false` turns the sender into a logging no-op (useful in dev/tests).
    pub fn new(enabled: bool) -> Self {
        Self {
            http: reqwest::Client::new(),
            enabled,
        }
    }

    /// Send a notification to every device registered to a user.
    pub async fn notify_user(&self, pool: &SqlitePool, user_id: Uuid, title: &str, body: &str) {
        let tokens = match device_repo::list_for_user(pool, user_id).await {
            Ok(t) => t,
            Err(err) => {
                tracing::warn!(%err, "could not load device tokens for push");
                return;
            }
        };
        for device in tokens {
            self.send(&device.token, title, body).await;
        }
    }

    async fn send(&self, token: &str, title: &str, body: &str) {
        if !self.enabled {
            tracing::info!(token, title, "push disabled — would have sent notification");
            return;
        }
        let msg = ExpoMessage {
            to: token,
            title,
            body,
            sound: "default",
        };
        match self.http.post(EXPO_PUSH_URL).json(&msg).send().await {
            Ok(resp) if resp.status().is_success() => {
                tracing::debug!(token, "push sent");
            }
            Ok(resp) => tracing::warn!(status = %resp.status(), "push rejected"),
            Err(err) => tracing::warn!(%err, "push send failed"),
        }
    }
}
