import { del, get, patch, post, put } from '../core/client';
import { DateTime } from 'luxon';
import { BehaviorSubject } from 'rxjs';

export type RepeatUnit = "Day" | "Month";

export type RepeatBehavior = "FromScheduled" | "FromScheduledInFuture" | "FromCompleted";

interface RawTask {
  text: string;
  note: string;
  completed: boolean;
  repeat_value: number;
  repeat_unit: RepeatUnit;
  repeat_behavior: RepeatBehavior;
  category: string;
  at: string;
  postponed_at?: string;
};

export interface Task extends RawTask {
  time: DateTime;
  postponedTime?: DateTime;
  id: number;
  visible: boolean;
}

export class Task {
  static new() {
    const raw: RawTask = {
      text: '',
      note: '',
      completed: false,
      repeat_value: 0,
      repeat_unit: "Day",
      repeat_behavior: "FromScheduled",
      category: '',
      at: DateTime.local().toISO(),
    }
    return new Task([-1, raw]);
  }

  constructor(raw: [number, RawTask]) {
    this.time = DateTime.fromISO(raw[1].at);
    if (raw[1].postponed_at != null) {
      this.postponedTime = DateTime.fromISO(raw[1].postponed_at);
    }
    this.id = raw[0];
    this.visible = true;
    Object.assign(this, raw[1]);
  }

  effectiveTime(): DateTime {
    return this.postponedTime || this.time;
  }

  compare(b: Task): number {
    return this.effectiveTime().diff(b.effectiveTime()).as("millisecond")
  }

  period(): number {
    let today = DateTime.local().startOf("day");
    let taskTime = this.effectiveTime().startOf("day");
    let diff = taskTime.diff(today).as("days") + 2;
    if (diff < 0) diff = 0;
    if (diff > 4) diff = 4;
    return diff;
  }

  toggleVisible() {
    this.visible = !this.visible;
  }
};

export class Category {
  enabled: boolean = true;

  constructor(public category: string) {  }

  toggle() {
    this.enabled = !this.enabled;
  }
}

export class TaskStore extends BehaviorSubject<TaskStore> {
  static instance: TaskStore;
  static labels = ["past", "yesterday", "today", "tomorrow", "future"];
  categories: Category[] = [];
  tasks: Task[][] = [[], [], [], [], []];

  constructor(tasks: Task[]) {
    super(null);
    new Set(tasks.map(t => t.category)).forEach(category => 
      this.categories.push(new Category(category)));
    tasks.forEach(task => this.tasks[task.period()].push(task));
    super.next(this);
  }

  public static async getInstance(): Promise<TaskStore> {
    if (TaskStore.instance) {
      return TaskStore.instance;
    }
    const tasks = await getTasks();
    TaskStore.instance = new TaskStore(tasks);
    return TaskStore.instance;
  }

  public toggleCategory(index: number) {
    this.categories[index].toggle();
    this.tasks.forEach(tasks => tasks.forEach(task => {
      if (task.category === this.categories[index].category) {
        task.toggleVisible();
      }
    }));
    super.next(this);
  }

  async createTask(task: Task) {
    task.id = await createTask(task);
    if (this.categories.find(category => task.category === category.category) == null) {
      this.categories.push(new Category(task.category));
    }
    this.updateTaskPosition(task);
  }

  async updateTask(task: Task) {
    await updateTask(task);
    this.updateTaskPosition(task);
  }

  async deleteTask(task: Task) {
    await deleteTask(task);
    const oldPosition = this.findTask(task.id);
    this.tasks[oldPosition[0]].splice(oldPosition[1], 1);
    super.next(this);
  }

  async completeTask(task: Task) {
    const newTime = await patchTask(task.id);
    task.completed = newTime[0];
    task.time = newTime[1];
    task.postponedTime = null;
    this.updateTaskPosition(task);
  }

  private updateTaskPosition(task: Task) {
    const oldPosition = this.findTask(task.id);
    const newPeriod = task.period();
    console.log(oldPosition, newPeriod);
    if (oldPosition !== null && newPeriod !== oldPosition[0]) {
      this.tasks[oldPosition[0]].splice(oldPosition[1], 1);
    }
    if (oldPosition === null || newPeriod !== oldPosition[0]) {
      this.tasks[newPeriod].push(task);
    }
    this.tasks[newPeriod].sort((a, b) => a.compare(b));
    super.next(this);
  }

  private findTask(id: number): [number, number] {
    for (let i = 0; i < this.tasks.length; i++) {
      for (let j = 0; j < this.tasks[i].length; j++) {
        if (this.tasks[i][j].id === id) {
          return [i, j];
        }
      }
    }
    throw "Task not found";
  }
}

async function createTask(task: Task): Promise<number> {
  task.at = task.time.toISO();
  task.postponed_at = task.postponedTime?.toISO();
  return await put('/task', task);
}

async function updateTask(task: Task): Promise<void> {
  task.at = task.time.toISO();
  task.postponed_at = task.postponedTime?.toISO();
  return await post(`/task/${task.id}`, task);
}

async function getTasks(): Promise<Task[]> {
  let rawTasks: [number, RawTask][] = await get(`/task`);
  let tasks = rawTasks.map(t => new Task(t));
  tasks.sort((a, b) => a.compare(b));
  return tasks;
}

async function deleteTask(task: Task) {
  return del(`/task/${task.id}`)
}

async function patchTask(id: number): Promise<[boolean, DateTime]> {
  let [completed, newDate]: [boolean, string] = await patch(`/task/${id}/complete`);
  return [completed, DateTime.fromISO(newDate)];
}
