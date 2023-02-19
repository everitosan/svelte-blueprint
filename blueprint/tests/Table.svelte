<!--D Componente para tablas chidas -->

<script>

  export let text = 'Button' // Variable del texto en el botón:
  export let index = 5 // Número de botón
  export let data = {} 
  export let options = [] 
  export let shadow // Sombra del componente

</script>
  

<table>
  <slot name="head" ></slot>
  <slot></slot>
  <slot name="footer" ></slot>
</table>

<style lang="scss">
  @use "../Styles/typography";
  @use "../Styles/utils";

  :root {
    --table-row-background-1: var(--gray-color-80);
    --table-row-background-2: var(--clear-color);

    --table-border-color:  var(--gray-color-70);
  }

  table {
    @include typography.normal-text;
    
    border-collapse: collapse;
    border: 1px solid var(--table-border-color);
    border-spacing: 2px;
    border-radius: 4px;
    display: table;
    width: 100%;

    :global(th) {
      border: 1px solid var(--table-border-color);
      font-weight: 600;
      padding: utils.sizing(1);
    }

    :global(td) {
      padding: utils.sizing(1);
      border: 1px solid var(--table-border-color);
    }

    :global(tr:nth-child(2n+1)) {
      background: var(--table-row-background-1);
    }
  
    :global(tr:nth-child(2n)) {
      background: vr(--clear-colr);
    }
  }
</style>

<!--E
  <Table>
    <tr slot='head' >
      <th> Campo 1</th>
      <th> Campo 2</th>
      <th> Campo 3</th>
    </tr>

    <tr>
      <td> Resultados </td>
      <td> Resultados </td>
      <td> Resultados </td>
    </tr>

    <tr slot='footer' >
      <td> Final 1 </td>
      <td> Final 2 </td>
      <td> Final 3 </td>
    </tr>
    
    <tr>
      <td> Resultados </td>
      <td> Resultados </td>
      <td> Resultados </td>
    </tr>

  </Table>
-->