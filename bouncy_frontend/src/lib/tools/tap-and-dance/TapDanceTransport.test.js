import { afterEach, describe, expect, it, vi } from 'vitest';
import { TapDanceTransport } from './TapDanceTransport.js';

/** @param {import('./TapDanceTransport.js').PlaybackConfig['speedRamp']} [speedRamp] */
function setup(speedRamp = null) {
  const config = { bpm: 124, speedRamp };
  const transport = new TapDanceTransport(() => config, vi.fn(), vi.fn());
  const context = { currentTime: 0, state: 'running', resume: vi.fn() };
  transport.context = /** @type {AudioContext} */ (
    /** @type {unknown} */ (context)
  );
  vi.spyOn(transport, 'ensureContext').mockResolvedValue(transport.context);
  vi.spyOn(transport, 'loadAudio').mockResolvedValue();
  vi.spyOn(transport, 'scheduleStep').mockImplementation(() => {});
  vi.stubGlobal('window', { setInterval: vi.fn(), clearInterval: vi.fn() });
  return { transport, context, config };
}

afterEach(() => vi.unstubAllGlobals());

describe('Tap & Dance demo transport', () => {
  it('loops eight beats at the configured tempo without a ramp', async () => {
    const { transport, context } = setup();
    await transport.start();
    for (let step = 1; step <= 16; step++) {
      context.currentTime = transport.nextStepTime;
      transport.schedulerTick();
    }
    expect(
      vi.mocked(transport.scheduleStep).mock.calls.map(([step]) => step)
    ).toEqual([...Array.from({ length: 16 }, (_, step) => step), 0]);
    expect(
      vi
        .mocked(transport.scheduleStep)
        .mock.calls.every(([, , config]) => config.bpm === 124)
    ).toBe(true);
  });

  it.each(/** @type {const} */ ([4, 8]))(
    'ramps only at %i-beat boundaries, independently of the visual loop',
    async (everyBeats) => {
      const { transport, context, config } = setup({
        startBpm: 100,
        endBpm: 130,
        duration: 20,
        everyBeats,
      });
      await transport.start();
      if (!config.speedRamp) throw new Error('Expected a speed ramp');
      config.speedRamp.endBpm = 220; // Changes during a run must not affect its ramp.
      for (let step = 1; step <= everyBeats * 2; step++) {
        context.currentTime = transport.nextStepTime;
        transport.schedulerTick();
      }
      const calls = vi.mocked(transport.scheduleStep).mock.calls;
      expect(
        calls.slice(0, -1).every(([, , config]) => config.bpm === 100)
      ).toBe(true);
      expect(calls[calls.length - 1][2].bpm).toBeCloseTo(
        100 + ((everyBeats * 0.6) / 20) * 30
      );
      expect(calls[calls.length - 1][0]).toBe((everyBeats * 2) % 16);
    }
  );

  it('completes at the exercise deadline', async () => {
    const { transport, context } = setup({
      startBpm: 100,
      endBpm: 130,
      duration: 20,
      everyBeats: 4,
      stopAfterDuration: true,
    });
    await transport.start();
    context.currentTime = transport.exerciseEndTime;
    transport.schedulerTick();
    expect(transport.running).toBe(false);
    expect(transport.onComplete).toHaveBeenCalledOnce();
  });

  it.each(['context', 'samples', 'resume'])(
    'cancels a start stopped during %s preparation',
    async (stage) => {
      const { transport, context } = setup();
      if (stage === 'context') {
        vi.mocked(transport.ensureContext).mockImplementation(async () => {
          transport.stop();
          return /** @type {AudioContext} */ (transport.context);
        });
      } else if (stage === 'samples') {
        vi.mocked(transport.loadAudio).mockImplementation(async () =>
          transport.stop()
        );
      } else {
        context.resume.mockImplementation(async () => transport.stop());
      }
      expect(await transport.start()).toBe(false);
      expect(transport.scheduleStep).not.toHaveBeenCalled();
      expect(transport.running).toBe(false);
    }
  );

  it('restarts a ramp at its starting tempo and first beat', async () => {
    const { transport, context } = setup({
      startBpm: 100,
      endBpm: 130,
      duration: 20,
      everyBeats: 4,
    });
    await transport.start();
    for (let step = 1; step <= 8; step++) {
      context.currentTime = transport.nextStepTime;
      transport.schedulerTick();
    }
    expect(transport.playbackBpm).toBeGreaterThan(100);
    transport.stop();
    vi.mocked(transport.scheduleStep).mockClear();
    await transport.start();
    expect(transport.playbackBpm).toBe(100);
    expect(transport.scheduleStep).toHaveBeenCalledExactlyOnceWith(
      0,
      context.currentTime + 0.06,
      expect.objectContaining({ bpm: 100 })
    );
  });
});
