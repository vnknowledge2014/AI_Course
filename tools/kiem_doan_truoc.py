#!/usr/bin/env python3
"""Cổng PHÉP ĐO: một khối chấm điểm phải thật sự đo được cái nó nói nó đo.

Hai luật độc lập, cùng chống một chuyện — một con số xanh không đo cái nó ghi.

## Luật A — `predict` không được hỏi lại đoạn mã đã in sẵn đáp án

Khối `predict` tồn tại để bắt người học CAM KẾT một câu trả lời trước khi thấy
kết quả — đó là toàn bộ giá trị đo lường của nó. Nếu đoạn mã trong `predict`
đã xuất hiện ở một khối phía trên KÈM khung `text` in ra kết quả, thì người
học chỉ cần cuộn lên hai mươi dòng là có đáp án, và cả bốn phương án sai cùng
mấy lời `::why` viết công phu bên dưới không còn ai đọc. Khối ấy đo số 0.

Vòng phản biện T1.1 tìm ra ĐÚNG BA ca như vậy trong một mạch ba mươi bài —
bài 12, bài 18, bài 27 — trong khi mười bốn cổng còn lại đều xanh. Ba lần cùng
một khuôn trong một mạch nghĩa là nó sẽ còn lặp lại ở những mạch chưa ai soi,
và soát bằng mắt qua 302 bài thì lần nào cũng sót.

Hai luật, cả hai đều bảo thủ — thà bỏ lọt còn hơn đánh trượt một bài lành:

  1. Mã trùng. So sau khi bỏ chú thích, bỏ khoảng trắng thừa, và thay RUỘT của
     mọi chuỗi bằng một dấu chấm hỏi — vì bài 12 chỉ khác nhau đúng một chuỗi
     ("khách boa" với "khách boa thêm") mà đáp án thì y hệt. Con số thì KHÔNG
     xoá: đổi số là đổi đáp án, và đó chính là cách sửa đúng.
  2. Đáp án trùng. Chữ của phương án `correct` xuất hiện nguyên văn trong một
     khung ```text phía trên.

Chỉ soi khối phía TRƯỚC trong cùng một bài. Một `example` dạy xong rồi một
`code` cho tập lại cùng cấu trúc là mạch bài bình thường, không phải lỗi —
luật ở đây chỉ đụng tới `predict`, chỗ mà biết trước đáp án là hỏng hẳn.

Ca cố ý thì khai vào `content/curriculum/doan-truoc-bo-qua.yaml` kèm lý do.

## Luật C — `assert` viết ra thì phải có đường chạy tới

Một bước khai `tests` trong `gradingMatrix` mà khối `:::validate` không có
`- tier: tests` thì mọi câu `assert` của nó là mã chết: người học không bao
giờ chạy tới. Cổng `kiem_ma_bai_hoc.mjs` vẫn xanh, vì nó chạy khối `test`
thẳng tay để kiểm lời giải mẫu — nó không hỏi bài có CHẤM bằng khối ấy không.

Đếm lần đầu: **121 câu `assert` trên 59 bước**, có bài tới 9 câu. Người viết
bỏ công nghĩ ra từng câu, viết thông điệp trượt cẩn thận, và không câu nào
tới tay người học.

## Luật B — chấm màn hình nhiều dòng thì không được nhìn mỗi một dòng

`- tier: output` không khai `match` thì compiler mặc định `contains`
(`packages/content-compiler/src/lesson.ts`). `contains` chỉ hỏi màn hình CÓ
CHỨA câu ấy ở đâu đó hay không. Với bài in đúng một dòng thì hợp lý. Với bài
in năm dòng thì nó bỏ qua bốn dòng: người học sai sạch bốn dòng kia vẫn đậu.

Vòng phản biện T1.2 tìm ra một ca cụ thể — bài `03-else-cua-if-nao` chấm bằng
một câu `contains` trên một cảnh duy nhất, nên `if True:` điền vào chỗ đáng lẽ
là `else:` cũng đậu, ở đúng bài dạy `else` thuộc về `if` nào.

Đếm lại toàn kho lúc ấy: 96/294 phép chấm màn hình rơi vào cảnh này. Không ai
CHỌN `contains` cho 96 chỗ đó — nó là giá trị mặc định, và mặc định thì không
ai đọc. Đã siết 93 chỗ bằng cách chạy thật lời giải rồi ghim trọn màn hình;
hai chỗ còn lại khai bỏ qua vì lời giải cần một tệp trên đĩa.

Luật này giữ cho con số ấy không bò lên lại.
"""
import ast
import re
import sys
from pathlib import Path

