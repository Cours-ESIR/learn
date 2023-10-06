<script lang="ts">
export let props: {
    name: string,
    color: string
};
</script>

<span class="tag" style="--tagColor: {props.color};">
    {props.name}
</span>

<style>
.tag {
    border-radius: 0.6em;
    padding: 0 0.5em;
    color: black;
    background-color: var(--tagColor);
}
</style>