#!/usr/bin/env python3
"""The Speech, sound & haptics page's looping clip: an A major chord with a slow tremolo,
synthesized here with no recordings or external samples.
Rebuild: python3 scripts/sound-hum.py

A loop repeats a clip from its last sample straight to its first, so the clip has to arrive back
where it started. Every partial is a whole number of hertz and the tremolo is 2 Hz, so one second
holds whole cycles of all of them, and there is no fade at either end.
"""
from pathlib import Path
import math, struct, wave

RATE = 44100
OUT = Path(__file__).resolve().parents[1] / 'resource/assets/sounds/hum.wav'
# frequency (Hz), amplitude: A3, C#4, E4, and A4 above them.
PARTIALS = [(220, 0.50), (277, 0.28), (330, 0.30), (440, 0.14)]
TREMOLO_HZ, TREMOLO_DEPTH = 2, 0.25
PEAK = 0.35  # about -9 dBFS: it sits under the page's other clips

samples = []
for i in range(RATE):
    t = i / RATE
    tone = sum(a * math.sin(2 * math.pi * f * t) for f, a in PARTIALS)
    swell = 1 - TREMOLO_DEPTH * (1 + math.cos(2 * math.pi * TREMOLO_HZ * t)) / 2
    samples.append(tone * swell)
gain = PEAK / max(abs(s) for s in samples)
with wave.open(str(OUT), 'wb') as w:
    w.setparams((1, 2, RATE, 0, 'NONE', 'not compressed'))
    w.writeframes(b''.join(struct.pack('<h', round(s * gain * 32767)) for s in samples))
print('wrote', OUT)
