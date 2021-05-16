import './core/css/index';
import {React, render, Router} from '@raviupadhyay/reactrx';
import Start from './home/Start';
import Login from './login/Login';
import Signup from './login/Signup';
import Home from './home/Home';
import {RouterTaskList as TaskList} from './task/list';
import {RouteTask as Task} from './task/task';

document.title = "Bemused";
const route = new Router.BrowserHistory();

render(
  <Router.default route={route}>
    <Router.Route path="/" component={Start}/>
    <Router.Route path="/login" component={Login}/>
    <Router.Route path="/signup" component={Signup}/>
    <Router.Route path="/home" component={Home}/>
    <Router.Route path="/task" component={TaskList}/>
    <Router.Route path="/task/:id" component={Task}/>
  </Router.default>,
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