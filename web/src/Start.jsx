import {React} from 'reactrx';
import {verifySession} from 'client';
import s from 'css/start';
import Link from "Link";

const Start = ({setRoute, Link}) => {
  verifySession()
    .then(() => setRoute("/home"))
    .catch(() => {});
  return (
    <div class={s.start}>
      <Link href="/login" class={s.button}>Login</Link>
      <Link href="/signup" class={s.button}>Signup</Link>
    </div>
  );
};

export default Start;
