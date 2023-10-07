<script lang="ts">
import Tag from "./Tag.svelte";
import { type ITag } from "../types/Tags";

export let qid: string;
export let title: string;
export let description: string;
export let tags: ITag[];

</script>

<div class="quizCard">
    <h2>{ title }</h2>
    <p>
        {#if description}
            {description}
        {/if}
    </p>
    <span>
        {#each tags as props}
            <Tag {props} />
        {/each}
    </span>
</div>

<style>
.quizCard {
    background-color: var(--cardColor);
    border-radius: 16px;
    box-shadow: 0px 2px 4px 0 var(--shadow);
    padding:16px;
    display: grid;
    grid-template-rows: 30px 1fr 22px ;
    gap:3px;
}

p {
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;  
    overflow: hidden;

}

</style>