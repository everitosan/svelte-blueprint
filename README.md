# Svelte blueprint 🏗️

##  Biuld svelte components documentation.

<br/>

Svelte-blueprint reads svelte files placed in a path known as **source** and then creates svelte files based on a **template** in the **dst** path

<br/>
<br/>

Install
---
```
$ npm i https://github.com/everitosan/svelte-blueprint
```

Svelte adapts
---
The .svelte files inside **source** path should have extra comments.

- To set description add:
    ```html
    <!--D Description of the component -->
    ```


- To set examples add:
    ```html
    <!--E
        <Component  />
    -->
    ```

<br/>
<br/>


Generate blueprints
---
Usage:
```bash
$ svelte-blueprint
```
Options:  
|Short|Long|Default|Description| Type|
|--|--|--|--|--|
||--help||Muestra ayuda| [booleano]
||--version || Muestra número de versión | [booleano]
|-s | --source |src/Components| Source path of components |[cadena de caracteres]
|-d | --dst | Blueprints |Destination path for blueprints | [cadena de caracteres]
|-w | --watch | false | Should watch for changes in source |[booleano]
|-t | --template | svelte-blueprint/templates/Doc.svelte | Path of a template for the output page |[cadena de caracteres]  
  
<br/>
<br/>

## Templates
By default, the genrated Blueprint file uses a Doc Component contained in this library, so if you need to use your own template this are the svelete *slots* you should implement:
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