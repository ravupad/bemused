import {DateTime} from 'luxon';

type RepeatUnit = "Day" | "Week" | "Year" | "Month";

type RepeatBehavior = "FromScheduled" | "FromScheduledInFuture" | "FromCompleted";

interface Task {
  text: string,
  note: string,
  completed: boolean,
  at: DateTime,
  repeat_value: number;
  repeat_unit: RepeatUnit;
  repeat_behavior: RepeatBehavior;
  category: string;
}

type TaskWithId = [number, Task];

type RawTask = Omit<Task, 'at'> & { at: string };

type RawTaskWithId = [number, RawTask];

export {
  Task, TaskWithId, RawTask, RawTaskWithId
}
