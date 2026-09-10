// Allow importing .html files as raw strings (esbuild `text` loader).
declare module '*.html' {
  const content: string;
  export default content;
}
