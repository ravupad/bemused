import { React } from './reactrx';

export type LinkProps = {
  setRoute: (route: string) => void;
  href?: string;
  props: any[];
}

function Link({setRoute, href, ...props}: LinkProps, children: JSX.Element | JSX.Element[])  {
  const changeRoute = (e: Event) => {
    setRoute(href);
    e.preventDefault();
    return false;
  };
  return (
    <a href={href} onclick={changeRoute} {...props}>{children}</a>
  );
};

export default Link;
