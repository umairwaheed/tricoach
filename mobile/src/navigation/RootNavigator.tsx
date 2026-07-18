import React from 'react';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import { createBottomTabNavigator } from '@react-navigation/bottom-tabs';
import { Ionicons } from '@expo/vector-icons';
import { useAuth } from '../state/auth';
import { Loading } from '../components/ui';
import { colors } from '../theme';
import type { AppStackParams, AuthStackParams, TabParams } from './types';

import LoginScreen from '../screens/LoginScreen';
import RegisterScreen from '../screens/RegisterScreen';
import OnboardingScreen from '../screens/OnboardingScreen';
import GeneratePlanScreen from '../screens/GeneratePlanScreen';
import TodayScreen from '../screens/TodayScreen';
import PlanScreen from '../screens/PlanScreen';
import ScheduleScreen from '../screens/ScheduleScreen';
import WorkoutDetailScreen from '../screens/WorkoutDetailScreen';

const AuthStack = createNativeStackNavigator<AuthStackParams>();
const AppStack = createNativeStackNavigator<AppStackParams>();
const Tabs = createBottomTabNavigator<TabParams>();

const headerStyle = {
  headerStyle: { backgroundColor: colors.bg },
  headerTintColor: colors.text,
  headerShadowVisible: false,
} as const;

function TabsNavigator() {
  return (
    <Tabs.Navigator
      screenOptions={({ route }) => ({
        headerShown: false,
        tabBarStyle: { backgroundColor: colors.surface, borderTopColor: colors.border, height: 62, paddingBottom: 8, paddingTop: 6 },
        tabBarActiveTintColor: colors.primary,
        tabBarInactiveTintColor: colors.textMuted,
        tabBarIcon: ({ color, size }) => {
          const icon =
            route.name === 'Today' ? 'today' : route.name === 'Plan' ? 'calendar' : 'time';
          return <Ionicons name={icon as any} size={size} color={color} />;
        },
      })}
    >
      <Tabs.Screen name="Today" component={TodayScreen} />
      <Tabs.Screen name="Plan" component={PlanScreen} />
      <Tabs.Screen name="Schedule" component={ScheduleScreen} />
    </Tabs.Navigator>
  );
}

function AuthNavigator() {
  return (
    <AuthStack.Navigator screenOptions={{ ...headerStyle, headerShown: false }}>
      <AuthStack.Screen name="Login" component={LoginScreen} />
      <AuthStack.Screen name="Register" component={RegisterScreen} />
    </AuthStack.Navigator>
  );
}

function AppNavigator() {
  return (
    <AppStack.Navigator screenOptions={headerStyle}>
      <AppStack.Screen name="Tabs" component={TabsNavigator} options={{ headerShown: false }} />
      <AppStack.Screen name="Onboarding" component={OnboardingScreen} options={{ title: 'Your profile' }} />
      <AppStack.Screen name="GeneratePlan" component={GeneratePlanScreen} options={{ title: 'New plan' }} />
      <AppStack.Screen
        name="WorkoutDetail"
        component={WorkoutDetailScreen}
        options={{ title: 'Workout', headerBackTitle: 'Back' }}
      />
    </AppStack.Navigator>
  );
}

export default function RootNavigator() {
  const { token, initializing } = useAuth();
  if (initializing) return <Loading label="Starting TriCoach…" />;
  return token ? <AppNavigator /> : <AuthNavigator />;
}
