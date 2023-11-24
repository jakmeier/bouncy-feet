<script>
  import { distance2d } from '$lib/math';
  import { I, TORSO, bodyOutlinePairs } from '$lib/pose';
  import { getContext } from 'svelte';

  /** @type import('@mediapipe/tasks-vision').NormalizedLandmark[] */
  export let landmarks;
  export let width = 100;
  export let height = 100;

  getContext('canvas').addItem(draw);

  /**
   *
   * @param {CanvasRenderingContext2D} ctx
   */
  function draw(ctx) {
    if (landmarks.length === 0) {
      return;
    }
    const mainColor = '#382eeb';

    ctx.strokeStyle = mainColor;
    ctx.lineWidth = 10;
    ctx.lineCap = 'round';

    bodyOutlinePairs.forEach(([a, b]) => {
      ctx.beginPath();
      ctx.moveTo(landmarks[a].x * width, landmarks[a].y * height);
      ctx.lineTo(landmarks[b].x * width, landmarks[b].y * height);
      ctx.stroke();
    });

    // torso
    ctx.fillStyle = '#c2bfff';
    ctx.beginPath();
    ctx.moveTo(landmarks[TORSO[0]].x * width, landmarks[TORSO[0]].y * height);
    TORSO.slice(1).forEach((i) => {
      ctx.lineTo(landmarks[i].x * width, landmarks[i].y * height);
    });
    ctx.fill();

    const shoulder = distance2d(
      landmarks[I.LEFT_SHOULDER],
      landmarks[I.RIGHT_SHOULDER]
    );
    const headRadius = 0.4 * shoulder * width;
    // head
    ctx.fillStyle = mainColor;
    ctx.beginPath();
    ctx.arc(
      landmarks[I.NOSE].x * width,
      landmarks[I.NOSE].y * height,
      headRadius,
      0,
      2 * Math.PI
    );
    ctx.fill();
  }
</script>
