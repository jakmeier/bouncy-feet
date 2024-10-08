<script>
  import { Skeleton } from '$lib/instructor/bouncy_instructor';

  let bodyParts = [
    { name: 'Left Arm', angle: 0, isKey: false, weight: null },
    { name: 'Right Arm', angle: 0, isKey: false, weight: null },
    { name: 'Left Forearm', angle: 0, isKey: false, weight: null },
    { name: 'Right Forearm', angle: 0, isKey: false, weight: null },
    { name: 'Left Thigh', angle: 0, isKey: false, weight: null },
    { name: 'Right Thigh', angle: 0, isKey: false, weight: null },
    { name: 'Left Shin', angle: 0, isKey: false, weight: null },
    { name: 'Right Shin', angle: 0, isKey: false, weight: null },
    { name: 'Left Foot', angle: 0, isKey: false, weight: null },
    { name: 'Right Foot', angle: 0, isKey: false, weight: null },
  ];

  /** @param {Skeleton} skeleton */
  export function loadSkeleton(skeleton) {
    bodyParts[0].angle = formatAngle(skeleton.left.arm.angle);
    bodyParts[1].angle = formatAngle(skeleton.right.arm.angle);
    bodyParts[2].angle = formatAngle(skeleton.left.forearm.angle);
    bodyParts[3].angle = formatAngle(skeleton.right.forearm.angle);
    bodyParts[4].angle = formatAngle(skeleton.left.thigh.angle);
    bodyParts[5].angle = formatAngle(skeleton.right.thigh.angle);
    bodyParts[6].angle = formatAngle(skeleton.left.shin.angle);
    bodyParts[7].angle = formatAngle(skeleton.right.shin.angle);
    bodyParts[8].angle = formatAngle(skeleton.left.foot.angle);
    bodyParts[9].angle = formatAngle(skeleton.right.foot.angle);
  }

  /**
   * @param {number} number
   * @returns {number}
   */
  function formatAngle(number) {
    return parseFloat(((180 * number) / Math.PI).toFixed(0));
  }

  //   function toggleKeyPart(index) {
  //     bodyParts[index].isKey = !bodyParts[index].isKey;
  //     if (!bodyParts[index].isKey) {
  //       bodyParts[index].weight = null;
  //     }
  //   }
</script>

<div class="body-part-group">
  {#each bodyParts as part, index}
    <div class="body-part">
      <label>{part.name}</label>
      <input
        type="number"
        class="angle-input"
        bind:value={part.angle}
        placeholder="Angle"
      />
      <!-- <button class="key-button" on:click={() => toggleKeyPart(index)}>
        {part.isKey ? 'Unset Key' : 'Set as Key'}
      </button> -->

      {#if part.isKey}
        <div class="weight-popup">
          <label>Weight</label>
          <input type="number" bind:value={part.weight} placeholder="Weight" />
        </div>
      {/if}
    </div>
  {/each}
</div>

<style>
  .body-part-group {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    justify-content: space-between;
  }
  .body-part {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 45%; /* Adjust for mobile view */
  }
  .angle-input {
    width: 100%;
    text-align: center;
  }
  .key-button {
    background: #e0e0e0;
    border: none;
    padding: 4px;
    margin-top: 4px;
    cursor: pointer;
  }
  .weight-popup {
    position: absolute;
    background-color: #f9f9f9;
    padding: 8px;
    border: 1px solid #ccc;
    border-radius: 4px;
    z-index: 10;
    width: 80px;
    top: 35px;
  }
</style>
