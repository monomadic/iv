<p align="center">
  <img src="assets/banner.jpg" />
</p>

# IV: Lightning Fast Image Browsing

Designed in a suckless way, mostly for terminal users.

https://github.com/monomadic/iv/assets/129359/b418bcdd-55c9-4618-bf0d-69749212a5b0

There is a longer, higher quality video [on youtube](https://www.youtube.com/watch?v=fRtqc5lyWBE).

IV (pronounced ivy) is a fast minimal image gallery viewer. There seem to be many image viewer apps (even in rust) and they all seemed terrible to me and my workflow, and this is the result of that frustration.

Developed on MacOS but should run fine on other platforms. IV was designed for terminal users, but is a gui app. I'll package an .app later for cocoa monkeys.

- vim keybindings, keyboard controlled
- fast fullscreen (by default) viewing
- galleries or single image browsing
- works well with tools like `ranger` and `lf`.

## Usage

```bash
iv <glob>
```

## Installation

```bash
cargo install --git https://github.com/monomadic/iv
```
