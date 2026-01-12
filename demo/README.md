Hongdown demo
=============

This is an online playground for [Hongdown], a Markdown formatter that enforces
Hong Minhee's Markdown style conventions.

[Hongdown]: https://github.com/dahlia/hongdown


Features
--------

 -  *Live Preview*: See the formatted results instantly as you type.
 -  *Customizable Options*: Adjust formatting parameters to suit your needs.
 -  *Dark Mode*: Automatically respects your system's appearance settings.
 -  *WASM-powered*: Runs entirely in your browser using WebAssembly.


Getting started
---------------

### Local development

1.  Install dependencies:

    ~~~~ bash
    pnpm install
    ~~~~

2.  Start the development server:

    ~~~~ bash
    pnpm --filter hongdown-demo dev
    ~~~~

3.  Open <http://localhost:5173> in your browser.

### Building for production

To create a production build:

~~~~ bash
pnpm --filter hongdown-demo build
~~~~

The output will be in the \`dist/\` directory.


Technical details
-----------------

 -  **Framework**: [Solid.js]
 -  **Styling**: [UnoCSS]
 -  **Bundler**: [Vite]
 -  **Core**: [@hongdown/wasm]

[Solid.js]: https://www.solidjs.com/
[UnoCSS]: https://unocss.dev/
[Vite]: https://vitejs.dev/
[@hongdown/wasm]: https://www.npmjs.com/package/@hongdown/wasm
