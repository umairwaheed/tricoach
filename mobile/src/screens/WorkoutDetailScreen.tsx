import React, { useState } from 'react';
import { KeyboardAvoidingView, Platform, ScrollView, StyleSheet, Text, View } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import type { NativeStackScreenProps } from '@react-navigation/native-stack';
import { Button, Card, ErrorText, Pill, TextField } from '../components/ui';
import * as endpoints from '../api/endpoints';
import { apiErrorMessage } from '../api/client';
import { colors, disciplineColors, disciplineIcon, font, intensityColor, radius, spacing } from '../theme';
import { formatDayMonth, weekdayShort } from '../lib/date';
import type { AppStackParams } from '../navigation/types';

type Props = NativeStackScreenProps<AppStackParams, 'WorkoutDetail'>;

export default function WorkoutDetailScreen({ route, navigation }: Props) {
  const { workout } = route.params;
  const qc = useQueryClient();
  const color = disciplineColors[workout.discipline] ?? colors.primary;
  const isRest = workout.discipline === 'rest';

  const [duration, setDuration] = useState(
    workout.planned_duration_min ? String(workout.planned_duration_min) : '',
  );
  const [distance, setDistance] = useState('');
  const [avgHr, setAvgHr] = useState('');
  const [maxHr, setMaxHr] = useState('');
  const [rpe, setRpe] = useState('');
  const [notes, setNotes] = useState('');
  const [error, setError] = useState('');

  // Load any previously-saved feedback for completed workouts.
  const existing = useQuery({
    queryKey: ['feedback', workout.id],
    queryFn: () => endpoints.getFeedback(workout.id),
    enabled: workout.status === 'completed',
    retry: false,
  });

  const submit = useMutation({
    mutationFn: () =>
      endpoints.submitFeedback(workout.id, {
        actual_duration_min: duration ? Number(duration) : null,
        actual_distance_km: distance ? Number(distance) : null,
        avg_hr: avgHr ? Number(avgHr) : null,
        max_hr: maxHr ? Number(maxHr) : null,
        perceived_effort: rpe ? Number(rpe) : null,
        notes,
      }),
    onSuccess: async (fb) => {
      qc.setQueryData(['feedback', workout.id], fb);
      await qc.invalidateQueries({ queryKey: ['activePlan'] });
    },
    onError: (e) => setError(apiErrorMessage(e)),
  });

  const skip = useMutation({
    mutationFn: () => endpoints.updateWorkoutStatus(workout.id, 'skipped'),
    onSuccess: async () => {
      await qc.invalidateQueries({ queryKey: ['activePlan'] });
      navigation.goBack();
    },
  });

  const aiFeedback = submit.data?.ai_feedback ?? existing.data?.ai_feedback;

  return (
    <SafeAreaView style={styles.safe} edges={['bottom']}>
      <KeyboardAvoidingView
        behavior={Platform.OS === 'ios' ? 'padding' : undefined}
        style={{ flex: 1 }}
      >
        <ScrollView contentContainerStyle={styles.content} keyboardShouldPersistTaps="handled">
          <View style={styles.headerRow}>
            <View style={[styles.icon, { backgroundColor: `${color}22` }]}>
              <Text style={{ fontSize: 26 }}>{disciplineIcon[workout.discipline]}</Text>
            </View>
            <View style={{ flex: 1 }}>
              <Text style={styles.title}>{workout.title}</Text>
              <Text style={styles.date}>
                {weekdayShort(workout.scheduled_date)} · {formatDayMonth(workout.scheduled_date)} · Week {workout.week_number}
              </Text>
            </View>
          </View>

          <View style={styles.pills}>
            {!isRest && (
              <Pill label={workout.intensity} color={intensityColor[workout.intensity]} />
            )}
            {workout.planned_duration_min > 0 && (
              <Pill label={`${workout.planned_duration_min} min`} color={colors.primary} />
            )}
            {workout.planned_distance_km ? (
              <Pill label={`${workout.planned_distance_km.toFixed(1)} km`} color={colors.primary} />
            ) : null}
            {workout.status === 'completed' && <Pill label="Completed" color={colors.success} />}
          </View>

          <Card style={{ marginTop: spacing(2) }}>
            <Text style={styles.sectionLabel}>The session</Text>
            <Text style={styles.description}>{workout.description}</Text>
          </Card>

          {aiFeedback && (
            <Card style={[styles.aiCard, { marginTop: spacing(2) }]}>
              <Text style={styles.aiLabel}>🤖 Coach feedback</Text>
              <Text style={styles.aiText}>{aiFeedback}</Text>
            </Card>
          )}

          {!isRest && (
            <>
              <Text style={styles.logTitle}>Log your workout</Text>
              <Card>
                <View style={styles.row}>
                  <View style={styles.half}>
                    <TextField label="Duration (min)" value={duration} onChangeText={setDuration} keyboardType="number-pad" />
                  </View>
                  <View style={styles.half}>
                    <TextField label="Distance (km)" value={distance} onChangeText={setDistance} keyboardType="decimal-pad" placeholder="—" />
                  </View>
                </View>
                <View style={styles.row}>
                  <View style={styles.half}>
                    <TextField label="Avg HR" value={avgHr} onChangeText={setAvgHr} keyboardType="number-pad" placeholder="—" />
                  </View>
                  <View style={styles.half}>
                    <TextField label="Max HR" value={maxHr} onChangeText={setMaxHr} keyboardType="number-pad" placeholder="—" />
                  </View>
                </View>
                <TextField label="Perceived effort (RPE 1–10)" value={rpe} onChangeText={setRpe} keyboardType="number-pad" placeholder="7" />
                <TextField label="Notes" value={notes} onChangeText={setNotes} placeholder="How did it feel?" multiline />
              </Card>

              <View style={{ height: spacing(2) }} />
              {error ? <ErrorText message={error} /> : null}
              <Button
                title={submit.isSuccess ? 'Update log' : 'Save & get feedback'}
                variant="success"
                onPress={() => {
                  setError('');
                  submit.mutate();
                }}
                loading={submit.isPending}
              />
              {workout.status === 'scheduled' && (
                <>
                  <View style={{ height: spacing(1.5) }} />
                  <Button title="Skip this workout" variant="ghost" onPress={() => skip.mutate()} loading={skip.isPending} />
                </>
              )}
            </>
          )}
          <View style={{ height: spacing(4) }} />
        </ScrollView>
      </KeyboardAvoidingView>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  safe: { flex: 1, backgroundColor: colors.bg },
  content: { padding: spacing(2.5) },
  headerRow: { flexDirection: 'row', alignItems: 'center', gap: spacing(1.5) },
  icon: { width: 56, height: 56, borderRadius: radius.md, alignItems: 'center', justifyContent: 'center' },
  title: { color: colors.text, fontSize: font.h2, fontWeight: '800' },
  date: { color: colors.textMuted, fontSize: font.small, marginTop: 2 },
  pills: { flexDirection: 'row', flexWrap: 'wrap', gap: spacing(1), marginTop: spacing(2) },
  sectionLabel: { color: colors.textMuted, fontSize: font.small, fontWeight: '700', marginBottom: 6, textTransform: 'uppercase' },
  description: { color: colors.text, fontSize: font.body, lineHeight: 22 },
  aiCard: { backgroundColor: `${colors.primary}14`, borderColor: `${colors.primary}55` },
  aiLabel: { color: colors.primary, fontSize: font.small, fontWeight: '800', marginBottom: 6 },
  aiText: { color: colors.text, fontSize: font.body, lineHeight: 22 },
  logTitle: { color: colors.text, fontSize: font.h3, fontWeight: '800', marginTop: spacing(2.5), marginBottom: spacing(1) },
  row: { flexDirection: 'row', gap: spacing(1.5) },
  half: { flex: 1 },
});
