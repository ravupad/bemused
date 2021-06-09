import {React} from '@raviupadhyay/reactrx';
import s from './css/home.scss';
import {verifySession} from '../core/client';
import { RouterComponentProps } from '@raviupadhyay/reactrx/dist/router';
import { Observable } from 'rxjs';

function Home({route, Link}: RouterComponentProps): Observable<JSX.Element> {
  return new Observable(view => {
    view.next(<div>Loading ...</div>);
    verifySession()
      .catch(() => route.next("/login"));
    view.next(
      <div class={s.home}>
        <Link href="/task" class={s.button}>Task</Link>
      </div>
    );
  });
};

export default Home;
