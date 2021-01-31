import { React } from '../reactrx';
import { put, post, del, patch } from '../client';
import { DateTime } from 'luxon';
import { blockingErrorPromise } from '../Error';
import { Task, TaskWithId } from './model';
import { addTask, removeTask, TaskStore, updateTask } from './VMain';
import { Subject, Subscriber } from 'rxjs';
import { VList } from './VList';
import { Function } from '../store';

interface ViewProps {
  id?: number;
  task?: Task; 
  store: TaskStore; 
  view: Subscriber<JSX.Element>;
}

export function VExpanded({id, task, store, view}: ViewProps): JSX.Element {
  if (task == null) {
    task = {
      text: "Text",
      note: "Note",
      completed: false,
      at: DateTime.local(),
      repeat_value: 0,
      repeat_unit: "Day",
      repeat_behavior: "FromScheduled",
      category: "Task"
    };
  }
  let storeAction = (action: Function<TaskWithId[], TaskWithId[]>) => store.tasks.action.next(action);
  let createHandler = () => put('/task', task).then(id => storeAction(addTask([id, task])));
  let updateHandler = () => post(`/task/${id}`, task).then(() => storeAction(updateTask([id, task])));
  let deleteHandler = () => del(`/task/${id}`).then(() => storeAction(removeTask(id))).then(() => close());
  let completeHandler = () => patch(`/task/${id}/complete`).then(result => {
    if (result == null) {
      task.completed = true;
    } else {
      task.at = result;
    }
  }).then(() => storeAction(updateTask([id, task])));
  let close = () => view.next(<VList view={view} store={store}/>);
  let modal = new Subject();
  let resetModal = () => modal.next(<div></div>);
  let action = async (fun: () => Promise<void>) => {
    modal.next(<div>Wait...</div>);
    fun().catch(e => blockingErrorPromise(e.message)).finally(() => resetModal());
  };
  return (
    <div>
      {modal}
      <input value={task.text} oninput={(e: any) => task.text = e.target.value}/>
      <br/>
      <textarea style="width: 100px" value={task.note} oninput={(e: any) => task.note = e.target.value}/>
      <br/>
      <input type="datetime-local" value={task.at.toFormat("yyyy-MM-dd'T'HH:mm")}
        oninput={(e: any) => task.at = DateTime.fromISO(e.target.value)}/>
      <br/>
      <input value={task.category} oninput={(e: any) => task.category = e.target.value}/>
      <div>
        Repeat After:
        <input style="width: 75px; margin-left: 10px" type="number" min="0" value={task.repeat_value}
          oninput={(e: any) => task.repeat_value=parseInt(e.target.value)}/>
        <select style="width: 75px" value={task.repeat_unit} oninput={(e: any) => task.repeat_unit = e.target.value}>
          <option>Day</option>
          <option>Week</option>
          <option>Month</option>
          <option>Year</option>
        </select>
        <select style="width: 180px" value={task.repeat_behavior} oninput={(e: any) => task.repeat_behavior = e.target.value}>
          <option>FromScheduled</option>
          <option>FromScheduledInFuture</option>
          <option>FromCompleted</option>
        </select>
      </div>
      {id != null ? <button onclick={() => action(updateHandler)}>Update</button> : []}
      {id == null ? <button onclick={() => action(createHandler)}>Create</button> : []}
      {id != null ? <button onclick={() => action(completeHandler)}>Complete</button> : []}
      {id != null ? <button onclick={() => action(deleteHandler)}>Delete</button> : []}
      <button onclick={close}>Close</button>
    </div>
  );
}
