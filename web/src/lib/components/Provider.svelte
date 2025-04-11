<script lang="ts">
    import { setContext } from 'svelte';
    import { load } from "$lib/api";
    import { onDestroy, onMount } from "svelte";
    import { writable } from 'svelte/store';
	import { setAppState } from '$lib/appState';
	import { type AppState } from '$lib/types';
	import TemperatureMinus from '@tabler/icons-svelte/icons/temperature-minus';

    const { children } = $props();
    const appState = writable<AppState>({
        isLoading: true,
        version: "",
        githubUrl: ""
    });

    onMount(() => {
        onLoad()
    });

    onDestroy(() => {
        
    })

    async function onLoad() {
        let result = await load();

        appState.set({
            ...result,
            isLoading: false
        })
    }

    setAppState(appState);
	
   
</script>

{@render children()}