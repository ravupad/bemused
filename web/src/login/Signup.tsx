import {React, Router} from '@raviupadhyay/reactrx';
import {signup} from '../core/client';
import {blockingErrorPromise, getMessageFromException} from '../core/error';
import styles from './css/login.scss';
import classNames from 'classnames/bind';

const cx = classNames.bind(styles);

class Store {
  username: string = "";
  password: string = "";
  errorEl?: HTMLInputElement;

  constructor(private route: Router.BrowserHistory) {  
  }

  private checkError(): boolean {
    if (this.username === "" || this.password === "") {
      this.errorEl.value = "Username or Password cannot be blank"
      return false;
    } else {
      this.errorEl.value = "";
      return true;
    }
  }

  async handleSignup() {
    if (!this.checkError()) {
      return;
    }
    try {
      await signup(this.username, this.password);
      this.route.next("/home");
    } catch(ex) {
      await blockingErrorPromise(getMessageFromException(ex));
    }
  }
}

const Signup = ({route, Link}: Router.RouterComponentProps) => {
  const store = new Store(route);
  return (
    <div class={cx("login")}>
      <h2 class={cx("header")}>Signup</h2>
      <div class={cx("error")} after={(el: any) => store.errorEl = el}></div>
      <div class={cx("label")}>Username</div>
      <input class={cx("input")} placeholder="Username" 
          oninupt={(e: any) => store.username = e.target.value}/>
      <div class={cx("label")}>Password</div>
      <input class={cx("input")} type="password" placeholder="Password" 
          oninput={(e: any) => store.password = e.target.value}/>
      <button class={cx("button")} onclick={() => store.handleSignup()}>Signup</button>
      <Link href="/login" class={cx("button")}>Login</Link>
    </div>
  );
};

export default Signup;