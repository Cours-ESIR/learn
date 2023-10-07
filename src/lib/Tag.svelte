<script lang="ts">
	import { type ITag } from "../types/Tags";
	export let props: ITag;
</script>

<span class="tag" style="--tagColor: {props.color};">{ props.name }</span>

<style>

	.tag {
		/* spacing */
		padding: 0 0.5em;

		/* ui */
		color: var(--backgroundColor);
		background-color: var(--tagColor);
		font-weight: 600;
		border-radius: 0.6em;
	}

</style>