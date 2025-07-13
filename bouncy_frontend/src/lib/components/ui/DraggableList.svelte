<script>
  import Area from '$lib/components/ui/Area.svelte';
  import Symbol from '$lib/components/ui/Symbol.svelte';

  /**
   * @typedef {Object} Props
   * @property {any[]} items
   * @property {number} [elementSize]
   * @property {string} [borderRadius]
   * @property {(item: any, index: number) => void} [onRemove]
   * @property {(item: any, index: number) => void} [onDragStart]
   * @property {any} [onDragMove] - Called when a dragged element is moved in place of another element.
Returns the new index of the dragged element.
   * @property {(item: any, index: number) => void} [onDrop]
   * @property {any} [selectedIndex]
   * @property {boolean} [showAddNewItem]
   * @property {import('svelte').Snippet<[any]>} [main]
   * @property {import('svelte').Snippet<[any]>} [name]
   */

  /** @type {Props} */
  let {
    items = $bindable(),
    elementSize = 100,
    borderRadius = '25px',
    onRemove = (_item, index) => {
      items.splice(index, 1);
    },
    onDragStart = (_item, _index) => {},
    onDragMove = (_draggedItem, draggedIndex, _swappedItem, swappedIndex) => {
      // swap two array elements
      [items[draggedIndex], items[swappedIndex]] = [
        items[swappedIndex],
        items[draggedIndex],
      ];
      return swappedIndex;
    },
    onDrop = (_item, _index) => {},
    selectedIndex = $bindable(-1),
    showAddNewItem = $bindable(false),
    main,
    name,
  } = $props();

  /**
   * @param {{preventDefault: () => void;}} event
   * @param {number} index
   */
  function handleRemove(event, index) {
    event.preventDefault();
    selectedIndex = -1;
    onRemove(items[index], index);
  }

  // Drag-and-drop stuff below
  /** @type {string | null} */
  let draggedItem = null;
  /** @type {number} */
  let draggedIndex = $state(-1);

  /**
   * @param {DragEvent} event
   * @param {number} index
   */
  function handleDragStart(event, index) {
    draggedItem = items[index];
    draggedIndex = index;
    selectedIndex = -1;
    if (event.dataTransfer) {
      event.dataTransfer.effectAllowed = 'move';
      event.dataTransfer.dropEffect = 'move';
    }
    onDragStart(items[index], index);
  }

  /**
   * @param {{preventDefault: () => void;}} event
   * @param {number} index
   */
  function handleDragOver(event, index) {
    event.preventDefault();

    if (draggedItem && draggedIndex !== -1 && draggedIndex !== index) {
      draggedIndex = onDragMove(draggedItem, draggedIndex, items[index], index);
    }
  }

  /**
   * @param {TouchEvent | DragEvent } event
   */
  function handleDrop(event) {
    event.preventDefault();
    onDrop(items[draggedIndex], draggedIndex);
    draggedItem = null;
    draggedIndex = -1;
    if (touchDrag.element) {
      document.body.removeChild(touchDrag.element);
      touchDrag.element = null;
    }
  }

  function clickAddButton() {
    showAddNewItem = !showAddNewItem;
    selectedIndex = -1;
  }

  /**
   * @param {number} i
   */
  function selectItem(i) {
    if (selectedIndex === i) {
      selectedIndex = -1;
    } else {
      selectedIndex = i;
      showAddNewItem = false;
    }
  }

  // Drag and drop doesn't work natively on mobile.
  // I tried libraries but they don't work well with the level of interactivity I want.
  // And I got tons of undocumented behavior, if not straight up bugs.
  // So... Rolling my own drag and drop on mobile!
  /**
   *  @type {{  element: HTMLElement | null, clientX: number, clientY: number}}s
   */
  const touchDrag = {
    element: null,
    clientX: 0,
    clientY: 0,
  };

  /**
   * @param {TouchEvent} event
   * @param {number} index
   */
  function handleTouchStart(event, index) {
    if (
      touchDrag.element ||
      // @ts-ignore
      !(event.target && event.target.classList.contains('draggable'))
    ) {
      return;
    }
    if (event.touches.length === 1) {
      event.preventDefault();
      selectedIndex = -1;
      draggedItem = items[index];
      draggedIndex = index;

      touchDrag.clientX = event.touches[0].clientX;
      touchDrag.clientY = event.touches[0].clientY;

      // @ts-ignore
      touchDrag.element = event.target.cloneNode(true);
      // @ts-ignore
      touchDrag.element.style.position = 'fixed';
      // @ts-ignore
      touchDrag.element.style['z-index'] = '1000';
      // @ts-ignore
      document.body.appendChild(touchDrag.element);

      // @ts-ignore
      touchDrag.element.style.top = `${-touchDrag.element.offsetHeight / 2}px`;
      // @ts-ignore
      touchDrag.element.style.left = `${-touchDrag.element.offsetWidth / 2}px`;

      setTouchDragTranslate();
    }
  }

  function setTouchDragTranslate() {
    if (touchDrag.element) {
      const x = touchDrag.clientX;
      const y = touchDrag.clientY;
      touchDrag.element.style.transform = `translate3d(${x}px, ${y}px, 0)`;
    }
  }

  /**
   * @param {TouchEvent} event
   */
  function handleTouchMove(event) {
    if (touchDrag.element) {
      event.preventDefault();

      touchDrag.clientX = event.touches[0].clientX;
      touchDrag.clientY = event.touches[0].clientY;

      setTouchDragTranslate();

      const dropZone = isOverDropzone(touchDrag.clientX, touchDrag.clientY);
      if (dropZone && draggedItem) {
        let index = parseInt(dropZone.id);
        draggedIndex = onDragMove(
          draggedItem,
          draggedIndex,
          items[index],
          index
        );
      }
    }
  }

  /**
   * @param {number} x
   * @param {number} y
   */
  function isOverDropzone(x, y) {
    let dropZones = document.getElementsByClassName('drop-zone');
    for (let i = 0; i < dropZones.length; i++) {
      const dropZone = dropZones[i];
      const dropzoneRect = dropZone.getBoundingClientRect();
      if (
        touchDrag.element &&
        x + touchDrag.element.offsetWidth / 2 > dropzoneRect.left &&
        x + touchDrag.element.offsetWidth / 2 < dropzoneRect.right &&
        y + touchDrag.element.offsetHeight / 2 > dropzoneRect.top &&
        y + touchDrag.element.offsetHeight / 2 < dropzoneRect.bottom
      ) {
        return dropZone;
      }
    }
    return null;
  }
