#!/usr/bin/env python3
"""Render the mirui dual-handheld storyboard and encode its README WebP."""

import argparse
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
sys.path.insert(0, str(ROOT / "scripts"))
import build_nearby_gui as gui  # noqa: E402


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument(
        "--frames", type=Path, default=Path("/tmp/arkos-nearby-demo-frames")
    )
    parser.add_argument("--toolchain", default="1.92")
    parser.add_argument("--quality", type=int, default=75)
    parser.add_argument("--width", type=int, default=840)
    parser.add_argument("--lossless", action="store_true")
    parser.add_argument("--reuse-frames", action="store_true")
    args = parser.parse_args()
    if args.output.suffix.lower() != ".webp":
        parser.error("--output must use the .webp extension")
    if not args.reuse_frames:
        cargo, env = gui.toolchain(args.toolchain)
        env["CARGO_INCREMENTAL"] = "0"
        subprocess.run(
            [
                str(cargo),
                "build",
                "--manifest-path",
                str(gui.MANIFEST),
                "--release",
                "--example",
                "dual_demo",
                "--locked",
                "--offline",
            ],
            env=env,
            check=True,
            cwd=ROOT,
        )
        binary = Path(env["CARGO_TARGET_DIR"]) / "release/examples/dual_demo"
        subprocess.run([str(binary), str(args.frames.resolve())], check=True)
    animation = json.loads((args.frames / "animation.json").read_text())
    scaled = args.frames / "scaled"
    scaled.mkdir(exist_ok=True)
    height = round(animation["height"] * args.width / animation["width"])
    subprocess.run(
        [
            "ffmpeg",
            "-hide_banner",
            "-loglevel",
            "error",
            "-y",
            "-framerate",
            str(animation["fps"]),
            "-i",
            str(args.frames / "%04d.png"),
            "-vf",
            f"scale={args.width}:{height}:flags=lanczos",
            "-frames:v",
            str(animation["frames"]),
            "-start_number",
            "0",
            str(scaled / "%04d.png"),
        ],
        check=True,
    )
    args.output.parent.mkdir(parents=True, exist_ok=True)
    encoded = args.output.with_suffix(".part.webp").resolve()
    subprocess.run(
        [
            "img2webp",
            "-min_size",
            "-sharp_yuv",
            "-loop",
            "0",
            "-d",
            str(round(1000 / animation["fps"])),
            "-lossless" if args.lossless else "-lossy",
            "-q",
            str(args.quality),
            "-m",
            "4",
            *(
                str(scaled.resolve() / f"{index:04}.png")
                for index in range(animation["frames"])
            ),
            "-o",
            str(encoded),
        ],
        check=True,
    )
    encoded.replace(args.output.resolve())
    print(args.output.resolve())


if __name__ == "__main__":
    main()
