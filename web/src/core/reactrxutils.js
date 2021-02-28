
import {Observable, fromEvent} from 'rxjs';
import {tap, switchMap, finalize, map, scan} from 'rxjs/operators';

const feedEvent = (subscriber) => (e) => subscriber.next(e);

const feedValue = (subscriber) => (e) => subscriber.next(e.target.value);

const feedNamedValue = (subscriber) => (e) => subscriber.next({[e.target.name]: e.target.value});

const observableElementToEvent = (observableElement, event) =>
      observableElement.pipe(switchMap(el => fromEvent(el, event)));

const delayedRef = (delayer) => (ref) => (el) =>
      delayer.pipe(finalize(() => ref.next(el))).subscribe();

const debug = (marker) => tap(e => console.log(marker, e));

export {
  feedEvent,
  feedValue,
  feedNamedValue,
  observableElementToEvent,
  debug,
  delayedRef,
};
