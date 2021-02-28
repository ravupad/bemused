import { Subject } from 'rxjs';
import { React } from './reactrx';

export type LinkProps = {
  route: Subject<string>;
  href?: string;
  props: any[];
}

function Link({route, href, ...props}: LinkProps, children: JSX.Element | JSX.Element[])  {
  const changeRoute = (e: Event) => {
    route.next(href);
    e.preventDefault();
    return false;
  };
  return (
    <a href={href} onclick={changeRoute} {...props}>{children}</a>
  );
};

export default Link;
