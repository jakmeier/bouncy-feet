# Bouncy Feet UI Style Guide

## 1. Design tokens

All product colors, spacing, radii, typography, shadows, and motion values must come from tokens. Do not scatter unexplained hex values through components.

### 1.1 Color

Use theme colors defined `app.css`.

### 1.2 Color rules

- Prefer one dominant brand color per view or major section. Supporting colors should clarify hierarchy, state, or action.
- Orange is the preferred energetic primary. Green works well on light backgrounds; lime works well on black.
- Keep instructional video on black or orange when it supports legibility and focus.
- Never encode status with color alone. Add text, an icon, or a shape.
- Do not assume a brand pairing is accessible. Test contrast for the actual text size and weight.
- Use gradients sparingly.
- Do not invent opacity-based palette steps as brand tokens; the booklet's 60% and 40% variations are unresolved.

### 1.3 Typography

The primary display face is Pleasure Inktrap Tria Medium. Sofia Pro Soft Regular is the secondary face for longer text when it is more readable.

- Use the display face for short headlines, numbers, slogans, and moments of brand personality.
- Use the text face for instructions, labels, forms, course descriptions, and longer paragraphs.
- Keep fallback fonts explicit. Do not silently substitute a decorative font.
- Use sentence case for interface labels. Reserve stacked or unusually broken lines for large editorial headlines.
- Keep headings bold and concise; keep body copy plain and easy to scan.

Font files and licenses must be supplied by the project. Never download or approximate proprietary fonts without approval.

### 1.4 Spacing, shape, and layout

Use a 4px base scale with generous breathing room and a strong, asymmetric editorial rhythm.

```css
:root {
  --space-1: 0.25rem;
  --space-2: 0.5rem;
  --space-3: 0.75rem;
  --space-4: 1rem;
  --space-6: 1.5rem;
  --space-8: 2rem;
  --space-12: 3rem;
  --space-16: 4rem;
  --radius-sm: 0.5rem;
  --radius-md: 1rem;
  --radius-pill: 999px;
  --content-max: 72rem;
  --text-max: 65ch;
}
```

- Start mobile-first. Use fluid sizes and content-driven breakpoints at max-width 360px, max-width 600px or min-width 730px.
- Keep long text to roughly 45-65 characters per line.
- Use large type and graphic number treatments as accents, not behind critical content.
- Avoid small detailed shapes, prefer a few large and simple shapes
- Rounded, flowing lines fit the brand. Keep arrow ends and line-art joints rounded.
- Prefer clean blocks and strong contrast over stacks of shadows and glass effects.
- Avoid a card around every item. Group by meaning first; use containers only when they clarify structure or interaction.
- Maintain at least a 44x44 CSS pixel target for pointer actions.
- Avoid visible borders, except for animations

### 1.5 Symbols

- Brand symbols are available as SVGs in `bouncy_frontend/static/img/symbols/`
- Some brand symbols are available with color parameters in Svelte components in `bouncy_frontend/src/lib/components/ui/svg/`
- Avoid external symbols and emojis (only allowe for prototyping)
- New brand symbols can be created as needed

## 2. Voice and interface copy

The brand archetype is the Jester: lively, spontaneous, humorous, optimistic, charmingly unconventional, and inclusive.

### Write like this

- Short, active, conversational sentences
- Friendly encouragement from dancer to dancer
- Specific next steps
- Light wordplay in low-risk moments
- International English that is easy to translate
- Inclusive language that does not assume skill, body, gender, or access
- Direct speach

### Avoid

- Mocking mistakes, bodies, rhythm, age, or ability
- Forced slang or an exaggerated “youth” voice
- Jokes in errors, payment, consent, safety, or privacy flows
- Exclamation marks on every line
- Ambiguous dance puns where clarity matters
- Claims that progress is effortless or identical for everyone

| Instead of | Prefer |
| --- | --- |
| `Go!` | `Start lesson` |
| `Wrong move` | `Try that step once more` |
| `You failed to upload` | `Your video wasn't uploaded. Check your connection and try again.` |
| `Are you sure?` | `Remove this practice video? This can't be undone.` |

## 3. Motion

Motion should feel bouncy and responsive, never restless.

- Use transform and opacity for most transitions.
- Keep functional feedback around 120-220ms and larger transitions around 220-400ms.
- Use one playful motion cue at a time.
- A “shake” or “wiggle” may call attention to the next action once; do not loop it indefinitely.
- Pause non-essential motion when the page is not active.
- Provide a reduced-motion version that removes travel, shake, bounce, parallax, and celebratory loops.

### Svelte animation preference

- Prefer Svelte transition directives (`in:`, `out:`, or `transition:`) over CSS keyframe animations for component motion.
- An intro transition must enter the DOM: conditionally render it, then enable that state in `onMount` when it should play on initial page load.
- Keep the animation declaration beside the element it animates, with short, explicit duration and delay values.

```css
.next-arrow { animation: wiggle 360ms ease-out 1; }

@keyframes wiggle {
  0%, 100% { transform: rotate(0); }
  35% { transform: rotate(-5deg); }
  70% { transform: rotate(4deg); }
}

@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    scroll-behavior: auto !important;
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
  }
}
```
