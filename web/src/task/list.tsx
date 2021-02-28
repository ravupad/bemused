import { React } from '../core/reactrx';
import style from './css/list.scss';
import classnames from 'classnames/bind';
import { finalize, map, take, takeUntil, tap, withLatestFrom } from 'rxjs/operators';
import { Observable, Subject } from 'rxjs';
import { Category, Task, TaskStore } from './main';
import { RouterComponentProps } from '../core/router';
import { List, ViewProps } from '../core/list';

const cx = classnames.bind(style);

export const RouterTaskList = ({route}: RouterComponentProps): Observable<JSX.Element> => {
  return new Observable(view => {
    route.pipe(take(1)).subscribe(() => view.complete());
    view.next(<div>Loading</div>);
    TaskStore.getInstance().then(store => {
      view.next(<TaskList {...{route, store}}/>);
    });
  });
}

type ListProps = {
  route: Subject<string>;
  store: TaskStore;
}

function TaskList({route, store}: ListProps): JSX.Element {
  const observable = store.pipe(takeUntil(route));
  let filterButton = (props: ViewProps<Category>) => FilterButton({...props, store});
  let filtersList = observable.pipe(map(value => value.categories));
  let filtersElement = (<List list={filtersList} view={filterButton}/>);
  let periodViews = TaskStore.labels.map((timeRelative, i) => {
    let tasks = observable.pipe(map(v => v.tasks[i]));
    return (<MiniTasks {...{route, store, timeRelative, tasks}}/>);
  });
  let handleNewTask = () => route.next("/task/new");
  return (
    <div class={cx("task-container")}>
      <h2 class={cx("page-title")}>Tasks</h2>
      <div class={cx('filters-container')}>{filtersElement}</div>
      {periodViews}
      <button onclick={handleNewTask} class={cx('create-class-button')}>
        <div class={cx('add-icon')}>+</div>
      </button>
    </div>
  );
};

type FilterButtonProps = {
  store: TaskStore;
  observable: Observable<Category>;
  index: number;
};

function FilterButton({store, observable, index}: FilterButtonProps): JSX.Element {
  let category = observable.pipe(map(v => v.category));
  let className = observable.pipe(map(v => cx('filter', {
    'filter-selected': v.enabled
  })));
  let buttonClick = () => store.toggleCategory(index);
  return <button onclick={buttonClick} class={className}>{category}</button>;
}

type VMiniContainerProps = {
  timeRelative: string;
  tasks: Observable<Task[]>;
  store: TaskStore; 
  route: Subject<string>;
}

function MiniTasks({timeRelative, tasks, store, route}: VMiniContainerProps): JSX.Element {
  let view = (props: ViewProps<Task>) => MiniTask({...props, route, store});
  return (
    <div class={cx('mini-container', timeRelative)}>
      <div class={cx('mini-task-header')}>{timeRelative}</div>
      <List list={tasks} view={view}/>
    </div>
  );
}

type MiniTaskProps = {
  observable: Observable<Task>;
  store: TaskStore; 
  route: Subject<string>;
}

function MiniTask({observable, store, route}: MiniTaskProps): JSX.Element {
  let taskClass = observable.pipe(map(task => cx('mini-task', {
    'completed': task.completed,
    'hide': !task.visible,
  })));
  let text = observable.pipe(map(task => task.text));
  let expandTaskClick = new Subject();
  expandTaskClick.pipe(
    withLatestFrom(observable),
    tap(([_, task]) => route.next(`task/${task.id}`))
  ).subscribe();
  return (
    <div onclick={() => expandTaskClick.next(null)} class={taskClass}>
      {text}
    </div>
  );
}
