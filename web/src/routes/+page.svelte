<script lang="ts">
	import { onUpdate, startCapturing, stopCapturing, type UnlistenFn } from "$lib/api";
	import { IconDeviceFloppy } from "@tabler/icons-svelte";
	import { onDestroy, onMount } from "svelte";

</script>

<div class="">
    <div class="">
        <IconDeviceFloppy size={30} stroke={1}/>
    </div>
    
    <span class="loading loading-ring loading-xl"></span>
</div>
