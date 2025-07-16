
import { goto } from '$app/navigation';
import { tick } from 'svelte';
import { Tween } from 'svelte/motion';
import { writable } from 'svelte/store';

export const hideNavigation = writable(true);
export const wideView = writable(false);
export const backgroundColor = writable("var(--theme-neutral-black)");
export const fontColor = writable("var(--theme-neutral-white)");


export const fadingOut = $state({
    state: false,
    text: undefined,
    color: undefined,
    fontSize: new Tween(16, { duration: 300, delay: 0 }),
});
export const floatingTitleRect = $state({
    top: 0,
    left: 0,
    width: 0,
    height: 0,
});

// Delayed navigation to allow fading out
export async function fadeOutAndNavigate(href, preservedNode, text) {

    const rect = preservedNode?.getBoundingClientRect();
    floatingTitleRect.top = rect?.top;
    floatingTitleRect.left = rect?.left;
    floatingTitleRect.width = rect?.width;
    floatingTitleRect.height = rect?.height;

    const style = getComputedStyle(preservedNode);


    fadingOut.state = true;
    fadingOut.text = text;
    fadingOut.fontSize.set(parseFloat(style.fontSize), { duration: 0, delay: 0 });
    fadingOut.color = style.color;
    await tick();

    await new Promise((r) => setTimeout(r, 600));
    fadingOut.fontSize.set(36);
    await tick();
    await new Promise((r) => setTimeout(r, 600));

    goto(href);
    fadingOut.state = false;
    await tick();
    fadingOut.text = undefined;
    fadingOut.color = undefined;
}