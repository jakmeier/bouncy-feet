
import { writable } from 'svelte/store';

export const hideNavigation = writable(true);
export const wideView = writable(false);
export const backgroundColor = writable("var(--theme-neutral-white)");
