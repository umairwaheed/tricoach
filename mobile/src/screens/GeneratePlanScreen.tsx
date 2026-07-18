import React, { useState } from 'react';
import { ScrollView, StyleSheet, Text, View } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import type { NativeStackScreenProps } from '@react-navigation/native-stack';
import { Button, Card, ErrorText, TextField } from '../components/ui';
import * as endpoints from '../api/endpoints';
import { apiErrorMessage } from '../api/client';
import { colors, font, radius, spacing } from '../theme';
import type { RaceDistance } from '../api/types';
import { addDays, daysUntil, todayISO } from '../lib/date';
import type { AppStackParams } from '../navigation/types';

type Props = NativeStackScreenProps<AppStackParams, 'GeneratePlan'>;

const DISTANCES: { value: RaceDistance; label: string; blurb: string; weeks: number }[] = [
  { value: 'sprint', label: 'Sprint', blurb: '750m swim · 20k bike · 5k run', weeks: 8 },
  { value: 'olympic', label: 'Olympic', blurb: '1.5k swim · 40k bike · 10k run', weeks: 12 },
  { value: 'half_ironman', label: 'Half Ironman (70.3)', blurb: '1.9k swim · 90k bike · 21k run', weeks: 16 },
  { value: 'ironman', label: 'Ironman (140.6)', blurb: '3.8k swim · 180k bike · 42k run', weeks: 24 },
];

export default function GeneratePlanScreen({ navigation }: Props) {
  const qc = useQueryClient();
  const [distance, setDistance] = useState<RaceDistance>('olympic');
  const selected = DISTANCES.find((d) => d.value === distance)!;
  const [raceDate, setRaceDate] = useState(addDays(todayISO(), selected.weeks * 7));
  const [error, setError] = useState('');

  const mutation = useMutation({
    mutationFn: () => endpoints.generatePlan({ race_distance: distance, race_date: raceDate }),
    onSuccess: async () => {
      await qc.invalidateQueries({ queryKey: ['activePlan'] });
      navigation.replace('Tabs');
    },
    onError: (e) => setError(apiErrorMessage(e)),
  });

  function pickDistance(d: RaceDistance) {
    setDistance(d);
    const wk = DISTANCES.find((x) => x.value === d)!.weeks;
    setRaceDate(addDays(todayISO(), wk * 7));
  }

  const until = daysUntil(raceDate);

  return (
    <SafeAreaView style={styles.safe} edges={['bottom']}>
      <ScrollView contentContainerStyle={styles.content}>
        <Text style={styles.title}>Pick your race</Text>
        <Text style={styles.subtitle}>We build a periodised plan that peaks on race day.</Text>

        <View style={{ height: spacing(2) }} />
        {DISTANCES.map((d) => {
          const active = d.value === distance;
          return (
            <View key={d.value} style={{ marginBottom: spacing(1) }}>
              <Card
                style={{
                  borderColor: active ? colors.primary : colors.border,
                  backgroundColor: active ? colors.surfaceAlt : colors.surface,
                }}
              >
                <Text style={styles.distanceLabel} onPress={() => pickDistance(d.value)}>
                  {active ? '● ' : '○ '}
                  {d.label}
                </Text>
                <Text style={styles.distanceBlurb} onPress={() => pickDistance(d.value)}>
                  {d.blurb} · ~{d.weeks} weeks
                </Text>
              </Card>
            </View>
          );
        })}

        <View style={{ height: spacing(2) }} />
        <TextField
          label="Race date (YYYY-MM-DD)"
          value={raceDate}
          onChangeText={setRaceDate}
          placeholder="2026-10-11"
          autoCapitalize="none"
        />
        <View style={styles.countdown}>
          <Text style={styles.countdownText}>
            {until > 0 ? `${until} days until race day 🏁` : 'Pick a date in the future'}
          </Text>
        </View>

        <View style={{ height: spacing(3) }} />
        {error ? <ErrorText message={error} /> : null}
        <Button
          title="Generate my plan"
          onPress={() => {
            setError('');
            mutation.mutate();
          }}
          loading={mutation.isPending}
          disabled={until <= 0}
        />
      </ScrollView>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  safe: { flex: 1, backgroundColor: colors.bg },
  content: { padding: spacing(3) },
  title: { fontSize: font.h2, fontWeight: '800', color: colors.text },
  subtitle: { fontSize: font.body, color: colors.textMuted, marginTop: spacing(0.5) },
  distanceLabel: { color: colors.text, fontSize: font.h3, fontWeight: '700' },
  distanceBlurb: { color: colors.textMuted, fontSize: font.small, marginTop: 4 },
  countdown: {
    backgroundColor: `${colors.primary}18`,
    borderRadius: radius.md,
    padding: spacing(1.5),
    marginTop: spacing(1),
  },
  countdownText: { color: colors.primary, fontWeight: '700', textAlign: 'center' },
});
