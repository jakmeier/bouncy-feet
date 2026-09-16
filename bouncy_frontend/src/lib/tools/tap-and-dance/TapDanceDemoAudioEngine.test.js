import { afterEach, describe, expect, it, vi } from 'vitest';
import { TapDanceDemoAudioEngine } from './TapDanceDemoAudioEngine.js';

function setup() {
  const config = {
    bpm: 124,
    enabled: { kick: true, clap: true, hat: true },
    speedRamp: null,
  };
  const engine = new TapDanceDemoAudioEngine(
    { kick: '/kick.mp3', clap: '/clap.mp3', hat: '/hat.mp3' },
    () => config,
    vi.fn()
  );
  const source = {
    buffer: null,
    connect: vi.fn(),
    disconnect: vi.fn(),
    start: vi.fn(),
    stop: vi.fn(),
    onended: () => {},
  };
  const buffer = /** @type {AudioBuffer} */ (
    /** @type {unknown} */ ({ duration: 1 })
  );
  const context = {
    currentTime: 0,
    state: 'running',
    resume: vi.fn(),
    destination: {},
    createBufferSource: vi.fn(() => source),
    decodeAudioData: vi.fn(async () => buffer),
  };
  engine.context = /** @type {AudioContext} */ (
    /** @type {unknown} */ (context)
  );
  return { engine, config, source, context, buffer };
}

afterEach(() => vi.unstubAllGlobals());

describe('Tap & Dance demo audio', () => {
  it('plays the house pattern, with only on/off per instrument', () => {
    const { engine, config } = setup();
    vi.spyOn(engine, 'setTimeout').mockImplementation(() => {});
    const play = vi.spyOn(engine, 'playSample').mockImplementation(() => {});
    for (let step = 0; step < 16; step++)
      engine.scheduleStep(step, step, config);
    expect(
      play.mock.calls.filter(([id]) => id === 'kick').map(([, time]) => time)
    ).toEqual([0, 2, 4, 6, 8, 10, 12, 14]);
    expect(
      play.mock.calls.filter(([id]) => id === 'clap').map(([, time]) => time)
    ).toEqual([2, 6, 10, 14]);
    expect(
      play.mock.calls.filter(([id]) => id === 'hat').map(([, time]) => time)
    ).toEqual([1, 3, 5, 7, 9, 11, 13, 15]);

    play.mockClear();
    config.enabled = { kick: false, clap: false, hat: false };
    for (let step = 0; step < 16; step++)
      engine.scheduleStep(step, step, config);
    expect(play).not.toHaveBeenCalled();
    config.enabled.clap = true;
    engine.scheduleStep(2, 20, config);
    expect(play.mock.calls).toEqual([['clap', 20]]);
  });

  it('caches loaded samples', async () => {
    const { engine, config, context, buffer } = setup();
    const fetch = vi.fn(async () => ({
      ok: true,
      arrayBuffer: async () => new ArrayBuffer(0),
    }));
    vi.stubGlobal('fetch', fetch);
    await engine.loadAudio(
      config,
      /** @type {AudioContext} */ (/** @type {unknown} */ (context))
    );
    await engine.loadAudio(
      config,
      /** @type {AudioContext} */ (/** @type {unknown} */ (context))
    );
    expect(fetch.mock.calls).toHaveLength(3);
    expect(context.decodeAudioData).toHaveBeenCalledTimes(3);
    expect(engine.buffers).toEqual({ kick: buffer, clap: buffer, hat: buffer });
  });

  it('allows retrying failed sample loads', async () => {
    const { engine, config } = setup();
    const fetch = vi.fn().mockResolvedValue({ ok: false });
    vi.stubGlobal('fetch', fetch);
    const context = /** @type {AudioContext} */ (engine.context);
    await expect(engine.loadAudio(config, context)).rejects.toThrow(
      'Could not load sample'
    );
    fetch.mockResolvedValue({
      ok: true,
      arrayBuffer: async () => new ArrayBuffer(0),
    });
    await expect(engine.loadAudio(config, context)).resolves.toBeUndefined();
    expect(fetch).toHaveBeenCalledTimes(6);
  });

  it('cuts off sample tails at the exercise deadline and stops active sources', () => {
    const { engine, source, buffer } = setup();
    engine.buffers.hat = buffer;
    engine.exerciseEndTime = 20;
    engine.playSample('hat', 19.9);
    expect(source.stop).toHaveBeenCalledWith(20);
    engine.stop();
    expect(source.stop).toHaveBeenLastCalledWith();
    expect(engine.activeSources.size).toBe(0);
  });

  it('keeps the visual clock running while sounds are muted', () => {
    const { engine, config } = setup();
    config.enabled = { kick: false, clap: false, hat: false };
    engine.running = true;
    vi.spyOn(engine, 'setTimeout').mockImplementation((callback) => callback());
    engine.scheduleStep(7, 0, config);
    expect(engine.onStep).toHaveBeenCalledWith(7, 124);
  });
});
