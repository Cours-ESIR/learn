<script lang="ts">
	import Home from "./pages/Home.svelte";
	import Quiz from "./pages/Quiz.svelte";
	import Catalog from "./pages/Catalog.svelte";
	import MenuBar from "./lib/MenuBar.svelte";
	import SearchBar from "./lib/SearchBar.svelte";

	let page: string = "Home";
</script>

<div class="layout">
	<main>
		<header>
			<h1>{ page === "Home" ? "Mon Application" : page }</h1>
		</header>


		{#if page == "Home"}
			<Home/>
		{:else if page == "Quiz"}
			<Quiz/>
		{:else if page == "Catalog"}
			<Catalog/>
		{/if}
	</main>
	<div class="menuBar">
		<SearchBar />
		<MenuBar bind:page={page}></MenuBar>
	</div>
</div>

<style>

	.layout {
		display: flex;
		flex-direction: row-reverse;
		width : calc( 100dvw - 16px );
		height : calc( 100dvh - 16px );
		padding: 8px;
		gap : 8px;
	}

	h1 {
		/* layout */
		text-align: center;

		/* spacing */
		margin-top: 16px;
	}

	main {
		/* layout */
		display: grid;

		/* spacing */
		gap: 16px;
		margin: auto;

		/* sizing */
		width: clamp(0px, calc( 100% - 16px ), 932px);

		/* ui */
		overflow: auto;
	}

	.menuBar {
		width: 230px;
		height:100%;
		overflow:hidden;
		border-radius: 8px;
	}

	@media screen and (max-width: 616px) {
        .layout {
            flex-direction: column;
        }
		.menuBar {
			width:100%;
			height:    70px;
			min-height:70px;
			max-height:70px;
		}
		
		main {

			
		}

    }

</style>