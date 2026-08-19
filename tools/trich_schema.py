#!/usr/bin/env python3
"""Trích schema nội dung v2 từ MASTERPLAN.md §5 sang packages/content-schema.

Schema sống ở MASTERPLAN vì đó là nơi nó được lập luận cùng mọi quyết định khác.
Chép tay sang code sẽ trôi. Script này giữ hai bên khớp nhau.

    python3 tools/trich_schema.py           # trích lại
    python3 tools/trich_schema.py --check    # thoát 1 nếu code lệch MASTERPLAN
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

GOC = Path(__file__).resolve().parent.parent
NGUON = GOC / "MASTERPLAN.md"
DICH = GOC / "packages" / "content-schema" / "src" / "v2.ts"

HEADER = '''/**
 * Schema nội dung v2 — HỢP ĐỒNG TRUNG TÂM.
 *
 * Mọi package khác phụ thuộc vào file này. **ĐÓNG BĂNG** trước khi viết lesson
 * đầu tiên: sửa schema sau đó nghĩa là sửa lại mọi bài đã viết.
 *
 * Nguồn: `MASTERPLAN.md` §5. Trích tự động bởi `tools/trich_schema.py` —
 * nếu cần đổi, sửa MASTERPLAN rồi trích lại, đừng sửa trực tiếp ở đây.
 */

'''


def trich() -> str:
    lines = NGUON.read_text(encoding="utf-8").split("\n")
    bd = next(i for i, l in enumerate(lines) if l.startswith("## 5. SCHEMA NỘI DUNG v2"))
    kt = next(i for i, l in enumerate(lines) if i > bd and l.startswith("## 6."))
    phan = "\n".join(lines[bd:kt])
    khoi = re.findall(r"```ts\n(.*?)```", phan, re.S)
    if not khoi:
        raise SystemExit("Không tìm thấy khối TypeScript nào trong §5")
    return HEADER + "\n\n".join(k.rstrip() for k in khoi) + "\n"


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--check", action="store_true")
    a = ap.parse_args()

    moi = trich()
    if a.check:
        hien = DICH.read_text(encoding="utf-8") if DICH.exists() else ""
        if hien != moi:
            print("❌ packages/content-schema/src/v2.ts lệch MASTERPLAN.md §5")
            print("   Chạy: python3 tools/trich_schema.py")
            return 1
        print("✅ schema khớp MASTERPLAN.md §5")
        return 0

    DICH.parent.mkdir(parents=True, exist_ok=True)
    DICH.write_text(moi, encoding="utf-8")
    print(f"✅ đã trích {moi.count(chr(10))} dòng sang {DICH.relative_to(GOC)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
