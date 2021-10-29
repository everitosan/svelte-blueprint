# Svelte blueprint 🏗️

##  Build svelte components documentation.

> For spanish go [here](./README-es.md).

<br/>

Svelte-blueprint reads svelte files placed in a path known as **source** and then creates svelte files based on a **template** in the **dst** path

<br/>

![example1](./img/ex1.png)

<br/>

Install
---
```
$ npm i -D svelte-blueprint
```

Svelte adjustments
---
The .svelte files inside **source** path should have extra comments.

- To set description add:
    ```html
    <!--D Description of the component -->
    ```
- To add a description of a prop just add a single line comment next to it's definition.
    ```js
    export let name // Name string
    ```

- To set usage example add:  
    *If you need to add quote marks use **single ones ' '***
    ```html
    <!--E
        <Component  />
    -->
    ```

<br/>



Generate blueprints
---
To generate the blueprint files you can use a **plugin** for specific module bundlers or use the cli.

### Plugins
-  [Rollup](https://www.npmjs.com/package/rollup-plugin-svelte-blueprint)


### Cli  

Usage:
```bash
$ svelte-blueprint
```
Options:  
|Short|Long|Default|Description| Type|
|--|--|--|--|--|
||--help||Shows help| [bool]
||--version || Shows the version | [bool]
|-s | --source |src/Components| Source path of components |[string]
|-d | --dst | Blueprints |Destination path for blueprints | [string]
|-w | --watch | false | Should watch for changes in source |[bool]
|-t | --template | svelte-blueprint/templates/Blueprint.svelte | Path of a template for the output page |[string]  
  
<br/>
<br/>

## Templates
By default, the genrated Blueprint file uses a Blueprint Component contained in this library.

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


If you need to use your own template this are the svelete *slots* you should define:
```html
<slot name='description' > 
    Component description
</slot>
<slot name='props'>
    Component properties
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

## Guides
- [Youtube](https://www.youtube.com/watch?v=Z-znFCs7Cuc&t=14s&ab_channel=evesan)
