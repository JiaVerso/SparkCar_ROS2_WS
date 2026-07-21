#!/usr/bin/env python3
"""Console entry point wrapper for kalibr_calibrate_imu_camera."""

from pathlib import Path
import runpy


def main() -> None:
    script_path = Path(__file__).resolve().parents[1] / "kalibr_calibrate_imu_camera"
    runpy.run_path(str(script_path), run_name="__main__")


if __name__ == "__main__":
    main()
