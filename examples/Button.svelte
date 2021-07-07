<!--D Componente de botón -->
<script>

export let text = 'Button' // Variable del texto en el botón
export let index = 5 // Número de botón
export let data = {} 
export let options = [] 
export let shadow // Sombra del componente
</script>

<button class="button">
    { index }
    { text }
    { data }
    { options }
    { shadow }
</button>

<!--E
<Button  />
<Button text="One" />
<Button text='Two' />
<Button text='Three' />
 -->