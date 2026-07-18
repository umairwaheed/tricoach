import React, { useMemo, useState } from 'react';
import { Pressable, ScrollView, StyleSheet, Text, View } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import { useNavigation } from '@react-navigation/native';
import type { NativeStackNavigationProp } from '@react-navigation/native-stack';
import { Card, Loading } from '../components/ui';
import { WorkoutCard } from '../components/WorkoutCard';
import { useActivePlan } from '../hooks';
import { colors, font, radius, spacing } from '../theme';
import { formatDayMonth, todayISO, weekdayShort } from '../lib/date';
import type { AppStackParams } from '../navigation/types';
import type { Workout } from '../api/types';

type Nav = NativeStackNavigationProp<AppStackParams>;

export default function PlanScreen() {
  const navigation = useNavigation<Nav>();
  const plan = useActivePlan();

  const weeks = useMemo(() => {
    if (!plan.data) return [];
    const set = Array.from(new Set(plan.data.workouts.map((w) => w.week_number)));
    return set.sort((a, b) => a - b);
  }, [plan.data]);

  // Default to the week containing today.
  const currentWeek = useMemo(() => {
    if (!plan.data) return 1;
    const today = todayISO();
    const match = plan.data.workouts.find((w) => w.scheduled_date >= today);
    return match?.week_number ?? weeks[0] ?? 1;
  }, [plan.data, weeks]);

  const [selected, setSelected] = useState<number | null>(null);
  const activeWeek = selected ?? currentWeek;

  if (plan.isLoading) return <Loading />;
  if (!plan.data) {
    return (
      <SafeAreaView style={styles.safe}>
        <View style={styles.center}>
          <Text style={styles.muted}>No active plan yet.</Text>
        </View>
      </SafeAreaView>
    );
  }

  const p = plan.data;
  const weekWorkouts = p.workouts
    .filter((w) => w.week_number === activeWeek)
    .sort((a, b) => a.scheduled_date.localeCompare(b.scheduled_date));

  const weekMinutes = weekWorkouts.reduce((sum, w) => sum + w.planned_duration_min, 0);
  const phase = phaseFor(activeWeek, p.total_weeks);

  const openWorkout = (workout: Workout) => navigation.navigate('WorkoutDetail', { workout });

  return (
    <SafeAreaView style={styles.safe} edges={['top']}>
      <View style={styles.header}>
        <Text style={styles.title}>Training Plan</Text>
        <Text style={styles.subtitle}>
          {p.total_weeks} weeks · {labelForDistance(p.race_distance)}
        </Text>
      </View>

      <ScrollView
        horizontal
        showsHorizontalScrollIndicator={false}
        contentContainerStyle={styles.weekBar}
      >
        {weeks.map((wk) => {
          const active = wk === activeWeek;
          return (
            <Pressable key={wk} onPress={() => setSelected(wk)} style={[styles.weekPill, active && styles.weekPillActive]}>
              <Text style={[styles.weekPillText, active && styles.weekPillTextActive]}>W{wk}</Text>
            </Pressable>
          );
        })}
      </ScrollView>

      <ScrollView contentContainerStyle={styles.content}>
        <Card style={styles.weekHeader}>
          <View>
            <Text style={styles.weekTitle}>Week {activeWeek}</Text>
            <Text style={styles.phase}>{phase}</Text>
          </View>
          <View style={{ alignItems: 'flex-end' }}>
            <Text style={styles.hours}>{(weekMinutes / 60).toFixed(1)}h</Text>
            <Text style={styles.phase}>planned</Text>
          </View>
        </Card>

        {weekWorkouts.map((w) => (
          <View key={w.id} style={styles.dayRow}>
            <View style={styles.dayCol}>
              <Text style={styles.dayName}>{weekdayShort(w.scheduled_date)}</Text>
              <Text style={styles.dayNum}>{formatDayMonth(w.scheduled_date).split(' ')[1]}</Text>
            </View>
            <View style={{ flex: 1 }}>
              <WorkoutCard workout={w} onPress={() => openWorkout(w)} />
            </View>
          </View>
        ))}
        <View style={{ height: spacing(4) }} />
      </ScrollView>
    </SafeAreaView>
  );
}

function phaseFor(week: number, total: number): string {
  if (week >= total - 1) return 'Taper · race prep';
  if (week % 4 === 0) return 'Recovery week';
  if (week <= total / 3) return 'Base · aerobic foundation';
  if (week <= (2 * total) / 3) return 'Build · raising the ceiling';
  return 'Peak · race-specific';
}

function labelForDistance(d: string): string {
  return (
    { sprint: 'Sprint', olympic: 'Olympic', half_ironman: 'Half Ironman', ironman: 'Ironman' }[d] ?? d
  );
}

const styles = StyleSheet.create({
  safe: { flex: 1, backgroundColor: colors.bg },
  header: { paddingHorizontal: spacing(2.5), paddingTop: spacing(1) },
  title: { fontSize: font.h2, fontWeight: '800', color: colors.text },
  subtitle: { fontSize: font.small, color: colors.textMuted, marginTop: 2 },
  weekBar: { paddingHorizontal: spacing(2.5), paddingVertical: spacing(1.5), gap: spacing(1) },
  weekPill: {
    paddingHorizontal: spacing(1.5),
    paddingVertical: spacing(0.75),
    borderRadius: radius.pill,
    backgroundColor: colors.surface,
    borderWidth: 1,
    borderColor: colors.border,
    minWidth: 46,
    alignItems: 'center',
  },
  weekPillActive: { backgroundColor: colors.primary, borderColor: colors.primary },
  weekPillText: { color: colors.textMuted, fontWeight: '700', fontSize: font.small },
  weekPillTextActive: { color: colors.bg },
  content: { paddingHorizontal: spacing(2.5) },
  weekHeader: { flexDirection: 'row', justifyContent: 'space-between', alignItems: 'center', marginBottom: spacing(1.5) },
  weekTitle: { color: colors.text, fontSize: font.h3, fontWeight: '800' },
  phase: { color: colors.textMuted, fontSize: font.small, marginTop: 2 },
  hours: { color: colors.primary, fontSize: font.h3, fontWeight: '800' },
  dayRow: { flexDirection: 'row', alignItems: 'center' },
  dayCol: { width: 44, alignItems: 'center', marginRight: spacing(1), marginBottom: spacing(1) },
  dayName: { color: colors.textMuted, fontSize: font.tiny, fontWeight: '700', textTransform: 'uppercase' },
  dayNum: { color: colors.text, fontSize: font.h3, fontWeight: '800' },
  center: { flex: 1, alignItems: 'center', justifyContent: 'center' },
  muted: { color: colors.textMuted },
});
