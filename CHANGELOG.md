# Changelog

## [Unreleased]

## [0.6.2] - 2026-10-16

- Start page entrance animations when the interface is ready to display.

## [0.6.1] - 2026-10-16

- Ensure the audio probe plays its confirmation cue.

## [0.6.0] - 2026-10-16

- Add 8-bit menu sounds for navigation, room discovery, readiness and connection results.
- Reduce animated SVG size and browser rendering overhead.
- Add animated SVG and H.264 MP4 export commands for the dual-handheld demo.

## [0.5.1] - 2026-10-16

- Add side markers and bottom shadows to room and game rows, with sparse lists aligned at the top.
- Keep selection stable while room lists refresh and refine button and character feedback.
- Improve connection panels and show join hints for available rooms.
- Correct network-only room guidance and keep hint text clear of panel shadows.

## [0.5.0] - 2026-10-16

- Add Forest, Sunset and Night themes with saved Y-button selection.
- Add a campsite emblem, mountain scenery, side ornaments and raised buttons.
- Use crisp pixel lettering throughout the interface.
- Animate page entry, focused buttons, character movement and room readiness.
- Improve menu rendering and controller response with GPU acceleration.
- Keep preparing-room buttons readable and jumping characters inside their panels.
- Align device names and addresses in network-only rooms.

## [0.4.0] - 2026-10-11

- Add Network Only and Choose Game entries in Options.
- Keep the nearby network connected while returning to the system and opening LAN games from Ports.
- Show connected device addresses and restore Wi-Fi when leaving the network.
- Clear peer addresses after disconnection.

## [0.3.2] - 2026-10-10

- Keep long game names and connection messages within their panels.
- Align the selection border with the selected item during movement and resizing.

## [0.3.1] - 2026-10-10

- Use gpSP automatic cartridge protocol detection for GBA Netpacket sessions.

- Select FBNeo for compatible arcade games through isolated ROM loading and rollback-state checks during room preparation.

## [0.3.0] - 2026-10-10

- Faster full-state compression for the verified ARM64 RetroArch build.
- Bundled Nestopia core with complete input-state restoration and zeroed state padding.
- FBNeo selection from MAME for verified Final Fight content, restricted to matching 64-bit source and target builds.
- Super Bomberman 5 retains its supported local core.
- Explicit controller ports and a 600-frame state verification interval for multiplayer sessions.

## [0.2.0] - 2026-10-09

- Native Rust installer and nearby room runtime for the checked ArkOS4Clone / RTL8188EU profile.
- Graphical local game selection from Options and current-game entry before launch with X.
- Selected-room discovery, manual refresh, owner-confirmed first pairing and remembered device identities.
- Session preparation, matching local frontend/core/content identities and explicit host start.
- Session-owned game handoff and restoration of the original wireless driver and network.
- Chinese mirui interface with cinnabar, ivory and ink colors, square corners and bounded dirty-region rendering.
- GB/GBC and selected GBA link configuration using installed cores, with game-specific acceptance limits.
- Complete Chinese title bounds and separate subtitle spacing.

The release supports two-device rooms and temporary session saves. Arcade performance and handheld link acceptance remain game-specific; see [compatibility](docs/compatibility.md).
