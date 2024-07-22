<script>
  import { add2dVector, distance2d } from '$lib/math';
  import {
    I,
    TORSO,
    leftSidePairs,
    rightSidePairs,
    torsoPairs,
  } from '$lib/pose';
  import { getContext } from 'svelte';

  /** @type import('@mediapipe/tasks-vision').NormalizedLandmark[] */
  export let landmarks = [];
  export let width = 100;
  export let height = 100;
  export let lineWidth = 10;
  export let torsoLineWidth = lineWidth;

  /** @type {AvatarColoring} */
  export let style = {
    leftColor: '#000000FF',
    rightColor: '#000000FF',
    headColor: '#00000040',
    bodyColor: '#00000010',
  };

  getContext('canvas').addItem(draw);

  /**
   * @param {CanvasRenderingContext2D} ctx
   *
   * Draws a stick figure from landmarks, exactly where they were detected on
   * the original image. Useful for overlapping a video stream.
   */
  function draw(ctx) {
    ctx.strokeStyle = style.bodyColor;
    ctx.lineWidth = lineWidth;
    ctx.lineCap = 'round';
    ctx.fillStyle = style.headColor;

    if (landmarks.length === 0) {
      return;
    }
    const h = height;
    const w = width;

    ctx.lineWidth = torsoLineWidth;
    torsoPairs.forEach(([a, b]) => {
      drawLine(
        ctx,
        { x: landmarks[a].x * w, y: landmarks[a].y * h },
        { x: landmarks[b].x * w, y: landmarks[b].y * h }
      );
    });
    ctx.lineWidth = lineWidth;

    ctx.strokeStyle = style.leftColor;
    leftSidePairs.forEach(([a, b]) => {
      drawLine(
        ctx,
        { x: landmarks[a].x * w, y: landmarks[a].y * h },
        { x: landmarks[b].x * w, y: landmarks[b].y * h }
      );
    });

    ctx.strokeStyle = style.rightColor;
    rightSidePairs.forEach(([a, b]) => {
      drawLine(
        ctx,
        { x: landmarks[a].x * w, y: landmarks[a].y * h },
        { x: landmarks[b].x * w, y: landmarks[b].y * h }
      );
    });

    // torso fill
    ctx.beginPath();
    ctx.moveTo(landmarks[TORSO[0]].x * w, landmarks[TORSO[0]].y * h);
    TORSO.slice(1).forEach((i) => {
      ctx.lineTo(landmarks[i].x * w, landmarks[i].y * h);
    });
    ctx.fill();

    const shoulder = distance2d(
      landmarks[I.LEFT_SHOULDER],
      landmarks[I.RIGHT_SHOULDER]
    );
    const eyeDist = Math.max(
      distance2d(landmarks[I.RIGHT_EYE_OUTER], landmarks[I.NOSE]),
      distance2d(landmarks[I.LEFT_EYE_OUTER], landmarks[I.NOSE])
    );
    const headRadius = Math.max(0.05, 0.4 * shoulder, eyeDist * 2) * w;
    const x = landmarks[I.NOSE].x * w;
    const y = landmarks[I.NOSE].y * h;
    drawHead(ctx, x, y, headRadius);
  }

  /**
   * @param {CanvasRenderingContext2D} ctx
   * @param {{ x: number; y: number; }} start
   * @param {{ x: number; y: number; }} end
   */
  function drawLine(ctx, start, end) {
    ctx.beginPath();
    ctx.moveTo(start.x, start.y);
    ctx.lineTo(end.x, end.y);
    ctx.stroke();
  }

  /**
   * @param {CanvasRenderingContext2D} ctx
   * @param {number} x
   * @param {number} y
   * @param {number} headRadius
   */
  function drawHead(ctx, x, y, headRadius) {
    ctx.fillStyle = style.headColor;
    ctx.beginPath();
    ctx.arc(x, y, headRadius, 0, 2 * Math.PI);
    ctx.fill();
  }
</script>
