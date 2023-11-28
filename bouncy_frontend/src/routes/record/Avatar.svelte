<script>
  import { distance2d } from '$lib/math';
  import { I, TORSO, bodyOutlinePairs } from '$lib/pose';
  import { getContext } from 'svelte';

  /** @type import('@mediapipe/tasks-vision').NormalizedLandmark[] */
  export let landmarks;
  /** @type import('$lib/instructor/bouncy_instructor').Skeleton */
  export let skeleton;
  export let width = 100;
  export let height = 100;

  const mainColor = '#382eeb';
  const secondColor = '#c2bfff';

  getContext('canvas').addItem(draw);

  /**
   * @param {CanvasRenderingContext2D} ctx
   */
  function draw(ctx) {
    if (skeleton) {
      drawSkeleton(ctx);
    } else if (landmarks) {
      drawLandmarks(ctx);
    }
  }

  /**
   * @param {CanvasRenderingContext2D} ctx
   */
  function drawLandmarks(ctx) {
    if (landmarks.length === 0) {
      return;
    }
    // Goal: Scale the avatar to fit the canvas height even if it doesn't fill
    // the camera field.
    // Note: Something is fishy. I thought landmarks are normalized to [0,1]? So
    // using 1 instead of 2 below should scale the body beyond what fits in the
    // frame. But in all testing I've done so far, using 2 looks pretty good. It
    // scales the figure to fill about 80% of the height. Which makes no sense.
    // Probably I'm being stupid. For now, use 2 as it produces good results.
    const scaling =
      2 / Math.abs(landmarks[I.NOSE].y - landmarks[I.LEFT_FOOT_INDEX].y);
    const h = height * scaling;
    const w = width * scaling;

    ctx.strokeStyle = mainColor;
    ctx.lineWidth = 10;
    ctx.lineCap = 'round';

    bodyOutlinePairs.forEach(([a, b]) => {
      ctx.beginPath();
      ctx.moveTo(landmarks[a].x * w, landmarks[a].y * h);
      ctx.lineTo(landmarks[b].x * w, landmarks[b].y * h);
      ctx.stroke();
    });

    // torso
    ctx.fillStyle = secondColor;
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
    const headRadius = 0.4 * shoulder * w;
    // head
    ctx.fillStyle = mainColor;
    ctx.beginPath();
    ctx.arc(
      landmarks[I.NOSE].x * w,
      landmarks[I.NOSE].y * h,
      headRadius,
      0,
      2 * Math.PI
    );
    ctx.fill();
  }

  /**
   * @param {CanvasRenderingContext2D} ctx
   */
  function drawSkeleton(ctx) {
    console.log(
      'TODO: draw this skelly',
      skeleton.left.shin,
      skeleton.left.thigh,
      skeleton.right.shin,
      skeleton.right.thigh
    );
  }
</script>
