import { React } from '../reactrx';
import style from '../css/task.scss';
import classnames from 'classnames/bind';
import { map } from 'rxjs/operators';
import { TaskStore, toggleCategorySelection } from './VMain';
import { combineLatest, Subscriber } from 'rxjs';
import { Task } from './model';
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
    map(tasks => tasks.map(task => <VMini id={task[0]} task={task[1]} store={store} view={view}/>)),
  );
  return (
    <div class={cx("task-container")}>
      <h2>Tasks</h2>
      {filters}
      {tasks}
      <br/>
      <button onclick={() => view.next(<VExpanded store={store} view={view}/>)}>Create New Task</button>
    </div>
  );
};

function FilterButton({store, category}: {store: TaskStore, category: string}): JSX.Element {
  let className = store.selectedCategories.value.pipe(
    map(c => c.has(category)), map(isSelected => isSelected ? cx('filter-selected') : ''));
  let toggleFilter = () => store.selectedCategories.action.next(toggleCategorySelection(category));
  return <button onclick={toggleFilter} class={className}>{category}</button>;
}

type VMiniProps = {
  id: number; 
  task: Task; 
  store: TaskStore; 
  view: Subscriber<JSX.Element>;
}

export function VMini({id, task, store, view}: VMiniProps): JSX.Element {
  let taskClass = cx('mini-task', {'completed': task.completed});
  let expandTask = () => view.next(<VExpanded id={id} task={task} store={store} view={view} />);
  return <div onclick={expandTask} class={taskClass}>{task.text}</div>;
}
