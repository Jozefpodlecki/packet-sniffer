<script lang="ts">
    import { invoke,  } from "@tauri-apps/api/core";
    import { listen, emit, type UnlistenFn } from "@tauri-apps/api/event";
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
        handle = await listen("update", (event) => {
            console.log(event);
        });

        handle = await listen<string>("onchange", (event) => {
            console.log(event.payload);
        });
    }

    async function onStart() {
        emit("start");
    }

    async function onStop() {
        emit("stop");
    }
</script>

<button on:click={onStart}>Start</button>
<button on:click={onStop}>Stop</button>