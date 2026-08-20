#!/usr/bin/env python3
"""Kiểm hiến chương sư phạm trên mọi file .lesson.md.

Trình biên dịch (`packages/content-compiler`) đã kiểm phần CẤU TRÚC: bài có
biên dịch được không, có đủ solution/test/hints không, thời lượng có nằm trong
7–15 phút không. Nó không kiểm được phần SƯ PHẠM — những luật nói về việc bài
học đối xử với người mới ra sao. Đó là việc của file này.

Ranh giới giữa hai bên: compiler từ chối bài KHÔNG DÙNG ĐƯỢC; script này từ
chối bài dùng được nhưng DẠY DỞ. Cả hai đều chạy trong CI.

    python3 tools/kiem_bai_hoc.py                     # kiểm toàn bộ content/
    python3 tools/kiem_bai_hoc.py --json              # xuất máy đọc
    python3 tools/kiem_bai_hoc.py a.lesson.md b.md    # chỉ kiểm các file này

Truyền đường dẫn cụ thể khi nhiều người viết song song: chạy trên toàn
`content/` lúc đó sẽ đỏ vì bài của NGƯỜI KHÁC còn dở, và người viết không có
cách nào biết mình đã xong hay chưa.

Cú pháp thang hai chấm (MASTERPLAN §6):
    ::::step      bước
    :::child        con của bước
    ::leaf            lá
"""
from __future__ import annotations

import argparse
import json
import pathlib
import re
import sys

GOC = pathlib.Path(__file__).resolve().parent.parent
NOI_DUNG = GOC / "content"

# Byte cố tình KHÔNG có tâm trạng tiêu cực. Người mới học đã đủ thấy mình kém;
# nhân vật hướng dẫn mà tỏ ra thất vọng thì biến sai lầm thành nỗi xấu hổ.
# Xem MASTERPLAN §0 quyết định 11.
TAM_TRANG_CAM = {"buon", "that-vong", "sad", "gian", "chan", "bucboi", "disappointed"}

# Những từ hạ thấp người đọc. "Đơn giản mà" chỉ đơn giản với người đã biết.
TU_HA_THAP = [
    "rõ ràng là", "hiển nhiên", "dĩ nhiên là", "chỉ cần đơn giản",
    "ai cũng biết", "quá dễ", "dễ ợt", "tất nhiên rồi",
]


def cac_bai() -> list[pathlib.Path]:
    return sorted(NOI_DUNG.rglob("*.lesson.md"))


def tach_buoc(t: str) -> list[tuple[str, str]]:
    """Cắt thân bài thành [(tên bước, nội dung)] theo mốc `::::ten`."""
    moc = list(re.finditer(r"^::::([a-zA-Z][\w-]*)", t, re.M))
    ra = []
    for i, m in enumerate(moc):
        het = moc[i + 1].start() if i + 1 < len(moc) else len(t)
        ra.append((m.group(1), t[m.end() : het]))
    return ra


