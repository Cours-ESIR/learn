<script lang="ts">
	export let page: string;
</script>

<nav>
	<button class="menuItem" on:click={ () => page = "Home" }>
		<svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" clip-rule="evenodd" d="M6 22.8787C4.34315 22.8787 3 21.5355 3 19.8787V9.87866C3 9.84477 3.00169 9.81126 3.00498 9.77823H3C3 9.20227 3.2288 8.64989 3.63607 8.24262L9.87868 2.00002C11.0502 0.828445 12.9497 0.828445 14.1213 2.00002L20.3639 8.24264C20.7712 8.6499 21 9.20227 21 9.77823H20.995C20.9983 9.81126 21 9.84477 21 9.87866V19.8787C21 21.5355 19.6569 22.8787 18 22.8787H6ZM12.7071 3.41423L19 9.70713V19.8787C19 20.4309 18.5523 20.8787 18 20.8787H15V15.8787C15 14.2218 13.6569 12.8787 12 12.8787C10.3431 12.8787 9 14.2218 9 15.8787V20.8787H6C5.44772 20.8787 5 20.4309 5 19.8787V9.7072L11.2929 3.41423C11.6834 3.02371 12.3166 3.02371 12.7071 3.41423Z" fill="currentColor" /></svg>
        Home
    </button>
	
	<button class="menuItem" on:click={ () => page = "Quiz" }>
		<svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M20 4V16H22V2H8V4H20Z" fill="currentColor" /><path fill-rule="evenodd" clip-rule="evenodd" d="M2 8V22H16V8H2ZM14 10H4V20H14V10Z" fill="currentColor" /><path d="M17 7H5V5H19V19H17V7Z" fill="currentColor" /></svg>
        Quiz
    </button>

	<button class="menuItem" on:click={ () => page = "Catalog" }>
		<svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M20 4V16H22V2H8V4H20Z" fill="currentColor" /><path fill-rule="evenodd" clip-rule="evenodd" d="M2 8V22H16V8H2ZM14 10H4V20H14V10Z" fill="currentColor" /><path d="M17 7H5V5H19V19H17V7Z" fill="currentColor" /></svg>
        Catalog
    </button>
	
	<button class="menuItem" on:click={ () => page = "Settings" }>
		<svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" clip-rule="evenodd" d="M7 3C8.86384 3 10.4299 4.27477 10.874 6H19V8H10.874C10.4299 9.72523 8.86384 11 7 11C4.79086 11 3 9.20914 3 7C3 4.79086 4.79086 3 7 3ZM7 9C8.10457 9 9 8.10457 9 7C9 5.89543 8.10457 5 7 5C5.89543 5 5 5.89543 5 7C5 8.10457 5.89543 9 7 9Z" fill="currentColor" /><path fill-rule="evenodd" clip-rule="evenodd" d="M17 20C15.1362 20 13.5701 18.7252 13.126 17H5V15H13.126C13.5701 13.2748 15.1362 12 17 12C19.2091 12 21 13.7909 21 16C21 18.2091 19.2091 20 17 20ZM17 18C18.1046 18 19 17.1046 19 16C19 14.8954 18.1046 14 17 14C15.8954 14 15 14.8954 15 16C15 17.1046 15.8954 18 17 18Z" fill="currentColor" /></svg>
        Settings
    </button>
</nav>

<style>

	nav {
		/* layout */
		display: flex;
		flex-direction:column ;


		/* sizing */
		width: 100%;
		height: 100%;

		/* ui */
		color: var(--text);
		background-color: var(--cardColor);

		transition: 0.3s;
		user-select: none;
	}

	nav > .menuItem {
		/* reset */
		all: unset;
		
		/* layout */
		display: flex;
        place-items: center;
        height: 60px;
        flex-direction: row;
        padding: 4px 8px;
        gap: 8px;

		/* ui */
		cursor: pointer;

	}
	
	nav > .menuItem > .item-title {
		/* layout */
		flex: auto;
	}
	
	nav > .menuItem:hover * {
		/* ui */
		border-color: var(--action);
		color: var(--action);
		fill: var(--action);
	}

    @media screen and (max-width: 616px) {
        nav {
            flex-direction:row ;
            place-items: center;
            justify-content: space-around !important;
        }
        nav > .menuItem {
            flex-direction: column;
            height: unset;
        }
    }

</style>