// TypeScript mirrors of the Rust API DTOs.

export type ExperienceLevel = 'beginner' | 'intermediate' | 'advanced';
export type RaceDistance = 'sprint' | 'olympic' | 'half_ironman' | 'ironman';
export type Discipline = 'swim' | 'bike' | 'run' | 'brick' | 'strength' | 'rest';
export type Intensity = 'recovery' | 'endurance' | 'tempo' | 'threshold' | 'vo2max';
export type WorkoutStatus = 'scheduled' | 'completed' | 'skipped';

export interface AuthResponse {
  token: string;
  user_id: string;
  email: string;
}

export interface AthleteProfile {
  user_id: string;
  display_name: string;
  age: number;
  weight_kg: number;
  experience_level: ExperienceLevel;
  weekly_hours_available: number;
  resting_hr: number | null;
  max_hr: number | null;
  updated_at: string;
}

export interface Workout {
  id: string;
  plan_id: string;
  week_number: number;
  scheduled_date: string; // YYYY-MM-DD
  discipline: Discipline;
  title: string;
  description: string;
  planned_duration_min: number;
  planned_distance_km: number | null;
  intensity: Intensity;
  status: WorkoutStatus;
  created_at: string;
}

export interface TrainingPlan {
  id: string;
  user_id: string;
  race_distance: RaceDistance;
  race_date: string;
  start_date: string;
  total_weeks: number;
  status: string;
  generated_by: 'gemini' | 'rule_based';
  summary: string;
  created_at: string;
}

export interface PlanWithWorkouts extends TrainingPlan {
  workouts: Workout[];
}

export interface WorkoutFeedback {
  id: string;
  workout_id: string;
  actual_duration_min: number | null;
  actual_distance_km: number | null;
  avg_hr: number | null;
  max_hr: number | null;
  perceived_effort: number | null;
  notes: string;
  ai_feedback: string;
  created_at: string;
}

export interface ScheduleBlock {
  id: string;
  user_id: string;
  title: string;
  starts_at: string;
  ends_at: string;
  created_at: string;
}

export interface ScheduleView {
  from: string;
  to: string;
  workouts: Workout[];
  busy_blocks: ScheduleBlock[];
}
