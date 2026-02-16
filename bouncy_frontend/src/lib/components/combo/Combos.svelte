<script>
  import Juggler from '../ui/Juggler.svelte';
  import Symbol from '../ui/Symbol.svelte';
  import LoadAndShowPeertubeVideo from '../ui/video/LoadAndShowPeertubeVideo.svelte';

  /**
   * @typedef {Object} Props
   * @prop {ComboInfo[]} combos
   * @prop {boolean} [showEditLink]
   */

  /** @type {Props}*/
  let { combos, showEditLink = false } = $props();

  let jugglerHeight = $state();

  /**
   * @param {ComboInfo} combo
   * @returns {string}
   */
  function editLink(combo) {
    const params = new URLSearchParams();
    for (const key in combo) {
      const value = combo[key];
      if (value !== undefined && value !== null) {
        params.set(
          key,
          typeof value === 'object' ? JSON.stringify(value) : String(value)
        );
      }
    }
    const query = params.toString();
    return `/profile/combo/edit/${combo.id}?${query}`;
  }
</script>

<div class="juggler-container" bind:clientHeight={jugglerHeight}>
  <Juggler items={combos} buttonHeight="calc({jugglerHeight / 2}px - 1.5rem)">
    {#snippet element(
      /** @type {{item: ComboInfo, index: number}}*/
      { item: combo, index }
    )}
      <p class="title">
        {combo.title}
      </p>
      <div class="edit-link" class:centered={!combo.title}>
        {#if showEditLink}
          <a href={editLink(combo)}>
            <Symbol size={48}>edit</Symbol>
          </a>
        {/if}
      </div>
      {#if combo.video_short_uuid}
        <div class="video">
          <LoadAndShowPeertubeVideo
            videoId={combo.video_short_uuid}
            timeline="external"
          />
        </div>
      {/if}
    {/snippet}
  </Juggler>
</div>

<style>
  p.title {
    min-height: 48px;
  }

  .juggler-container {
    min-height: 150px;
    margin: 0 0 2rem;
  }

  .edit-link {
    position: absolute;
    right: 0;
    top: 0;
  }

  .edit-link.centered {
    right: 50%;
    transform: translate(50%, 0);
  }
</style>
