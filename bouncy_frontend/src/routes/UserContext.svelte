<script>
  /**
   * Provides access to a user local storage
   */
  import { browser } from '$app/environment';
  import { submitStats } from '$lib/stats';
  import { generateRandomUsername } from '$lib/username';
  import { setContext } from 'svelte';
  import { writable } from 'svelte/store';

  function fromLocalStorage() {
    try {
      return JSON.parse(localStorage.user);
    } catch (e) {
      return null;
    }
  }

  const stored = browser ? fromLocalStorage() : null;
  if (stored && !stored.id) {
    stored.id = crypto.randomUUID();
  }
  if (stored && !stored.userSteps) {
    stored.userSteps = {};
  }
  const user = writable(
    stored || {
      id: crypto.randomUUID(),
      publicName: generateRandomUsername(),
      score: 0,
      recordedDances: 0,
      recordedSeconds: 0,
      recordedSteps: 0,
      userSteps: {},
    }
  );
  if (browser) {
    user.subscribe((value) => (localStorage.user = JSON.stringify(value)));
  }

  /**
   * @param {import("$lib/instructor/bouncy_instructor").DetectedStep[]} dance
   * @returns {DanceSessionResult?}
   */
  function addDanceToStats(dance) {
    if (dance.length < 1) {
      return null;
    }

    let stats = {};
    let totalSteps = 0;
    let totalExp = 0;
    const bpms = [];
    for (const step of dance) {
      if (!step.name.includes('Idle')) {
        totalSteps += 1;
        if (!stats[step.name]) {
          stats[step.name] = {
            slow: 0,
            mid: 0,
            fast: 0,
            veryFast: 0,
          };
        }
        const halfBeat = (step.end - step.start) / (step.poses.length - 1);
        bpms.push(60000 / halfBeat);
        if (halfBeat < 231) {
          stats[step.name].veryFast += 1;
        } else if (halfBeat < 300) {
          stats[step.name].fast += 1;
        } else if (halfBeat < 500) {
          stats[step.name].mid += 1;
        } else {
          stats[step.name].slow += 1;
        }
      }
    }

    const duration = dance[dance.length - 1].end - dance[0].start;
    $user.recordedDances += 1;
    $user.recordedSteps += totalSteps;
    $user.recordedSeconds += duration;
    for (let [key, stat] of Object.entries(stats)) {
      if (!$user.userSteps[key]) {
        $user.userSteps[key] = {
          experience: 0,
          count: 0,
        };
      }
      const numSteps = stat.slow + stat.mid + stat.fast + stat.veryFast;
      $user.userSteps[key].count = $user.userSteps[key].count + numSteps;
      const stepGainedExp =
        stat.slow * 10 + stat.mid * 15 + stat.fast * 20 + stat.veryFast * 25;
      $user.userSteps[key].experience =
        $user.userSteps[key].count + stepGainedExp;
      totalExp += stepGainedExp;
    }

    // trigger subscribers
    $user = $user;

    try {
      submitStats($user);
    } catch (err) {
      console.warn('Submitting stats failed', err);
    }

    return {
      numSteps: totalSteps,
      experience: totalExp,
      duration,
      stats,
      bpms,
    };
  }

  setContext('user', {
    store: user,
    addDanceToStats,
  });
</script>

<slot />
