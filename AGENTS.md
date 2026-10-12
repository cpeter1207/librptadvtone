# librptadvtone development rules

Follow the shared rpt_advanced project baseline in
`../rpt_advanced/AGENTS.md` for development, quality, packaging, and release.
This library is a portable, versioned, dynamically linked tone boundary with
no Asterisk or hardware dependency. Its audio processing path must not
allocate, lock, block, log, or perform I/O.

Use Rustdoc for Rust implementation documentation and Doxygen for the C ABI
header. Keep the ABI, Debian packages, pkg-config metadata, tests, and staged
installation synchronized. Consumers must dynamically link the released
library rather than embed its implementation.
