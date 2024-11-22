<script>
  import { t } from '$lib/i18n';
  import {
    PoseWrapper,
    StepPositionBuilder,
  } from '$lib/instructor/bouncy_instructor';
  import { Orientation } from '$lib/instructor/bouncy_instructor_bg';
  import Info from '../ui/Info.svelte';
  import UiBox from '../ui/UiBox.svelte';

  /** @type {StepPositionBuilder | undefined} */
  let position;
  /** @type {PoseWrapper | undefined}*/
  let pose;
  let jumpHeight = 1.0;
  let selectedOrientation = Orientation[Orientation.Any];

  /** @param {StepPositionBuilder|undefined} newPosition */
  export function loadPosition(newPosition) {
    position = newPosition;
    jumpHeight = position?.jumpHeight || 1.0;
    selectedOrientation = Orientation[position?.orientation || Orientation.Any];
    pose = position?.pose();
  }

  /** @param {StepPositionBuilder} newPosition */
  export let onChange = (newPosition) => {};

  $: console.log('selectedOrientation', selectedOrientation);

  const orientationOptions = Object.keys(Orientation).filter((key) =>
    isNaN(key)
  );

  function changedOrientation() {
    if (!position) {
      return;
    }
    position.setOrientation(Orientation[selectedOrientation]);
    onChange(position);
  }

  function updateJumpHeight() {
    if (!position) {
      return;
    }
    position.setJumpHeight(jumpHeight);
    onChange(position);
  }
</script>

{#if position}
  <div class="container">
    <UiBox title="editor.step.edit-position">
      <div class="columns">
        <Info
          title="editor.step.jump-height"
          text="editor.step.jump-height-info"
        />
        <label for="jh">{$t('editor.step.jump-height')}</label>
        <input
          name="jh"
          type="number"
          bind:value={jumpHeight}
          on:change={updateJumpHeight}
          min={0.0}
          max={10.0}
          step="0.1"
        />

        <Info title="editor.step.direction" text="editor.step.direction-info" />
        <label for="direction">{$t('editor.step.direction')}</label>
        <select
          name="direction"
          bind:value={selectedOrientation}
          on:change={changedOrientation}
        >
          {#each orientationOptions as option}
            <option value={option}>
              {option}
            </option>
          {/each}
        </select>
      </div>
    </UiBox>
  </div>
{/if}

<style>
  .container {
    max-width: 500px;
    margin: 20px auto;
  }

  .columns {
    display: grid;
    grid-template-columns: 32px 1fr 1fr;
    justify-items: stretch;
    align-items: center;
    justify-content: space-around;
    gap: 15px 5px;
    width: 67%;
  }

  .columns label {
    justify-self: end;
    padding-right: 10px;
  }

  input {
    font-size: 20px;
  }
</style>
