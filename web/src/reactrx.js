import {Observable} from 'rxjs';
import {finalize} from 'rxjs/operators';

function h(tag, attrs, ...children) {
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

function setAttribute (element, name, value) {
  if (name === "after") {
    value(element);
  } else if (value instanceof Observable) {    
    value.subscribe(value => setAttribute(element, name, value, true));
  } else if (name === "type") {
    element.setAttribute(name, value);
  } else {
    element[name === "class" ? "className" : name] = value;
  }
}

function render(component, host) {
  if (component instanceof Observable) {
    let container = document.createElement("span");
    host.appendChild(container);
    component.pipe(
      finalize(() => host.remove())
    ).subscribe(newcomponent => {
      while (container.firstChild && !container.firstChild.remove());
      render(newcomponent, container);
    });
  } else if (Array.isArray(component)) {
    for (let i = 0; i < component.length; i++) {
      render(component[i], host);
    }
  } else if (component.nodeType == null) {
    host.appendChild(document.createTextNode(component));
  } else {
    host.appendChild(component);
  }
}

const React = {
  createElement: h
};

export {
  render,
  React,
};
