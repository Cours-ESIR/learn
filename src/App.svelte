<script lang="ts">
import { invoke } from '@tauri-apps/api'

import Home from "./pages/Home.svelte";
import Quiz from "./pages/Quiz.svelte";
import SearchBar from "./lib/SearchBar.svelte";

let page = "Home"

</script>

<header>
	<h1>Mon Application</h1>
	<SearchBar />
	<a on:click={() => page="Home"}>Home</a>
	<a on:click={() => page="Quiz"}>Quiz</a>
</header>

<main>
	{#if page == "Home"}
		<Home/>
	{:else if page == "Quiz"}
		<Quiz/>
	{/if}
</main>

<style>

h1 {
	text-align: center;
	margin-top: 16px;
}



main {
	margin: auto;
	width: clamp(0px, 100%, 1012px);
	overflow: hidden;
}
</style>