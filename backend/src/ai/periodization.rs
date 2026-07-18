//! Deterministic periodization engine.
//!
//! This is the reliable backbone of plan generation. It builds a week-by-week,
//! day-by-day schedule using classic triathlon periodization (base → build →
//! peak → taper, with recovery weeks). Because it is pure and deterministic it
//! is fully unit-testable and *always* produces correctly-timed workouts —
//! directly addressing the "incorrect day or workout timing" issue in the brief.
//!
//! The AI layer (Gemini) is layered on top to write the human-facing narrative;
//! it never controls scheduling, so timing can never regress.

use chrono::{Datelike, Duration, NaiveDate, Weekday};

use crate::domain::enums::*;

/// Inputs required to build a plan skeleton.
#[derive(Debug, Clone)]
pub struct PlanInput {
    pub display_name: String,
    pub race_distance: RaceDistance,
    pub start_date: NaiveDate,
    pub race_date: NaiveDate,
    pub experience: ExperienceLevel,
    pub weekly_hours: f64,
}

/// A single scheduled session before it is persisted (no ids yet).
#[derive(Debug, Clone, PartialEq)]
pub struct PlannedWorkout {
    pub week_number: i32,
    pub scheduled_date: NaiveDate,
    pub discipline: Discipline,
    pub title: String,
    pub description: String,
    pub planned_duration_min: i32,
    pub planned_distance_km: Option<f64>,
    pub intensity: Intensity,
}

/// A slot in the weekly microcycle template.
struct Slot {
    weekday: Weekday,
    discipline: Discipline,
    /// Fraction of the week's training minutes allocated to this session.
    share: f64,
    /// Whether this is an interval session (intensity ramps across phases).
    interval: bool,
    long: bool,
}

/// Approximate speeds (km/h) used to translate duration into distance.
fn speed_kmh(discipline: Discipline, experience: ExperienceLevel) -> Option<f64> {
    let factor = match experience {
        ExperienceLevel::Beginner => 0.9,
        ExperienceLevel::Intermediate => 1.0,
        ExperienceLevel::Advanced => 1.1,
    };
    let base = match discipline {
        Discipline::Swim => 2.6,
        Discipline::Bike => 28.0,
        Discipline::Run => 9.5,
        Discipline::Brick => 20.0, // blended bike+run
        Discipline::Strength | Discipline::Rest => return None,
    };
    Some(base * factor)
}

/// The fixed weekly microcycle. A stable day pattern is what makes workout
/// timing predictable and correct.
fn weekly_template(distance: RaceDistance) -> Vec<Slot> {
    // For Olympic and longer, the weekend long ride becomes a brick (bike→run).
    let long_saturday = match distance {
        RaceDistance::Sprint => Discipline::Bike,
        _ => Discipline::Brick,
    };
    vec![
        Slot { weekday: Weekday::Mon, discipline: Discipline::Rest, share: 0.0, interval: false, long: false },
        Slot { weekday: Weekday::Tue, discipline: Discipline::Swim, share: 0.12, interval: true, long: false },
        Slot { weekday: Weekday::Wed, discipline: Discipline::Bike, share: 0.18, interval: false, long: false },
        Slot { weekday: Weekday::Thu, discipline: Discipline::Run, share: 0.15, interval: true, long: false },
        Slot { weekday: Weekday::Fri, discipline: Discipline::Swim, share: 0.10, interval: false, long: false },
        Slot { weekday: Weekday::Sat, discipline: long_saturday, share: 0.27, interval: false, long: true },
        Slot { weekday: Weekday::Sun, discipline: Discipline::Run, share: 0.18, interval: false, long: true },
    ]
}

/// Volume multiplier for a given (1-indexed) week within the plan.
/// Encodes base ramp, recovery weeks, and a two-week taper.
pub fn week_multiplier(week: i32, total_weeks: i32) -> f64 {
    if total_weeks <= 1 {
        return 1.0;
    }
    let taper_start = total_weeks - 1; // final two weeks taper down
    if week >= taper_start {
        return if week >= total_weeks { 0.5 } else { 0.7 };
    }
    // Every 4th week is a recovery week (reduced load).
    if week % 4 == 0 {
        return 0.65;
    }
    // Progressive ramp from 0.75 up to 1.2 across the build weeks.
    let build_span = (taper_start - 1).max(1) as f64;
    let t = ((week - 1) as f64 / build_span).clamp(0.0, 1.0);
    0.75 + t * 0.45
}

