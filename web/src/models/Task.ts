import {DateTime} from 'luxon';

export interface Task {
  id: number;
  text: string;
  note: string;
  schedule_time: string;
  schedule_interval_value: number;
  schedule_interval_type: ScheduleIntervalType;
  completed: boolean;
  category: string;
}

export type ScheduleIntervalType = 'Day' | 'Week' | 'Month' | 'Year';
export const ScheduleIntervalTypes = ['Day', 'Week', 'Month', 'Year'];

export function newTask(category: string): Task {
  return {
    id: 0,
    text: 'New Task',
    note: '',
    schedule_time: DateTime.local().toISO(),
    schedule_interval_value: 0,
    schedule_interval_type: 'Day',
    completed: false,
    category,
    };
}
