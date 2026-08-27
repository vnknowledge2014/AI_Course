#!/usr/bin/env python3
"""Cổng SỔ SỰ THẬT: hằng số của thế giới hư cấu không được tự đổi giữa hai bài.

Vòng phản biện T2.2 tìm ra 29 chỗ "mâu thuẫn bài trước", và phần lớn là đúng
MỘT lớp lỗi: cái xe bánh mì của Byte đổi số liệu mà không bài nào nói gì.

  - tiền nguyên liệu mỗi ổ: bài 2 chốt 9 000, bài 4 viết 5 000;
  - tiền thuê chỗ: bài 27 chốt 30 nghìn, bài 34 viết 100 nghìn;
  - dì Tư lấy sẵn: bài 7 chốt 2 ổ, bài 9 viết 6 ổ.

Người học không có cách nào biết đó là chuyện khác hay là mình nhớ nhầm — họ
mang con số của bài trước sang, ra đáp án khác, và tưởng mình sai.

Không cổng nào bắt được, vì mỗi bài tự nó nhất quán. Chỉ khi đặt cạnh nhau thì
mâu thuẫn mới lộ. Nên sự thật phải khai MỘT chỗ, và máy đối chiếu — soát bằng
mắt qua 302 bài thì lần nào cũng sót.

Đổi số có chủ ý (bài BOSS mở một buổi bán khác) thì khai vào `ngoai_le`, kèm
lý do. Ngoại lệ không dùng tới là LỖI: nó nghĩa là sự thật đã trôi đi chỗ khác
mà tờ giấy miễn trừ vẫn nằm lại, sẵn sàng tha cho một lỗi thật.
"""
import re
import sys
from pathlib import Path

GOC = Path(__file__).resolve().parent.parent
SO_SU_THAT = GOC / 'content/curriculum/su-that-the-gioi.yaml'

# Một con số trong văn xuôi tiếng Việt: có thể nhóm nghìn bằng dấu cách,
# dấu cách cứng hoặc dấu chấm.
SO = r'\d[\d]*(?:[.\u00a0 ]\d{3})*'


def doc_so(s: str) -> int:
    return int(re.sub(r'[.\u00a0 ]', '', s))


def doc_so_su_that():
    """Đọc YAML bằng tay — không kéo pyyaml vào chỉ để đọc một tệp phẳng."""
    if not SO_SU_THAT.exists():
        return []
    facts, cur, khoa_ds = [], None, None
    for dong in SO_SU_THAT.read_text().split('\n'):
        if not dong.strip() or dong.lstrip().startswith('#'):
            continue
        m = re.match(r'^-\s+(\w+):\s*(.*)$', dong)
        if m:
            cur = {}
            facts.append(cur)
            khoa_ds = None
            _dat(cur, m.group(1), m.group(2))
            continue
        m = re.match(r'^  (\w+):\s*(.*)$', dong)
        if m and cur is not None:
            khoa, gt = m.group(1), m.group(2).strip()
            if gt == '':
                khoa_ds = khoa
                cur[khoa] = []
            else:
                khoa_ds = None
                _dat(cur, khoa, gt)
            continue
        m = re.match(r'^    -\s*(.*)$', dong)
        if m and cur is not None and khoa_ds:
            cur[khoa_ds].append(_boc(m.group(1)))
    return facts


def _boc(s: str) -> str:
    s = s.strip()
    if len(s) >= 2 and s[0] == s[-1] and s[0] in '"\'':
        return s[1:-1]
    return s


def _dat(d, khoa, gt):
    gt = _boc(gt)
    d[khoa] = int(gt) if re.fullmatch(r'-?\d+', gt) else gt


def kiem():
    facts = doc_so_su_that()
    if not facts:
        print(f'Chưa có sổ sự thật ({SO_SU_THAT.relative_to(GOC)}) — bỏ qua.')
        return 0

    loi, da_dung_ngoai_le, tong_kiem = [], set(), 0

    for f in facts:
        ten = f.get('ten', '?')
        gia_tri = f.get('gia_tri')
        # `nghin: 1` nghĩa là con số trong văn xuôi viết theo đơn vị nghìn
        # ("30 nghìn"), nên phải nhân lên trước khi so.
        cho_nghin = int(f.get('cho_nghin', 0) or 0)
        tu_khoa = f.get('tu_khoa') or []
        ngoai_le = {}
        for d in f.get('ngoai_le') or []:
            if ':' in d:
                t, ly_do = d.split(':', 1)
                ngoai_le[t.strip()] = ly_do.strip()
        pham_vi = f.get('pham_vi') or 'content'

        for tep in sorted((GOC / pham_vi).rglob('*.lesson.md')):
            ten_tep = str(tep.relative_to(GOC))
            trong_khoi_ma = False
            for i, dong in enumerate(tep.read_text().split('\n'), 1):
                # Khối mã có trọng tài riêng (`kiem_ma_bai_hoc.mjs` chạy thật).
                if re.match(r'^\s*```', dong):
                    trong_khoi_ma = not trong_khoi_ma
                    continue
                if trong_khoi_ma:
                    continue
                for kw in tu_khoa:
                    for m in re.finditer(kw, dong, re.I):
                        so_thay = next(
                            (g for g in m.groups() if g and re.fullmatch(rf'{SO}', g.strip())),
                            None,
                        )
                        if so_thay is None:
                            continue
                        tong_kiem += 1
                        v = doc_so(so_thay)
                        if cho_nghin and v < 1000:
                            v *= 1000
                        if v == gia_tri:
                            continue
                        if ten_tep in ngoai_le:
                            da_dung_ngoai_le.add((ten, ten_tep))
                            continue
                        loi.append((ten_tep, i, ten, v, gia_tri, dong.strip()[:96]))

        for t in ngoai_le:
            if (ten, t) not in da_dung_ngoai_le:
                loi.append((t, 0, ten, None, gia_tri,
                            'NGOẠI LỆ THỪA — khai miễn trừ mà không có gì để miễn'))

    print(f'Đã đối chiếu {tong_kiem} chỗ nhắc tới sự thật thế giới, trên {len(facts)} sự thật đã khai.')
    if not loi:
        print('✅ Không hằng số nào của thế giới tự đổi giữa các bài.')
        return 0

    print(f'❌ {len(loi)} chỗ lệch sổ sự thật:\n')
    for tep, dong, ten, v, mong, trich in loi:
        vt = f'{tep}:{dong}' if dong else tep
        print(f'  ✗ {vt}')
        if v is None:
            print(f'     `{ten}` — {trich}')
        else:
            print(f'     `{ten}` đã khai là {mong:,}, ở đây là {v:,}')
            print(f'     {trich}')
    print(f'\nĐổi số có chủ ý thì khai vào {SO_SU_THAT.relative_to(GOC)} kèm lý do,')
    print('và NÓI THẲNG trong bài rằng số đã đổi — người học không đoán được.')
    return 1


if __name__ == '__main__':
    sys.exit(kiem())