GOC = Path(__file__).resolve().parent.parent
SO_BO_QUA = GOC / 'content/curriculum/doan-truoc-bo-qua.yaml'

MO_KHOI = re.compile(r'^(:{2,4})(\w+)(?:\{([^}]*)\})?\s*$')
HANG_RAO = re.compile(r'^```(\w+)?([^\n]*)\n(.*?)^```\s*$', re.M | re.S)
CHU_THICH = re.compile(r'#.*$', re.M)
CHUOI = re.compile(r'''(['"])(?:\\.|(?!\1).)*\1''', re.S)


def chuoi_ra_man_hinh(ma: str) -> bool:
    """Trong đoạn này có chuỗi nào ĐI THẲNG ra màn hình không?

    Làm mờ ruột chuỗi là để bắt bài 12 của T1.1, chỗ hai đoạn chỉ khác nhau
    đúng một chuỗi nằm trong nhánh không bao giờ chạy — đáp án y hệt.

    Nhưng cùng cách làm mờ ấy báo oan bài `noi-hai-cau-chu` của Realm 0:
    `print("Phở" + " bò")` với `print("Phở" + "bò")` khác nhau đúng một dấu
    cách, và dấu cách ấy CHÍNH LÀ bài học. Ở đó chuỗi là đáp án, không phải
    chi tiết trang trí.

    Nên chỉ làm mờ khi không chuỗi nào của đoạn chạy thẳng ra `print` hoặc
    nằm trong f-string. Đọc hụt (mã không phân tích nổi) thì coi như CÓ —
    thà bỏ lọt một ca còn hơn đánh trượt một bài lành.
    """
    try:
        cay = ast.parse(ma)
    except SyntaxError:
        return True
    for nut in ast.walk(cay):
        if isinstance(nut, ast.JoinedStr):
            return True
        if (isinstance(nut, ast.Call)
                and isinstance(nut.func, ast.Name) and nut.func.id == 'print'):
            for con in ast.walk(nut):
                if isinstance(con, ast.Constant) and isinstance(con.value, str):
                    return True
    return False


def chuan_hoa(ma: str) -> str:
    """Bỏ chú thích, bỏ khoảng trắng thừa, làm mờ ruột chuỗi khi được phép."""
    ma = CHU_THICH.sub('', ma)
    if not chuoi_ra_man_hinh(ma):
        ma = CHUOI.sub('"?"', ma)
    return '\n'.join(d.rstrip() for d in ma.split('\n') if d.strip())


def doc_khoi(van: str):
    """Cắt bài thành danh sách khối theo thang dấu hai chấm, giữ thứ tự."""
    khoi, hien, than = [], None, []
    for dong in van.split('\n'):
        m = MO_KHOI.match(dong)
        if m and m.group(2) != '' and len(m.group(1)) == 4:
            if hien:
                khoi.append((hien, '\n'.join(than)))
            hien, than = m.group(2), []
            continue
        if dong.rstrip() == '::::' and hien:
            khoi.append((hien, '\n'.join(than)))
            hien, than = None, []
            continue
        if hien:
            than.append(dong)
    if hien:
        khoi.append((hien, '\n'.join(than)))
    return khoi


def rao(than: str):
    """Mọi hàng rào ``` trong một khối: (ngôn ngữ, nhãn, ruột)."""
    return [(m.group(1) or '', m.group(2).strip(), m.group(3))
            for m in HANG_RAO.finditer(than)]


