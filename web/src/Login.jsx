import {React} from 'reactrx';
import {observableElementToEvent, delayedRef, debug} from "reactrxutils";
import {Subject, combineLatest, merge, from} from 'rxjs';
import {
  scan, map, switchMap, withLatestFrom, tap, takeUntil, filter,
  startWith, share, catchError
} from 'rxjs/operators';
import {login} from 'client';
import {blockingError} from 'Error';
import styles from 'css/login';
import classNames from 'classnames/bind';

const Login = ({setRoute, Link}) => {
  const cx = classNames.bind(styles);
  const delayer = new Subject();
  const ref = delayedRef(delayer);
  const usernameInputElement = new Subject();
  const passwordInputElement = new Subject();
  const loginButtonElement = new Subject();
  const username = observableElementToEvent(usernameInputElement, "input").pipe(
    map(ev => ev.target.value),
    startWith(""),
  );
  const password = observableElementToEvent(passwordInputElement, "input").pipe(
    map(ev => ev.target.value),
    startWith(""),
  );
  const error = combineLatest(username, password).pipe(
    map(([username, password]) => {
      if (username === '' || password === '') {
        return "Username and Password cannot be blank.";
      } else {
        return "";
      }
    }),
  );
  const loginHandlerStart = merge(
    merge(
      observableElementToEvent(usernameInputElement, "keydown"),
      observableElementToEvent(passwordInputElement, "keydown"),
    ).pipe(filter(ev => ev.key === "Enter")),
    observableElementToEvent(loginButtonElement, "click"),
  ).pipe(
    withLatestFrom(error),
    map(x => x[1]),
    filter(x => x === ""),
    withLatestFrom(username, password),
    map(([_, username, password]) => ([username, password])),
  );
  const loginHandler = loginHandlerStart.pipe(
    switchMap(([username, password]) => from(login(username, password)).pipe(
      catchError(ex => blockingError(ex.message)),
    )),
    tap(() => setRoute("/home")),
    share(),
  );
  const buttonClass = merge(
    error.pipe(map(error => error !== "")),
    loginHandlerStart.pipe(map(() => true)),
    loginHandler.pipe(map(() => false)),
  ).pipe(
    startWith(true),
    map(disabled => cx({
      button: true,
      disabled: disabled,
    })),
  );
  return (
    <div class={cx("login")} after={() => delayer.complete()}>
      <h2 class={cx("header")}>Login</h2>
      <div class={cx("error")}>{error}</div>
      <div class={cx("label")}>Username</div>
      <input class={cx("input")} placeholder="Username" after={ref(usernameInputElement)} />
      <div class={cx("label")}>Password</div>
      <input class={cx("input")} type="password" placeholder="Password"
             after={ref(passwordInputElement)} />
      <button class={buttonClass} after={ref(loginButtonElement)}>Login</button>
      <Link class={cx("button")} href="/signup">Signup</Link>
    </div>
  );
};

export default Login;
