<script>
  let { text, color } = $props();

  let parts = $derived(splitIntoParts(text));

  const regex = /\*\*(.*?)\*\*/g;

  /**
   * @param {string} input
   */
  function splitIntoParts(input) {
    let parts = [];
    let lastIndex = 0;
    let match;

    while ((match = regex.exec(input)) !== null) {
      if (match.index > lastIndex) {
        parts.push({
          text: input.slice(lastIndex, match.index),
          highlight: false,
        });
      }
      parts.push({ text: match[1], highlight: true });
      lastIndex = regex.lastIndex;
    }

    if (lastIndex < text.length) {
      parts.push({ text: text.slice(lastIndex), highlight: false });
    }
    return parts;
  }
</script>

<p>
  {#each parts as part}
    {#if part.highlight}
      <span style="color: {color};">{part.text}</span>
    {:else}
      {part.text}
    {/if}
  {/each}
</p>
