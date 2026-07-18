import React, { useState } from 'react';
import { KeyboardAvoidingView, Platform, ScrollView, StyleSheet, Text, View } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import type { NativeStackScreenProps } from '@react-navigation/native-stack';
import { Button, ErrorText, TextField } from '../components/ui';
import { useAuth } from '../state/auth';
import { apiErrorMessage } from '../api/client';
import { colors, font, spacing } from '../theme';
import type { AuthStackParams } from '../navigation/types';

type Props = NativeStackScreenProps<AuthStackParams, 'Login'>;

export default function LoginScreen({ navigation }: Props) {
  const { login } = useAuth();
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);

  async function onSubmit() {
    setError('');
    setLoading(true);
    try {
      await login(email.trim(), password);
    } catch (e) {
      setError(apiErrorMessage(e));
    } finally {
      setLoading(false);
    }
  }

  return (
    <SafeAreaView style={styles.safe}>
      <KeyboardAvoidingView
        behavior={Platform.OS === 'ios' ? 'padding' : undefined}
        style={{ flex: 1 }}
      >
        <ScrollView contentContainerStyle={styles.content} keyboardShouldPersistTaps="handled">
          <Text style={styles.logo}>🏊‍♂️ 🚴 🏃</Text>
          <Text style={styles.title}>TriCoach</Text>
          <Text style={styles.subtitle}>
            AI-powered training plans for your first triathlon.
          </Text>

          <View style={{ height: spacing(4) }} />

          {error ? <ErrorText message={error} /> : null}
          <TextField
            label="Email"
            value={email}
            onChangeText={setEmail}
            autoCapitalize="none"
            keyboardType="email-address"
            placeholder="you@example.com"
          />
          <TextField
            label="Password"
            value={password}
            onChangeText={setPassword}
            secureTextEntry
            placeholder="••••••••"
          />
          <View style={{ height: spacing(1) }} />
          <Button title="Log in" onPress={onSubmit} loading={loading} />
          <View style={{ height: spacing(2) }} />
          <Button
            title="Create an account"
            variant="ghost"
            onPress={() => navigation.navigate('Register')}
          />
        </ScrollView>
      </KeyboardAvoidingView>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  safe: { flex: 1, backgroundColor: colors.bg },
  content: { padding: spacing(3), paddingTop: spacing(6) },
  logo: { fontSize: 34, textAlign: 'center', marginBottom: spacing(1) },
  title: {
    fontSize: font.h1,
    fontWeight: '800',
    color: colors.text,
    textAlign: 'center',
  },
  subtitle: {
    fontSize: font.body,
    color: colors.textMuted,
    textAlign: 'center',
    marginTop: spacing(1),
    paddingHorizontal: spacing(2),
  },
});
