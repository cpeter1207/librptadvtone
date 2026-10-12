"""Check that package identities and the public ABI advance together."""

import re
import tomllib
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


class PackagingContracts(unittest.TestCase):
    def test_initial_tone_abi_and_packages(self):
        self.assertTrue((ROOT / "Cargo.toml").exists(), "tone package manifest is missing")
        manifest = tomllib.loads((ROOT / "Cargo.toml").read_text())
        self.assertEqual(manifest["package"]["version"], "0.1.0-alpha.1")
        self.assertEqual(manifest["lib"]["crate-type"], ["cdylib"])
        makefile = (ROOT / "Makefile").read_text()
        self.assertIn("SOVERSION := 1", makefile)
        header = (ROOT / "include/rptadvtone/rptadvtone.h").read_text()
        self.assertRegex(header, r"#define RPTADV_TONE_ABI_VERSION 1U\b")
        self.assertIn("rptadv_tone_descriptor(void)", header)
        control = (ROOT / "debian/control").read_text()
        self.assertEqual(re.findall(r"^Package: (.+)$", control, re.MULTILINE),
                         ["librptadvtone1", "librptadvtone-dev"])
        self.assertIn("librptadvtone1 (= ${binary:Version})", control)
        self.assertTrue((ROOT / "debian/changelog").read_text().startswith(
            "librptadvtone (0.1.0~alpha1-1)"))


if __name__ == "__main__":
    unittest.main()
