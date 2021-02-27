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

Notification.requestPermission(function(status) {
  console.log('Notification permission status:', status);
});

// if (Notification.permission == 'granted') {
//   navigator.serviceWorker.getRegistration().then(function(reg) {
//     reg.showNotification('Hello world! Beginning of notifications!');
//   });
// }

export function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}

export {
  setRoute
};