def dap_an_dung(than: str):
    """Chữ của phương án `correct`, nếu có."""
    m = re.search(r'^:::opt\{correct\}\s*\n(.*?)^:::', than, re.M | re.S)
    if not m:
        return None
    return ' '.join(m.group(1).split())


def khoi_ma(kh: str):
    """Lời giải mẫu của một khối, nếu có."""
    for ngon_ngu, nhan, ruot in rao(kh):
        if nhan.startswith('title=solution'):
            return ruot
    return None


def soi_assert_chet(van: str, ten: str, vi_pham: list):
    """Luật C: khối `test` có `assert` mà bước không chấm bằng tier `tests`."""
    dau = van.split('---')
    if len(dau) < 3 or 'tests' not in dau[1]:
        return
    for kh in re.split(r'\n(?=::::\w)', van):
        m = re.search(r'```python title=test\n(.*?)^```', kh, re.S | re.M)
        v = re.search(r':::validate\n(.*?)^:::$', kh, re.S | re.M)
        if not m or not v or 'assert' not in m.group(1):
            continue
        if '- tier: tests' in v.group(1):
            continue
        n = m.group(1).count('assert')
        vi_pham.append((f'{ten} · {n} câu `assert` không bao giờ chạy',
                        'bước khai `tests` trong `gradingMatrix` nhưng `:::validate` không có\n'
                        '     `- tier: tests` — người học không có đường chạy tới mấy câu ấy'))


def soi_phep_cham(van: str, ten: str, bo_qua: set, dung_khoa: set, vi_pham: list):
    """Luật B: `contains` ngầm trên một lời giải in từ hai dòng trở lên."""
    for kh in re.split(r'\n(?=::::\w)', van):
        m = re.search(r'- tier: output\n((?:  \w+:.*\n)+)', kh)
        if not m or 'match:' in m.group(1):
            continue
        sol = khoi_ma(kh)
        if not sol or sol.count('print(') < 2:
            continue
        khoa = f'{ten} · chấm màn hình bằng `contains` ngầm'
        if khoa in bo_qua:
            dung_khoa.add(khoa)
        else:
            vi_pham.append((khoa,
                f'lời giải in ra {sol.count("print(")} lệnh `print` mà phép chấm chỉ soi\n'
                '     một câu — khai `match:` tường minh, hoặc ghim trọn màn hình'))


