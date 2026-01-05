<script>
  import { t } from '$lib/i18n';
  import * as api from '$lib/peertube-openapi';
  import ActorAvatar from '../profile/ActorAvatar.svelte';

  /**
   * @typedef {Object} Props
   * @property {string} name -- bindable
   * @property {string} description -- bindable
   * @property {api.Actor & api.ActorInfo} [userOrChannel]
   * @property {()=>void} onSubmit
   */
  /** @type {Props} */
  let {
    name = $bindable(),
    description = $bindable(),
    userOrChannel,
    onSubmit,
  } = $props();

  /** @param {Event} event */
  async function save(event) {
    event.preventDefault();
    onSubmit();
  }
</script>

{#if userOrChannel}
  <div class="actor">
    <div class="logo">
      <ActorAvatar actor={userOrChannel} targetWidth={48} />
    </div>
    <p>{userOrChannel.displayName}</p>
  </div>
{/if}

<form onsubmit={save}>
  <label for="title"> {$t('playlist.form-title')}: </label>
  <input type="text" name="title" required maxlength="120" bind:value={name} />

  <label for="description"> {$t('playlist.form-description')}: </label>
  <textarea
    name="description"
    maxlength="1000"
    rows="5"
    cols="36"
    bind:value={description}
  ></textarea>

  <button type="submit">{$t('playlist.save-button')}</button>
</form>

<style>
  form {
    display: grid;
    gap: 1rem;
    grid-template-columns: 1fr;
  }

  .actor {
    display: grid;
    grid-template-columns: 1fr 1fr;
    align-items: center;
    width: min-content;
    gap: 1rem;
    margin-bottom: 1rem;
  }

  .logo {
    width: 48px;
  }

  @media (min-width: 730px) {
    form {
      grid-template-columns: min-content auto;
    }
  }
</style>
