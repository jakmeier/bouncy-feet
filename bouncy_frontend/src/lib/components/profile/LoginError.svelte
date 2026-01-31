<script>
  import { t } from '$lib/i18n';
  import { asset } from '$app/paths';
  import { goto } from '$app/navigation';

  /**
   * @typedef {Object} Props
   * @property {BfError} loginError
   */

  /** @type {Props} */
  let { loginError } = $props();
</script>

<h2>{$t(loginError.title)}</h2>
<div class="error-symbol">
  <img src={asset('/img/symbols/bf_error.svg')} alt="bf_error" />
</div>
<div class="block">
  {#if loginError.description}
    <div>{$t(loginError.description)}</div>
  {/if}
  <div>{$t('common.sorry-error')}</div>
</div>
<button onclick={() => history.back()}>
  {$t('common.go-to-prev-page-button')}
</button>
<button onclick={() => goto('/')}>
  {$t('common.go-home-button')}
</button>
<div class="block">
  <div>{$t('common.report-error')}</div>
</div>

<style>
  .error-symbol img {
    max-width: 6rem;
  }

  .block {
    padding: 1rem 0;
  }
</style>