</script>

<div class="items">
  {#each items as item, i}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div
      class="item drop-zone"
      draggable="true"
      id={`${i}`}
      ondragstart={(event) => handleDragStart(event, i)}
      ondragover={(event) => handleDragOver(event, i)}
      ondrop={handleDrop}
      ondragend={handleDrop}
      onclick={() => selectItem(i)}
      style="opacity: {i === draggedIndex ? 0.3 : 1.0}"
    >
      <div class="fixed-size">
        <div class="center">
          {@render main?.({ item, index: i })}
        </div>
      </div>
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <div class="delete-button" onclick={(event) => handleRemove(event, i)}>
        <Symbol>close</Symbol>
      </div>
      <p
        class="handle draggable"
        style="width: {elementSize}px"
        ontouchstart={(event) => handleTouchStart(event, i)}
        ontouchend={handleDrop}
        ontouchmove={handleTouchMove}
      >
        <span
          class="material-symbols-outlined"
          style="pointer-events: none;"
          translate="no">open_with</span
        >
      </p>
      <p class="label" style="width: {elementSize}px">
        {@render name?.({ item, index: i })}
      </p>
    </div>
  {/each}

  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div onclick={clickAddButton}>
    <Area width="{elementSize}px" height="{elementSize}px" {borderRadius}>
      <span
        class="material-symbols-outlined add-button"
        style="font-size: {elementSize / 2}px"
        translate="no"
      >
        add_circle
      </span>
    </Area>
  </div>
</div>

<style>
  .items {
    display: flex;
    overflow: auto hidden;
  }
  .item {
    position: relative;
    margin: 2px;
    transition: all 0.2s ease-in-out;
  }
  .handle,
  .label,
  div.delete-button {
    border-radius: 5px;
    text-align: center;
    margin: 0;
    padding: 2px;
  }
  .item p {
    background-color: var(--theme-neutral-light);
  }
  p.handle {
    background-color: var(--theme-accent-light);
    margin-bottom: 5px;
  }
  div.delete-button {
    background-color: var(--theme-accent-light);
    position: absolute;
    width: 35px;
    height: 25px;
    top: 0px;
    right: 0px;
  }
  div.delete-button:hover {
    background-color: var(--theme-accent);
  }

  .fixed-size {
    height: 115px;
    width: 110px;
  }
  .center {
    margin: auto;
  }
</style>
