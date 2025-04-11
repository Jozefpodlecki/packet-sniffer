import { invoke } from "@tauri-apps/api/core";
import { listen, emit } from "@tauri-apps/api/event";
import type { LoadResult } from "./types";

export type UnlistenFn = () => void;

export async function startCapturing() {
    await emit("start");
}

export async function stopCapturing() {
    await emit("stop");
}


export async function load(): Promise<LoadResult> {
    const result = await invoke<LoadResult>("load");

    return result;
}

export async function onUpdate(handler: () => void): Promise<UnlistenFn> {
    // const handle = await listen("update", handler);
    // return handle;
    return () => {}
}

export async function onChange(handler: () => void): Promise<UnlistenFn> {
    const handle = await listen("onchange", handler);
    return handle;
}