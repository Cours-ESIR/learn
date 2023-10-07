<script lang="ts">
	import Tag from "./Tag.svelte";
	import { type ITag } from "../types/Tags";

	export let qid: string;
	export let title: string;
	export let description: string|undefined;
	export let tags: ITag[];
</script>

<div class="quizCard">
	<h2>{ title }</h2>
	<p>
		{ description || "" }
	</p>
	<span>
		{#each tags as props}
			<Tag {props} />
		{/each}
	</span>
</div>

<style>

	.quizCard {
		/* layout */
		display: grid;
		grid-template-rows: 30px 1fr 22px ;
		
		/* spacing */
		gap: 3px;
		padding: 16px;
		
		/* ui */
		border-radius: 16px;
		background-color: var(--cardColor);
		box-shadow: 0px 2px 4px 0 var(--shadow);
	}

	p {
		/* layout */
		display: -webkit-box;
		-webkit-line-clamp: 3;
		-webkit-box-orient: vertical;

		/* ui */
		overflow: hidden;
	}

</style>