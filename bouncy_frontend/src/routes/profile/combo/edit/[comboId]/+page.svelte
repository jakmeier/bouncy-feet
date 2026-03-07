<script>
  import * as api from '$lib/peertube-openapi';
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import ComboForm from '$lib/components/combo/ComboForm.svelte';
  import LoginRequiredContent from '$lib/components/profile/LoginRequiredContent.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import LimeSection from '$lib/components/ui/sections/LimeSection.svelte';
  import LoadAndShowPeertubeVideo from '$lib/components/ui/video/LoadAndShowPeertubeVideo.svelte';
  import { t } from '$lib/i18n';
  import { detectBpm } from '$lib/bounce_listener';

  /** @type {import('./$types').PageProps} */
  let { data } = $props();

  const comboId = Number.parseInt(page.params.comboId || '0');

  let dirty = $state(false);
  let details = $state(deserializeFromQuery(page.url.search));
  /** @type {LoadAndShowPeertubeVideo | undefined} */
  let player = $state();
  /** @type {api.VideoDetails | undefined}*/
  let video = $state();
  /** @type {Beat[] | undefined} */
  let beats = $state([]);
  /** @type {Beat | undefined} */
  let beat = $state();

  // quick and dirty sync on initial loading, just pick the last beat and ignore others
  $effect(() => {
    if (beat === undefined && beats && beats.length >= 1) {
      beat = beats[beats.length - 1];
    }
  });

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
    if (response) {
      /** @type {ComboInfo}*/
      const newCombo = await response.json();
      details = newCombo;
      dirty = false;
    }
  }

  /** @param {ApiUser} apiUser */
  async function saveAndLeave(apiUser) {
    // TODO: only update what is needed, and do it concurrently
    await saveCombo(apiUser);
    if (beat) {
      await saveBeat(apiUser);
    }
    goto('../../..', { replaceState: true });
  }

  /** @param {ApiUser} apiUser */
  async function addTimestamp(apiUser) {
    if (player) {
      const secs = await player.getCurrentTime();
      const body = {
        ms: Math.round(secs * 1000),
      };
      const response = await apiUser.authenticatedPost(
        `/combos/${comboId}/timestamp/new`,
        body
      );
    }
  }

  /** @param {ApiUser} apiUser */
  async function saveBeat(apiUser) {
    if (beat) {
      const body = {
        start: Math.round(beat.start),
        duration: Math.round(beat.duration),
        bpm: beat.bpm,
        subbeat_per_move: beat.subbeat_per_move,
      };

      // Update case
      // TODO: implement update
      // For now, just delete the old beat(s) and replace it with a new one.
      for (var b of beats?.slice(0, -1) || []) {
        if (b.hasOwnProperty('id')) {
          const response = await apiUser.authenticatedApiRequest(
            'DELETE',
            `/combos/${comboId}/beat/${b.id}`,
            {},
            undefined
          );
          if (response.error) {
            console.error(
              'failed deleting old beat',
              response.error,
              response.errorBody
            );
            return;
          }
        }
      }

      // Store new beat
      const response = await apiUser.authenticatedPost(
        `/combos/${comboId}/beat/new`,
        body
      );
      if (!response?.ok) {
        console.error(
          'failed deleting old beat',
          response?.status,
          await response?.text()
        );
        return;
      }
      beat = await response.json();
    }
  }

  async function startBpmAnalysis() {
    if (!video) {
      console.warn('no video loaded, cannot run BPM detection');
      return;
    }

    if ((await player?.getCurrentTime()) === 0) {
      // Nothing will be shown to the user, until the video player loads the
      // video. Force that to give user feedback after the beat has been
      // detected. (This is hacky but at least better than no feedback.)
      // Also, seeking afterwards only works if video is loaded.
      await player?.play();
      await player?.pause();
    }

    const beatResult = await detectBpm(video);
    if (beatResult) {
      beat = {
        bpm: Math.round(beatResult?.bpm * 10) / 10,
        start: beatResult.offset,
        duration: (video.duration || 30) * 1000 - beatResult.offset,
        subbeat_per_move: 1,
      };
      await player?.seek((beat?.start || 0) / 1000);
      if (beats) {
        beats.push(beat);
      } else {
        beats = [beat];
      }
      dirty = true;
    } else {
      // TODO: show user something
      console.warn('beat detection failed');
    }

    // TODO: show detected BPM value to user somewhere and store it as meta data, too
    // TODO: allow editing, such as changing bpm + offset
    // TODO: on accept => store timestamps online
  }
</script>

<LoginRequiredContent>
  {#snippet children({ apiUser })}
    <LimeSection>
      <h1>{details.title || $t('profile.combo.edit-title')}</h1>

      <div class="video">
        <LoadAndShowPeertubeVideo
          bind:this={player}
          bind:video
          videoId={details.video_short_uuid}
          timeline={{
            position: 'external',
            beatCounts: true,
            subbeat_per_move: beat?.subbeat_per_move,
          }}
          {apiUser}
          {comboId}
          bind:beats
        />
      </div>

      <!-- TODO: make this more useful -->
      <!-- <button class="full-width action" onclick={() => addTimestamp(apiUser)}>
        {$t('profile.combo.add-timestamp-button')}
      </button> -->

      <div class="form">
        <ComboForm bind:details bind:dirty bind:beat />
      </div>

      <button class="full-width action" onclick={startBpmAnalysis}>
        {$t('editor.video.detect-bpm-button')}
      </button>

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
