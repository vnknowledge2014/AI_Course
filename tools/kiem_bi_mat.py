#!/usr/bin/env python3
"""Chặn credential lọt vào git.

Dự án này từng có một khoá API thật nằm trong `fp/omni.config.yaml`. Nó không
lọt vào commit nào — nhưng chỉ vì có người nhớ thêm dòng gitignore đúng lúc.
"Nhớ đúng lúc" không phải một biện pháp; một khoá đã vào lịch sử git thì coi
như đã công khai, kể cả khi commit sau xoá nó đi.

Script này quét những file GIT ĐANG THEO DÕI, tìm giá trị credential thật chứ
không tìm chữ "password". Phân biệt ấy quan trọng: tài liệu nói *về* bảo mật
và mã mẫu dạy băm mật khẩu đều chứa những từ đó, và một cổng kêu ở mọi chỗ thì
chẳng ai còn nghe nó nữa.

    python3 tools/kiem_bi_mat.py           # quét file git đang theo dõi
    python3 tools/kiem_bi_mat.py --tat-ca  # quét cả file chưa theo dõi
"""
from __future__ import annotations

import argparse
import math
import pathlib
import re
import subprocess
import sys

GOC = pathlib.Path(__file__).resolve().parent.parent

# Khoá có hình dạng NHẬN RA ĐƯỢC — bắt theo tiền tố của nhà cung cấp thì gần
# như không bao giờ báo nhầm.
KHUON_RO = [
    (r"sk-[A-Za-z0-9]{20,}", "khoá OpenAI"),
    (r"sk-ant-[A-Za-z0-9_-]{20,}", "khoá Anthropic"),
    (r"gh[pousr]_[A-Za-z0-9]{30,}", "token GitHub"),
    (r"AKIA[0-9A-Z]{16}", "khoá truy cập AWS"),
    (r"AIza[0-9A-Za-z_-]{30,}", "khoá Google API"),
    (r"xox[baprs]-[0-9A-Za-z-]{10,}", "token Slack"),
    (r"-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----", "khoá riêng"),
]

# Gán một chuỗi dài vào một cái tên nghe như credential.
GAN = re.compile(
    r"""(?ix)
    \b(
        api[_-]?key | apikey | access[_-]?token | auth[_-]?token |
        secret[_-]?key | client[_-]?secret | private[_-]?key |
        [a-z_]*_pass(?:word)? | passwd
    )\b
    \s* [:=] \s*
    ['"]? ([A-Za-z0-9._\-/+]{16,}) ['"]?
    """
)

# Giá trị rõ ràng KHÔNG phải bí mật: chỗ giữ chỗ, biến môi trường, ví dụ.
VO_HAI = re.compile(
    r"""(?ix)
    ^( \$\{.*\} | \$[A-Z_]+ | <.*> | \.\.\. | x{4,} | \*{4,}
     | (?:your|my|the)[_-] | change[_-]?me | placeholder | example | dummy
     | fake | sample | test[_-]?only | redacted | none | null | true | false
     # Đọc từ biến môi trường là cách ĐÚNG, không phải rò rỉ. Cổng kêu ở đây
     # sẽ dạy người ta bỏ qua nó, mà một cổng bị bỏ qua thì không còn là cổng.
     | process\.env | os\.environ | std::env | System\.getenv | ENV\[
     )
    """
)

DUOI_BO_QUA = {
    ".lock", ".png", ".jpg", ".jpeg", ".gif", ".ico", ".icns", ".wasm",
    ".zip", ".pdf", ".woff", ".woff2", ".ttf",
}


def do_hon_loan(s: str) -> float:
    """Entropy Shannon. Khoá thật gần như luôn > 3.5; chữ tiếng Anh ~2.5."""
    if not s:
        return 0.0
    return -sum(
        (n := s.count(c) / len(s)) and n * math.log2(n) for c in set(s)
    )


