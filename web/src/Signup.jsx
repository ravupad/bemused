import {
  Observable, Subject, BehaviorSubject, combineLatest,
  of, fromEvent, from, merge
} from 'rxjs';
import {
  map, tap, takeUntil, scan, withLatestFrom, startWith, take, debounceTime, filter,
  switchMap, share, catchError
} from 'rxjs/operators';
import {React} from 'reactrx';
import {feedValue, getState, debug, delayedRef, observableElementToEvent} from 'reactrxutils';
import cx from 'classnames';
import s from 'css/login';
import {signup, checkUsernameAvailability, login} from 'client';
import {getMessageFromException, blockingErrorPromise} from 'Error';

const Signup = ({setRoute, Link}) => {
  const delayer = new Subject();
  const ref = delayedRef(delayer);
  const usernameInput = new Subject();
  const passwordInput = new Subject();
  const signupButton = new Subject();
  const username = observableElementToEvent(usernameInput, "input").pipe(
    map(ev => ev.target.value),
  );
  const password = observableElementToEvent(passwordInput, "input").pipe(
    map(ev => ev.target.value),
  );
  const usernameError = username.pipe(
    switchMap(username => username.length < 4 ? [true] : checkUsernameAvailability(username)),
    share(),
    map(usernameTaken => usernameTaken ? "Username is not available" : ""),
  );
  const passwordError = password.pipe(
    map(password => password.length < 4 ? "Password should have four or more characters" : ""),
  );
  const isError = combineLatest(usernameError, passwordError).pipe(
    map(a => a[0] !== "" || a[1] !== ""),
  );
  const buttonClass = isError.pipe(
    startWith(true),
    map(isError => cx({
      [s.button]: true,
      [s.disabled]: isError,
    })),
  );
  merge(
    merge(
      observableElementToEvent(passwordInput, "keydown"),
      observableElementToEvent(usernameInput, "keydown"),
    ).pipe(filter(ev => ev.key == "Enter")),
    observableElementToEvent(signupButton, "click"),
  ).pipe(
    withLatestFrom(username, password, isError),
    filter(([_e, _u, _p, isError]) => !isError),
    switchMap(([_e, username, password, _i]) =>
      signup(username, password)
        .then(() => login(username, password))
        .then(() => true)
        .catch(ex => blockingErrorPromise(getMessageFromException(ex)).then(false))
    ),
    filter(result => result),
    tap(() => setRoute("/home")),
  ).subscribe();
  return (
    <div class={s.login} after={() => delayer.complete()}>
      <h2 class={s.header}>Signup</h2>
      <div class={s.label}>Username</div>
      <input class={s.input} type="text" after={ref(usernameInput)}/>
      <div class={s.error}>{usernameError}</div>
      <div class={s.label}>Password</div>
      <input class={s.input} type="password" after={ref(passwordInput)}/>
      <div class={s.error}>{passwordError}</div>
      <button class={buttonClass} after={ref(signupButton)}>Signup</button>
      <Link class={s.button} href="/login">Go to Login page</Link>
    </div>
  );
};

export default Signup;
