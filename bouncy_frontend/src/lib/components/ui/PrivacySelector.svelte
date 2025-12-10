<script>
  import { t } from '$lib/i18n';
  import { VIDEO_PRIVACY } from '$lib/peertube';
  import * as api from '$lib/peertube-openapi';
  import { privacySymbol, privacyText } from '$lib/peertube_utils';
  import Symbol from './Symbol.svelte';

  /**
   * @typedef {Object} Props
   * @property {api.VideoPrivacySet} selected
   * @property {(p: api.VideoPrivacySet) => void} onSelected
   */

  /** @type {Props} */
  let { selected, onSelected } = $props();

  /** @type {HTMLElement[]} */
  let radios = $state([]);

  const OPTIONS = [
    VIDEO_PRIVACY.PRIVATE,
    VIDEO_PRIVACY.UNLISTED,
    VIDEO_PRIVACY.PUBLIC,
  ].map((id) => {
    return { id, symbol: privacySymbol(id), desc: privacyText(id) };
  });

  /**
   * @param {api.VideoPrivacySet} id
   */
  function select(id) {
    selected = id;
  }

  function onKeydown(e) {
    const keys = [
      'ArrowLeft',
      'ArrowUp',
      'ArrowRight',
      'ArrowDown',
      'Home',
      'End',
    ];
    if (!keys.includes(e.key)) return;
    e.preventDefault();

    const idx = OPTIONS.findIndex((o) => o.id === selected);
    let next = idx;

    if (e.key === 'ArrowLeft' || e.key === 'ArrowUp') {
      next = (idx - 1 + OPTIONS.length) % OPTIONS.length;
    } else if (e.key === 'ArrowRight' || e.key === 'ArrowDown') {
      next = (idx + 1) % OPTIONS.length;
    } else if (e.key === 'Home') {
      next = 0;
    } else if (e.key === 'End') {
      next = OPTIONS.length - 1;
    }

    const nextId = OPTIONS[next].id;
    select(nextId);
    // focus the newly selected radio
    radios[next]?.focus();
  }

  function save() {
    onSelected(selected);
  }
</script>

<h3>{$t('video.privacy-selection-title')}</h3>
<div
  role="radiogroup"
  aria-label="Privacy selector"
  class="group"
  tabindex="0"
  onkeydown={onKeydown}
>
  {#each OPTIONS as option, i}
    <div
      bind:this={radios[i]}
      role="radio"
      tabindex={selected === option.id ? 0 : -1}
      aria-checked={selected === option.id}
      class="radio"
      onclick={() => select(option.id)}
      onkeydown={(e) => {
        // allow space/enter to activate when focus is on option itself
        if (e.key === ' ' || e.key === 'Enter') {
          e.preventDefault();
          select(option.id);
        }
      }}
    >
      <Symbol size={32}>{option.symbol}</Symbol>
      <span class="label">
        <span class="desc">{$t(option.desc)}</span>
      </span>
    </div>
  {/each}
</div>

<button onclick={save}>Save</button>

<style>
  .group {
    display: grid;
    gap: 1rem;
  }

  .radio {
    /* display: inline-flex; */
    /* gap: 0.5rem; */
    /* align-items: center; */
    padding: 0.5rem 0.75rem;
    border-radius: 0.5rem;
    cursor: pointer;
    user-select: none;
    background: var(--theme-neutral-almost-black);
  }

  .radio[aria-checked='true'] {
    background: var(--theme-neutral-light);
    color: var(--theme-neutral-black);
  }

  .radio:focus {
    outline: none;
  }

  .label {
    display: flex;
    flex-direction: column;
    line-height: 1;
  }

  .label .desc {
    font-size: var(--font-small);
    opacity: 0.8;
  }

  button {
    margin-top: 1rem;
  }
</style>
