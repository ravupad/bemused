import Router, {Route, BrowserHistory} from "./core/router";
import './core/css/index';
import { React, render } from './core/reactrx';
import Start from './home/Start';
import Login from './login/Login';
import Signup from './login/Signup';
import Home from './home/Home';
import { RouterTaskList as TaskList } from './task/list';
import { RouteTask as Task } from './task/task';

document.title = "Bemused";
const route = new BrowserHistory();

render(
  <Router route={route}>
    <Route path="/" component={Start}/>
    <Route path="/login" component={Login}/>
    <Route path="/signup" component={Signup}/>
    <Route path="/home" component={Home}/>
    <Route path="/task" component={TaskList}/>
    <Route path="/task/:id" component={Task}/>
  </Router>,
  document.getElementById('app')
);

Notification.requestPermission(function(status) {
  console.log('Notification permission status:', status);
});

export function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}

export {
  route
}