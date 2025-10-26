<script>
  import PumpingNote from '$lib/components/ui/PumpingNote.svelte';

  let text = selectRandomText();

  function selectRandomText() {
    const texts = [
      "Every day I'm shuffling!",
      'A dance a day keeps worries at bay',
      'Feel the beat, move your feet!',
      'Step. Groove. Repeat.',
      'Keep calm and shuffle on.',
      'Dance it out!',
      'One beat at a time.',
      'Find your flow.',
      'Just groove with it.',
      "Feet don't fail me now!",
      'Small steps, big vibes.',
      'Let the floor feel you.',
      'Shuffle your worries away.',
      'Made to move.',
      'You got the moves!',
      'Born to bounce.',
      'Move like you mean it.',
      'Step up, shine on.',
      'Dancing is my cardio.',
      'Groove therapy in session.',
      'Your feet know the way.',
      'Smile. Step. Repeat.',
      'Flow state: activated.',
      'Energy flows where rhythm goes.',
      'Light feet, happy heart.',
      'Tap into the rhythm.',
      'Dance now, think later.',
      "Life's better in motion.",
    ];
    return texts[Math.floor(Math.random() * texts.length)];
  }
</script>

<PumpingNote {text} />
