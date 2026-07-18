import React, { useMemo, useState } from 'react';
import { ScrollView, StyleSheet, Text, View } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import { useNavigation } from '@react-navigation/native';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import type { NativeStackNavigationProp } from '@react-navigation/native-stack';
import { Button, Card, ErrorText, Loading, TextField } from '../components/ui';
import { WorkoutCard } from '../components/WorkoutCard';
import * as endpoints from '../api/endpoints';
import { apiErrorMessage } from '../api/client';
import { colors, font, radius, spacing } from '../theme';
import { addDays, formatDayMonth, todayISO, weekdayShort } from '../lib/date';
import type { AppStackParams } from '../navigation/types';
import type { ScheduleBlock, Workout } from '../api/types';

type Nav = NativeStackNavigationProp<AppStackParams>;

export default function ScheduleScreen() {
  const navigation = useNavigation<Nav>();
  const qc = useQueryClient();
  const from = todayISO();
  const to = addDays(from, 13);

  const schedule = useQuery({
    queryKey: ['schedule', from, to],
    queryFn: () => endpoints.getSchedule(from, to),
  });

  const [showForm, setShowForm] = useState(false);
  const [title, setTitle] = useState('');
  const [date, setDate] = useState(from);
  const [start, setStart] = useState('09:00');
  const [end, setEnd] = useState('17:00');
  const [error, setError] = useState('');

  const addBlock = useMutation({
    mutationFn: () =>
      endpoints.createScheduleBlock({
        title: title.trim(),
        starts_at: new Date(`${date}T${start}:00`).toISOString(),
        ends_at: new Date(`${date}T${end}:00`).toISOString(),
      }),
    onSuccess: async () => {
      setShowForm(false);
      setTitle('');
      await qc.invalidateQueries({ queryKey: ['schedule'] });
    },
    onError: (e) => setError(apiErrorMessage(e)),
  });

  // Group workouts + busy blocks by day.
  const days = useMemo(() => {
    const list: { date: string; workouts: Workout[]; blocks: ScheduleBlock[] }[] = [];
    for (let i = 0; i < 14; i++) {
      const d = addDays(from, i);
      list.push({
        date: d,
        workouts: schedule.data?.workouts.filter((w) => w.scheduled_date === d) ?? [],
        blocks:
          schedule.data?.busy_blocks.filter((b) => b.starts_at.slice(0, 10) === d) ?? [],
      });
    }
    return list;
  }, [schedule.data, from]);

  if (schedule.isLoading) return <Loading />;

  return (
    <SafeAreaView style={styles.safe} edges={['top']}>
      <View style={styles.header}>
        <View>
          <Text style={styles.title}>Schedule</Text>
          <Text style={styles.subtitle}>Next 14 days · training around your life</Text>
        </View>
        <Button title={showForm ? 'Close' : '+ Busy'} variant="ghost" onPress={() => setShowForm((s) => !s)} />
      </View>

      <ScrollView contentContainerStyle={styles.content}>
        {showForm && (
          <Card style={{ marginBottom: spacing(2) }}>
            <Text style={styles.formTitle}>Add a busy block</Text>
            <Text style={styles.formHint}>
              Mark commitments so training can be planned around them.
            </Text>
            <View style={{ height: spacing(1.5) }} />
            <TextField label="Title" value={title} onChangeText={setTitle} placeholder="Work offsite" />
            <TextField label="Date (YYYY-MM-DD)" value={date} onChangeText={setDate} autoCapitalize="none" />
            <View style={styles.row}>
              <View style={styles.half}>
                <TextField label="Start (HH:MM)" value={start} onChangeText={setStart} />
              </View>
              <View style={styles.half}>
                <TextField label="End (HH:MM)" value={end} onChangeText={setEnd} />
              </View>
            </View>
            {error ? <ErrorText message={error} /> : null}
            <Button
              title="Add block"
              onPress={() => {
                setError('');
                if (!title.trim()) return setError('Give the block a title.');
                addBlock.mutate();
              }}
              loading={addBlock.isPending}
            />
          </Card>
        )}

        {days.map((day) => {
          const isToday = day.date === from;
          const empty = day.workouts.length === 0 && day.blocks.length === 0;
          return (
            <View key={day.date} style={styles.day}>
              <View style={styles.dayHead}>
                <Text style={[styles.dayLabel, isToday && { color: colors.primary }]}>
                  {weekdayShort(day.date)} · {formatDayMonth(day.date)}
                  {isToday ? '  • Today' : ''}
                </Text>
              </View>
              {day.blocks.map((b) => (
                <View key={b.id} style={styles.block}>
                  <Text style={styles.blockText}>📌 {b.title}</Text>
                  <Text style={styles.blockTime}>
                    {b.starts_at.slice(11, 16)}–{b.ends_at.slice(11, 16)}
                  </Text>
                </View>
              ))}
              {day.workouts.map((w) => (
                <WorkoutCard key={w.id} workout={w} onPress={() => navigation.navigate('WorkoutDetail', { workout: w })} />
              ))}
              {empty && <Text style={styles.rest}>Rest day</Text>}
            </View>
          );
        })}
        <View style={{ height: spacing(4) }} />
      </ScrollView>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  safe: { flex: 1, backgroundColor: colors.bg },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    paddingHorizontal: spacing(2.5),
    paddingTop: spacing(1),
  },
  title: { fontSize: font.h2, fontWeight: '800', color: colors.text },
  subtitle: { fontSize: font.small, color: colors.textMuted, marginTop: 2 },
  content: { padding: spacing(2.5), paddingTop: spacing(1.5) },
  day: { marginBottom: spacing(2) },
  dayHead: { marginBottom: spacing(1) },
  dayLabel: { color: colors.text, fontSize: font.small, fontWeight: '800', textTransform: 'uppercase' },
  block: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    backgroundColor: `${colors.warning}18`,
    borderColor: `${colors.warning}44`,
    borderWidth: 1,
    borderRadius: radius.md,
    padding: spacing(1.25),
    marginBottom: spacing(1),
  },
  blockText: { color: colors.warning, fontWeight: '700', fontSize: font.small },
  blockTime: { color: colors.warning, fontSize: font.small },
  rest: { color: colors.textMuted, fontSize: font.small, fontStyle: 'italic' },
  formTitle: { color: colors.text, fontSize: font.h3, fontWeight: '800' },
  formHint: { color: colors.textMuted, fontSize: font.small, marginTop: 2 },
  row: { flexDirection: 'row', gap: spacing(1.5) },
  half: { flex: 1 },
});
