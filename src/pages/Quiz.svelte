<div>
    <h2>De quel nationalité sont ceux qui ne savent pas placer la France sur une carte ?</h2>    
    <div class="answer">  
        <button>Français</button>
        <button>Néerlandais</button>
        <button>Japonais</button>
        <button>Américains</button>
    </div>  
</div>

<style>

    .answer {
        /* layout */
        display: grid;
        grid-template-columns: 1fr 1fr;
        grid-template-rows: 1fr 1fr;

        /* spacing */
        gap: 8px;
    }

    .answer > * {
        /* reset */
        all: unset;

        /* layout */
        flex: 50px;
        text-align: center;

        /* sizing */
        height: 50px;

        /* ui */
        background-color: var(--cardColor);
    }

</style>
