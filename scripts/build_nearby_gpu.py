#!/usr/bin/env python3
"""Build the SDL2/GLES interface against an existing ARM64 GNU sysroot."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tarfile
import tomllib

import build_nearby_gui as gui
import build_paths

ROOT = Path(__file__).resolve().parents[1]
TRIPLE = "aarch64-unknown-linux-gnu"
PATCH = ROOT / "gui/nearby/patches/mirui-0.49.0-gles.patch"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def prepare_source():
    packages = tomllib.loads(gui.MANIFEST.with_name("Cargo.lock").read_text())[
        "package"
    ]
    package = next(p for p in packages if p["name"] == "mirui")
    name = "mirui-" + package["version"]
    cargo_home = Path(os.environ.get("CARGO_HOME", Path.home() / ".cargo"))
    matches = list((cargo_home / "registry/cache").glob("*/" + name + ".crate"))
    archive = next((p for p in matches if sha(p) == package["checksum"]), None)
    if archive is None:
        raise ValueError("Prepare the locked mirui crate before the offline GPU build")
    output = build_paths.BUILD_ROOT / "gpu/sources" / sha(PATCH)
    source = output / name
    if not (output / "ready").exists():
        if output.exists():
            shutil.rmtree(output)
        output.mkdir(parents=True)
        with tarfile.open(archive) as tar:
            tar.extractall(output, filter="data")
        subprocess.run(["git", "apply", str(PATCH)], cwd=source, check=True)
        (output / "ready").write_text(package["checksum"] + "\n")
    return source


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("action", choices=("build", "check"))
    parser.add_argument("--sysroot", type=Path, required=True)
    parser.add_argument("--sdl-library", type=Path, required=True)
    parser.add_argument("--linker", type=Path, required=True)
    args = parser.parse_args()
    sysroot = args.sysroot.resolve()
    library = args.sdl_library.resolve()
    data = library.read_bytes()
    if data[:6] != b"\x7fELF\x02\x01" or int.from_bytes(data[18:20], "little") != 183:
        raise ValueError("SDL2 must be a little-endian ARM64 ELF library")
    if not (library.parent / "libSDL2.so").is_file():
        raise ValueError("The SDL2 library directory needs its libSDL2.so linker name")
    if not (sysroot / "usr/lib/aarch64-linux-gnu/Scrt1.o").is_file():
        raise ValueError("The GNU sysroot needs ARM64 startup objects")
    if not args.linker.is_file():
        raise ValueError("Pass the existing ARM64 GNU C linker")
    inputs = gui.build_inputs()
    patch_sha = sha(PATCH)
    source = prepare_source()
    staging = build_paths.BUILD_ROOT / "gpu/application"
    staging.mkdir(parents=True, exist_ok=True)
    shutil.copytree(gui.MANIFEST.parent / "src", staging / "src", dirs_exist_ok=True)
    shutil.copytree(
        gui.MANIFEST.parent / "assets", staging / "assets", dirs_exist_ok=True
    )
    shutil.copy2(gui.MANIFEST, staging / "Cargo.toml")
    lock = gui.MANIFEST.with_name("Cargo.lock").read_text()
    blocks = lock.split("[[package]]")
    for index, block in enumerate(blocks):
        if block.startswith('\nname = "mirui"\n'):
            blocks[index] = "\n".join(
                line
                for line in block.split("\n")
                if not line.startswith(("source =", "checksum ="))
            )
    (staging / "Cargo.lock").write_text("[[package]]".join(blocks))
    cargo, environment = gui.toolchain("1.92")
    target = build_paths.target("gpu-interface")
    environment.update(
        CARGO_TARGET_DIR=str(target),
        CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=str(args.linker.resolve()),
        RUSTFLAGS="-L native=" + str(library.parent),
    )
    common = [
        "--manifest-path",
        str(staging / "Cargo.toml"),
        "--offline",
        "--locked",
        "--target",
        TRIPLE,
        "--features",
        "gpu",
        "--config",
        'patch.crates-io.mirui.path="' + str(source) + '"',
    ]
    if args.action == "check":
        gui.run(
            [str(cargo), "clippy", *common, "--all-targets", "--", "-D", "warnings"],
            environment,
        )
        return
    gui.run([str(cargo), "build", *common, "--release"], environment)
    if inputs != gui.build_inputs() or patch_sha != sha(PATCH):
        raise ValueError("GPU sources changed during compilation")
    binary = target / TRIPLE / "release/arkos-nearby-gui"
    output = ROOT / "dist/gpu"
    output.mkdir(parents=True, exist_ok=True)
    shutil.copy2(binary, output / "arkos-nearby-gui")
    (output / "lib").mkdir(exist_ok=True)
    shutil.copy2(library, output / "lib/libSDL2-2.0.so.0")
    shutil.copytree(ROOT / "docs/licenses", output / "licenses", dirs_exist_ok=True)
    report = {
        "schema": 2,
        "target": TRIPLE,
        "backend": "sdl-gpu",
        **gui.locked_dependencies(),
        **inputs,
        "mirui_patch_sha256": patch_sha,
        "sha256": sha(binary),
        "bytes": binary.stat().st_size,
        "libraries": {"libSDL2-2.0.so.0": sha(library)},
    }
    (output / "nearby-gui-build.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report))


if __name__ == "__main__":
    main()
