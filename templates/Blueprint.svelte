<svelte:head>
  <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Open+Sans:400,700" type="text/css">  
  <link rel="stylesheet" href="https://tutsplus.github.io/syntax-highlighter-demos/highlighters/Prism/prism_okaidia.css">    
</svelte:head>
<script>
export let title = 'Title'
export let code = ''
import Prism from 'svelte-prism'
</script>

<style>

    :global(:root) {
        --svelte-blueprint-background: transparent;
        --svelte-blueprint-color: #3e3e3e;
        --svelte-blueprint-accent: #55c1ff;
        --svelte-blueprint-table-border: #e7e7e7;
        --svelte-blueprint-table-background: #fff;
        --svelte-blueprint-table-hover: #eaeaea;
        --svelte-blueprint-table-header-color: #fff;
    }
    .svelte-blueprint {
        background-color: var(--svelte-blueprint-background);
        border-bottom: 1px solid var(--svelte-blueprint-table-border);
        text-align: left;
        color: var(--svelte-blueprint-color);
        padding: 1em;
    }

    .svelte-blueprint :global(table) {
        border-collapse: collapse;    
    }

    .svelte-blueprint :global(table th ) {
        padding: 15px 10px;
        cursor: pointer;
    }

    .svelte-blueprint :global(table td ) {
        padding: 15px 10px;
        cursor: pointer;
    }

    .svelte-blueprint :global(table tr) {
        border-bottom: 1px solid var(--svelte-blueprint-table-border);
        background: var(--svelte-blueprint-table-background);
    }
    
    .svelte-blueprint :global(table tr:last-child ) {
        border-bottom: 0;
    }
    
    .svelte-blueprint  :global(table tr:hover) {
        background: var(--svelte-blueprint-table-hover);
    }

    .svelte-blueprint  :global(table thead tr ) {
        background: var(--svelte-blueprint-accent);
        color: var(--svelte-blueprint-table-header-color);
    }
    
    .svelte-blueprint  :global(table thead tr:hover) {
        background: var(--svelte-blueprint-accent);
    }

    h1 {
        font-size: 34px;
        text-align: right;
    }

    h2 {
        font-size: 18px;
        font-weight: 100;
    }

    .props {
        margin-top: 2em;
    }

    .table {
        border: 1px solid var(--svelte-blueprint-table-border);
        border-radius: 0 0 5px 5px;
        padding: 0;
        width: fit-content;
        transition: all 0.4s;
        overflow: hidden;
    }

    .table:hover {
        box-shadow: 0 3px 4px 0 rgba(0, 0, 0, 0.08);
    }

    .usage {
        display: grid;
        grid-template-columns: 1fr;
        grid-gap: 20px;
    }

    .example {
        margin: 8px 0;
    }
</style>


<div class="svelte-blueprint">
    
    <h1 class="title">
        {title}
    </h1>
    
    <h2 class="subtitle">
        <slot name='description' > 
            Component description
        </slot>
    </h2>

    <div class='props' >
        <h4>Props</h4>
        <div class="table">
            <slot name='props'>
                Component properties
            </slot>
        </div>
    </div>
    <br/>
    <div >
        <h4>Usage</h4>
        <div class="usage" >
            <div>
                <Prism language="html" source={code} />
            </div>
            <div class='example' >
                <slot name='example'>
                    Component example
                </slot>
            </div>
        </div>
    </div>

</div>
