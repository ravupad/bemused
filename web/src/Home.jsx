import {React} from 'reactrx';
import s from 'css/home';
import {logout, verifySession} from 'client';

const Home = ({setRoute, Link}) => {
  verifySession().catch(ex => setRoute("/login"));
  const logout2 = () => logout().then(() => setRoute("/"));
  return (
    <div class={s.home}>
      <Link href="/task" class={s.button}>Task</Link>
      <button onclick={logout2} class={s.button}>Logout</button>
    </div>
  );
};

export default Home;
