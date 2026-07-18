import React, { createContext, useContext, useEffect, useMemo, useState } from 'react';
import { setAuthToken } from '../api/client';
import * as endpoints from '../api/endpoints';
import { tokenStore } from './storage';

const TOKEN_KEY = 'tricoach.token';
const EMAIL_KEY = 'tricoach.email';

interface AuthState {
  token: string | null;
  email: string | null;
  initializing: boolean;
  login: (email: string, password: string) => Promise<void>;
  register: (email: string, password: string) => Promise<void>;
  logout: () => Promise<void>;
}

const AuthContext = createContext<AuthState | undefined>(undefined);

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [token, setToken] = useState<string | null>(null);
  const [email, setEmail] = useState<string | null>(null);
  const [initializing, setInitializing] = useState(true);

  // Restore a persisted session on cold start.
  useEffect(() => {
    (async () => {
      const [t, e] = await Promise.all([
        tokenStore.get(TOKEN_KEY),
        tokenStore.get(EMAIL_KEY),
      ]);
      if (t) {
        setAuthToken(t);
        setToken(t);
        setEmail(e);
      }
      setInitializing(false);
    })();
  }, []);

  async function persist(t: string, e: string) {
    setAuthToken(t);
    setToken(t);
    setEmail(e);
    await Promise.all([tokenStore.set(TOKEN_KEY, t), tokenStore.set(EMAIL_KEY, e)]);
  }

  const value = useMemo<AuthState>(
    () => ({
      token,
      email,
      initializing,
      login: async (em, pw) => {
        const res = await endpoints.login(em, pw);
        await persist(res.token, res.email);
      },
      register: async (em, pw) => {
        const res = await endpoints.register(em, pw);
        await persist(res.token, res.email);
      },
      logout: async () => {
        setAuthToken(null);
        setToken(null);
        setEmail(null);
        await Promise.all([tokenStore.remove(TOKEN_KEY), tokenStore.remove(EMAIL_KEY)]);
      },
    }),
    [token, email, initializing],
  );

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}

export function useAuth(): AuthState {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error('useAuth must be used within AuthProvider');
  return ctx;
}
