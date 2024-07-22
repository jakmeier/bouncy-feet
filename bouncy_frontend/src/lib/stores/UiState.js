
import { writable } from 'svelte/store';

export const hideNavigation = writable(false);
export const wideView = writable(false);
export const backgroundColor = writable("var(--theme-neutral-white)");