fn is_recovery_week(week: i32, total_weeks: i32) -> bool {
    let taper_start = total_weeks - 1;
    week < taper_start && week % 4 == 0
}

/// Choose the intensity for a session given the training phase.
fn session_intensity(slot: &Slot, week: i32, total_weeks: i32) -> Intensity {
    if slot.discipline == Discipline::Rest {
        return Intensity::Recovery;
    }
    if is_recovery_week(week, total_weeks) || week >= total_weeks - 1 {
        return Intensity::Endurance;
    }
    if slot.long {
        return Intensity::Endurance;
    }
    if slot.interval {
        // Base → Build → Peak progression for hard sessions.
        let third = (total_weeks as f64 / 3.0).ceil() as i32;
        return if week <= third {
            Intensity::Tempo
        } else if week <= 2 * third {
            Intensity::Threshold
        } else {
            Intensity::Vo2max
        };
    }
    Intensity::Endurance
}

fn round_to_5(minutes: f64) -> i32 {
    let r = ((minutes / 5.0).round() * 5.0) as i32;
    r.max(0)
}

fn describe(discipline: Discipline, intensity: Intensity, long: bool, minutes: i32) -> (String, String) {
    let title = match (discipline, long, intensity) {
        (Discipline::Rest, _, _) => "Rest & mobility".to_string(),
        (Discipline::Brick, _, _) => "Brick: bike → run".to_string(),
        (_, true, _) => format!("Long {}", discipline),
        (_, _, Intensity::Vo2max) => format!("{} VO2max intervals", cap(discipline)),
        (_, _, Intensity::Threshold) => format!("{} threshold intervals", cap(discipline)),
        (_, _, Intensity::Tempo) => format!("{} tempo", cap(discipline)),
        (_, _, _) => format!("{} endurance", cap(discipline)),
    };
    let desc = match discipline {
        Discipline::Rest => {
            "Full rest day. Optional 15–20 min mobility or easy stretching.".to_string()
        }
        Discipline::Brick => format!(
            "{} min total. Ride at endurance effort, then transition straight into a short run to train race legs.",
            minutes
        ),
        Discipline::Strength => format!("{minutes} min general strength & core circuit."),
        _ => match intensity {
            Intensity::Vo2max => format!(
                "{minutes} min. After a warm-up, hold hard efforts at ~90% max HR with equal recovery."
            ),
            Intensity::Threshold => format!(
                "{minutes} min. Sustained efforts near threshold (comfortably hard, ~85% max HR)."
            ),
            Intensity::Tempo => format!(
                "{minutes} min steady tempo at a controlled, moderately-hard effort (~80% max HR)."
            ),
            _ => format!(
                "{minutes} min easy aerobic effort. Keep it conversational (~70% max HR)."
            ),
        },
    };
    (title, desc)
}

