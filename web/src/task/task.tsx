import { React } from '../reactrx';
import { put, post, del, patch } from '../client';
import { DateTime } from 'luxon';
import { blockingErrorPromise } from '../Error';
import { getNewTask, getTaskStore, patchTask, Task as TaskModel, TaskWithId } from './main';
import { addTask, removeTask, TaskStore, updateTask } from './main';
import { Observable, Subject } from 'rxjs';
import { Function } from '../store';
import { RouterComponentProps } from '../Router';
import { map } from 'rxjs/operators';
import classnames from 'classnames/bind';
import style from './task.scss';

const cx = classnames.bind(style);

export const RouteTask = ({setRoute, params}: RouterComponentProps): Observable<JSX.Element> => {
  const id = params.get('id');
  return new Observable(view => {
    view.next(<div>Loading</div>);
    getTaskStore().then(store => {
      if (id === "new") {
        const task = getNewTask();
        view.next(<Task setRoute={setRoute} store={store} id={0} task={task} create={true}/>);
      } else {
        const number_id = Number(id);
        store.tasks.value.pipe(
          map(tasks => tasks.filter(task => task[0] === number_id)),
          map(task => {
            if (task.length === 1) {
              view.next(<Task setRoute={setRoute} store={store} id={number_id} task={task[0][1]} create={false}/>);
            } else {
              view.next(<div>Task Not Found</div>);
            }
          })
        ).subscribe();
      }
    });
  });
}

type TaskProps = {
  setRoute: (route: string) => void;
  store: TaskStore;
  id: number;
  task: TaskModel;
  create: boolean;
}

function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}

function addDelay<T>(ms: number, promise: Promise<T>): Promise<T> {
  return Promise.all([sleep(ms), promise]).then(([x, value]) => value);
}

const Task = ({setRoute, store, id, task, create}: TaskProps) => {
  const storeAction = (action: Function<TaskWithId[], TaskWithId[]>) => store.tasks.action.next(action);
  const createHandler = () => put('/task', task)
      .then(id => storeAction(addTask([id, task])));
  const updateHandler = () => addDelay(1000, post(`/task/${id}`, task))
      .then(() => storeAction(updateTask([id, task])));
  const deleteHandler = () => del(`/task/${id}`)
      .then(() => storeAction(removeTask(id)))
      .then(() => close());
  const completeHandler = () => addDelay(1000, patchTask(id))
      .then(result => {
        if (result == null) {
          task.completed = true;
        } else {
          task.at = DateTime.fromISO(result);
        }
      })
      .then(() => storeAction(updateTask([id, task])));
  const close = () => setRoute("/task");
  const modal = new Subject();
  const resetModal = () => modal.next(<div></div>);
  const action = (fun: () => Promise<void>) => {
    modal.next(<div>Wait...</div>);
    fun().catch(e => blockingErrorPromise(e.message)).finally(() => resetModal());
  };
  return (
    <div class={cx('task-container')}>
      {modal}
      <input class={cx('text')} value={task.text} oninput={(e: any) => task.text = e.target.value}/>
      <textarea class={cx('note')} style="width: 100px" value={task.note} oninput={(e: any) => task.note = e.target.value}/>
      <input class={cx('datetime')} type="datetime-local" value={task.at.toFormat("yyyy-MM-dd'T'HH:mm")}
        oninput={(e: any) => task.at = DateTime.fromISO(e.target.value)}/>
      <input class={cx('category')} value={task.category} oninput={(e: any) => task.category = e.target.value}/>
      <div class={cx('repeat-wrapper')}>
        Repeat After:
        <input class={cx('repeat-value')} style="width: 75px; margin-left: 10px" type="number" min="0" value={task.repeat_value}
          oninput={(e: any) => task.repeat_value=parseInt(e.target.value)}/>
      </div>
      <select class={cx('repeat-unit')} style="width: 75px" value={task.repeat_unit} oninput={(e: any) => task.repeat_unit = e.target.value}>
        <option>Day</option>
        <option>Month</option>
      </select>
      <select class={cx('repeat-type')} style="width: 180px" value={task.repeat_behavior} oninput={(e: any) => task.repeat_behavior = e.target.value}>
        <option>FromScheduled</option>
        <option>FromScheduledInFuture</option>
        <option>FromCompleted</option>
      </select>
      <div  class={cx('button-wrapper')}>
        {id != null ? <button class={cx('button')} onclick={() => action(updateHandler)}>Update</button> : []}
        {id == null ? <button class={cx('button')} onclick={() => action(createHandler)}>Create</button> : []}
        {id != null ? <button class={cx('button')} onclick={() => action(completeHandler)}>Complete</button> : []}
        {id != null ? <button class={cx('button')} onclick={() => action(deleteHandler)}>Delete</button> : []}
        <button class={cx('button')} onclick={close}>Close</button>
      </div>
    </div>
  );
}
