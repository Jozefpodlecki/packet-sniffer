import { getContext, setContext } from "svelte";
import type { Writable } from "svelte/store";
import type { AppState } from "./types";

export function useAppState(): Writable<AppState> {
    return getContext('app-state');
}

export function setAppState(appState: Writable<AppState>) {
    setContext('app-state', appState);
}