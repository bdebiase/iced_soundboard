# Iced Soundboard
I wanted to use a soundboard with some extra fun features like changing the speed and applying audio effects and couldn't find one, so I made one! Made completely in Rust using [Iced](https://github.com/iced-rs/iced) for the GUI and [Kira](https://github.com/tesselode/kira) for the audio backend. Some features will take a while or may never be implemented because of how difficult and immature the Rust ecosystem currently is.

> [!TIP]
> On Linux, it is possible to route the audio if you know how to use audio patchbay software.
> * PipeWire: Use [Helvum](https://gitlab.freedesktop.org/pipewire/helvum) or [qpwgraph](https://github.com/rncbc/qpwgraph), made specifically for audio routing.
> * JACK: Control with [qjackctl](https://github.com/rncbc/qjackctl), a fully-featured GUI application for JACK.

> On Windows, audio routing can be achieved with tools like [VB-Audio VoiceMeeter](https://vb-audio.com/Voicemeeter/) or [JACK Audio Connection Kit for Windows](https://jackaudio.org/), though this was not tested.

![image](https://github.com/bdebiase/iced_soundboard/assets/66143154/2c120bd1-d5ed-4b27-ac8b-d17eb6a3ca56)

## Features
- [x] List of active playbacks
- [x] Audio seeking
- [x] Variable speed & volume
- [x] Tabs
- [x] Saving
- [ ] Async file loading
- [ ] Theme support
- [ ] Realtime audio effects
- [ ] Per-audio settings (volume & speed)
- [ ] YouTube-DL
- [ ] Audio routing to other applications like Discord (done with VB-Cable on Windows or Jack/PipeWire on Linux)
