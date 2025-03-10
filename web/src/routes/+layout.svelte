<script lang="ts">
	import AppShell from '$lib/components/AppShell.svelte';
	import Provider from '$lib/components/Provider.svelte';
	import '../app.css';
	
	let { children } = $props();
	
</script>

<Provider>
	<AppShell>
		{@render children()}
	</AppShell>
</Provider>

