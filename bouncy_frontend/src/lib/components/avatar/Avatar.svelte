<script>
  import { LimbError } from '$lib/instructor/bouncy_instructor';
  import { distance2d } from '$lib/math';
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
  /** @type LimbError[] */
  export let markedLimbs = [];

  /** @type {AvatarColoring} */
  export let style = {
    leftColor: '#000000FF',
    rightColor: '#000000FF',
    headColor: '#00000040',
    bodyColor: '#00000010',
  };

  getContext('canvas').addItem(draw);

  const markerColor = '#ff111188';
  const markerLineWidth = 20;

  /**
   * @param {number} i
   * @param {number} w
   * @param {number} h
   */
  function index_to_cartesian(i, w, h) {
    return { x: landmarks[i].x * w, y: landmarks[i].y * h };
  }

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
      drawLine(ctx, index_to_cartesian(a, w, h), index_to_cartesian(b, w, h));
    });
    ctx.lineWidth = lineWidth;

    ctx.strokeStyle = style.leftColor;
    leftSidePairs.forEach(([a, b]) => {
      drawLine(ctx, index_to_cartesian(a, w, h), index_to_cartesian(b, w, h));
    });

    ctx.strokeStyle = style.rightColor;
    rightSidePairs.forEach(([a, b]) => {
      drawLine(ctx, index_to_cartesian(a, w, h), index_to_cartesian(b, w, h));
    });

    draw_torso(ctx, w, h);
    drawHead(ctx, w, h);
    drawMarkers(ctx, w, h);
  }

  /**
   * @param {CanvasRenderingContext2D} ctx
   * @param {number} w
   * @param {number} h
   */
  function draw_torso(ctx, w, h) {
    ctx.beginPath();
    ctx.moveTo(landmarks[TORSO[0]].x * w, landmarks[TORSO[0]].y * h);
    TORSO.slice(1).forEach((i) => {
      ctx.lineTo(landmarks[i].x * w, landmarks[i].y * h);
    });
    ctx.fill();
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
   * @param {number} w
   * @param {number} h
   */
  function drawHead(ctx, w, h) {
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

    ctx.fillStyle = style.headColor;
    ctx.beginPath();
    ctx.arc(x, y, headRadius, 0, 2 * Math.PI);
    ctx.fill();
  }

  /**
   * @param {CanvasRenderingContext2D} ctx
   * @param {number} w
   * @param {number} h
   */
  function drawMarkers(ctx, w, h) {
    ctx.strokeStyle = markerColor;
    ctx.lineWidth = markerLineWidth;
    for (let i = 0; i < markedLimbs.length; i++) {
      let start, end;
      switch (markedLimbs[i].name) {
        case 'LeftThigh':
          start = I.LEFT_HIP;
          end = I.LEFT_KNEE;
          break;
        case 'LeftShin':
          start = I.LEFT_KNEE;
          end = I.LEFT_ANKLE;
          break;
        case 'LeftFoot':
          start = I.LEFT_HEEL;
          end = I.LEFT_FOOT_INDEX;
          break;
        case 'LeftArm':
          start = I.LEFT_SHOULDER;
          end = I.LEFT_ELBOW;
          break;
        case 'LeftForearm':
          start = I.LEFT_ELBOW;
          end = I.LEFT_WRIST;
          break;
        case 'RightThigh':
          start = I.RIGHT_HIP;
          end = I.RIGHT_KNEE;
          break;
        case 'RightShin':
          start = I.RIGHT_KNEE;
          end = I.RIGHT_ANKLE;
          break;
        case 'RightFoot':
          start = I.RIGHT_HEEL;
          end = I.RIGHT_FOOT_INDEX;
          break;
        case 'RightArm':
          start = I.RIGHT_SHOULDER;
          end = I.RIGHT_ELBOW;
          break;
        case 'RightForearm':
          start = I.RIGHT_ELBOW;
          end = I.RIGHT_WRIST;
          break;
        default:
          console.warn('could not draw marked limb', markedLimbs[i].name);
          continue;
      }
      drawLine(
        ctx,
        index_to_cartesian(start, w, h),
        index_to_cartesian(end, w, h)
      );
    }
    ctx.lineWidth = lineWidth;
  }
</script>
