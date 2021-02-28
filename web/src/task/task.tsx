import { React } from '../core/reactrx';
import { DateTime } from 'luxon';
import { blockingErrorPromise } from '../core/error';
import { TaskStore, Task } from './main';
import { BehaviorSubject, Observable, Subject } from 'rxjs';
import { RouterComponentProps } from '../core/router';
import classnames from 'classnames/bind';
import style from './css/task.scss';
import { map, take, tap } from 'rxjs/operators';

const cx = classnames.bind(style);

export const RouteTask = ({route, params}: RouterComponentProps): Observable<JSX.Element> => {
  const id = params.get('id');
  return new Observable(view => {
    route.pipe(take(1)).subscribe(() => view.complete());
    view.next(<div>Loading</div>);
    TaskStore.getInstance().then(store => {
      if (id === "new") {
        const task = Task.new();
        view.next(<TaskView {...{route, store, task, create: true}}/>);
        return;
      }
      const number_id = Number(id);
      let task = store.value.tasks.flat().find(task => task.id === number_id);
      if (task == null) {
        view.next(<div>Task Not Found</div>);
        return;
      }
      store.pipe(
        tap(() => view.next(<TaskView {...{route, store, id: number_id, task, create: false}}/>))
      ).subscribe();
    });
  });
}

type TaskProps = {
  route: Subject<string>;
  store: TaskStore;
  task: Task;
  create: boolean;
}

function TaskView({route, store, task, create}: TaskProps) {
  const state = {
    atDate: task.time.toFormat("yyyy-MM-dd"),
    atTime: task.time.toFormat("HH:mm"),
    postponedDate: task.postponedTime?.toFormat("yyyy-MM-dd") || "",
    postponedTime: task.postponedTime?.toFormat("HH:mm") || "",
  };
  const putTime = () => {
    task.time = DateTime.fromISO(state.atDate + "T" + state.atTime);
    if (state.postponedDate !== "" && state.postponedTime !== "") {
      task.postponedTime = DateTime.fromISO(state.postponedDate + "T" + state.postponedTime);
    }
  };
  const close = () => route.next("/task");
  const createHandler = () => store.createTask(task);
  const updateHandler = () => store.updateTask(task);
  const deleteHandler = () => store.deleteTask(task).then(() => close());
  const completeHandler = () => store.completeTask(task);
  const modal = new BehaviorSubject('hide');
  const asyncAction = (action: () => Promise<void>) => () => {
    modal.next('modal');
    putTime();
    action()
      .catch(e => blockingErrorPromise(e.message))
      .finally(() => modal.next('hide'));
  };
  return (
    <div class={cx('task-container')}>
      <div class={modal.pipe(map(val => cx(val)))}>Wait ...</div>
      <input class={cx('text')} value={task.text} oninput={(e: any) => task.text = e.target.value}/>
      <textarea class={cx('note')} style="width: 100px" value={task.note} oninput={(e: any) => task.note = e.target.value}/>
      <div class={cx('row')}>
        <div class={cx('label')}>Scheduled At</div>
        <input class={cx('value')} type="date" value={state.atDate}
            oninput={(e: any) => state.atDate = e.target.value}/>
        <input class={cx('value')} type="time" value={state.atTime} 
            oninput={(e: any) => state.atTime = e.target.value}/>
      </div>
      <div class={cx('row')}>
        <div class={cx('label')}>Postponed To</div>
        <input class={cx('value')} type="date" value={state.postponedDate}
            oninput={(e: any) => state.postponedDate = e.target.value}/>
        <input class={cx('value')} type="time" value={state.postponedTime}
            oninput={(e: any) => state.postponedTime = e.target.value}/>            
      </div>
      <div class={cx('row')}>
        <div class={cx('label')}>Category</div>
        <input class={cx('value', 'category')} value={task.category} 
            oninput={(e: any) => task.category = e.target.value}/>
      </div>      
      <div class={cx('row')}>
        <div class={cx('label')}>Repeat After</div>
        <input class={cx('value', "schedule-value")} type="number" min="0" value={task.repeat_value}
          oninput={(e: any) => task.repeat_value=parseInt(e.target.value)}/>
        <select class={cx('value')} value={task.repeat_unit} oninput={(e: any) => task.repeat_unit = e.target.value}>
          <option>Day</option>
          <option>Month</option>
        </select>
        <select class={cx('value')} value={task.repeat_behavior} oninput={(e: any) => task.repeat_behavior = e.target.value}>
          <option value="FromScheduled">from schedule</option>
          <option value="FromScheduledInFuture">in future</option>
          <option value="FromCompleted">from completion</option>
        </select>        
      </div>
      <div  class={cx('button-wrapper')}>
        {task.id != -1 ? <button class={cx('button')} onclick={asyncAction(updateHandler)}>Update</button> : []}
        {task.id == -1 ? <button class={cx('button')} onclick={asyncAction(createHandler)}>Create</button> : []}
        {task.id != -1 ? <button class={cx('button')} onclick={asyncAction(completeHandler)}>Complete</button> : []}
        {task.id != -1 ? <button class={cx('button')} onclick={asyncAction(deleteHandler)}>Delete</button> : []}
        <button class={cx('button')} onclick={close}>Close</button>
      </div>
    </div>
  );
}
