<script>
  import { onMount } from 'svelte';

  /**
   * @typedef {Object} Props
   * @property {string} [name]
   * @property {number} width
   * @property {number} height
   */

  /** @type {Props} */
  let { width, height, name = 'image' } = $props();

  /** @type {HTMLInputElement} */
  let fileInput;
  /** @type {HTMLCanvasElement} */
  let canvas;
  /** @type {CanvasRenderingContext2D} */
  let ctx;

  let img = new Image();
  let imgLoaded = false;

  // Image transform
  let scale = 1;
  let offsetX = 0;
  let offsetY = 0;

  // Drag state
  let dragging = false;
  let lastX = 0;
  let lastY = 0;

  /**
   * Export cropped image as Blob (for FormData)
   *
   * @returns {Promise<Blob>}
   */
  export function getCroppedBlob() {
    return new Promise((resolve) => {
      canvas.toBlob((blob) => resolve(blob), 'image/png');
    });
  }

  /** @param {Event} e */
  function onFileChange(e) {
    /** @type {HTMLInputElement} */
    const target = e.target;
    const file = target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = () => {
      img.onload = () => {
        imgLoaded = true;

        // Scale image to fully cover crop area
        const scaleX = width / img.width;
        const scaleY = height / img.height;
        scale = Math.max(scaleX, scaleY);

        // Center image initially
        offsetX = (width - img.width * scale) / 2;
        offsetY = (height - img.height * scale) / 2;

        draw();
      };
      img.src = reader.result;
    };
    reader.readAsDataURL(file);
  }

  function draw() {
    ctx.clearRect(0, 0, width, height);

    if (!imgLoaded) {
      ctx.fillStyle = '#eee';
      ctx.fillRect(0, 0, width, height);
      return;
    }

    ctx.drawImage(img, offsetX, offsetY, img.width * scale, img.height * scale);
  }

  /** @param { PointerEvent} e */
  function onPointerDown(e) {
    if (!imgLoaded) return;
    dragging = true;
    lastX = e.clientX;
    lastY = e.clientY;
    canvas.setPointerCapture(e.pointerId);
  }

  /** @param { PointerEvent} e */
  function onPointerMove(e) {
    if (!dragging) return;

    const dx = e.clientX - lastX;
    const dy = e.clientY - lastY;

    offsetX += dx;
    offsetY += dy;

    lastX = e.clientX;
    lastY = e.clientY;

    // Clamp movement so image always covers crop area
    const minX = width - img.width * scale;
    const minY = height - img.height * scale;

    offsetX = Math.min(0, Math.max(minX, offsetX));
    offsetY = Math.min(0, Math.max(minY, offsetY));

    draw();
  }

  /** @param { PointerEvent} e */
  function onPointerUp(e) {
    dragging = false;
    canvas.releasePointerCapture(e.pointerId);
  }

  onMount(() => {
    ctx = canvas.getContext('2d');
    draw();
  });
</script>

<input
  type="file"
  accept="image/*"
  bind:this={fileInput}
  onchange={onFileChange}
/>

<canvas
  bind:this={canvas}
  {width}
  {height}
  onpointerdown={onPointerDown}
  onpointermove={onPointerMove}
  onpointerup={onPointerUp}
  onpointerleave={onPointerUp}
>
</canvas>

<style>
  canvas {
    display: block;
    margin-top: 0.5rem;
    border: 1px solid #ccc;
    touch-action: none;
    cursor: grab;
  }
</style>
