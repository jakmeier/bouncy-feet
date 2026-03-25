export const BASE_PRIMARY = '#a9eb66';
export const BASE_SECONDARY = '#ff7301';

const DARK = '#1c1c1c';
const WHITE = '#ffffff';
const LIGHT = '#ddf7c2';

export const PALETTES = [
  {
    id: 'base',
    ring: LIGHT,
    background: 'transparent',
    bodies: [LIGHT, BASE_SECONDARY, DARK],
  },
  {
    id: 'invert',
    ring: BASE_SECONDARY,
    background: DARK,
    bodies: [BASE_PRIMARY, LIGHT, BASE_SECONDARY],
  },
  {
    id: 'mono',
    ring: '#888',
    background: 'transparent',
    bodies: [DARK, WHITE, '#888'],
  },
  {
    id: 'neon',
    ring: BASE_PRIMARY,
    background: DARK,
    bodies: [BASE_PRIMARY, BASE_SECONDARY, '#d838db'],
    // background: '#572a52',
  },
  {
    id: 'teal',
    ring: '#858585',
    background: '#ecf2f1',
    bodies: ['#95b6b1', '#206679', '#31c3e9'],
  },
];

/** Returns a palette by id, falling back to the first. */
export function getPalette(id) {
  return PALETTES.find((p) => p.id === id) ?? PALETTES[0];
}
