<script lang="ts">
    import { invoke,  } from "@tauri-apps/api/core";
    import { listen, type UnlistenFn } from "@tauri-apps/api/event";
	import { onDestroy, onMount } from "svelte";
    let handle: UnlistenFn;

    onMount(() => {
        onLoad()
    });

    onDestroy(() => {
        handle && handle();
    })

    async function onLoad() {
        console.log("onload");
        handle = await listen("update", (payload) => {
            console.log(payload);
        });
    }
</script>