<svelte:head>
  <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Open+Sans:400,700" type="text/css">  
  <link rel="stylesheet" href="https://tutsplus.github.io/syntax-highlighter-demos/highlighters/Prism/prism_okaidia.css">    
</svelte:head>
<script>
export let title = 'Title'
export let code = ''
import Prism from 'svelte-prism'
</script>

<style type='text/scss'>
section :global(.tag:not(body)) {
    background-color: transparent;
}
</style>

<section class="doc hero">
    <div class="hero-body">
        <div class="container">
            
            <h1 class="title">
                {title}
            </h1>
            
            <h2 class="subtitle">
                <slot name='description' > 
                    Component description
                </slot>
            </h2>

            <div class='props' >
                <h4 class="title is-4">Props</h4>
                <table class="table is-hoverable ">
                    <thead>
                        <tr>
                            <th>Name</th>
                            <th>Type</th>
                            <th>Default</th>
                            <!-- <th>Description</th> -->
                        </tr>
                    </thead>
                    <slot name='props'>
                        Component properties
                    </slot>
                </table>
            </div>
            <br/>
            <div class='block' >
                <h4 class="title is-4">Usage</h4>
                <div class="columns" >
                    <div class="column">
                        <Prism language="html" source="{code}" />
                    </div>
                    <div class="column">
                        <slot name='example'>
                            Component example
                        </slot>
                    </div>
                </div>
            </div>

        </div>
    </div>
</section>