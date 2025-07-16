import { noFadeCrossfade } from '$lib/sveltex/cross_transition';

export const [sendPersonalityTitle, receivePersonalityTitle] =
  noFadeCrossfade({ duration: 600 });

