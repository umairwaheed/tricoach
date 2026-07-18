/**
 * Design tokens for TriCoach. A single source of truth for colour, spacing and
 * type keeps the UI feeling like one cohesive, polished product.
 */
export const colors = {
  bg: '#0B1220',
  surface: '#141E33',
  surfaceAlt: '#1C2842',
  border: '#26334F',
  text: '#F5F7FB',
  textMuted: '#93A0BC',
  primary: '#3BB2F6',
  primaryDark: '#1E7FC2',
  success: '#37D399',
  warning: '#F5B84E',
  danger: '#F0685A',
  white: '#FFFFFF',
};

/** Discipline colour coding used consistently across the app. */
export const disciplineColors: Record<string, string> = {
  swim: '#38BDF8',
  bike: '#F59E0B',
  run: '#34D399',
  brick: '#C084FC',
  strength: '#FB7185',
  rest: '#64748B',
};

export const disciplineIcon: Record<string, string> = {
  swim: '🏊',
  bike: '🚴',
  run: '🏃',
  brick: '🔁',
  strength: '💪',
  rest: '😴',
};

export const intensityColor: Record<string, string> = {
  recovery: '#64748B',
  endurance: '#34D399',
  tempo: '#38BDF8',
  threshold: '#F59E0B',
  vo2max: '#F0685A',
};

export const spacing = (n: number) => n * 8;

export const radius = { sm: 8, md: 12, lg: 18, pill: 999 };

export const font = {
  h1: 30,
  h2: 22,
  h3: 18,
  body: 15,
  small: 13,
  tiny: 11,
};
