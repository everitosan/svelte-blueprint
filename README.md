# Svelte blueprint

*Create documentation files for components based in comments in the source code.*  

![example1](./img/example.png)

> Notes

- This package can be understood as a cli/lib tool, if you need to integrate it with a vite project check de complementary plugin [vite-plugin-svelte-blueprint](https://www.npmjs.com/package/vite-plugin-svelte-blueprint)
- For version 2.0.0 all `js` code has been migrated to `rust-lang`
- This package requires nodev16 due to ffi-napi
- Link a [documentación en español](./README-es.md)


## 📜 Requirements
As some process and compilation is required for the installation of the npm package, you might need some tools in your system.

```bash
$ apt-get install make gcc g++ -y 
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # rust installation
```


## 🛠️ How to use

To install the library, just use npm as always.  
```
$ npm i -D svelte-blueprint
```

Svelte component files should include comments to generate more explicit documentation.

- To add a descripton of the component place a comment like this one.
    ```html
    <!--D Description of the component -->
    ```
- To add a description of a prop just add a single line comment next to it's definition.
    ```js
    export let name // Name string
    ```

- To set usage example add:
    ```html
    <!--E
        <Component  />
    -->
    ```

Now you can use the cli to generate a documentation component file indicating source file, destination directory and an optional template to use.

- -s, --source <FILE>            Sets a source path
- -d, --destination <Directory>  Sets a destination path
- -t, --template <FILE>          Sets a template for the final component

```bash
$ ./node_modules/svelte-blueprint/blueprint/target/release/blueprint document --source ./hello.svelte --destination ./docs
```

Rememnber there is a complementary plugin [vite-plugin-svelte-blueprint](https://www.npmjs.com/package/vite-plugin-svelte-blueprint) for vite.
  


## 🍱 Templates
By default, the genrated Blueprint file uses a Blueprint Template Component contained in this library.

You can customize the colors of that template or use your own template.

These are the variables you may want to override to match your own style.

```css
 :global(:root) {
    --svelte-blueprint-background: transparent;
    --svelte-blueprint-color: #3e3e3e;
    --svelte-blueprint-accent: #55c1ff;
    --svelte-blueprint-table-border: #e7e7e7;
    --svelte-blueprint-table-background: #fff;
    --svelte-blueprint-table-hover: #eaeaea;
    --svelte-blueprint-table-header-color: #fff;
}
```


If you need to use your own template these are the svelete *slots* you should define:
```html
<slot name='description' > 
    Component description
</slot>
<slot name='props'>
    Component properties
</slot>
<slot name='slots'>
    Component slots
</slot>
<slot name='example'>
    Component example
</slot>
```

Also you may want to use these props:
```js
title
code
```

For a better understanding, checkout the [default template](./templates/Blueprint.svelte)

🌄 Guides
- [Youtube](https://www.youtube.com/watch?v=Z-znFCs7Cuc&t=14s&ab_channel=evesan) for svelte-blueprint < 2.0.0
