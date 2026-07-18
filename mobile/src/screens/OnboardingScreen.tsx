import React, { useState } from 'react';
import { KeyboardAvoidingView, Platform, ScrollView, StyleSheet, Text, View } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import type { NativeStackScreenProps } from '@react-navigation/native-stack';
import { Button, Card, ErrorText, SegmentedControl, TextField } from '../components/ui';
import * as endpoints from '../api/endpoints';
import { apiErrorMessage } from '../api/client';
import { colors, font, spacing } from '../theme';
import type { ExperienceLevel } from '../api/types';
import type { AppStackParams } from '../navigation/types';

type Props = NativeStackScreenProps<AppStackParams, 'Onboarding'>;

export default function OnboardingScreen({ navigation }: Props) {
  const qc = useQueryClient();
  const [name, setName] = useState('');
  const [age, setAge] = useState('');
  const [weight, setWeight] = useState('');
  const [hours, setHours] = useState('');
  const [restingHr, setRestingHr] = useState('');
  const [maxHr, setMaxHr] = useState('');
  const [experience, setExperience] = useState<ExperienceLevel>('beginner');
  const [error, setError] = useState('');

  const mutation = useMutation({
    mutationFn: () =>
      endpoints.upsertProfile({
        display_name: name.trim(),
        age: Number(age),
        weight_kg: Number(weight),
        experience_level: experience,
        weekly_hours_available: Number(hours),
        resting_hr: restingHr ? Number(restingHr) : null,
        max_hr: maxHr ? Number(maxHr) : null,
      }),
    onSuccess: async () => {
      await qc.invalidateQueries({ queryKey: ['profile'] });
      navigation.replace('GeneratePlan');
    },
    onError: (e) => setError(apiErrorMessage(e)),
  });

  function onSubmit() {
    setError('');
    if (!name.trim()) return setError('Please enter your name.');
    if (!age || !weight || !hours) return setError('Age, weight and weekly hours are required.');
    mutation.mutate();
  }

  return (
    <SafeAreaView style={styles.safe} edges={['bottom']}>
      <KeyboardAvoidingView
        behavior={Platform.OS === 'ios' ? 'padding' : undefined}
        style={{ flex: 1 }}
      >
        <ScrollView contentContainerStyle={styles.content} keyboardShouldPersistTaps="handled">
          <Text style={styles.title}>Tell us about you</Text>
          <Text style={styles.subtitle}>
            We tailor training volume and intensity to your level.
          </Text>

          <View style={{ height: spacing(2) }} />
          {error ? <ErrorText message={error} /> : null}

          <Card>
            <TextField label="Name" value={name} onChangeText={setName} placeholder="Alex" />
            <View style={styles.row}>
              <View style={styles.half}>
                <TextField label="Age" value={age} onChangeText={setAge} keyboardType="number-pad" placeholder="34" />
              </View>
              <View style={styles.half}>
                <TextField label="Weight (kg)" value={weight} onChangeText={setWeight} keyboardType="decimal-pad" placeholder="72" />
              </View>
            </View>

            <Text style={styles.label}>Experience level</Text>
            <SegmentedControl
              value={experience}
              onChange={setExperience}
              options={[
                { label: 'Beginner', value: 'beginner' },
                { label: 'Intermediate', value: 'intermediate' },
                { label: 'Advanced', value: 'advanced' },
              ]}
            />

            <View style={{ height: spacing(2) }} />
            <TextField
              label="Hours available per week"
              value={hours}
              onChangeText={setHours}
              keyboardType="decimal-pad"
              placeholder="8"
            />
            <View style={styles.row}>
              <View style={styles.half}>
                <TextField label="Resting HR (optional)" value={restingHr} onChangeText={setRestingHr} keyboardType="number-pad" placeholder="52" />
              </View>
              <View style={styles.half}>
                <TextField label="Max HR (optional)" value={maxHr} onChangeText={setMaxHr} keyboardType="number-pad" placeholder="186" />
              </View>
            </View>
          </Card>

          <View style={{ height: spacing(3) }} />
          <Button title="Continue" onPress={onSubmit} loading={mutation.isPending} />
        </ScrollView>
      </KeyboardAvoidingView>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  safe: { flex: 1, backgroundColor: colors.bg },
  content: { padding: spacing(3) },
  title: { fontSize: font.h2, fontWeight: '800', color: colors.text },
  subtitle: { fontSize: font.body, color: colors.textMuted, marginTop: spacing(0.5) },
  row: { flexDirection: 'row', gap: spacing(1.5) },
  half: { flex: 1 },
  label: { color: colors.textMuted, fontSize: font.small, marginBottom: 6, fontWeight: '600' },
});
