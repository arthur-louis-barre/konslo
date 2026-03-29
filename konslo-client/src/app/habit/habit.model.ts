export type GoalPeriod = 'day' | 'week' | 'month';

export interface Habit {
  id: number;
  name: string;
  goal_value: number;
  goal_unit: string;
  goal_period: GoalPeriod;
  created_at: string;
}

export interface HabitWithCheck extends Habit {
  checks: Check[];
}

export interface CreateHabitRequest {
  name: string;
  goal_value: number;
  goal_unit: string;
  goal_period: GoalPeriod;
}

export interface Check {
  id: number;
  habit_id: number;
  value: number;
  checked_at: string;
}

export interface CreateCheckRequest {
  value: number;
  checked_at: string;
}

export interface UpdateCheckRequest {
  value: number;
}