import type { Workout } from '../api/types';

export type AuthStackParams = {
  Login: undefined;
  Register: undefined;
};

export type AppStackParams = {
  Tabs: undefined;
  Onboarding: undefined;
  GeneratePlan: undefined;
  WorkoutDetail: { workout: Workout };
};

export type TabParams = {
  Today: undefined;
  Plan: undefined;
  Schedule: undefined;
};
