import React from 'react';
import { Pressable, RefreshControl, ScrollView, StyleSheet, Text, View } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import { useNavigation } from '@react-navigation/native';
import type { NativeStackNavigationProp } from '@react-navigation/native-stack';
import { Button, Card, Loading, Pill } from '../components/ui';
import { WorkoutCard } from '../components/WorkoutCard';
import { useAuth } from '../state/auth';
import { useActivePlan, useProfile } from '../hooks';
import { colors, font, radius, spacing } from '../theme';
import { daysUntil, formatDayMonth, todayISO } from '../lib/date';
import type { AppStackParams } from '../navigation/types';
import type { Workout } from '../api/types';

type Nav = NativeStackNavigationProp<AppStackParams>;

export default function TodayScreen() {
  const navigation = useNavigation<Nav>();
  const { logout } = useAuth();
  const profile = useProfile();
  const plan = useActivePlan();

  if (profile.isLoading || plan.isLoading) return <Loading label="Loading your training…" />;

  // Empty states drive the user through onboarding → plan generation.
  if (!profile.data) {
    return (
      <EmptyState
        title="Welcome to TriCoach 👋"
        body="Set up your athlete profile so we can build a plan that fits your level."
        cta="Set up profile"
        onPress={() => navigation.navigate('Onboarding')}
      />
    );
  }
  if (!plan.data) {
    return (
      <EmptyState
        title={`Hi ${profile.data.display_name} 👋`}
        body="You don't have an active plan yet. Pick a race and we'll generate one."
        cta="Create a plan"
        onPress={() => navigation.navigate('GeneratePlan')}
      />
    );
  }

  const p = plan.data;
  const today = todayISO();
  const todays = p.workouts.filter((w) => w.scheduled_date === today);
  const upcoming = p.workouts
    .filter((w) => w.scheduled_date > today && w.status === 'scheduled')
    .slice(0, 3);
  const completed = p.workouts.filter((w) => w.status === 'completed').length;
  const trainingTotal = p.workouts.filter((w) => w.discipline !== 'rest').length;
  const until = daysUntil(p.race_date);

  const openWorkout = (workout: Workout) => navigation.navigate('WorkoutDetail', { workout });

  return (
    <SafeAreaView style={styles.safe} edges={['top']}>
      <ScrollView
        contentContainerStyle={styles.content}
        refreshControl={
          <RefreshControl
            refreshing={plan.isRefetching}
            onRefresh={() => plan.refetch()}
            tintColor={colors.primary}
          />
        }
      >
        <View style={styles.topRow}>
          <Text style={styles.hi}>Hi {profile.data.display_name} 👋</Text>
          <Pressable onPress={logout} hitSlop={10}>
            <Text style={styles.logout}>Log out</Text>
          </Pressable>
        </View>

        <Card style={styles.raceCard}>
          <View style={{ flexDirection: 'row', justifyContent: 'space-between', alignItems: 'center' }}>
            <View>
              <Text style={styles.raceLabel}>{labelForDistance(p.race_distance)}</Text>
              <Text style={styles.raceDate}>Race day · {formatDayMonth(p.race_date)}</Text>
            </View>
            <View style={styles.countBubble}>
              <Text style={styles.countNum}>{Math.max(until, 0)}</Text>
              <Text style={styles.countUnit}>days</Text>
            </View>
          </View>
          <Text style={styles.summary}>{p.summary}</Text>
          <View style={{ flexDirection: 'row', gap: spacing(1), marginTop: spacing(1.5) }}>
            <Pill label={p.generated_by === 'gemini' ? 'AI: Gemini' : 'AI coach'} color={colors.primary} />
            <Pill label={`${completed}/${trainingTotal} done`} color={colors.success} />
          </View>
        </Card>

        <Text style={styles.section}>Today · {formatDayMonth(today)}</Text>
        {todays.length > 0 ? (
          todays.map((w) => <WorkoutCard key={w.id} workout={w} onPress={() => openWorkout(w)} />)
        ) : (
          <Card>
            <Text style={styles.restText}>No workout scheduled today. Rest and recover. 😴</Text>
          </Card>
        )}

        {upcoming.length > 0 && (
          <>
            <Text style={styles.section}>Coming up</Text>
            {upcoming.map((w) => (
              <View key={w.id}>
                <Text style={styles.upcomingDate}>
                  {formatDayMonth(w.scheduled_date)}
                </Text>
                <WorkoutCard workout={w} onPress={() => openWorkout(w)} />
              </View>
            ))}
          </>
        )}

        <View style={{ height: spacing(2) }} />
        <Button title="View full plan" variant="ghost" onPress={() => navigation.navigate('Tabs')} />
      </ScrollView>
    </SafeAreaView>
  );
}

function EmptyState({
  title,
  body,
  cta,
  onPress,
}: {
  title: string;
  body: string;
  cta: string;
  onPress: () => void;
}) {
  return (
    <SafeAreaView style={styles.safe}>
      <View style={styles.emptyWrap}>
        <Text style={styles.emptyTitle}>{title}</Text>
        <Text style={styles.emptyBody}>{body}</Text>
        <View style={{ height: spacing(3) }} />
        <Button title={cta} onPress={onPress} />
      </View>
    </SafeAreaView>
  );
}

function labelForDistance(d: string): string {
  return (
    {
      sprint: 'Sprint Triathlon',
      olympic: 'Olympic Triathlon',
      half_ironman: 'Half Ironman 70.3',
      ironman: 'Ironman 140.6',
    }[d] ?? d
  );
}

const styles = StyleSheet.create({
  safe: { flex: 1, backgroundColor: colors.bg },
  content: { padding: spacing(2.5), paddingBottom: spacing(6) },
  topRow: { flexDirection: 'row', justifyContent: 'space-between', alignItems: 'center', marginBottom: spacing(1.5) },
  hi: { fontSize: font.h2, fontWeight: '800', color: colors.text },
  logout: { color: colors.textMuted, fontSize: font.small, fontWeight: '600' },
  raceCard: { marginBottom: spacing(2) },
  raceLabel: { color: colors.text, fontSize: font.h3, fontWeight: '800' },
  raceDate: { color: colors.textMuted, fontSize: font.small, marginTop: 2 },
  countBubble: {
    backgroundColor: `${colors.primary}22`,
    borderRadius: radius.md,
    paddingVertical: spacing(1),
    paddingHorizontal: spacing(1.5),
    alignItems: 'center',
    minWidth: 64,
  },
  countNum: { color: colors.primary, fontSize: font.h2, fontWeight: '800' },
  countUnit: { color: colors.primary, fontSize: font.tiny, fontWeight: '700' },
  summary: { color: colors.textMuted, fontSize: font.body, lineHeight: 21, marginTop: spacing(1.5) },
  section: {
    color: colors.text,
    fontSize: font.h3,
    fontWeight: '800',
    marginTop: spacing(2),
    marginBottom: spacing(1),
  },
  restText: { color: colors.textMuted, fontSize: font.body },
  upcomingDate: { color: colors.textMuted, fontSize: font.tiny, fontWeight: '700', marginBottom: 4, textTransform: 'uppercase' },
  emptyWrap: { flex: 1, justifyContent: 'center', padding: spacing(4) },
  emptyTitle: { fontSize: font.h1, fontWeight: '800', color: colors.text, textAlign: 'center' },
  emptyBody: { fontSize: font.body, color: colors.textMuted, textAlign: 'center', marginTop: spacing(1.5), lineHeight: 22 },
});
