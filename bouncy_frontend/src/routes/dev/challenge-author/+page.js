/** @param {string | null} value */
function timestamps(value) {
  return (value ?? '')
    .split(',')
    .map(Number)
    .filter(Number.isFinite);
}

export function load({ url }) {
  const startTimestamp = Number(url.searchParams.get('startTimestamp'));
  return {
    videoUrl: url.searchParams.get('videoUrl') ?? undefined,
    beats: timestamps(url.searchParams.get('beats')),
    startTimestamp: Number.isFinite(startTimestamp) ? startTimestamp : undefined,
  };
}
