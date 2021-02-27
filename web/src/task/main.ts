import { get, patch } from '../client';
import { Store } from '../store';
import { DateTime } from 'luxon';

/// Interfaces
export type RepeatUnit = "Day" | "Month";

export type RepeatBehavior = "FromScheduled" | "FromScheduledInFuture" | "FromCompleted";

export interface Task {
  text: string;
  note: string;
  completed: boolean;
  at: DateTime;
  postponed_at?: DateTime;
  repeat_value: number;
  repeat_unit: RepeatUnit;
  repeat_behavior: RepeatBehavior;
  category: string;
}

export type TaskWithId = [number, Task];

export type RawTask = Omit<Task, 'at|postponed_at'> & { 
  at: string;
  postponed_at: string;
};

export type RawTaskWithId = [number, RawTask];

// Methods on Interface
export function getPostponedOrScheduledTime(task: Task): DateTime {
  return task.postponed_at != null ? task.postponed_at : task.at;
}



// Store
export type TaskStore = {
  tasks: Store<TaskWithId[]>;
  selectedCategories: Store<Set<string>>;
}

/// Store Actions
let taskStore: TaskStore = null;

export const getTaskStore = async (): Promise<TaskStore> => {
  if (taskStore != null) {
    return taskStore;
  }
  const rawTasks: RawTaskWithId[] = await get('/task');
  const tasks: TaskWithId[] = rawTasks.map(task => {
    return [task[0], {
      ...task[1], 
      at: DateTime.fromISO(task[1].at),
      postponed_at: task[1].postponed_at != null ? DateTime.fromISO(task[1].postponed_at) : null,
    }];
  });
  let allCategories = new Set(tasks.map(task => task[1].category));
  taskStore = {
    tasks: new Store(tasks),
    selectedCategories: new Store(allCategories),
  };
  return taskStore;
}

export const addTask = (task: TaskWithId) => (tasks: TaskWithId[]) => {
  tasks.push(task);
  return tasks;
}

export const removeTask = (id: number) => (tasks: TaskWithId[]) => {
  return tasks.filter(task => task[0] !== id);
}

export const updateTask = (newTask: TaskWithId) => (tasks: TaskWithId[]) => {
  return tasks.map(task => task[0] === newTask[0] ? newTask : task);
}

export const toggleCategorySelection = (category: string) => (categories: Set<string>) => {
  if (categories.has(category)) {
    categories.delete(category);
  } else {
    categories.add(category);
  }
  return categories;
}

export const getNewTask = (): Task => ({
  text: "Text",
  note: "Note",
  completed: false,
  at: DateTime.local(),
  repeat_value: 0,
  repeat_unit: "Day",
  repeat_behavior: "FromScheduled",
  category: "Task"
});


/// Network
export const patchTask = (id: number): Promise<string> => patch(`/task/${id}/complete`);