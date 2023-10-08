<script lang="ts">
    
    let questionnaire = [
        {"question" : "Quel est la traduction de 'crouler sous' en anglais", "answer" : "to be saddled with"},
        {"question" : "Quel est la traduction de 'à la suite de' en anglais", "answer" : "in the wake of"},
        {"question" : "Quel est la traduction de 'obligatoire' en anglais", "answer" : "compulsory"},
        {"question" : "Quel est la traduction de 'une aubaine' en anglais", "answer" : "a boon"},
        {"question" : "Quel est la traduction de 'égaliser les chances' en anglais", "answer" : "to level the playing field"},
        {"question" : "Quel est la traduction de 'la paie' en anglais", "answer" : "the wage"},
        {"question" : "Quel est la traduction de 'l'inscription' en anglais", "answer" : "the enrollment"},
        {"question" : "Quel est la traduction de 'avoir les moyens' en anglais", "answer" : "to afford"},
        {"question" : "Quel est la traduction de 'prendre pour aquis' en anglais", "answer" : "take it for granted"},
        {"question" : "Quel est la traduction de 'abandonner' en anglais", "answer" : "to drop out"},
        {"question" : "Quel est la traduction de 'le minimum syndicale' en anglais", "answer" : "the bare minimum"},
    ]

    let qi = 0
    let qTries = 3
    let input = ""

    function submit() {
        if ( input == questionnaire[qi]["answer"] ) {
            qi += 1
            qTries = 3
        }
        else {
            qTries -= 1
        }
        input = ""
    }

    function pass() {
        qi += 1
        qTries = 3
    }
</script>


<div>

    <h2>{questionnaire[qi]["question"]}</h2>    
    
    {#if qTries == 0} 
        <p>La réponse était : {questionnaire[qi]["answer"]}</p>
        <button on:click={pass}>Passer</button>
    {:else}
        <p>{qTries} tentatives</p>
        <input bind:value={input}>
        <button on:click={submit}>Vérifier</button>
    {/if}
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

    
    button, input {
        /* reset */
        all: unset;

        /* layout */
        flex: 50px;
        text-align: center;
        padding:8px;

        /* ui */
        background-color: var(--cardColor);
    }

</style>
