<script lang="ts">
import { type IQuiz } from "../types/Quiz";
import QuizCard from "./QuizCard.svelte";

export let quizzes : IQuiz[];

</script>

<div class="mainGrid">
    {#each quizzes as quiz}
        <QuizCard description={quiz.description} title={quiz.title} tags={quiz.tags} qid={quiz.qid} />
    {/each}
</div>

<style> 

.mainGrid {
    /* layout */
    display: grid;
    grid-auto-rows: 160px ;
    grid-template-columns: repeat(3,1fr);
    gap:16px;
    width: 100%;

    /* positionning */
    position: relative;
}


@media screen and (max-width: 932px) {
    .mainGrid {
        grid-template-columns: repeat(2,2fr);
    }
}

@media screen and (max-width: 616px) {
    .mainGrid {
        grid-template-columns: repeat(1,1fr);
    }
}

</style>