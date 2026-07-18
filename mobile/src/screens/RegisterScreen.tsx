import React, { useState } from 'react';
import { KeyboardAvoidingView, Platform, ScrollView, StyleSheet, Text } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import type { NativeStackScreenProps } from '@react-navigation/native-stack';
import { Button, ErrorText, TextField } from '../components/ui';
import { useAuth } from '../state/auth';
import { apiErrorMessage } from '../api/client';
import { colors, font, spacing } from '../theme';
import type { AuthStackParams } from '../navigation/types';

type Props = NativeStackScreenProps<AuthStackParams, 'Register'>;

export default function RegisterScreen({ navigation }: Props) {
  const { register } = useAuth();
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);

  async function onSubmit() {
    setError('');
    if (password.length < 8) {
      setError('Password must be at least 8 characters.');
      return;
    }
    setLoading(true);
    try {
      await register(email.trim(), password);
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
          <Text style={styles.title}>Create your account</Text>
          <Text style={styles.subtitle}>Start training smarter in two minutes.</Text>
          <ScrollSpacer />
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
            placeholder="At least 8 characters"
          />
          <Button title="Sign up" onPress={onSubmit} loading={loading} />
          <ScrollSpacer />
          <Button title="I already have an account" variant="ghost" onPress={() => navigation.goBack()} />
        </ScrollView>
      </KeyboardAvoidingView>
    </SafeAreaView>
  );
}

const ScrollSpacer = () => <Text style={{ height: spacing(2) }} />;

const styles = StyleSheet.create({
  safe: { flex: 1, backgroundColor: colors.bg },
  content: { padding: spacing(3), paddingTop: spacing(6) },
  title: { fontSize: font.h1, fontWeight: '800', color: colors.text },
  subtitle: { fontSize: font.body, color: colors.textMuted, marginTop: spacing(0.5) },
});
