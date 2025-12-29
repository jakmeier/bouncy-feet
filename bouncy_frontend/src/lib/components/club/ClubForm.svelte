<script>
  import { t } from '$lib/i18n';
  import * as api from '$lib/peertube-openapi';
  import { onDestroy, tick } from 'svelte';
  import ActorAvatar from '../profile/ActorAvatar.svelte';
  import ImageCropper from '../ui/ImageCropper.svelte';
  import PopupWithRunes from '../ui/PopupWithRunes.svelte';
  import Symbol from '../ui/Symbol.svelte';

  /**
   * @typedef {Object} Props
   * @property {EditableClubDetails} details
   * @property {string} [name] -- for binding
   * @property {Club} [club]
   * @property {api.VideoChannel} [clubChannel]
   * @property {(imageBlob: Blob|null)=>void} onSubmit
   */
  /** @type {Props} */
  let {
    club,
    details = $bindable(),
    name = $bindable(club?.name),
    clubChannel,
    onSubmit,
  } = $props();

  let cropper = $state();
  let showCropPopup = $state(false);
  /** @type {Blob|null} */
  let imageBlob = $state(null);
  /** @type {string|null} */
  let imageBlobUrl = $state(null);

  let isNew = $derived(!club);

  export function reset() {
    if (imageBlobUrl) {
      URL.revokeObjectURL(imageBlobUrl);
    }
    imageBlobUrl = null;
    imageBlob = null;
  }

  /** @param {Event} event */
  async function openCropper(event) {
    event.preventDefault();
    showCropPopup = true;
    await tick();
    cropper.openFileInput();
  }

  async function imageSelected() {
    imageBlob = await cropper.getCroppedBlob();
    if (imageBlobUrl) {
      URL.revokeObjectURL(imageBlobUrl);
    }
    if (imageBlob) {
      imageBlobUrl = URL.createObjectURL(imageBlob);
    }
    showCropPopup = false;
  }

  /** @param {Event} event */
  async function save(event) {
    event.preventDefault();
    onSubmit(imageBlob);
  }

  onDestroy(() => {
    if (imageBlobUrl) {
      URL.revokeObjectURL(imageBlobUrl);
    }
    imageBlobUrl = null;
  });

  /** @param {{ target: HTMLInputElement } } event */
  function fixUrl(event) {
    const target = event.target;
    if (target && target.value && !target.value.startsWith('http')) {
      target.value = 'https://' + target.value;
    }
  }
</script>

<form onsubmit={save}>
  <label for="title"> {$t('club.form-title')}: </label>
  <input
    type="text"
    name="title"
    required
    maxlength="64"
    bind:value={name}
    disabled={!isNew}
  />

  <label for="description"> {$t('club.form-description')}: </label>
  <textarea
    name="description"
    maxlength="1024"
    rows="5"
    cols="36"
    required
    bind:value={details.description}
  ></textarea>

  <label for="url"> {$t('club.form-url')}: </label>
  <input
    type="url"
    name="url"
    maxlength="255"
    bind:value={details.url}
    oninvalid={fixUrl}
  />

  <label for="image"> {$t('club.form-logo')}: </label>
  <div class="logo">
    <button onclick={openCropper}>
      <Symbol size={28}>edit</Symbol>
    </button>
    {#if imageBlobUrl}
      <img class="new-logo" src={imageBlobUrl} alt="new logo" />
    {:else}
      <ActorAvatar actor={clubChannel} />
    {/if}
  </div>

  <button type="submit"
    >{$t(isNew ? 'club.create-new-button' : 'club.update-button')}</button
  >
</form>

<PopupWithRunes isOpen={showCropPopup}>
  <div class="logo">
    <ImageCropper bind:this={cropper} width={600} height={600} />
    <button onclick={imageSelected}>{$t('profile.button-select-image')}</button>
  </div>
</PopupWithRunes>

<style>
  form {
    display: grid;
    gap: 1rem;
    grid-template-columns: 1fr;
  }

  .logo {
    display: grid;
    gap: 1rem;
    padding: 1rem;
    justify-content: center;
    justify-items: center;
    background-color: var(--theme-main-alt);
    color: var(--theme-neutral-dark);
  }

  .new-logo {
    width: 192px;
    border-radius: 1rem;
    border: solid var(--theme-main-medium) 0.25rem;
  }

  @media (min-width: 730px) {
    form {
      grid-template-columns: min-content auto;
    }
  }
</style>
