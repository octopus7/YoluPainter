#!/usr/bin/env python3
"""Build the bundled Korean fallback fonts from Noto Sans CJK 2.004.

Requires fontTools. Pass the directory containing the upstream Regular/Bold
TTC collections (Debian: /usr/share/fonts/opentype/noto). Only Hangul glyphs
are retained; Japanese and Latin text continue to use the existing UI font.
The derived family is named YoluPainter Hangul and remains under SIL OFL 1.1.
"""
import argparse
from pathlib import Path

from fontTools import subset
from fontTools.ttLib import TTFont

ROOT = Path(__file__).resolve().parents[1]
DEST = ROOT / "crates/yolu-app/assets/fonts"
RANGES = ((0x1100, 0x1200), (0x3130, 0x3190), (0xA960, 0xA980),
          (0xAC00, 0xD7A4), (0xD7B0, 0xD800), (0xFFA0, 0xFFDD))


def build(source: Path, weight: str) -> Path:
    font = TTFont(source, fontNumber=1, recalcTimestamp=False)
    assert font["name"].getDebugName(1) == "Noto Sans CJK KR"
    assert "Version 2.004" in font["name"].getDebugName(5)
    options = subset.Options()
    options.name_IDs = ["*"]
    options.name_languages = ["*"]
    options.notdef_outline = True
    options.layout_features = ["*"]
    sub = subset.Subsetter(options)
    sub.populate(unicodes=[cp for start, end in RANGES for cp in range(start, end)])
    sub.subset(font)
    names = font["name"]
    names.names = [n for n in names.names if n.nameID not in (1, 2, 3, 4, 6, 16, 17)]
    for name_id, value in {
        1: "YoluPainter Hangul", 2: weight,
        3: f"YoluPainterHangul-{weight}-2.004",
        4: f"YoluPainter Hangul {weight}",
        6: f"YoluPainterHangul-{weight}",
        16: "YoluPainter Hangul", 17: weight,
    }.items():
        names.setName(value, name_id, 3, 1, 0x409)
    cff = font["CFF "].cff
    cff.fontNames = [f"YoluPainterHangul-{weight}"]
    top = cff.topDictIndex[0]
    top.FamilyName = "YoluPainter Hangul"
    top.FullName = f"YoluPainter Hangul {weight}"
    cmap = font.getBestCmap()
    assert all(cp in cmap for cp in range(0xAC00, 0xD7A4)), "Missing Hangul syllables"
    target = DEST / f"YoluPainterHangul-{weight}.otf"
    font.save(target)
    return target


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source", type=Path, help="Directory containing NotoSansCJK-*.ttc")
    args = parser.parse_args()
    for weight in ("Regular", "Bold"):
        print(build(args.source / f"NotoSansCJK-{weight}.ttc", weight))
