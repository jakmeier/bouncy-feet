<script>
  import InfoHeader from './InfoHeader.svelte';
  import * as api from '$lib/peertube-openapi';
  import ActorAvatar from '$lib/components/profile/ActorAvatar.svelte';

  /**
   * @typedef {Object} Props
   * @property {string} [title]
   * @property {boolean} [white]
   * @property {boolean} [black]
   * @property {boolean} [accent]
   * @property {boolean} [mainColor]
   * @property {boolean} [transparent]
   * @property {()=>void} [onBack]
   * @property {api.Actor & api.ActorInfo} userOrChannel
   */

  /** @type {Props} */
  let {
    white = false,
    black = false,
    accent = false,
    mainColor = false,
    transparent,
    title = '',
    onBack = () => {
      window.history.back();
    },
    userOrChannel,
  } = $props();
</script>

<InfoHeader {white} {black} {accent} {mainColor} {transparent} {title} {onBack}>
  <div class="actor">
    <div class="logo">
      <ActorAvatar actor={userOrChannel} targetWidth={32} />
    </div>
    <div>{userOrChannel.displayName}</div>
  </div>
</InfoHeader>

<style>
  .actor {
    display: grid;
    grid-template-columns: 32px auto;
    align-items: center;
    max-width: 50vw;
    gap: 1rem;
    margin-bottom: 1rem;
  }

  .logo {
    width: 32px;
    margin-top: -0.25rem;
  }
</style>
