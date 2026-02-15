<script>
  import Juggler from '../ui/Juggler.svelte';
  import LoadAndShowPeertubeVideo from '../ui/video/LoadAndShowPeertubeVideo.svelte';

  /**
   * @typedef {Object} Props
   * @prop {ComboInfo[]} combos
   */

  /** @type {Props}*/
  let { combos } = $props();

  let jugglerHeight = $state();
</script>

<div class="juggler-container" bind:clientHeight={jugglerHeight}>
  <Juggler items={combos} buttonHeight="calc({jugglerHeight / 2}px - 1.5rem)">
    {#snippet element(
      /** @type {{item: ComboInfo, index: number}}*/
      { item: combo, index }
    )}
      <p>
        {combo.title}
      </p>
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
  .juggler-container {
    min-height: 150px;
    margin: 0 0 2rem;
  }
</style>
