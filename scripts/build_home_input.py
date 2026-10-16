#!/usr/bin/env python3
"""Build the frontend-scoped SDL input adapter from local toolchain assets."""

import argparse
import hashlib
import json
from pathlib import Path
import struct
import subprocess

from build_paths import BUILD_ROOT

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "native/home_input"
ASSETS = ROOT / "build/assets/assets"
NAME = "home-input.so"
REPORT = "home-input-build.json"


def source_sha256():
    digest = hashlib.sha256()
    for name in ("chord.h", "filter.c"):
        digest.update(name.encode() + b"\0" + (SOURCE / name).read_bytes())
    return digest.hexdigest()


def artifacts(assets=ASSETS):
    data = (assets / NAME).read_bytes()
    report = json.loads((assets / REPORT).read_text())
    if (
        data[:6] != b"\x7fELF\x02\x01"
        or struct.unpack_from("<H", data, 18)[0] != 183
        or report["sha256"] != hashlib.sha256(data).hexdigest()
        or report["source_sha256"] != source_sha256()
        or report["bytes"] != len(data)
    ):
        raise ValueError("Home input adapter differs from its build record")
    return {"assets/" + NAME: data, "assets/" + REPORT: (assets / REPORT).read_bytes()}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("action", choices=("build", "check"))
    parser.add_argument("--compiler")
    parser.add_argument("--sdl-include", type=Path)
    parser.add_argument("--assets", type=Path, default=ASSETS)
    args = parser.parse_args()
    if args.action == "check":
        print(json.dumps({"files": len(artifacts(args.assets))}))
        return
    if not args.compiler or not args.sdl_include:
        parser.error("Provide the ARM64 GNU compiler and SDL2 header directory")
    output = BUILD_ROOT / "home-input" / NAME
    output.parent.mkdir(parents=True, exist_ok=True)
    subprocess.run(
        [
            args.compiler,
            "-std=gnu11",
            "-O2",
            "-Wall",
            "-Wextra",
            "-Werror",
            "-shared",
            "-fPIC",
            "-fvisibility=hidden",
            "-Wl,-z,defs",
            "-Wl,-z,noexecstack",
            "-I" + str(args.sdl_include),
            str(SOURCE / "filter.c"),
            "-ldl",
            "-o",
            str(output),
        ],
        check=True,
    )
    data = output.read_bytes()
    report = {
        "schema": 1,
        "target": "aarch64-linux-gnu",
        "bytes": len(data),
        "sha256": hashlib.sha256(data).hexdigest(),
        "source_sha256": source_sha256(),
    }
    args.assets.mkdir(parents=True, exist_ok=True)
    (args.assets / NAME).write_bytes(data)
    (args.assets / REPORT).write_text(json.dumps(report, indent=2) + "\n")
    artifacts(args.assets)
    print(json.dumps(report))


if __name__ == "__main__":
    main()
