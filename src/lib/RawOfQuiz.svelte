<script lang="ts">
	import { type IQuiz } from "../types/Quiz";
	import QuizCard from "./QuizCard.svelte";

	export let quizzes : IQuiz[];
	export let title : string;

	let row : HTMLDivElement;
	function scroll(side: number) {
		row.scrollBy(316*side, 316*side);
	}
</script>



<div class="mainRow">

	<div class="header">
		<h2>{ title }</h2>
		
		<button class="left" on:click={() => scroll(-1)}><svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M16.2426 6.34317L14.8284 4.92896L7.75739 12L14.8285 19.0711L16.2427 17.6569L10.5858 12L16.2426 6.34317Z" fill="currentColor" /></svg></button>
		<button class="right" on:click={() => scroll(1)}><svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M10.5858 6.34317L12 4.92896L19.0711 12L12 19.0711L10.5858 17.6569L16.2427 12L10.5858 6.34317Z" fill="currentColor" /></svg></button>
	</div>
		
	<div bind:this={row} class="row">
		{#each quizzes as quiz}
			<QuizCard description={quiz.description} title={quiz.title} tags={quiz.tags} qid={quiz.qid} />
		{/each}
	</div>
</div>

<style>

	.mainRow {
		/* layout */
		display: grid;
		gap : 8px;

		/* positionning */
		position: relative;
	}

	.header {
		display: grid;
		grid-template-columns: 1fr 40px 40px;
	}

	button.left, button.right {
		/* reset */
		all: unset;

		/* layout*/
		display: grid;
		place-items: center;
		
		/* sizing */
		height: 30px;
		
		/* ui */
		background-color: var(--cardColor);
		aspect-ratio: 1/1;
		border-radius: 360% ;
		transition: 0.1s;
	}

	button:hover {
		/* ui */
		opacity: 50%;
	}

	.row {
		
		/* layout */
		display: grid;
		grid-auto-flow: column;
		grid-auto-columns: 300px;
		grid-template-rows: 160px;

		/* sizing */
		width: 100%;

		/* spacing */
		gap: 16px;

		/* ui */
		overflow: hidden;
		scroll-behavior: smooth;
	}

</style>