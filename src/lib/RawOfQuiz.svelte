<script lang="ts">
import { type IQuiz } from "../types/Quiz";
import QuizCard from "./QuizCard.svelte";

export let quizzes : IQuiz[];

let row : HTMLDivElement;
function scroll(side: number) {
    row.scrollBy(316*side, 316*side);
}
</script>

<div class="mainRow">
    <button class="left" on:click={() => scroll(-1)}><svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M16.2426 6.34317L14.8284 4.92896L7.75739 12L14.8285 19.0711L16.2427 17.6569L10.5858 12L16.2426 6.34317Z" fill="currentColor" /></svg></button>

    <div bind:this={row} class="row">
        {#each quizzes as quiz}
            <QuizCard description={quiz.description} title={quiz.title} tags={quiz.tags} qid={quiz.qid} />
        {/each}
    </div>

    <button class="right" on:click={() => scroll(1)}><svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M10.5858 6.34317L12 4.92896L19.0711 12L12 19.0711L10.5858 17.6569L16.2427 12L10.5858 6.34317Z" fill="currentColor" /></svg></button>
</div>

<style> 

.mainRow {
    /* layout */
    display: block;

    /* positionning */
    position: relative;
}

button.left, button.right {
    /* reset */
    all: unset;

    /* layout*/
    display: grid;
    place-items: center;
    
    /* positionning */
    position: absolute;
    top: calc( 50% - 15px ) ;
    
    /* sizing */
    height:30px;
    
    /* ui */
    background-color: #26292f;
    aspect-ratio: 1/1;
    border-radius: 360% ;
    transition: 0.1s;
}

button:hover {
    /* ui */
    opacity: 50%;
}

button.left {
    /* positionning */
    left : 0px;
}

button.right {
    /* positionning */
    right : 0px;
}

.row {
    
    /* layout */
    display: grid;
    grid-auto-flow: column;
    grid-auto-columns: 300px;
    grid-template-rows: 160px;
    
    padding:4px 0px;
    margin : 0 40px;

    /* sizing */
    width: calc( 100% - 80px );
    overflow:hidden;
    
    /* spacing */
    gap: 16px;

    /* ui */
    scroll-behavior: smooth;
}

</style>