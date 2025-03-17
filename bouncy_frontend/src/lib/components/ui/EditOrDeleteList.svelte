<!-- @migration-task Error while migrating Svelte code: This migration would change the name of a slot making the component unusable -->
<script>
  import { t } from '$lib/i18n';
  import Button from '$lib/components/ui/Button.svelte';
  import Popup from '$lib/components/ui/Popup.svelte';
  import { writable } from 'svelte/store';

  /** @typedef {Object} Props
   * @property {import('svelte').Snippet<[{item: any, index: number, selected: boolean}]>} itemWrapper
   * @property {import('svelte').Snippet<[{item: any, index: number}]>} confirmDeleteText
   * @property {any[]} items
   * @property { (index: number) => void } onEdit
   * @property { (index: number) => void } onDelete
   */

  /** @type {Props} */
  let { itemWrapper, confirmDeleteText, items, onEdit, onDelete } = $props();

  let selectedIndex = $state(-1);
  /** @type {import('svelte/store').Writable.<boolean>}*/
  let showDeleteConfirmation = $state(writable(false));

  /** @param {number} index */
  function select(index) {
    selectedIndex = index;
  }

  function editItem() {
    onEdit(selectedIndex);
  }

  function deleteItem() {
    $showDeleteConfirmation = true;
  }

  function deleteConfirmed() {
    onDelete(selectedIndex);
    $showDeleteConfirmation = false;
    selectedIndex = -1;
  }

  function cancelDelete() {
    $showDeleteConfirmation = false;
  }
</script>

<div class="items">
  {#each items as item, index}
    <div
      class="item"
      onclick={() => select(index)}
      onkeydown={() => select(index)}
      role="button"
      tabindex={0}
    >
      <div class="inner-item">
        {@render itemWrapper({
          selected: index === selectedIndex,
          index,
          item,
        })}
        {#if selectedIndex === index}
          <div class="selected">
            <Button
              class="small"
              symbolSize={18}
              symbol="edit"
              on:click={editItem}
            />
            <Button
              class="small"
              symbolSize={18}
              symbol="delete"
              on:click={deleteItem}
            />
          </div>
        {/if}
      </div>
    </div>
  {/each}
</div>

<Popup
  bind:isOpen={showDeleteConfirmation}
  title={'editor.delete-confirmation-title'}
>
  {@render confirmDeleteText({
    index: selectedIndex,
    item: items[selectedIndex],
  })}

  <button class="danger wide" onclick={deleteConfirmed}
    >{$t('editor.confirm-delete-button')}</button
  >
  <button class="cancel wide" onclick={cancelDelete}
    >{$t('editor.cancel-delete-button')}</button
  >
</Popup>

<style>
  .items {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
  }
  .item {
    max-width: 200px;
  }
  .inner-item {
    max-width: 200px;
    height: 100%;
    position: relative;
  }

  .selected {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;

    background-color: var(--theme-main-60);
    border-radius: 5px;

    display: flex;
    align-items: center;
    justify-content: space-around;
  }
</style>
