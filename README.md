# librptadvtone

Portable 48 kHz tone generation shared library for rpt_advanced.

Implementation source: `rpt_advanced` commit
`c63db090e90de2c97444ccb2e4df7f53328701f8`,
`rust/core/src/audio/tone.rs` and `tone_tests.rs`.
The `AudioSource` trait integration remains in the controller.

The initial implementation is developed on `extract/initial-library` after
the bootstrap and required quality gate are established.
