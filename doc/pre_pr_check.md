# Before submitting a PR, check

## UI

- All UI text is loaded dynamically from translated source files. (`$t('key')`)
- Fonts sizes match an existing font size in app.css
- Displayed colors match theme colors in app.css
- @media max-width thresholds are one of 360px, 600px, and (min-width) 730px

## JS / Svelte

- Use js-docs typed props

## Brand

- [ ] The screen feels joyful, bold, clear, and inclusive.
- [ ] Colors use semantic tokens; orange is preferred for primary energy.
- [ ] The display font is limited to short expressive text.
- [ ] Photography and illustration feel natural, expressive, and dance-led.
- [ ] Prefer first to second person dialog for instruction: "Hello, how can I help you?"
- [ ] "Bouncy Feet" in texts or "BouncyFeet" as single word, never with hyphen

## UX and accessibility

- [ ] The primary next action is obvious and specifically labeled.
- [ ] Every state is communicated without relying on color alone.
- [ ] Keyboard, focus, zoom, contrast, labels, and text alternatives are verified.
- [ ] Motion is purposeful, finite, and reduced-motion safe.
- [ ] Touch targets are at least 44x44 CSS pixels.

### Code

- [ ] The implementation follows the repository's Svelte version and patterns.
- [ ] Components use domain names and have clear ownership.
- [ ] Inputs and API data are validated at boundaries.
- [ ] No unsanitized HTML, leaked listeners, magic hex values, or unexplained global styles.
- [ ] Formatting, linting, checks, and relevant tests pass.
- [ ] Avoid non-ascii characters in comment blocks and translated text (unless the language requires it)
- [ ] Comments are brief and meaningful in the long-term, not just relevant in the context of the PR.
- [ ] All added tests are meaningful

### Architecture

- [ ] Frontend remains a stateless, static webserver
- [ ] Backend is an API server without UI
    => The QR admin is an exception tolrated for now

