//! Google Gemini-backed coach. Uses the deterministic engine for scheduling and
//! Gemini for the coaching *voice*. Any network/parse failure transparently
//! falls back to the rule-based text, so the API never fails because the LLM did.

use std::time::Duration;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::periodization::{self, PlanInput};
use super::rule_based::RuleBasedCoach;
use super::{AiCoach, FeedbackContext, PlanBlueprint, WorkoutContext};
use crate::domain::enums::GeneratedBy;
use crate::error::AppResult;

#[derive(Clone)]
pub struct GeminiCoach {
    http: reqwest::Client,
    api_key: String,
    model: String,
    fallback: RuleBasedCoach,
}

impl GeminiCoach {
    pub fn new(api_key: String, model: String) -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(15))
            .build()
            .expect("failed to build reqwest client");
        Self {
            http,
            api_key,
            model,
            fallback: RuleBasedCoach,
        }
    }

    async fn generate(&self, prompt: &str, max_tokens: i32) -> anyhow::Result<String> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model, self.api_key
        );
        let body = GenerateRequest {
            contents: vec![Content {
                parts: vec![Part { text: prompt.to_string() }],
            }],
            generation_config: GenerationConfig {
                temperature: 0.7,
                max_output_tokens: max_tokens,
            },
        };

        let resp = self.http.post(&url).json(&body).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("gemini returned {status}: {text}");
        }
        let parsed: GenerateResponse = resp.json().await?;
        let text = parsed
            .candidates
            .into_iter()
            .next()
            .and_then(|c| c.content.parts.into_iter().next())
            .map(|p| p.text)
            .unwrap_or_default();

        let trimmed = text.trim().to_string();
        if trimmed.is_empty() {
            anyhow::bail!("gemini returned an empty response");
        }
        Ok(trimmed)
    }
}

#[async_trait]
impl AiCoach for GeminiCoach {
    async fn generate_plan(&self, input: &PlanInput) -> AppResult<PlanBlueprint> {
        // Scheduling is always deterministic.
        let workouts = periodization::build_plan(input);
        let total_weeks = periodization::weeks_between(input.start_date, input.race_date);

        let prompt = format!(
            "You are an encouraging triathlon coach writing to a {experience} athlete named {name}. \
             In 2-3 short sentences (max ~60 words), summarise their {weeks}-week plan for a {distance} race, \
             training about {hours:.0} hours per week. Be motivating and specific, not generic. \
             Do not use markdown, bullet points, or headings.",
            experience = input.experience.as_str(),
            name = input.display_name,
            weeks = total_weeks,
            distance = input.race_distance.label(),
            hours = input.weekly_hours,
        );

        match self.generate(&prompt, 200).await {
            Ok(summary) => Ok(PlanBlueprint {
                total_weeks,
                summary,
                generated_by: GeneratedBy::Gemini,
                workouts,
            }),
            Err(err) => {
                tracing::warn!(%err, "gemini plan summary failed; using rule-based summary");
                Ok(PlanBlueprint {
                    total_weeks,
                    summary: periodization::templated_summary(input, total_weeks),
                    generated_by: GeneratedBy::RuleBased,
                    workouts,
                })
            }
        }
    }

    async fn analyze_workout(
        &self,
        workout: &WorkoutContext,
        feedback: &FeedbackContext,
    ) -> AppResult<String> {
        let prompt = format!(
            "You are a supportive triathlon coach. Give concise post-workout feedback in 2-3 sentences \
             (max ~55 words). No markdown or lists.\n\
             Planned: {title} ({discipline}, {intensity}), {planned_min} min{planned_dist}.\n\
             Actual: {actual_min} min{actual_dist}, avg HR {avg_hr}, max HR {max_hr}, RPE {rpe}.\n\
             Athlete max HR: {athlete_max}. Athlete notes: {notes}",
            title = workout.title,
            discipline = workout.discipline,
            intensity = workout.intensity,
            planned_min = workout.planned_duration_min,
            planned_dist = opt_km(workout.planned_distance_km),
            actual_min = opt_i(feedback.actual_duration_min),
            actual_dist = opt_km(feedback.actual_distance_km),
            avg_hr = opt_i(feedback.avg_hr),
            max_hr = opt_i(feedback.max_hr),
            rpe = opt_i(feedback.perceived_effort),
            athlete_max = opt_i(feedback.athlete_max_hr),
            notes = if feedback.notes.trim().is_empty() { "none" } else { feedback.notes.trim() },
        );

        match self.generate(&prompt, 160).await {
            Ok(text) => Ok(text),
            Err(err) => {
                tracing::warn!(%err, "gemini workout analysis failed; using rule-based feedback");
                self.fallback.analyze_workout(workout, feedback).await
            }
        }
    }
}

fn opt_i(v: Option<i32>) -> String {
    v.map(|x| x.to_string()).unwrap_or_else(|| "n/a".into())
}

fn opt_km(v: Option<f64>) -> String {
    v.map(|x| format!(", {x:.1} km")).unwrap_or_default()
}

// ---- Gemini wire types ----

#[derive(Serialize)]
struct GenerateRequest {
    contents: Vec<Content>,
    #[serde(rename = "generationConfig")]
    generation_config: GenerationConfig,
}

#[derive(Serialize)]
struct GenerationConfig {
    temperature: f32,
    #[serde(rename = "maxOutputTokens")]
    max_output_tokens: i32,
}

#[derive(Serialize, Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize, Deserialize)]
struct Part {
    text: String,
}

#[derive(Deserialize)]
struct GenerateResponse {
    #[serde(default)]
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: Content,
}
