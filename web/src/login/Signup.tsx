import {React, Router} from '@raviupadhyay/reactrx';
import {signup, verifySession} from '../core/client';
import {blockingErrorPromise, getMessageFromException} from '../core/error';
import styles from './css/login.scss';
import classNames from 'classnames/bind';
import { Observable } from 'rxjs';

const cx = classNames.bind(styles);

async function handleLogin(username: string, password: string, router: Router.BrowserHistory) {
  if (username == null || password == null || username === '' || password === '') {
    return await blockingErrorPromise("Username or Password cannot be blank");
  }
  await signup(username, password)
    .then(() => router.next("/home"))
    .catch(ex => blockingErrorPromise(getMessageFromException(ex)));
}

function Login(pageProps: Router.RouterComponentProps): Observable<JSX.Element> {
  return new Observable(view => {
    view.next(<div>Loading ...</div>);
    verifySession()
      .then(() => pageProps.route.next("/home"))
      .catch(() => view.next(<SignupForm {...pageProps}/>));
  });
}

function SignupForm({route, Link}: Router.RouterComponentProps): JSX.Element {
  let username: string;
  let password: string;
  return (
    <div class={cx("login")}>
      <h2 class={cx("header")}>Signup</h2>
      <div class={cx("label")}>Username</div>
      <input class={cx("input")} placeholder="Username" 
          oninput={(e: any)=>username=e.target.value}/>
      <div class={cx("label")}>Password</div>
      <input class={cx("input")} type="password" placeholder="Password" 
          oninput={(e: any) => password = e.target.value}/>
      <button class={cx("button")} 
          onclick={() => handleLogin(username, password, route)}>Signup</button>
      <Link href="/login" class={cx("button")}>Go to Login Page</Link>
    </div>
  );
};

export default Login;