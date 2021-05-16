import {React} from '@raviupadhyay/reactrx';
import s from './css/home.scss';
import {logout, verifySession} from '../core/client';

const Home = ({route, Link}) => {
  verifySession().catch(ex => route.next("/login"));
  const logout2 = () => logout().then(() => route.next("/"));
  return (
    <div class={s.home}>
      <Link href="/task" class={s.button}>Task</Link>
      <button onclick={logout2} class={s.button}>Logout</button>
    </div>
  );
};

export default Home;
