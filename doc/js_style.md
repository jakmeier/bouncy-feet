# Svelte and JavaScript conventions

Use the Svelte version and conventions already established by the repository. Do not migrate syntax or state management as part of an unrelated UI change.

### Components

- Keep route components focused on composition and data loading.
- Extract a component when it owns a reusable behavior or meaningful visual unit—not merely to reduce line count.
- Keep components small enough to understand in one pass, with explicit inputs and events.
- Prefer composition and snippets over a growing matrix of boolean props.
- Keep domain language in names: `LessonProgress`, `CoachIntro`, `BpmPicker`, not `GreenCard` or `BigBox`.
- Co-locate component-specific styles. Put shared tokens and primitives in the global theme layer.
- Avoid `:global(...)` except in the theme/reset layer or for a documented third-party integration.

### JavaScript

- Use plain JavaScript with JSDoc types.
- Prefer `const`; use `let` only for reassignment.
- Parse and validate API data at the boundary. A successful fetch does not imply valid data.
- Handle loading, empty, error, offline, and success states explicitly.
- Use `async`/`await` with `try`/`catch` where recovery or user feedback is possible.
- Do not swallow errors or leave debug logging in production.
- Avoid hidden global state. Shared state must have a clear owner and lifecycle.
- Never render untrusted HTML. Avoid `{@html}` unless content is sanitized at a documented boundary.

```js
/**
 * @typedef {'not-started' | 'current' | 'complete' | 'locked'} LessonStatus
 * @typedef {{ id: string, title: string, order: number, status: LessonStatus }} Lesson
 */

/** @param {Response} response */
export async function readJson(response) {
  if (!response.ok) {
    throw new Error(`Request failed with ${response.status}`);
  }

  return response.json();
}
```

### State and effects

- Derive values instead of duplicating them in state.
- Effects are for synchronizing with external systems, not for ordinary calculations.
- Clean up timers, observers, media listeners, and subscriptions.
- Put browser-only code behind the appropriate lifecycle or environment guard.
- Do not mutate props or shared collections in place.

### CSS

- Use logical properties such as `margin-inline` and `padding-block`.
- Prefer Grid for two-dimensional layout and Flexbox for one-dimensional alignment.
- Avoid absolute positioning for ordinary document layout.
- Do not style by DOM depth or generated class names.
- Use container queries when a component's own width matters more than the viewport.
- Keep selector specificity low and do not use `!important` outside narrowly documented utility/accessibility cases.
