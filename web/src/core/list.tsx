import { Observable, Subject } from "rxjs";
import { finalize, tap } from "rxjs/operators";
import {React, render, Renderable} from './reactrx';
import {isEqual, clone} from 'lodash';

export type ViewProps<T> = {
  observable: Observable<T>;
  index: number;
}

export type Props<T> = {
  list: Observable<T[]>;
  view: (props: ViewProps<T>) => Renderable;
}

export function List<T>(props: Props<T>): Renderable {
  let container = document.createElement("span");
  let views: [Subject<T>, T][] = [];
  props.list.pipe(
    tap((list) => {
      // update existing views
      for (let i = 0; i < list.length && i < views.length; i++) {
        if (!isEqual(views[i][1], list[i])) {
          views[i][0].next(list[i]);
          views[i][1] = clone(list[i]);
        }
      }
      // add new views
      for (let i = views.length; i < list.length; i++) {
        const subject = new Subject<T>();
        const renderable = props.view({index: i, observable: subject});
        render(renderable, container);
        views.push([subject, clone(list[i])]);
        subject.next(list[i]);
      }
      // remove truncated views
      for (let i = list.length; i < views.length; i++) {
        views[i][0].complete();
      }
      views = views.slice(0, list.length);
    }),
    finalize(() => {
      for (let i = 0; i < views.length; i++) {
        views[i][0].complete();
      }
      container.remove();
    })
  ).subscribe();
  return container;
}
