// This allows TypeScript to recognize CSS files as modules
declare module "*.css" {
  const content: { [className: string]: string };
  export default content;
}