import React from 'react';
import {
  ActivityIndicator,
  Pressable,
  StyleProp,
  StyleSheet,
  Text,
  TextInput,
  TextInputProps,
  View,
  ViewStyle,
} from 'react-native';
import { colors, font, radius, spacing } from '../theme';

export function Card({
  children,
  style,
}: {
  children: React.ReactNode;
  style?: StyleProp<ViewStyle>;
}) {
  return <View style={[styles.card, style]}>{children}</View>;
}

export function Button({
  title,
  onPress,
  loading,
  variant = 'primary',
  disabled,
}: {
  title: string;
  onPress: () => void;
  loading?: boolean;
  variant?: 'primary' | 'ghost' | 'success';
  disabled?: boolean;
}) {
  const bg =
    variant === 'primary'
      ? colors.primary
      : variant === 'success'
        ? colors.success
        : 'transparent';
  const isDisabled = disabled || loading;
  return (
    <Pressable
      onPress={onPress}
      disabled={isDisabled}
      style={({ pressed }) => [
        styles.button,
        { backgroundColor: bg, opacity: isDisabled ? 0.5 : pressed ? 0.85 : 1 },
        variant === 'ghost' && styles.buttonGhost,
      ]}
    >
      {loading ? (
        <ActivityIndicator color={variant === 'ghost' ? colors.primary : colors.bg} />
      ) : (
        <Text
          style={[
            styles.buttonText,
            { color: variant === 'ghost' ? colors.primary : colors.bg },
          ]}
        >
          {title}
        </Text>
      )}
    </Pressable>
  );
}

export function TextField({
  label,
  ...props
}: TextInputProps & { label: string }) {
  return (
    <View style={{ marginBottom: spacing(2) }}>
      <Text style={styles.label}>{label}</Text>
      <TextInput
        placeholderTextColor={colors.textMuted}
        style={styles.input}
        {...props}
      />
    </View>
  );
}

export function Pill({
  label,
  color = colors.primary,
}: {
  label: string;
  color?: string;
}) {
  return (
    <View style={[styles.pill, { backgroundColor: `${color}22`, borderColor: `${color}55` }]}>
      <Text style={[styles.pillText, { color }]}>{label}</Text>
    </View>
  );
}

export function Loading({ label }: { label?: string }) {
  return (
    <View style={styles.centered}>
      <ActivityIndicator color={colors.primary} size="large" />
      {label ? <Text style={styles.mutedText}>{label}</Text> : null}
    </View>
  );
}

export function ErrorText({ message }: { message: string }) {
  return <Text style={styles.error}>{message}</Text>;
}

export function SegmentedControl<T extends string>({
  options,
  value,
  onChange,
}: {
  options: { label: string; value: T }[];
  value: T;
  onChange: (v: T) => void;
}) {
  return (
    <View style={styles.segment}>
      {options.map((opt) => {
        const active = opt.value === value;
        return (
          <Pressable
            key={opt.value}
            onPress={() => onChange(opt.value)}
            style={[styles.segmentItem, active && styles.segmentItemActive]}
          >
            <Text style={[styles.segmentText, active && styles.segmentTextActive]}>
              {opt.label}
            </Text>
          </Pressable>
        );
      })}
    </View>
  );
}

const styles = StyleSheet.create({
  card: {
    backgroundColor: colors.surface,
    borderRadius: radius.lg,
    padding: spacing(2),
    borderWidth: 1,
    borderColor: colors.border,
  },
  button: {
    height: 52,
    borderRadius: radius.md,
    alignItems: 'center',
    justifyContent: 'center',
    paddingHorizontal: spacing(2),
  },
  buttonGhost: { borderWidth: 1, borderColor: colors.border },
  buttonText: { fontSize: font.body, fontWeight: '700' },
  label: {
    color: colors.textMuted,
    fontSize: font.small,
    marginBottom: 6,
    fontWeight: '600',
  },
  input: {
    backgroundColor: colors.surfaceAlt,
    borderRadius: radius.md,
    borderWidth: 1,
    borderColor: colors.border,
    color: colors.text,
    paddingHorizontal: spacing(1.5),
    height: 48,
    fontSize: font.body,
  },
  pill: {
    paddingHorizontal: 10,
    paddingVertical: 4,
    borderRadius: radius.pill,
    borderWidth: 1,
    alignSelf: 'flex-start',
  },
  pillText: { fontSize: font.tiny, fontWeight: '700', textTransform: 'uppercase' },
  centered: { flex: 1, alignItems: 'center', justifyContent: 'center', padding: spacing(3) },
  mutedText: { color: colors.textMuted, marginTop: spacing(1) },
  error: { color: colors.danger, fontSize: font.small, marginBottom: spacing(1) },
  segment: {
    flexDirection: 'row',
    backgroundColor: colors.surfaceAlt,
    borderRadius: radius.md,
    padding: 4,
    borderWidth: 1,
    borderColor: colors.border,
  },
  segmentItem: {
    flex: 1,
    paddingVertical: 10,
    borderRadius: radius.sm,
    alignItems: 'center',
  },
  segmentItemActive: { backgroundColor: colors.primary },
  segmentText: { color: colors.textMuted, fontWeight: '600', fontSize: font.small },
  segmentTextActive: { color: colors.bg },
});
