import {React, render} from './reactrx';
import './css/index';
import Router, {Route, BrowserHistory} from "./Router";
import Start from './Start';
import Login from './Login';
import Signup from './Signup';
import Home from './Home';
import { VMain as Tasks } from './task/VMain';

const [route, setRoute] = BrowserHistory();
document.title = "Bemused";

render(
  <Router route={route} setRoute={setRoute}>
    <Route path="/"         component={Start}/>
    <Route path="/login"    component={Login}/>
    <Route path="/signup"   component={Signup}/>
    <Route path="/home"     component={Home}/>
    <Route path="/task"     component={Tasks}/>
  </Router>,
  document.body
);

export {
  setRoute
};
