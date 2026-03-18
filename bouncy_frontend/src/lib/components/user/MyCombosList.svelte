<script>
  // TODO: pagination, search, sorting based on activity, etc etc

  import { onMount } from 'svelte';
  import UnstyledButton from '$lib/components/ui/UnstyledButton.svelte';
  import { apiRequest } from '$lib/stats';
  import ActorAvatar from '../profile/ActorAvatar.svelte';

  /**
   * @typedef {Object} Props
   * @property {(user: ComboInfo)=>void} onSelect
   */

  /** @type {Props} */
  let { onSelect } = $props();

  /** @return {Promise<ComboInfo[]>} */
  async function load() {
    const res = await apiRequest('/user/combos');
    let result = await res.okResponse?.json();
    return result?.combos;
  }

  onMount(load);
</script>

{#await load()}
  <p>Loading...</p>
{:then combos}
  <ul>
    {#each combos as combo}
      <UnstyledButton onClick={() => onSelect(combo)}>
        <li>
          <div>
            {combo.title}
            ({combo.id})
          </div>
        </li>
      </UnstyledButton>
    {/each}
  </ul>
{:catch error}
  <p style="color:red">{error.message}</p>
{/await}

<style>
  ul {
    color: var(--theme-neutral-dark);
    width: 100%;
    max-height: 40vh;
    overflow-y: auto;
  }

  li {
    display: flex;
    gap: 0.5rem;
    text-align: left;
    align-items: center;
    background-color: var(--theme-main-light);
    padding: 0.5rem;
    border-radius: 0.5rem;
    border-top: var(--theme-main) solid 1px;
  }

  li:last-child {
    border-bottom: var(--theme-main) solid 1px;
  }
</style>
