import { api } from './client';
import type {
  AthleteProfile,
  AuthResponse,
  PlanWithWorkouts,
  RaceDistance,
  ScheduleBlock,
  ScheduleView,
  TrainingPlan,
  Workout,
  WorkoutFeedback,
  WorkoutStatus,
} from './types';

// ---- Auth ----
export const register = (email: string, password: string) =>
  api.post<AuthResponse>('/auth/register', { email, password }).then((r) => r.data);

export const login = (email: string, password: string) =>
  api.post<AuthResponse>('/auth/login', { email, password }).then((r) => r.data);

// ---- Profile ----
export interface ProfileInput {
  display_name: string;
  age: number;
  weight_kg: number;
  experience_level: AthleteProfile['experience_level'];
  weekly_hours_available: number;
  resting_hr?: number | null;
  max_hr?: number | null;
}

export const getProfile = () =>
  api.get<AthleteProfile>('/profile').then((r) => r.data);

export const upsertProfile = (input: ProfileInput) =>
  api.put<AthleteProfile>('/profile', input).then((r) => r.data);

// ---- Plans ----
export const generatePlan = (input: {
  race_distance: RaceDistance;
  race_date: string;
  start_date?: string;
}) => api.post<PlanWithWorkouts>('/plans', input).then((r) => r.data);

export const getActivePlan = () =>
  api.get<PlanWithWorkouts>('/plans/active').then((r) => r.data);

export const listPlans = () =>
  api.get<TrainingPlan[]>('/plans').then((r) => r.data);

// ---- Workouts ----
export const submitFeedback = (
  workoutId: string,
  input: {
    actual_duration_min?: number | null;
    actual_distance_km?: number | null;
    avg_hr?: number | null;
    max_hr?: number | null;
    perceived_effort?: number | null;
    notes?: string;
  },
) => api.post<WorkoutFeedback>(`/workouts/${workoutId}/feedback`, input).then((r) => r.data);

export const getFeedback = (workoutId: string) =>
  api.get<WorkoutFeedback>(`/workouts/${workoutId}/feedback`).then((r) => r.data);

export const updateWorkoutStatus = (workoutId: string, status: WorkoutStatus) =>
  api.patch<Workout>(`/workouts/${workoutId}/status`, { status }).then((r) => r.data);

// ---- Schedule ----
export const getSchedule = (from: string, to: string) =>
  api
    .get<ScheduleView>('/schedule', { params: { from, to } })
    .then((r) => r.data);

export const createScheduleBlock = (input: {
  title: string;
  starts_at: string;
  ends_at: string;
}) => api.post<ScheduleBlock>('/schedule/blocks', input).then((r) => r.data);