# Thư mục không bao giờ đáng quét: phụ thuộc bên thứ ba và sản phẩm build.
BO_QUA_THU_MUC = {"node_modules", "target", "dist", ".git", "venv", ".venv", "vendor"}


def cac_tep(tat_ca: bool) -> list[pathlib.Path]:
    ra = subprocess.run(
        ["git", "ls-files"], cwd=GOC, capture_output=True, text=True, check=False
    )
    tep = [GOC / d for d in ra.stdout.splitlines() if d]

    if tat_ca:
        # `git ls-files --others --exclude-standard` bỏ qua file BỊ GITIGNORE —
        # tức là bỏ qua đúng chỗ credential hay nằm nhất. Chính khoá Ollama của
        # dự án này nằm trong một file gitignore. Nên chế độ `--tat-ca` phải đi
        # thẳng vào đĩa, không hỏi git.
        # Cắt nhánh NGAY LÚC DUYỆT, không lọc sau: `rglob` vẫn đi vào
        # `node_modules` rồi mới bỏ, và cây ấy đủ lớn để việc quét mất hàng
        # phút thay vì vài giây.
        import os

        for goc_hien, thu_muc, ten_tep in os.walk(GOC):
            thu_muc[:] = [d for d in thu_muc if d not in BO_QUA_THU_MUC]
            for n in ten_tep:
                tep.append(pathlib.Path(goc_hien) / n)

    thay = []
    da_thay = set()
    for t in tep:
        if t in da_thay or not t.is_file() or t.suffix.lower() in DUOI_BO_QUA:
            continue
        da_thay.add(t)
        thay.append(t)
    return thay


def quet(t: pathlib.Path) -> list[tuple[int, str, str]]:
    try:
        noi_dung = t.read_text(encoding="utf-8")
    except (UnicodeDecodeError, OSError):
        return []

    ra: list[tuple[int, str, str]] = []
    for i, dong in enumerate(noi_dung.splitlines(), 1):
        for khuon, ten in KHUON_RO:
            m = re.search(khuon, dong)
            if m:
                ra.append((i, ten, m.group(0)[:24] + "…"))

        for m in GAN.finditer(dong):
            khoa, gia_tri = m.group(1), m.group(2)
            if VO_HAI.match(gia_tri):
                continue
            # Entropy là bộ lọc cuối: `api_key: my_local_dev_setting` thấp,
            # `api_key: ae2beef2ab52...` cao.
            if do_hon_loan(gia_tri) < 3.5:
                continue
            ra.append((i, f"gán vào `{khoa}`", gia_tri[:12] + "…"))
    return ra


def main() -> int:
    ap = argparse.ArgumentParser(description="Chặn credential lọt vào git")
    ap.add_argument("--tat-ca", action="store_true", help="quét cả file chưa theo dõi")
    ns = ap.parse_args()

    tep = cac_tep(ns.tat_ca)
    thay: list[tuple[pathlib.Path, int, str, str]] = []
    for t in tep:
        for dong, loai, mau in quet(t):
            thay.append((t, dong, loai, mau))

    pham_vi = "mọi file (kể cả chưa theo dõi)" if ns.tat_ca else "file git đang theo dõi"
    print(f"Đã quét {len(tep)} {pham_vi}.")
    if not thay:
        print("✅ Không thấy credential nào.")
        return 0

    print(f"❌ {len(thay)} chỗ nghi là credential:\n")
    for t, dong, loai, mau in thay:
        try:
            ten = t.relative_to(GOC)
        except ValueError:
            ten = t
        print(f"  ✗ {ten}:{dong}")
        print(f"     {loai}: {mau}")
    print(
        "\n   Nếu đây là khoá thật: XOAY NÓ NGAY, đừng chỉ xoá dòng đó đi.\n"
        "   Một khoá đã vào lịch sử git thì coi như đã công khai — commit sau\n"
        "   xoá nó không lấy lại được gì."
    )
    return 1


if __name__ == "__main__":
    sys.exit(main())
