import axios from 'axios';
import Constants from 'expo-constants';

/**
 * Base URL resolution:
 *  - `extra.apiUrl` in app.json (defaults to http://localhost:8080)
 *  - On a physical device replace `localhost` with your machine's LAN IP.
 */
const apiUrl =
  (Constants.expoConfig?.extra?.apiUrl as string | undefined) ?? 'http://localhost:8080';

export const api = axios.create({
  baseURL: `${apiUrl}/api/v1`,
  timeout: 20000,
});

let authToken: string | null = null;

/** Set (or clear) the bearer token used for all subsequent requests. */
export function setAuthToken(token: string | null) {
  authToken = token;
}

api.interceptors.request.use((config) => {
  if (authToken) {
    config.headers.Authorization = `Bearer ${authToken}`;
  }
  return config;
});

/** Normalise API errors into a readable message for the UI. */
export function apiErrorMessage(err: unknown): string {
  if (axios.isAxiosError(err)) {
    return (
      (err.response?.data as { error?: string } | undefined)?.error ??
      err.message ??
      'Network error'
    );
  }
  return 'Something went wrong';
}
