<script>
  import { page } from '$app/state';
  import ComboForm from '$lib/components/combo/ComboForm.svelte';
  import LoginRequiredContent from '$lib/components/profile/LoginRequiredContent.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import LimeSection from '$lib/components/ui/sections/LimeSection.svelte';
  import LoadAndShowPeertubeVideo from '$lib/components/ui/video/LoadAndShowPeertubeVideo.svelte';
  import { t } from '$lib/i18n';

  /** @type {import('./$types').PageProps} */
  let { data } = $props();

  const comboId = Number.parseInt(page.params.comboId || '0');

  let dirty = $state(false);
  let details = $state(deserializeFromQuery(page.url.search));
  /**
   * @param {string} queryString
   * @returns {ComboInfo}
   */
  function deserializeFromQuery(queryString) {
    const params = new URLSearchParams(queryString);
    return {
      id: Number(params.get('id')),
      is_private: params.get('is_private') !== 'false',
      free_form_category: params.get('free_form_category') || undefined,
      sort_order: params.has('sort_order')
        ? Number(params.get('sort_order'))
        : undefined,
      title: params.get('title') || undefined,
      video_short_uuid: params.get('video_short_uuid') || undefined,
    };
  }

  /** @param {ApiUser} apiUser */
  async function saveCombo(apiUser) {
    let response;
    if (details.hasOwnProperty('id')) {
      response = await apiUser.authenticatedPost('/combos/update', details);
    } else {
      response = await apiUser.authenticatedPost('/combos/new', details);
    }
    if (response?.ok) {
      /** @type {ComboInfo}*/
      const newCombo = await response.json();
      details = newCombo;
      dirty = false;
    }
  }

  /** @param {ApiUser} apiUser */
  async function saveAndLeave(apiUser) {
    await saveCombo(apiUser);
    history.back();
  }
</script>

<LoginRequiredContent>
  {#snippet children({ apiUser })}
    <LimeSection>
      <h1>{details.title || $t('profile.combo.edit-title')}</h1>

      <div class="video">
        <LoadAndShowPeertubeVideo
          videoId={details.video_short_uuid}
          timeline="external"
        />
      </div>

      <div class="form">
        <ComboForm bind:details bind:dirty />
      </div>

      <button
        class="full-width"
        onclick={() => saveAndLeave(apiUser)}
        disabled={!dirty}>{$t('profile.combo.save-button')}</button
      >
      <button class="full-width" onclick={() => history.back()}>
        {$t('profile.combo.cancel-button')}</button
      >

      <Footer white />
    </LimeSection>
  {/snippet}
</LoginRequiredContent>

<style>
  .form {
    margin: 1rem 0;
  }
</style>
