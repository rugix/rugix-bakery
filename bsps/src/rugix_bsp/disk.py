"""Disk size parsing utilities."""

from __future__ import annotations

import re

_UNITS: dict[str, int] = {
    "B": 1,
    "K": 1024,
    "KIB": 1024,
    "KB": 1000,
    "M": 1024**2,
    "MIB": 1024**2,
    "MB": 1000**2,
    "G": 1024**3,
    "GIB": 1024**3,
    "GB": 1000**3,
}

_SIZE_RE = re.compile(r"^\s*(\d+)\s*([A-Za-z]*)\s*$")


def parse_size(value: str) -> int:
    """Parse a human-readable size string into bytes."""
    m = _SIZE_RE.match(value)
    if not m:
        raise ValueError(f"invalid size: {value!r}")
    number = int(m.group(1))
    unit = m.group(2).upper() or "B"
    if unit not in _UNITS:
        raise ValueError(f"unknown unit {m.group(2)!r} in {value!r}")
    return number * _UNITS[unit]


def format_size(value: int) -> str:
    """Format bytes as a human-readable size string."""
    for unit, divisor in [("GiB", 1024**3), ("MiB", 1024**2), ("KiB", 1024)]:
        if value % divisor == 0 and value >= divisor:
            return f"{value // divisor}{unit}"
    return f"{value}B"
