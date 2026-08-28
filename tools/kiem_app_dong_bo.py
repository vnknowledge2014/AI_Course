#!/usr/bin/env python3
"""Cổng ĐỒNG BỘ APP: thứ app ship phải đúng bằng thứ repo có.

App đọc bài học từ `apps/byte/public/noi-dung/`, một BẢN CHÉP do
`pnpm run noi-dung` sinh ra. Không ai chạy lệnh ấy tự động, và không cổng nào
đối chiếu hai bên — nên bản chép cứ cũ dần mà mọi thứ vẫn xanh.

Lúc dựng cổng này, repo có 272 bài còn app ship 139. Một nửa học liệu không tới
tay người học, trong khi `pnpm build` chạy sạch, 12 cổng xanh, và tiến độ báo
90% phạm vi v1.0. Đúng cái hình dạng lỗi mà cả tập cổng này đi bắt: một con số
xanh không đo thứ nó nói là nó đo.

Cổng so cả SỐ LƯỢNG lẫn DANH SÁCH ID. So số lượng thôi thì hai bên cùng 272 mà
khác nhau một bài vẫn lọt.
"""
import json
import sys
from pathlib import Path

GOC = Path(__file__).resolve().parent.parent
REPO = GOC / "dist/content/index.json"
APP = GOC / "apps/byte/public/noi-dung/index.json"


def doc(p: Path) -> set[str]:
    if not p.is_file():
        print(f"❌ thiếu {p.relative_to(GOC)}")
        print("   Chạy: cd apps/byte && pnpm run noi-dung")
        sys.exit(1)
    d = json.loads(p.read_text(encoding="utf-8"))
    ds = d.get("lessons", d) if isinstance(d, dict) else d
    return {b["id"] for b in ds}


def main() -> int:
    repo, app = doc(REPO), doc(APP)
    print(f"Repo có {len(repo)} bài; app ship {len(app)} bài.")
    thieu = repo - app
    thua = app - repo
    if not thieu and not thua:
        print("✅ App ship đúng bằng thứ repo có.")
        return 0
    print("❌ App và repo LỆCH nhau:")
    if thieu:
        print(f"\n  {len(thieu)} bài CÓ trong repo mà app KHÔNG ship:")
        for b in sorted(thieu)[:10]:
            print(f"     · {b}")
        if len(thieu) > 10:
            print(f"     · … và {len(thieu) - 10} bài nữa")
    if thua:
        print(f"\n  {len(thua)} bài app ship mà repo KHÔNG còn:")
        for b in sorted(thua)[:10]:
            print(f"     · {b}")
    print("\nSửa: cd apps/byte && pnpm run noi-dung")
    return 1


if __name__ == "__main__":
    sys.exit(main())
