<script lang="ts">
	import { useAppState } from '$lib/appState';
	import Landing from '$lib/components/Landing.svelte';
	import Provider from '$lib/components/Provider.svelte';
	import Sidebar from '$lib/components/Sidebar.svelte';
	
	let { children } = $props();
	const appState = useAppState();
	
</script>

<Provider>
	{#if $appState.isLoading }
		<Landing></Landing>
	{:else}
		<div class="flex">
			<Sidebar/>
			<div>
				{@render children()}
			</div>
		</div>
	{/if}
</Provider>

