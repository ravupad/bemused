import { React } from "./reactrx";
import { Observable } from "rxjs";
import { map } from "rxjs/operators";
import Link, { LinkProps } from "./Link";

type RouterProps = {
  route: Observable<string>;
  setRoute: (route: string) => void;
  fallback?: () => JSX.Element;
}

export type RouterComponentProps = {
  setRoute: (route: string) => void;
  Link: () => JSX.Element;
  params: Map<string, string>;
}

type Component = (props: RouterComponentProps) => JSX.Element;

function Router (
  {route, setRoute, fallback}: RouterProps, 
  ...children: [string, () => JSX.Element][]): Observable<JSX.Element> 
{
  const routeMap: Map<string, () => JSX.Element> = new Map();
  for (let i = 0; i < children.length; i++) {
    routeMap.set(children[i][0], children[i][1]);
  }
  fallback = fallback || (() => <div>Location Not Found</div>);
  const routeLink = (props: LinkProps, ...children: JSX.Element[]) => {
    return Link({...props, setRoute}, children);
  };
  return route.pipe(map(route => {
    const [Selected, params] = matchTemplates(route, routeMap, fallback);
    return <Selected setRoute={setRoute} Link={routeLink} params={params}/>;
  }));
};

export function Route({path, component}: {path: string, component: Component}): [string, Component] {
  return [path, component];
}

function matchTemplates(
  route: string, 
  templates: Map<string, () => JSX.Element>, 
  fallback: () => JSX.Element): [() => JSX.Element, Map<string, string>] 
{
  let path = route.split('/').filter(a => a.length > 0);
  let params: Map<string, string> = new Map();
  window.location.search.substring(1).split('&')
      .filter(s => s !== "")
      .map(a => a.split('='))
      .forEach(p => params.set(p[0], p[1]));
  for (let [key, value] of templates) {
    let template = key.split('/').filter(a => a.length > 0);
    let res = matchTemplate(template, path);
    if (res[0] !== false) {
      for (const [key, value] of res[1]) {
        params.set(key, value);
      }
      return [value, params];
    }
  }
  return [fallback, params];
};

function matchTemplate(template: string[], route: string[]): [boolean, Map<string, string>] {
  let params: Map<string, string> = new Map();
  if (template.length !== route.length) {
    return [false, params];
  }
  for (let i = 0; i < template.length; i++) {
    let idx = template[i].indexOf(':');
    if (idx === -1) {
      if (template[i] !== route[i]) {
        return [false, params];
      }
    } else {
      if (template[i].substring(0, idx) !== route[i].substring(0, idx)) {
        return [false, params];
      }
      params.set(template[i].substring(idx+1), route[i].substring(idx));
    }            
  }
  return [true, params];
};

export function BrowserHistory(): [Observable<string>, (route: string) => void] {
  let setRouteInner = (route?: string) => alert("route is not subscribed yet");
  const setRoute = (route: string) => setRouteInner(route);
  const route: Observable<string> = new Observable(subscriber => {
    subscriber.next(window.location.pathname);
    const popstateListener = (_: any) => subscriber.next(window.location.pathname);
    window.addEventListener('popstate', popstateListener);
    setRouteInner = (path) => {
      subscriber.next(path);
      window.history.pushState(null, null, path);
    };
    return () => window.removeEventListener('popstate', popstateListener);
  });
  return [route, setRoute];
}

export default Router;