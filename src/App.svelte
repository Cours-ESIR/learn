<script lang="ts">
import { invoke } from '@tauri-apps/api'

import Home from "./pages/Home.svelte";
import Quiz from "./pages/Quiz.svelte";
import Catalog from "./pages/Catalog.svelte";
import MenuBar from "./lib/MenuBar.svelte";
import SearchBar from "./lib/SearchBar.svelte";

let page = "Home"

</script>

<header>
	<h1>Mon Application - {page}</h1>
</header>

<main>
	{#if page == "Home"}
	<Home/>
	{:else if page == "Quiz"}
	<Quiz/>
	{:else if page == "Catalog"}
	<Catalog/>
	{/if}
</main>

<SearchBar />

<MenuBar bind:page={page}></MenuBar>

<style>

h1 {
	text-align: center;
	margin-top: 16px;
}



main {
	margin: auto;
	width: clamp(0px, calc( 100% - 16px ), 1012px);
	overflow: hidden;
}
</style>