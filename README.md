# SoundCraft Studio

An open-source, sovereign digital audio workstation (DAW) and audio mixing console built in pure Rust, powered by the **Martensite** GPU-accelerated retained-mode GUI engine.

![SoundCraft Studio on Martensite](brag/demo.gif)

## Architecture

- **`crates/ui-martensite`**: Sovereign retained-mode mixing console interface with dB volume faders, stereo pan pots, solo/mute toggles, and peak meters.
- **`crates/engine`**: Low-latency multi-track audio mixing engine, DSP effects processor, and non-destructive audio editing pipeline.

## Legal & Compliance Notice

SoundCraft is an independent open-source audio editor. It is not affiliated with Avid Technology Inc. Avid, Pro Tools, and AAX are trademarks of Avid Technology Inc. Physical analog mixing console ergonomics (faders, pan pots, mute/solo switches) are standard utilitarian controls (17 U.S.C. § 102(b)).

## License

Dual-licensed under MIT OR Apache-2.0.
