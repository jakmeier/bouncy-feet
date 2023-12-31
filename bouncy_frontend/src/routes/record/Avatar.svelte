<script>
  import { add2dVector, distance2d } from '$lib/math';
  import { I, TORSO, bodyOutlinePairs } from '$lib/pose';
  import { getContext } from 'svelte';

  /** @type import('@mediapipe/tasks-vision').NormalizedLandmark[] */
  export let landmarks = [];
  /** @type import('$lib/instructor/bouncy_instructor').Skeleton */
  export let skeleton;
  export let width = 100;
  export let height = 100;
  export let lineWidth = 10;
  export let lengths = {
    thigh: 0.2,
    shin: 0.2,
    torso: 0.25,
    arm: 0.1,
    forearm: 0.15,
    foot: 0.05,
  };

  $: sideway = skeleton ? skeleton.sideway : false;

  const mainColor = '#382eeb';
  const secondColor = '#c2bfff';

  getContext('canvas').addItem(draw);

  /**
   * @param {CanvasRenderingContext2D} ctx
   */
  function draw(ctx) {
    ctx.strokeStyle = mainColor;
    ctx.lineWidth = lineWidth;
    ctx.lineCap = 'round';
    ctx.fillStyle = secondColor;

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

    bodyOutlinePairs.forEach(([a, b]) => {
      drawLine(
        ctx,
        { x: landmarks[a].x * w, y: landmarks[a].y * h },
        { x: landmarks[b].x * w, y: landmarks[b].y * h }
      );
    });

    // torso
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
    const x = landmarks[I.NOSE].x * w;
    const y = landmarks[I.NOSE].y * h;
    drawHead(ctx, x, y, headRadius);
  }

  /**
   * @param {CanvasRenderingContext2D} ctx
   */
  function drawSkeleton(ctx) {
    const s = Math.min(height, width);
    const hip = { x: 0.5 * width, y: 0.5 * height };
    const shoulder = { x: hip.x, y: hip.y - lengths.torso * s };
    const shoulderLen = sideway ? 0.0 : 0.05 * s;
    const hipLen = sideway ? 0.0 : 0.03 * s;

    // right body part is left on screen
    const leftHip = { x: hip.x + hipLen, y: hip.y };
    const leftShoulder = { x: shoulder.x + shoulderLen, y: shoulder.y };
    drawSide(ctx, leftHip, leftShoulder, skeleton.left, s);
    const rightHip = { x: hip.x - hipLen, y: hip.y };
    const rightShoulder = { x: shoulder.x - shoulderLen, y: shoulder.y };
    drawSide(ctx, rightHip, rightShoulder, skeleton.right, s);
    drawHead(ctx, shoulder.x, shoulder.y - 0.1 * s, 0.075 * s);

    if (!sideway) {
      drawLine(ctx, leftHip, rightHip);
      drawLine(ctx, leftShoulder, rightShoulder);
    }
  }

  /**
   * @param {CanvasRenderingContext2D} ctx
   * @param {{ x: number; y: number; }} hip
   * @param {import("$lib/instructor/bouncy_instructor").SkeletonSide} side
   * @param {number} s
   * @param {{ x: number; y: number; }} shoulder
   */
  function drawSide(ctx, hip, shoulder, side, s) {
    const knee = add2dVector(
      hip,
      side.thigh.angle,
      side.thigh.r * lengths.thigh * s
    );
    const ankle = add2dVector(
      knee,
      side.shin.angle,
      side.shin.r * lengths.shin * s
    );
    // The foot is actually measured between heel and toe, but the skelton
    // doesn't give information about wrist to heel. I could do some
    // approximation, or actually track this. But seems not necessary. This
    // looks good enough for the 2d render.
    const heel = ankle;
    const toe = add2dVector(
      heel,
      side.foot.angle,
      side.foot.r * lengths.foot * s
    );
    const elbow = add2dVector(
      shoulder,
      side.arm.angle,
      side.arm.r * lengths.arm * s
    );
    const wrist = add2dVector(
      elbow,
      side.forearm.angle,
      side.forearm.r * lengths.forearm * s
    );
    drawLine(ctx, hip, knee);
    drawLine(ctx, knee, ankle);
    drawLine(ctx, shoulder, hip);
    drawLine(ctx, shoulder, elbow);
    drawLine(ctx, elbow, wrist);
    drawLine(ctx, heel, toe);
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
    ctx.fillStyle = mainColor;
    ctx.beginPath();
    ctx.arc(x, y, headRadius, 0, 2 * Math.PI);
    ctx.fill();
  }
</script>
