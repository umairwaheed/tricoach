import { useQuery } from '@tanstack/react-query';
import axios from 'axios';
import * as endpoints from './api/endpoints';

/** Treat a 404 as "no data yet" (null) rather than an error. */
function isNotFound(err: unknown): boolean {
  return axios.isAxiosError(err) && err.response?.status === 404;
}

export function useProfile() {
  return useQuery({
    queryKey: ['profile'],
    queryFn: async () => {
      try {
        return await endpoints.getProfile();
      } catch (e) {
        if (isNotFound(e)) return null;
        throw e;
      }
    },
  });
}

export function useActivePlan() {
  return useQuery({
    queryKey: ['activePlan'],
    queryFn: async () => {
      try {
        return await endpoints.getActivePlan();
      } catch (e) {
        if (isNotFound(e)) return null;
        throw e;
      }
    },
  });
}
