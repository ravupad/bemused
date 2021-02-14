import { React } from '../reactrx';
import style from './task.scss';
import classnames from 'classnames/bind';
import { DateTime } from 'luxon';
import { map } from 'rxjs/operators';
import { TaskStore, toggleCategorySelection } from './VMain';
import { combineLatest, Subscriber } from 'rxjs';
import { Task, TaskWithId } from './model';
import { VExpanded } from './VExpanded';

const cx = classnames.bind(style);

export function VList({store, view}: {store: TaskStore; view: Subscriber<JSX.Element>}): JSX.Element {
  let filters = store.tasks.value.pipe(
    map(tasks => new Set(tasks.map(task => task[1].category))),
    map(categories => Array.from(categories)),
    map(categories => categories.map(category => <FilterButton store={store} category={category}/>)),
  );
  let tasks = combineLatest([store.tasks.value, store.selectedCategories.value]).pipe(
    map(([tasks, categories]) => tasks.filter(task => categories.has(task[1].category))),
    map(tasks => tasks.sort((a, b) => a[1].at.diff(b[1].at).as("milliseconds"))),
    map(tasks => {
      const timeRelativeCategories: Map<RelativeDuration, TaskWithId[]> = new Map();
      relativeDurations.forEach(duration => timeRelativeCategories.set(duration, []));
      tasks.forEach(task => timeRelativeCategories.get(taskTimeRelativeToToday(task[1])).push(task));
      return timeRelativeCategories;
    }),
    map(tasks => relativeDurations.map(duration => <VMiniContainer timeRelative={duration} tasks={tasks.get(duration)} store={store} view={view} />)),
  );
  let handleNewTask = () => view.next(<VExpanded store={store} view={view}/>);
  return (
    <div class={cx("task-container")}>
      <h2>Tasks</h2>
      <div class={cx('filters-container')}>{filters}</div>
      {tasks}
      <br/>
      <button onclick={handleNewTask} class={cx('create-class-button')}>Create New Task</button>
    </div>
  );
};

function FilterButton({store, category}: {store: TaskStore, category: string}): JSX.Element {
  let className = store.selectedCategories.value.pipe(
    map(c => c.has(category)), map(isSelected => isSelected ? cx('filter-selected') : ''));
  let toggleFilter = () => store.selectedCategories.action.next(toggleCategorySelection(category));
  return <button onclick={toggleFilter} class={className}>{category}</button>;
}

type VMiniContainerProps = {
  timeRelative: RelativeDuration;
  tasks: TaskWithId[]; 
  store: TaskStore; 
  view: Subscriber<JSX.Element>;
}

function VMiniContainer({timeRelative, tasks, store, view}: VMiniContainerProps): JSX.Element {
  return (
    <div class={cx('time-relative-container', timeRelative)}>
      <div class={cx('time-relative-header')}>{timeRelative}</div>
      {tasks.map(task => <VMini id={task[0]} task={task[1]} store={store} view={view}/>)}
    </div>
  );
}

type VMiniProps = {
  id: number; 
  task: Task; 
  store: TaskStore; 
  view: Subscriber<JSX.Element>;
}

function VMini({id, task, store, view}: VMiniProps): JSX.Element {
  let taskClass = cx('mini-task', {'completed': task.completed});
  let expandTask = () => view.next(<VExpanded id={id} task={task} store={store} view={view} />);
  return <div onclick={expandTask} class={taskClass}>{task.text}</div>;
}

const relativeDurations = ["past", "yesterday", "today", "tomorrow", "future"] as const;
type RelativeDuration = typeof relativeDurations[number];

function taskTimeRelativeToToday(task: Task): RelativeDuration {
  let today = DateTime.local().startOf("day");
  let taskTime = task.at.startOf("day");
  let diff = today.plus({days: -1}).diff(taskTime).as("days");
  switch (diff) {
    case 0: return "yesterday";
    case -1: return "today";
    case -2: return "tomorrow";
    default:
      return diff > 0 ? "past" : "future";
  }
}
