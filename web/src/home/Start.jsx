import {React} from '../core/reactrx';
import {verifySession} from '../core/client';
import s from './css/start.scss';
import Link from "../core/link";

const Start = ({route, Link}) => {
  verifySession()
    .then(() => route.next("/home"))
    .catch(() => {});
  return (
    <div class={s.start}>
      <Link href="/login" class={s.button}>Login</Link>
      <Link href="/signup" class={s.button}>Signup</Link>
    </div>
  );
};

export default Start;
