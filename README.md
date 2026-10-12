# librptadvtone

Portable 48 kHz tone generation shared library for rpt_advanced.

Implementation source: `rpt_advanced` commit
`c63db090e90de2c97444ccb2e4df7f53328701f8`,
`rust/core/src/audio/tone.rs` and `tone_tests.rs`.
The `AudioSource` trait integration remains in the controller.

The initial implementation is developed on `extract/initial-library` after
the bootstrap and required quality gate are established.

The library installs `librptadvtone.so.1`, the `rptadvtone/rptadvtone.h` C
header, and `rptadvtone.pc`. Consumers resolve `rptadv_tone_descriptor()`
during setup and verify `struct_size`, ABI version 1, and `rptadv.tone` before
using its operations. Sequence text is a borrowed UTF-8 byte span; the
library parses and owns its bounded sequence after `sequence_create` returns.
Owned sequence and oscillator handles must be destroyed after callback users
stop. A nonzero output length requires a valid writable `float` buffer.

`sequence_create` returns `RPTADV_TONE_INVALID_SEQUENCE` for the original
grammar, duration, level, segment, or total-sample rejection. Bad pointers,
invalid UTF-8, and invalid oscillator setup values return
`RPTADV_TONE_INVALID_ARGUMENT`. A successful render reports the number of
samples written; exhaustion reports zero without modifying the remainder.
The oscillator renders exactly the requested count with the existing Morse
`amplitude * sin(TAU * phase)` then `(phase + step).fract()` progression.

Source-move ledger: `src/tone.rs` is the original parser and renderer minus
its controller `AudioSource` trait implementation. `src/tone_tests.rs` carries
the original implementation tests minus the trait-only test, which remains
in the controller. `src/oscillator.rs` contains only the existing Morse
waveform primitive. `src/lib.rs` translates buffers, errors, and lifecycles
at the shared-object boundary; it does not parse or render tone grammar.
