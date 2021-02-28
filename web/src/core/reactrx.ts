import {Observable} from 'rxjs';
import {finalize, tap} from 'rxjs/operators';

type SimpleAttributeValue = 
    string |
    ((event: any) => void);

type AttributeValue = 
    SimpleAttributeValue |
    ((self: HTMLElement) => void) |
    Observable<string>;

export type Renderable = 
    JSX.Element |
    HTMLElement |
    string |
    Observable<Renderable> |
    Renderable[];

type Attributes = {
  [key: string]: AttributeValue
};

type JSXFn = (attrs: Attributes, ...children: HTMLElement[]) => Renderable;

type Tag = 
    string |
    JSXFn;
    

function h(tag: Tag, attrs: {[key: string]: AttributeValue}, ...children: HTMLElement[]): Renderable {
  if (typeof tag === "function") {
    return tag(attrs, ...children);
  } else {
    let element = document.createElement(tag);
    for (let i = 0; i < children.length; i++) {
      render(children[i], element);
    }
    for (let name in attrs) {
      setAttribute(element, name, attrs[name]);
    }
    return element;
  }
}

function setAttribute(element: HTMLElement, name: string, value: AttributeValue) {
  if (name === "after") {
    (value as Function)(element);
  } else if (value instanceof Observable) {
    value.subscribe(value => simpleSetAttribute(element, name, value));
  } else {
    simpleSetAttribute(element, name, value);
  }
}

function simpleSetAttribute(element: HTMLElement, name: string, value: SimpleAttributeValue) {
  if (name === "type") {
    element.setAttribute(name, value as string);
  } else {
    (element as any)[name === "class" ? "className" : name] = value;
  }
}

function render(component: Renderable, host: HTMLElement, placeHolder?: Text) {
  if (component instanceof Observable) {
    let start = document.createTextNode("");
    let end = document.createTextNode("");
    host.insertBefore(start, placeHolder);
    host.insertBefore(end, placeHolder);
    component.pipe(
      tap(newcomponent => {
        removeBetween(host, start, end);
        render(newcomponent, host, end);
      }),
      finalize(() => {
        removeBetween(host, start, end);
        start.remove();
        end.remove();
      }),
    ).subscribe();
  } else if (Array.isArray(component)) {
    for (let i = 0; i < component.length; i++) {
      render(component[i], host, placeHolder);
    }
  } else if (component instanceof HTMLElement) {
    host.insertBefore(component, placeHolder);
  } else if (typeof component === "string") {
    host.insertBefore(document.createTextNode(component), placeHolder);
  } else {
    throw("unexpected renderable");
  }
}

function removeBetween(parent: HTMLElement, start: Text, end: Text) {
  if (start.parentElement == null) {
    return;
  }
  while (start.nextSibling != end) {    
    parent.removeChild(start.nextSibling);
  }
}

const React = {
  createElement: h
};

export {
  render,
  React,
};
