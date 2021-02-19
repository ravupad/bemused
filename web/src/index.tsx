import {React, render} from './reactrx';
import './css/index';
import Router, {Route, BrowserHistory} from "./Router";
import Start from './Start';
import Login from './Login';
import Signup from './Signup';
import Home from './Home';
import { RouteList as TaskList } from './task/list';
import { RouteTask as Task } from './task/task';

const [route, setRoute] = BrowserHistory();
document.title = "Bemused";

render(
  <Router route={route} setRoute={setRoute}>
    <Route path="/" component={Start}/>
    <Route path="/login" component={Login}/>
    <Route path="/signup" component={Signup}/>
    <Route path="/home" component={Home}/>
    <Route path="/task" component={TaskList}/>
    <Route path="/task/:id" component={Task}/>
  </Router>,
  document.getElementById('app')
);

export {
  setRoute
};
