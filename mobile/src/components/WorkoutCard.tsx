import React from 'react';
import { Pressable, StyleSheet, Text, View } from 'react-native';
import { colors, disciplineColors, disciplineIcon, font, intensityColor, radius, spacing } from '../theme';
import type { Workout } from '../api/types';

function formatDistance(km: number | null): string | null {
  if (km == null || km === 0) return null;
  return `${km.toFixed(1)} km`;
}

export function WorkoutCard({
  workout,
  onPress,
}: {
  workout: Workout;
  onPress?: () => void;
}) {
  const color = disciplineColors[workout.discipline] ?? colors.primary;
  const distance = formatDistance(workout.planned_distance_km);
  const done = workout.status === 'completed';
  const skipped = workout.status === 'skipped';

  return (
    <Pressable
      onPress={onPress}
      style={({ pressed }) => [styles.row, pressed && { opacity: 0.85 }]}
    >
      <View style={[styles.accent, { backgroundColor: color }]} />
      <View style={[styles.icon, { backgroundColor: `${color}22` }]}>
        <Text style={{ fontSize: 20 }}>{disciplineIcon[workout.discipline]}</Text>
      </View>
      <View style={{ flex: 1 }}>
        <Text style={[styles.title, done && styles.strike]} numberOfLines={1}>
          {workout.title}
        </Text>
        <Text style={styles.meta}>
          {workout.planned_duration_min > 0
            ? `${workout.planned_duration_min} min`
            : 'Rest'}
          {distance ? ` · ${distance}` : ''}
          {workout.discipline !== 'rest'
            ? ` · ${workout.intensity}`
            : ''}
        </Text>
      </View>
      {done ? (
        <View style={[styles.badge, { backgroundColor: `${colors.success}22` }]}>
          <Text style={[styles.badgeText, { color: colors.success }]}>✓ Done</Text>
        </View>
      ) : skipped ? (
        <View style={[styles.badge, { backgroundColor: `${colors.textMuted}22` }]}>
          <Text style={[styles.badgeText, { color: colors.textMuted }]}>Skipped</Text>
        </View>
      ) : workout.discipline !== 'rest' ? (
        <View
          style={[
            styles.dot,
            { backgroundColor: intensityColor[workout.intensity] ?? colors.primary },
          ]}
        />
      ) : null}
    </Pressable>
  );
}

const styles = StyleSheet.create({
  row: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: colors.surface,
    borderRadius: radius.md,
    borderWidth: 1,
    borderColor: colors.border,
    padding: spacing(1.5),
    marginBottom: spacing(1),
    overflow: 'hidden',
  },
  accent: {
    position: 'absolute',
    left: 0,
    top: 0,
    bottom: 0,
    width: 4,
  },
  icon: {
    width: 42,
    height: 42,
    borderRadius: radius.md,
    alignItems: 'center',
    justifyContent: 'center',
    marginRight: spacing(1.5),
    marginLeft: 4,
  },
  title: { color: colors.text, fontSize: font.body, fontWeight: '700' },
  strike: { textDecorationLine: 'line-through', color: colors.textMuted },
  meta: { color: colors.textMuted, fontSize: font.small, marginTop: 2, textTransform: 'capitalize' },
  badge: { paddingHorizontal: 10, paddingVertical: 5, borderRadius: radius.pill },
  badgeText: { fontSize: font.tiny, fontWeight: '700' },
  dot: { width: 10, height: 10, borderRadius: 5, marginRight: 6 },
});
