import React, {useEffect} from 'react';

const concat = (array) => () => {
  for (let i = 0; i < array.length; i++) {
    if (array[i] != null) array[i]();
  }
};

const Custom = ({component}) => {
  useEffect(() => {
    const destructors = [];
    const cleanup = (f) => {
      destructors.push(f);
    }
    const node = component(cleanup);
    document.getElementById('root').appendChild(node);
    destructors.push(() => node.remove());
    return concat(destructors);
  }, []);
  return (<></>);
};

export default Custom;