fn cap(d: Discipline) -> String {
    let s = d.as_str();
    let mut c = s.chars();
    match c.next() {
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

/// Number of whole weeks between two dates (at least 1).
pub fn weeks_between(start: NaiveDate, end: NaiveDate) -> i32 {
    let days = (end - start).num_days();
    ((days as f64 / 7.0).round() as i32).max(1)
}

/// Build the complete, correctly-dated set of workouts for a plan.
pub fn build_plan(input: &PlanInput) -> Vec<PlannedWorkout> {
    let total_weeks = weeks_between(input.start_date, input.race_date);
    let template = weekly_template(input.race_distance);

    // Anchor week 1 to the Monday of the start date's week so the microcycle
    // lands on stable weekdays regardless of which day the athlete signs up.
    let start_monday = monday_of(input.start_date);

    let mut workouts = Vec::new();
    for week in 1..=total_weeks {
        let mult = week_multiplier(week, total_weeks);
        let weekly_minutes = input.weekly_hours * 60.0 * mult;
        let week_monday = start_monday + Duration::weeks((week - 1) as i64);

        for slot in &template {
            let date = week_monday + Duration::days(weekday_offset(slot.weekday));
            // Skip sessions scheduled before the athlete's real start date in week 1.
            if date < input.start_date {
                continue;
            }
            let intensity = session_intensity(slot, week, total_weeks);

            if slot.discipline == Discipline::Rest {
                let (title, description) = describe(Discipline::Rest, intensity, false, 0);
                workouts.push(PlannedWorkout {
                    week_number: week,
                    scheduled_date: date,
                    discipline: Discipline::Rest,
                    title,
                    description,
                    planned_duration_min: 0,
                    planned_distance_km: None,
                    intensity,
                });
                continue;
            }

            let minutes = round_to_5(weekly_minutes * slot.share).max(20);
            let distance = speed_kmh(slot.discipline, input.experience)
                .map(|kmh| ((minutes as f64 / 60.0) * kmh * 10.0).round() / 10.0);
            let (title, description) =
                describe(slot.discipline, intensity, slot.long, minutes);

            workouts.push(PlannedWorkout {
                week_number: week,
                scheduled_date: date,
                discipline: slot.discipline,
                title,
                description,
                planned_duration_min: minutes,
                planned_distance_km: distance,
                intensity,
            });
        }
    }
    workouts
}

fn monday_of(date: NaiveDate) -> NaiveDate {
    let offset = date.weekday().num_days_from_monday() as i64;
    date - Duration::days(offset)
}

fn weekday_offset(weekday: Weekday) -> i64 {
    weekday.num_days_from_monday() as i64
}

/// A short, templated plan summary used when no LLM is configured.
pub fn templated_summary(input: &PlanInput, total_weeks: i32) -> String {
    format!(
        "{}-week {} plan for {}. Structured in base, build and peak blocks with recovery weeks every fourth week and a 2-week taper into race day. Targeting ~{:.0} h/week at your {} level.",
        total_weeks,
        input.race_distance.label(),
        input.display_name,
        input.weekly_hours,
        input.experience.as_str(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(weeks_out: i64) -> PlanInput {
        let start = NaiveDate::from_ymd_opt(2026, 1, 5).unwrap(); // a Monday
        PlanInput {
            display_name: "Alex".into(),
            race_distance: RaceDistance::Olympic,
            start_date: start,
            race_date: start + Duration::weeks(weeks_out),
            experience: ExperienceLevel::Beginner,
            weekly_hours: 8.0,
        }
    }

    #[test]
    fn generates_workouts_for_every_week() {
        let input = sample(12);
        let plan = build_plan(&input);
        let weeks: std::collections::HashSet<i32> =
            plan.iter().map(|w| w.week_number).collect();
        assert_eq!(weeks.len(), 12);
        assert!(!plan.is_empty());
    }

    #[test]
    fn every_week_has_exactly_one_rest_day() {
        let plan = build_plan(&sample(12));
        for week in 1..=12 {
            let rests = plan
                .iter()
                .filter(|w| w.week_number == week && w.discipline == Discipline::Rest)
                .count();
            assert_eq!(rests, 1, "week {week} should have one rest day");
        }
    }

    #[test]
    fn rest_days_land_on_monday() {
        let plan = build_plan(&sample(12));
        for w in plan.iter().filter(|w| w.discipline == Discipline::Rest) {
            assert_eq!(w.scheduled_date.weekday(), Weekday::Mon);
        }
    }

    #[test]
    fn taper_reduces_volume_below_peak() {
        let plan = build_plan(&sample(12));
        let volume = |week: i32| -> i32 {
            plan.iter()
                .filter(|w| w.week_number == week)
                .map(|w| w.planned_duration_min)
                .sum()
        };
        // Race week (12) should be much lighter than a peak build week (e.g. 9).
        assert!(volume(12) < volume(9), "taper week should be lighter than peak");
    }

    #[test]
    fn recovery_week_is_lighter_than_neighbours() {
        // Week 4 is a recovery week; it should be lighter than week 3.
        assert!(week_multiplier(4, 16) < week_multiplier(3, 16));
    }

    #[test]
    fn olympic_uses_brick_on_the_weekend() {
        let plan = build_plan(&sample(12));
        assert!(plan.iter().any(|w| w.discipline == Discipline::Brick));
    }

    #[test]
    fn workouts_are_never_scheduled_before_start_date() {
        // Start mid-week (a Thursday) and ensure nothing lands earlier.
        let start = NaiveDate::from_ymd_opt(2026, 1, 8).unwrap(); // Thursday
        let input = PlanInput {
            start_date: start,
            race_date: start + Duration::weeks(10),
            ..sample(10)
        };
        let plan = build_plan(&input);
        assert!(plan.iter().all(|w| w.scheduled_date >= start));
    }
}