def kiem(f: pathlib.Path) -> list[str]:
    t = f.read_text(encoding="utf-8")
    try:
        ten: object = f.relative_to(NOI_DUNG)
    except ValueError:
        ten = f
    loi: list[str] = []

    def bao(m: str) -> None:
        loi.append(f"{ten}: {m}")

    # Khung `content new` chưa ai điền. Trình biên dịch cũng chặn cái này
    # (lesson.ts::KHUNG_CHUA_DIEN); kiểm cả hai nơi là cố ý — bộ kiểm sư phạm
    # phải tự đứng vững được, không dựa vào việc compiler đã chạy trước.
    if re.search(r"TODO\s+—", t):
        bao("còn chỗ `TODO —` chưa điền — đây là khung rỗng, không phải bài học")
        return loi

    buoc = tach_buoc(t)
    loai = [b for b, _ in buoc]

    # ── 1. Mọi lựa chọn SAI phải giải thích vì sao ────────────────────────
    # Đây là luật quan trọng nhất. Một câu hỏi chỉ báo "sai rồi" dạy người ta
    # đoán mò; một câu hỏi nói rõ suy nghĩ nào dẫn tới đáp án đó thì sửa được
    # chính cái suy nghĩ ấy.
    for _, than in buoc:
        for m in re.finditer(r"^:::opt(\{[^}]*\})?\s*$(.*?)^:::\s*$", than, re.M | re.S):
            attrs, noi = m.group(1) or "", m.group(2)
            if "correct" in attrs:
                continue
            if not re.search(r"^::why\s*$", noi, re.M):
                dau = noi.strip().splitlines()[0][:40] if noi.strip() else "(rỗng)"
                bao(f'lựa chọn sai "{dau}" không có ::why')

    # ── 2. Byte không bao giờ buồn hay thất vọng ─────────────────────────
    for m in re.finditer(r"mood\s*[:=]\s*['\"]?([\w-]+)", t):
        if m.group(1).lower() in TAM_TRANG_CAM:
            bao(f"Byte mang tâm trạng tiêu cực `{m.group(1)}`")

    # ── 3. Đoán trước khi chạy ────────────────────────────────────────────
    # Chỉ bắt buộc với bài có bước `code`: đoán trước chỉ có nghĩa khi ngay
    # sau đó người học thấy được kết quả thật để đối chiếu.
    if "code" in loai and "predict" not in loai:
        bao("có bước `code` nhưng không có bước `predict` nào trước đó")
    if "predict" in loai and "code" in loai and loai.index("predict") > loai.index("code"):
        bao("bước `predict` nằm SAU bước `code` — đoán sau khi đã thấy đáp án thì vô nghĩa")

    # ── 4. Thang gợi ý đủ ba nấc, đúng thứ tự ────────────────────────────
    # attention (hướng mắt) → strategy (hướng nghĩ) → one-line (đưa câu trả
    # lời). Nhảy thẳng tới one-line là làm hộ, không phải dạy.
    THU_TU = ["attention", "strategy", "one-line"]
    for m in re.finditer(r"^:::hints\s*$(.*?)^:::\s*$", t, re.M | re.S):
        nac = re.findall(r"^\s*-\s*kind:\s*([\w-]+)", m.group(1), re.M)
        if nac != THU_TU:
            bao(f"thang gợi ý là {nac or '[]'}, phải đúng {THU_TU}")

    # ── 5. Bước `code` phải có gợi ý ─────────────────────────────────────
    for b, than in buoc:
        if b == "code" and ":::hints" not in than:
            bao("bước `code` không có `:::hints` — người bí sẽ mắc kẹt")

    # ── 6. Không hạ thấp người đọc ───────────────────────────────────────
    thap = t.lower()
    for tu in TU_HA_THAP:
        if tu in thap:
            bao(f'dùng từ hạ thấp người đọc: "{tu}"')

    # ── 7. Mỗi bài phải chốt lại ─────────────────────────────────────────
    if not ({"reflect", "checkpoint"} & set(loai)):
        bao("không có bước `reflect` hay `checkpoint` để chốt lại điều vừa học")

    return loi


def main() -> int:
    ap = argparse.ArgumentParser(description="Kiểm hiến chương sư phạm cho .lesson.md")
    ap.add_argument("--json", action="store_true", help="xuất JSON thay vì chữ")
    ap.add_argument("tep", nargs="*", help="chỉ kiểm các file này (mặc định: toàn bộ content/)")
    ns = ap.parse_args()

    if ns.tep:
        bai = [pathlib.Path(t).resolve() for t in ns.tep]
        thieu = [t for t in bai if not t.is_file()]
        if thieu:
            for t in thieu:
                print(f"không tìm thấy: {t}", file=sys.stderr)
            return 2
    else:
        bai = cac_bai()
    if not bai:
        print("Không tìm thấy file .lesson.md nào trong content/", file=sys.stderr)
        return 1

    tat_ca = [l for f in bai for l in kiem(f)]

    if ns.json:
        print(json.dumps({"so_bai": len(bai), "vi_pham": tat_ca}, ensure_ascii=False, indent=2))
        return 1 if tat_ca else 0

    print(f"Đã kiểm {len(bai)} bài học.")
    if not tat_ca:
        print("✅ Không có vi phạm hiến chương sư phạm.")
        return 0
    print(f"❌ {len(tat_ca)} vi phạm:\n")
    for l in tat_ca:
        print(f"  ✗ {l}")
    return 1


if __name__ == "__main__":
    sys.exit(main())
