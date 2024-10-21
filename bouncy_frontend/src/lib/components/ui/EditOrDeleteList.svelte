<script>
  import { t } from '$lib/i18n';
  import { counter } from '$lib/timer';
  import { getContext } from 'svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import { goto } from '$app/navigation';
  import Popup from '$lib/components/ui/Popup.svelte';

  const localCollectionCtx = getContext('localCollection');

  /** @type {any[]} */
  export let items;

  let selectedIndex = -1;
  /** @type {import('svelte/store').Writable.<boolean>}*/
  let showDeleteConfirmation;

  /** @param {number} index */
  function select(index) {
    selectedIndex = index;
  }

  export let onEdit = (_index) => {};
  export let onDelete = (_index) => {};

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
      on:click={() => select(index)}
      on:keydown={() => select(index)}
      role="button"
      tabindex={0}
    >
      <div class="inner-item">
        <slot name="item" {index} {item} selected={index === selectedIndex} />
        {#if selectedIndex === index}
          <div class="selected">
            <Button
              class="light small"
              symbolSize={18}
              symbol="edit"
              on:click={editItem}
            />
            <Button
              class="light small"
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
  <slot
    name="confirm-delete-text"
    index={selectedIndex}
    item={items[selectedIndex]}
  />

  <button class="light wide" on:click={cancelDelete}
    >{$t('editor.cancel-delete-button')}</button
  >
  <button class="light wide" on:click={deleteConfirmed}
    >{$t('editor.confirm-delete-button')}</button
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
    position: relative;
  }

  .selected {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;

    background-color: #c2bfff80;
    border-radius: 20px;

    display: flex;
    align-items: center;
    justify-content: space-around;
  }
</style>
