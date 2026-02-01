# Examples

## Creating an Atlanta Hip Hop Beat

```
You: Create an Atlanta hip hop version of Twinkle Twinkle Little Star

Claude: I'll create a trap beat at 140 BPM with the melody...

[Creates 4 tracks:]
  - ATL Twinkle (Drift synth melody with trap rhythms)
  - 808 Bass (deep sub bass)
  - Trap Hats (rolling triplet hi-hats)
  - Snare/Clap (beats 2 & 4)

[Fires all clips, starts playback]
```

This creates:
- **Tempo**: 140 BPM (classic Atlanta trap tempo)
- **Melody**: Twinkle Twinkle with syncopated trap-style rhythms
- **808 Bass**: Deep sub following chord roots (C, G, F, D)
- **Hi-hats**: Rolling triplet patterns (signature trap sound)
- **Snare**: Hitting on beats 2 and 4

## Synthesizers vs Samplers

When creating MIDI tracks that produce sound, it's important to understand the difference between Ableton's instrument types:

| Instrument Type | Examples | Needs Samples? | Sound Immediately? |
|-----------------|----------|----------------|-------------------|
| **Synthesizers** | Drift, Analog, Wavetable, Operator | No | Yes |
| **Samplers** | Simpler, Sampler, Drum Rack | Yes | No (empty) |

### Why This Matters

**Synthesizers** generate sound mathematically using oscillators, filters, and envelopes. They work immediately without any additional setup.

**Samplers** play back audio recordings (samples). When empty, they're silent - you need to drag a sample onto them first.

### Recommendation

Use `load_default_instrument` which automatically loads **Drift** (a synthesizer):

```
# This loads Drift (synth) - works immediately
load_default_instrument

# vs. loading Simpler (sampler) - silent until you add samples
load_instrument "Simpler"
```

## Session View vs Arrangement View

Ableton has two main views (press **Tab** to toggle):

| Session View | Arrangement View |
|--------------|------------------|
| Clip grid - launch clips on the fly | Linear timeline - traditional DAW |
| Great for live performance, jamming | Great for recording, mixing, finalizing |
| Clips loop independently | Everything plays left-to-right |

The MCP server creates clips in **Session View** by default. Press Tab to see them.

## More Examples

### Simple Melody

```
You: Create a C major scale

Claude: [Creates MIDI track, loads Drift, adds notes C-D-E-F-G-A-B-C]
```

### Chord Progression

```
You: Create a I-V-vi-IV progression in C major

Claude: [Creates track with C, G, Am, F chords over 4 bars]
```

### Drum Pattern

```
You: Create a basic house beat at 120 BPM

Claude: [Creates kick on 1-2-3-4, snare on 2-4, hi-hats on 8ths]
```
