declare module '*.scss' {
    const content: {
        [className: string]: string
    };
    export default content;
}

declare module '*.css' {
    const content: {
        [className: string]: string
    };
    export default content;
}

declare namespace JSX {
    export interface Element {

    }

    export interface IntrinsicElements extends Element { 
        [name: string]: any;
    }
}