def main() -> int:
    bo_qua = set()
    if SO_BO_QUA.exists():
        for d in SO_BO_QUA.read_text().split('\n'):
            d = d.strip()
            if d.startswith('- '):
                bo_qua.add(d[2:].strip().strip('\'"'))

    vi_pham, dung_khoa = [], set()
    tep = sorted(GOC.glob('content/**/*.lesson.md'))
    so_predict = 0
    for t in tep:
        van = t.read_text()
        khoi = doc_khoi(van)
        ten = t.relative_to(GOC).as_posix()

        soi_phep_cham(van, ten, bo_qua, dung_khoa, vi_pham)
        soi_assert_chet(van, ten, vi_pham)

        # Mã đã in kèm kết quả, và mọi khung `text` — tính dồn khi đi xuống.
        ma_co_ket_qua, khung_text = {}, []
        for loai, than in khoi:
            if loai == 'predict':
                so_predict += 1
                cua_toi = [r for r in rao(than) if r[0] == 'python']
                for _, _, ruot in cua_toi:
                    k = chuan_hoa(ruot)
                    if k not in ma_co_ket_qua:
                        continue
                    khoa = f'{ten} · mã trùng khối `{ma_co_ket_qua[k]}`'
                    if khoa in bo_qua:
                        dung_khoa.add(khoa)
                    else:
                        vi_pham.append((khoa,
                            'đoạn mã trong `predict` đã chạy sẵn ở khối trên và kết quả đã in ra —\n'
                            '     người học chỉ cần cuộn lên là có đáp án'))
                dung = dap_an_dung(than)
                # Chỉ xét đáp án đủ dài. `"5"` hay `"True"` thì nằm lọt trong
                # bất cứ khung `text` nào — bắt kiểu ấy là đánh trượt hàng
                # loạt bài lành, mà một cổng hay báo oan thì người ta tắt nó
                # đi, và tắt rồi thì nó không còn bắt được ca thật nào nữa.
                if dung and len(dung) >= 15:
                    for nguon in khung_text:
                        # Phải trùng TRỌN một dòng, không phải nằm lẫn bên
                        # trong: đáp án của `predict` là cả dòng máy in ra.
                        if dung not in [' '.join(d.split())
                                        for d in nguon.split('\n') if d.strip()]:
                            continue
                            khoa = f'{ten} · đáp án đã in nguyên văn phía trên'
                            if khoa in bo_qua:
                                dung_khoa.add(khoa)
                            else:
                                vi_pham.append((khoa,
                                    f'phương án đúng — "{dung[:60]}" — nằm nguyên văn\n'
                                    '     trong một khung ```text ở phía trên'))
                            break
                continue

            # Khối thường: nhớ đoạn mã NÀO đã được in kèm kết quả.
            r = rao(than)
            for i, (ngon_ngu, _, ruot) in enumerate(r):
                if ngon_ngu == 'text':
                    khung_text.append(ruot)
                    # Đoạn python đứng ngay trước một khung `text` là đoạn đã
                    # lộ đáp án.
                    if i > 0 and r[i - 1][0] == 'python':
                        ma_co_ket_qua[chuan_hoa(r[i - 1][2])] = loai

    print(f'Đã soi {len(tep)} bài, {so_predict} khối `predict`.')
    print('\n⚠️  PHẠM VI: cổng này chỉ bắt được khi đoạn mã TRÙNG CHỮ (sau khi\n'
          '   bỏ chú thích và làm mờ ruột chuỗi), hoặc khi đáp án nằm nguyên\n'
          '   một dòng trong khung `text` phía trên.\n'
          '   Nó KHÔNG bắt được một `predict` tính lại đúng chuyện ấy bằng\n'
          '   cách viết khác — bài 27 của T1.1 là đúng ca đó, và vòng phản\n'
          '   biện phải tìm ra bằng người. Muốn bắt nốt thì phải CHẠY đoạn mã\n'
          '   rồi đối chiếu kết quả với các khung `text` phía trên; thử đo\n'
          '   bằng túi hằng số + túi tên thì báo oan 8 bài lành, vì cách ấy mù\n'
          '   hẳn cấu trúc — mà cấu trúc mới là thứ nhiều bài đem ra đổi.\n'
          '   Đừng đọc màu xanh dưới đây rộng hơn hai luật vừa nói.')
    if dung_khoa:
        print(f'  {len(dung_khoa)} ca được khai bỏ qua và có dùng tới.')

    thua = bo_qua - dung_khoa
    if thua:
        print('\n❌ Khai bỏ qua nhưng KHÔNG khớp ca nào — bài đã đổi, tờ giấy '
              'miễn trừ nằm lại:')
        for k in sorted(thua):
            print(f'  ✗ {k}')

    if vi_pham:
        print(f'\n❌ {len(vi_pham)} phép đo không đo được cái nó nói nó đo:')
        for khoa, vi_sao in vi_pham:
            print(f'  ✗ {khoa}\n     {vi_sao}')
        print('\nSửa bằng cách đổi khối `predict` sang một cảnh CHƯA chạy ở đâu\n'
              '(đổi số, đổi tên, đổi tình huống), hoặc chuyển lời giải thích\n'
              'xuống SAU khi người học đã cam kết câu trả lời.')

    if vi_pham or thua:
        return 1
    print('✅ Mọi phép đo đều đo được thứ nó nói nó đo.')
    return 0


if __name__ == '__main__':
    sys.exit(main())
