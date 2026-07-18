/** Small date helpers working purely on `YYYY-MM-DD` strings and Date objects. */

export function toISODate(d: Date): string {
  return d.toISOString().slice(0, 10);
}

export function todayISO(): string {
  return toISODate(new Date());
}

export function addDays(iso: string, days: number): string {
  const d = new Date(`${iso}T00:00:00Z`);
  d.setUTCDate(d.getUTCDate() + days);
  return toISODate(d);
}

const WEEKDAYS = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
const MONTHS = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];

export function weekdayShort(iso: string): string {
  return WEEKDAYS[new Date(`${iso}T00:00:00Z`).getUTCDay()];
}

export function formatDayMonth(iso: string): string {
  const d = new Date(`${iso}T00:00:00Z`);
  return `${MONTHS[d.getUTCMonth()]} ${d.getUTCDate()}`;
}

export function daysUntil(iso: string): number {
  const target = new Date(`${iso}T00:00:00Z`).getTime();
  const now = new Date(`${todayISO()}T00:00:00Z`).getTime();
  return Math.round((target - now) / 86400000);
}
