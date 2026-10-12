import unittest
import subprocess
import json
import tempfile
from pathlib import Path


class Functional(unittest.TestCase):
    def test_fixtures(self):
        for binary in ["target/debug/rust_fixture", "target/cpp_fixture"]:
            a = json.loads(subprocess.check_output([binary, "4"], timeout=3))
            b = json.loads(subprocess.check_output([binary, "12"], timeout=3))
            self.assertEqual(a["checksum"], b["checksum"])
            self.assertGreater(b["allocated_bytes"], a["allocated_bytes"])
            self.assertGreater(b["peak_live_bytes"], a["peak_live_bytes"])
            self.assertNotEqual(
                subprocess.run([binary, "65"], capture_output=True).returncode, 0
            )

    def test_failed_program(self):
        config = {
            "baseline": {"executable": "/bin/false", "args": []},
            "candidate": {"executable": "/bin/false", "args": []},
            "repeats": 3,
            "warmups": 0,
            "timeout_ms": 50,
        }
        with tempfile.TemporaryDirectory() as d:
            p = Path(d)
            (p / "config").write_text(json.dumps(config))
            r = subprocess.run(
                [
                    "target/debug/memory-regression-lab",
                    str(p / "config"),
                    str(p / "out"),
                    str(p / "md"),
                ],
                capture_output=True,
                timeout=3,
            )
            self.assertEqual(r.returncode, 2)
            self.assertFalse((p / "out").exists())

    def test_existing_reports_preserved(self):
        for existing in ["out", "md"]:
            with tempfile.TemporaryDirectory() as directory:
                p = Path(directory)
                (p / existing).write_text("retained evidence")
                result = subprocess.run(
                    [
                        "target/debug/memory-regression-lab",
                        "examples/experiment.json",
                        str(p / "out"),
                        str(p / "md"),
                    ],
                    capture_output=True,
                    timeout=3,
                )
                self.assertEqual(result.returncode, 2)
                self.assertIn(b"fresh output paths", result.stderr)
                self.assertEqual((p / existing).read_text(), "retained evidence")
                self.assertFalse((p / ("md" if existing == "out" else "out")).exists())

        with tempfile.TemporaryDirectory() as directory:
            result = subprocess.run(
                [
                    "target/debug/memory-regression-lab",
                    "missing-config",
                    directory + "/out",
                    directory + "/./out",
                ],
                capture_output=True,
                timeout=3,
            )
            self.assertEqual(result.returncode, 2)
            self.assertIn(b"fresh output paths", result.stderr)
            self.assertFalse((Path(directory) / "out").exists())

    def test_timeout(self):
        c = {
            "baseline": {"executable": "/bin/sleep", "args": ["1"]},
            "candidate": {"executable": "/bin/sleep", "args": ["1"]},
            "repeats": 3,
            "warmups": 0,
            "timeout_ms": 20,
        }
        with tempfile.TemporaryDirectory() as d:
            p = Path(d)
            (p / "config").write_text(json.dumps(c))
            r = subprocess.run(
                [
                    "target/debug/memory-regression-lab",
                    str(p / "config"),
                    str(p / "out"),
                    str(p / "md"),
                ],
                capture_output=True,
                timeout=3,
            )
            self.assertEqual(r.returncode, 2)
            self.assertIn(b"timed out", r.stderr)


if __name__ == "__main__":
    unittest.main()
