import { React } from '../reactrx';
import { get } from '../client';
import { Store } from '../store';
import { Observable } from 'rxjs';
import { RawTaskWithId, TaskWithId } from './model';
import { VExpanded } from './VExpanded';
import { VList } from './VList';
import { DateTime } from 'luxon';

export const VMain = (): Observable<JSX.Element> => {
  return new Observable(view => {
    view.next(<div>Loading Tasks</div>);
    get('/task').then((rawTasks: RawTaskWithId[]) => {
      const tasks: TaskWithId[] = rawTasks.map(task => {
        return [task[0], {...task[1], at: DateTime.fromISO(task[1].at)}];
      });
      let allCategories = new Set(tasks.map(task => task[1].category));
      let store = {
        tasks: new Store(tasks),
        selectedCategories: new Store(allCategories),
      };
      view.next(<VList view={view} store={store}/>);
    });
  });
}

export type TaskStore = {
  tasks: Store<TaskWithId[]>;
  selectedCategories: Store<Set<string>>;
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
