declare module '*.scss' {
    const content: {[className: string]: string};
    export default content;
}

declare namespace JSX {
    interface Element { }
    interface IntrinsicElements { 
        div: any; 
        h2: any;
        br: any;
        button: any;
        select: any;
        input: any;
        textarea: any;
        option: any;
        a: any;
    }
}