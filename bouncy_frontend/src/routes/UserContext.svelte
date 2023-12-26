<script>
  /**
   * Provides access to a user local storage
   */
  import { browser } from '$app/environment';
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
  const user = writable(
    stored || {
      id: crypto.randomUUID(),
      publicName: generateRandomUsername(),
      score: 0,
      recordedDances: 0,
      recordedSeconds: 0,
      recordedSteps: 0,
    }
  );
  if (browser) {
    user.subscribe((value) => (localStorage.user = JSON.stringify(value)));
  }

  /**
   * @param {import("$lib/instructor/bouncy_instructor").DetectedStep[]} dance
   */
  function addDanceToStats(dance) {
    if (dance.length < 2) {
      return;
    }

    let steps = 0;
    let prevPoseName = '';
    for (const step of dance) {
      if (step.name !== prevPoseName) {
        steps += 1;
        prevPoseName = step.name;
      }
    }

    const duration = dance[dance.length - 1].end - dance[0].start;
    $user.recordedDances += 1;
    $user.recordedSteps += steps;
    $user.recordedSeconds += duration;

    return {
      numSteps: steps,
      duration,
    };
  }

  setContext('user', {
    store: user,
    addDanceToStats,
  });
</script>

<slot />
