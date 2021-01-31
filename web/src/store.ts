import { Observable, Subject } from "rxjs";
import { scan, shareReplay, startWith } from "rxjs/operators";

export type Function<U, V> = (t: U) => V;

export class Store<T> {
  action: Subject<Function<T, T>>;
  value: Observable<T>;

  constructor(value: T) {
    this.action = new Subject();
    this.value = this.action.pipe(
      startWith((id: T) => id),
      scan((acc, updater) => updater(acc), value),
      shareReplay(1),
    );
  }
}
