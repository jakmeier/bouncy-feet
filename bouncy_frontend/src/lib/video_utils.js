
/** 
 * @param {Beat[]} beats
 * @return {{t: number, text: string}[]} 
 * 
*/
export function beatToMarkers(beats) {
    return beats?.flatMap((beat) => {
        const end = beat.start + beat.duration;
        const interval = (60000 / beat.bpm / 2) * beat.subbeat_per_move;
        const beatMarkers = [];
        if (beat.bpm && beat.start && beat.start > 0 && interval > 0) {
            var i = 0;
            for (var t = beat.start; t < end; t += interval, i++) {
                const spm = beat.subbeat_per_move || 1;
                const markersPerCount = spm === 1 ? 2 : 1;
                const text =
                    (i + 1) % markersPerCount === 0
                        ? `${(i + 1) / markersPerCount}`
                        : '+';
                beatMarkers.push({ t, text });
            }
        }
        return beatMarkers;
    });
}