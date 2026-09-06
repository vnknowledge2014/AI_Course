---
id: tri-tue-nhan-tao.da-tac-tu-va-quan-sat-ai.hang-doi-uu-tien-chong-doi
title: "Hàng đợi ưu tiên: trả phí trước, nhưng miễn phí KHÔNG BAO GIỜ bị đói mãi"
summary: "lay_request_tiep_theo(hang_doi_tra_phi, hang_doi_mien_phi, so_lan_da_uu_tien_lien_tiep, gioi_han_uu_tien) UU TIEN hang_doi_tra_phi NEU con request VA so_lan_da_uu_tien_lien_tiep < gioi_han_uu_tien (tang bo dem); khi da uu tien DU gioi_han_uu_tien lan LIEN TIEP (hoac hang_doi_tra_phi rong), chuyen sang lay TU hang_doi_mien_phi MOT LAN (RESET bo dem ve 0); neu ca hai deu het thi tra ve None. xu_ly_toan_bo_hai_hang_doi(...) lap lien tuc toi khi ca hai hang doi rong, dem so_mien_phi_da_phuc_vu. Tren 7 request tra phi + 2 request mien phi, gioi_han_uu_tien=3: thu tu phuc vu la [P1,P2,P3,F1,P4,P5,P6,F2,P7] -- CA HAI request mien phi DEU duoc phuc vu (khong bi doi vinh vien), request mien phi DAU TIEN duoc phuc vu o VI TRI 3 (khong phai cho toi khi tra_phi rong het moi duoc lot vao). Doi gioi_han_uu_tien tu 3 xuong 1: vi_tri_dau_tien_cua_mien_phi doi tu 3 xuong 1 (doi cho ngan hon) nhung so_mien_phi_da_phuc_vu VAN la 2 (ca hai request mien phi van duoc phuc vu day du, chi khac THOI DIEM)."
locale: vi
track: tri-tue-nhan-tao
module: da-tac-tu-va-quan-sat-ai
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.hang-doi-uu-tien-chong-doi]
requires: [ai.do-ty-le-cache-hit]
concepts: [ai.hang-doi-uu-tien-chong-doi]
gradingMatrix:
  web-chrome: [static, run, tests, output]
  web-firefox: [static, run, tests, output]
  macos: [static, run, tests, output]
  windows: [static, run, tests, output]
  linux: [static, run, tests, output]
  android: [static, run, tests, output]
  ios: [static, run, tests, output]
provenance:
  authoredBy: llm-assisted
  reviewed: false
---

::::byte{trigger=enter mood=curious pose=lean-in}
`Chương 39` Bài tập `3`: "Thiết kế một hàng đợi ưu tiên cho request LLM:
người dùng trả phí được phục vụ trước. Xử lý cả trường hợp **starvation**
(đói) của hàng đợi thấp." Ưu tiên là dễ — LUÔN phục vụ người trả phí trước.
Nhưng nếu LUÔN đúng nghĩa đen, người dùng miễn phí có bao giờ được phục vụ
không?
::::

::::explain{#uu_tien_co_gioi_han_khong_phai_uu_tien_tuyet_doi}
Nếu "ưu tiên" nghĩa LÀ "phục vụ hết SẠCH hàng đợi trả phí rồi mới đụng tới
hàng đợi miễn phí", và request trả phí LIÊN TỤC đổ về (như một hệ thống
thật, tải cao), hàng đợi miễn phí sẽ CHỜ VÔ HẠN — đây CHÍNH LÀ
**starvation** mà `Chương 39` Bài tập `3` yêu cầu xử lý.

Giải pháp: ưu tiên trả phí, nhưng có GIỚI HẠN — chỉ ưu tiên LIÊN TIẾP tối
đa `gioi_han_uu_tien` lần, rồi BẮT BUỘC nhường một lượt cho hàng đợi miễn
phí (dù hàng đợi trả phí vẫn còn request đang chờ):

```
lay_request_tiep_theo(hang_doi_tra_phi, hang_doi_mien_phi,
                       so_lan_da_uu_tien_lien_tiep, gioi_han_uu_tien):
  neu hang_doi_tra_phi CON request VA so_lan_da_uu_tien_lien_tiep < gioi_han_uu_tien:
    lay MOT request tu hang_doi_tra_phi, tang bo dem len 1
  khac neu hang_doi_mien_phi CON request:
    lay MOT request tu hang_doi_mien_phi, RESET bo dem ve 0
  khac neu hang_doi_tra_phi CON request:          # mien phi rong, danh phai lay tra phi
    lay MOT request tu hang_doi_tra_phi, tang bo dem len 1
  khac:
    khong con gi ca -> tra ve None
```

`gioi_han_uu_tien` HỮU HẠN LÀ chìa khoá: dù hàng đợi trả phí dài bao nhiêu,
sau ĐÚNG `gioi_han_uu_tien` lần ưu tiên liên tiếp, một request miễn phí
LUÔN được xen vào. Hàng đợi miễn phí có thể phải CHỜ (không phục vụ ngay
lập tức), nhưng KHÔNG BAO GIỜ chờ VĨNH VIỄN.
::::

::::example{#chin_request_khong_doi_mien_phi}
```python title=readonly
def lay_request_tiep_theo(hang_doi_tra_phi, hang_doi_mien_phi, so_lan_da_uu_tien_lien_tiep, gioi_han_uu_tien):
    if hang_doi_tra_phi and so_lan_da_uu_tien_lien_tiep < gioi_han_uu_tien:
        request = hang_doi_tra_phi.pop(0)
        return request, True, so_lan_da_uu_tien_lien_tiep + 1
    if hang_doi_mien_phi:
        request = hang_doi_mien_phi.pop(0)
        return request, False, 0
    if hang_doi_tra_phi:
        request = hang_doi_tra_phi.pop(0)
        return request, True, so_lan_da_uu_tien_lien_tiep + 1
    return None, None, so_lan_da_uu_tien_lien_tiep


def xu_ly_toan_bo_hai_hang_doi(hang_doi_tra_phi_ban_dau, hang_doi_mien_phi_ban_dau, gioi_han_uu_tien):
    hang_doi_tra_phi = list(hang_doi_tra_phi_ban_dau)
    hang_doi_mien_phi = list(hang_doi_mien_phi_ban_dau)
    thu_tu_phuc_vu = []
    so_lan_da_uu_tien_lien_tiep = 0
    while hang_doi_tra_phi or hang_doi_mien_phi:
        request, la_tra_phi, so_lan_da_uu_tien_lien_tiep = lay_request_tiep_theo(
            hang_doi_tra_phi, hang_doi_mien_phi, so_lan_da_uu_tien_lien_tiep, gioi_han_uu_tien
        )
        thu_tu_phuc_vu.append((request, la_tra_phi))
    so_mien_phi_da_phuc_vu = sum(1 for _, la_tra_phi in thu_tu_phuc_vu if la_tra_phi is False)
    return thu_tu_phuc_vu, so_mien_phi_da_phuc_vu


HANG_DOI_TRA_PHI = ["P1", "P2", "P3", "P4", "P5", "P6", "P7"]
HANG_DOI_MIEN_PHI = ["F1", "F2"]

THU_TU_3, SO_MIEN_PHI_3 = xu_ly_toan_bo_hai_hang_doi(HANG_DOI_TRA_PHI, HANG_DOI_MIEN_PHI, 3)
VI_TRI_DAU_TIEN_3 = min(i for i, (_, la_tra_phi) in enumerate(THU_TU_3) if la_tra_phi is False)

print([r for r, _ in THU_TU_3])
print(SO_MIEN_PHI_3)
print(VI_TRI_DAU_TIEN_3)
```

```text title=readonly
['P1', 'P2', 'P3', 'F1', 'P4', 'P5', 'P6', 'F2', 'P7']
2
3
```

`7` request trả phí, `2` request miễn phí, `gioi_han_uu_tien = 3`: hệ thống
phục vụ `P1, P2, P3` LIÊN TIẾP (đúng `3` lần ưu tiên), rồi bộ đếm chạm giới
hạn — request TIẾP THEO PHẢI LÀ `F1` (dù `P4, P5, P6, P7` vẫn đang chờ), bộ
đếm RESET về `0`. Ưu tiên lại `3` lần nữa (`P4, P5, P6`), rồi tới `F2`. Cuối
cùng `P7` (không còn ai để nhường, hàng đợi miễn phí đã rỗng). Tổng
`9` request được phục vụ ĐỦ, `SO_MIEN_PHI_3 = 2` — CẢ HAI request miễn phí
được phục vụ, không cái nào bị bỏ sót. `VI_TRI_DAU_TIEN_3 = 3` (chỉ số
tính từ `0`) — request miễn phí ĐẦU TIÊN xuất hiện ở vị trí thứ `4`, không
phải sau khi CẢ `7` request trả phí đã xong — đúng bằng chứng "không bị đói
vĩnh viễn".
::::

::::predict{#doan_doi_gioi_han_uu_tien commitOnce}
Xét đúng `HANG_DOI_TRA_PHI` và `HANG_DOI_MIEN_PHI` ở ví dụ trên. Với
`gioi_han_uu_tien = 3`, request miễn phí đầu tiên xuất hiện ở vị trí
`VI_TRI_DAU_TIEN_3 = 3` và `SO_MIEN_PHI_3 = 2`.

**Trước khi chạy thử**, bạn đoán: nếu đổi `gioi_han_uu_tien` từ `3` XUỐNG
`1` (ưu tiên trả phí tối đa MỘT lần liên tiếp trước khi phải nhường), giá
trị MỚI của vị trí đầu tiên VÀ của `so_mien_phi_da_phuc_vu` LÀ gì?

:::opt{correct}
Vị trí đầu tiên đổi thành `1` (chờ NGẮN hơn — chỉ sau `P1`, chưa cần tới
`P2` hay `P3`), nhưng `so_mien_phi_da_phuc_vu` VẪN LÀ `2` — CẢ hai request
miễn phí vẫn được phục vụ đầy đủ, hàm chạy tới khi cả hai hàng đợi rỗng nên
không có request nào bị "bỏ sót" bất kể `gioi_han_uu_tien` là bao nhiêu
:::

:::opt
Cả hai đều đổi: vị trí đầu tiên thành `1`, và `so_mien_phi_da_phuc_vu` cũng
giảm xuống `1` — vì giới hạn ưu tiên thấp hơn nghĩa là hệ thống "thiên vị"
trả phí ít hơn, nên phục vụ được ÍT request miễn phí hơn
::why
Gần đúng ở việc vị trí đầu tiên THẬT SỰ đổi thành `1` — quan sát đó đúng.

Chỗ lệch: `xu_ly_toan_bo_hai_hang_doi` chạy CHO TỚI KHI cả hai hàng đợi
rỗng hoàn toàn (`while hang_doi_tra_phi or hang_doi_mien_phi`) — nó không
dừng giữa chừng. Giảm `gioi_han_uu_tien` chỉ đổi THỨ TỰ xen kẽ (miễn phí
được phục vụ SỚM hơn), không đổi TỔNG SỐ request miễn phí cuối cùng được
phục vụ — cả `2` request miễn phí vẫn được xử lý trước khi hàm kết thúc.
::
:::

:::opt
Không đổi gì cả — `gioi_han_uu_tien` chỉ là một con số trang trí, hành vi
thật của hàng đợi phụ thuộc vào THỨ TỰ request được thêm vào, không phụ
thuộc tham số này
::why
Gần đúng ở việc THỨ TỰ request trong `HANG_DOI_TRA_PHI`/`HANG_DOI_MIEN_PHI`
THẬT SỰ không đổi — quan sát đó đúng.

Chỗ lệch: `gioi_han_uu_tien` là điều kiện TRỰC TIẾP trong
`lay_request_tiep_theo` (`so_lan_da_uu_tien_lien_tiep < gioi_han_uu_tien`)
— nó quyết định CHÍNH XÁC sau bao nhiêu lần ưu tiên liên tiếp thì phải
nhường. Đổi tham số này đổi TRỰC TIẾP vị trí mà request miễn phí đầu tiên
xuất hiện, dù không đổi tổng số request miễn phí được phục vụ.
::
:::
::::

::::code{#viet_lay_request_va_xu_ly_hai_hang_doi}
Hoàn thiện `lay_request_tiep_theo` (điều kiện QUYẾT ĐỊNH có ưu tiên hàng
đợi trả phí tiếp hay không) và `xu_ly_toan_bo_hai_hang_doi` (đếm SỐ request
miễn phí đã thật sự được phục vụ, sau khi cả hai hàng đợi đã rỗng).

```python title=starter
def lay_request_tiep_theo(hang_doi_tra_phi, hang_doi_mien_phi, so_lan_da_uu_tien_lien_tiep, gioi_han_uu_tien):
    if ___:                                                   # hang_doi_tra_phi and so_lan_da_uu_tien_lien_tiep < gioi_han_uu_tien
        request = hang_doi_tra_phi.pop(0)
        return request, True, so_lan_da_uu_tien_lien_tiep + 1
    if hang_doi_mien_phi:
        request = hang_doi_mien_phi.pop(0)
        return request, False, 0
    if hang_doi_tra_phi:
        request = hang_doi_tra_phi.pop(0)
        return request, True, so_lan_da_uu_tien_lien_tiep + 1
    return None, None, so_lan_da_uu_tien_lien_tiep


def xu_ly_toan_bo_hai_hang_doi(hang_doi_tra_phi_ban_dau, hang_doi_mien_phi_ban_dau, gioi_han_uu_tien):
    hang_doi_tra_phi = list(hang_doi_tra_phi_ban_dau)
    hang_doi_mien_phi = list(hang_doi_mien_phi_ban_dau)
    thu_tu_phuc_vu = []
    so_lan_da_uu_tien_lien_tiep = 0
    while hang_doi_tra_phi or hang_doi_mien_phi:
        request, la_tra_phi, so_lan_da_uu_tien_lien_tiep = lay_request_tiep_theo(
            hang_doi_tra_phi, hang_doi_mien_phi, so_lan_da_uu_tien_lien_tiep, gioi_han_uu_tien
        )
        thu_tu_phuc_vu.append((request, la_tra_phi))
    so_mien_phi_da_phuc_vu = ___                                # sum(1 for _, la_tra_phi in thu_tu_phuc_vu if la_tra_phi is False)
    return thu_tu_phuc_vu, so_mien_phi_da_phuc_vu


HANG_DOI_TRA_PHI = ["P1", "P2", "P3", "P4", "P5", "P6", "P7"]
HANG_DOI_MIEN_PHI = ["F1", "F2"]

THU_TU_3, SO_MIEN_PHI_3 = xu_ly_toan_bo_hai_hang_doi(HANG_DOI_TRA_PHI, HANG_DOI_MIEN_PHI, 3)
VI_TRI_DAU_TIEN_3 = min(i for i, (_, la_tra_phi) in enumerate(THU_TU_3) if la_tra_phi is False)

print([r for r, _ in THU_TU_3])
print(SO_MIEN_PHI_3)
print(VI_TRI_DAU_TIEN_3)
```

```python title=solution
def lay_request_tiep_theo(hang_doi_tra_phi, hang_doi_mien_phi, so_lan_da_uu_tien_lien_tiep, gioi_han_uu_tien):
    if hang_doi_tra_phi and so_lan_da_uu_tien_lien_tiep < gioi_han_uu_tien:
        request = hang_doi_tra_phi.pop(0)
        return request, True, so_lan_da_uu_tien_lien_tiep + 1
    if hang_doi_mien_phi:
        request = hang_doi_mien_phi.pop(0)
        return request, False, 0
    if hang_doi_tra_phi:
        request = hang_doi_tra_phi.pop(0)
        return request, True, so_lan_da_uu_tien_lien_tiep + 1
    return None, None, so_lan_da_uu_tien_lien_tiep


def xu_ly_toan_bo_hai_hang_doi(hang_doi_tra_phi_ban_dau, hang_doi_mien_phi_ban_dau, gioi_han_uu_tien):
    hang_doi_tra_phi = list(hang_doi_tra_phi_ban_dau)
    hang_doi_mien_phi = list(hang_doi_mien_phi_ban_dau)
    thu_tu_phuc_vu = []
    so_lan_da_uu_tien_lien_tiep = 0
    while hang_doi_tra_phi or hang_doi_mien_phi:
        request, la_tra_phi, so_lan_da_uu_tien_lien_tiep = lay_request_tiep_theo(
            hang_doi_tra_phi, hang_doi_mien_phi, so_lan_da_uu_tien_lien_tiep, gioi_han_uu_tien
        )
        thu_tu_phuc_vu.append((request, la_tra_phi))
    so_mien_phi_da_phuc_vu = sum(1 for _, la_tra_phi in thu_tu_phuc_vu if la_tra_phi is False)
    return thu_tu_phuc_vu, so_mien_phi_da_phuc_vu


HANG_DOI_TRA_PHI = ["P1", "P2", "P3", "P4", "P5", "P6", "P7"]
HANG_DOI_MIEN_PHI = ["F1", "F2"]

THU_TU_3, SO_MIEN_PHI_3 = xu_ly_toan_bo_hai_hang_doi(HANG_DOI_TRA_PHI, HANG_DOI_MIEN_PHI, 3)
VI_TRI_DAU_TIEN_3 = min(i for i, (_, la_tra_phi) in enumerate(THU_TU_3) if la_tra_phi is False)

print([r for r, _ in THU_TU_3])
print(SO_MIEN_PHI_3)
print(VI_TRI_DAU_TIEN_3)
```

```python title=test
assert [r for r, _ in THU_TU_3] == ["P1", "P2", "P3", "F1", "P4", "P5", "P6", "F2", "P7"], f"THU_TU_3 sai -- dang ra {[r for r, _ in THU_TU_3]}"
assert SO_MIEN_PHI_3 == 2, f"SO_MIEN_PHI_3 phai la 2 (ca hai request mien phi deu duoc phuc vu) -- dang ra {SO_MIEN_PHI_3}"
assert VI_TRI_DAU_TIEN_3 == 3, f"VI_TRI_DAU_TIEN_3 phai la 3 -- dang ra {VI_TRI_DAU_TIEN_3}"

# bien: gioi_han_uu_tien THAT SU rang buoc THU TU, nhung KHONG rang buoc TONG SO mien phi
THU_TU_1, SO_MIEN_PHI_1 = xu_ly_toan_bo_hai_hang_doi(HANG_DOI_TRA_PHI, HANG_DOI_MIEN_PHI, 1)
VI_TRI_DAU_TIEN_1 = min(i for i, (_, la_tra_phi) in enumerate(THU_TU_1) if la_tra_phi is False)
assert VI_TRI_DAU_TIEN_1 == 1, f"gioi_han_uu_tien=1 phai cho vi tri dau tien cua mien phi la 1 -- dang ra {VI_TRI_DAU_TIEN_1}"
assert SO_MIEN_PHI_1 == 2, f"du doi gioi_han_uu_tien, TONG SO mien phi van phai la 2 -- dang ra {SO_MIEN_PHI_1}"
assert VI_TRI_DAU_TIEN_1 != VI_TRI_DAU_TIEN_3, "doi gioi_han_uu_tien phai doi VI TRI request mien phi dau tien xuat hien"

# kiem tra truc tiep lay_request_tiep_theo -- ca hai hang doi rong
assert lay_request_tiep_theo([], [], 0, 3) == (None, None, 0), "ca hai hang doi rong phai tra ve (None, None, so_lan cu)"

# kiem tra truc tiep -- tra_phi rong, mien_phi con request -> lay mien phi, RESET bo dem
assert lay_request_tiep_theo([], ["F1"], 5, 3) == ("F1", False, 0), "tra_phi rong phai lay mien_phi va RESET bo dem ve 0"

# kiem tra truc tiep -- da uu tien DU gioi han (5 >= 3) NHUNG mien_phi rong -> danh phai lay tra_phi, bo dem VAN tang
assert lay_request_tiep_theo(["P1"], [], 5, 3) == ("P1", True, 6), "mien_phi rong thi du da uu tien du gioi han van phai lay tra_phi -- dang ra {}".format(lay_request_tiep_theo(["P1"], [], 5, 3))

# kiem tra truc tiep -- con trong gioi han (0 < 3) -> uu tien tra_phi, tang bo dem
assert lay_request_tiep_theo(["P1", "P2"], ["F1"], 0, 3) == ("P1", True, 1), "con trong gioi han thi phai uu tien tra_phi"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai hàm khác nhau. Chỗ đầu (trong `lay_request_tiep_theo`) là ĐIỀU KIỆN của `if` ĐẦU TIÊN — hai vế nối bằng `and`, quyết định có được ưu tiên hàng đợi trả phí LẦN NÀY không. Chỗ hai (trong `xu_ly_toan_bo_hai_hang_doi`) là GIÁ TRỊ gán cho `so_mien_phi_da_phuc_vu`, SAU KHI vòng `while` đã chạy xong — đếm số lần `la_tra_phi is False` trong `thu_tu_phuc_vu`.
- kind: strategy
  body: 'Chỗ đầu: `hang_doi_tra_phi and so_lan_da_uu_tien_lien_tiep < gioi_han_uu_tien` — CẢ hai điều kiện phải đúng: còn request trả phí, VÀ chưa ưu tiên đủ số lần liên tiếp cho phép. Chỗ hai: `sum(1 for _, la_tra_phi in thu_tu_phuc_vu if la_tra_phi is False)` — duyệt TOÀN BỘ lịch sử phục vụ, đếm đúng những lần phục vụ MIỄN PHÍ.'
- kind: one-line
  body: 'Chỗ đầu là `hang_doi_tra_phi and so_lan_da_uu_tien_lien_tiep < gioi_han_uu_tien`, chỗ hai là `sum(1 for _, la_tra_phi in thu_tu_phuc_vu if la_tra_phi is False)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai la DIEU KIEN hang_doi_tra_phi and so_lan_da_uu_tien_lien_tiep < gioi_han_uu_tien (noi bang and, khong duoc dung or hay chi mot ve); cho trong hai phai GOI sum(...) tren thu_tu_phuc_vu (khong duoc chep san mot con so co dinh hay dem bang vong lap thuong)
  requireAst:
  - kind: uses-operator, target: "and", min: 1
  - kind: uses-call, target: sum, min: 1
  # Da CHAY THAT qua kiemAst() (node + pyodide, dist build that, trich
  # CHINH XAC khoi solution cua file nay) -- xac nhan DUNG CHINH XAC (min
  # VA min+1): "and"=1, sum=1.
  # "and"=1: DUY NHAT o cho trong dau -- vong while ngoai cung dung "or"
  # (hang_doi_tra_phi or hang_doi_mien_phi), khong phai "and", nen khong bi
  # dem trung.
  # sum=1: DUY NHAT o cho trong hai -- khong noi nao khac trong solution
  # goi ham sum.
  # Dien bua "True" vao ca hai cho trong ("if True" va "so_mien_phi_da_phuc_vu
  # = True") cho "and"=0 VA sum=0 -- CA HAI luat CHAN DUNG (da CHAY THAT xac
  # nhan qua kiemAst). Rieng "if True:" con bi chan THEM boi tier 'run': no
  # goi hang_doi_tra_phi.pop(0) MOI lan du hang_doi_tra_phi da rong, nem
  # IndexError "pop from empty list" -- da tu chay THAT xac nhan.
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant hoan doi CA HAI cho
  # trong -- xac dinh ranh gioi TU CHINH khoi starter -- va CHAY THAT qua
  # kiemAst() THAT VA python3 THAT): dien "sum(1 for _, la_tra_phi in
  # thu_tu_phuc_vu if la_tra_phi is False)" vao cho trong dau ("if sum(1
  # for _, la_tra_phi in thu_tu_phuc_vu if la_tra_phi is False):" trong
  # lay_request_tiep_theo) VA dien "hang_doi_tra_phi and
  # so_lan_da_uu_tien_lien_tiep < gioi_han_uu_tien" vao cho trong hai
  # ("so_mien_phi_da_phuc_vu = hang_doi_tra_phi and
  # so_lan_da_uu_tien_lien_tiep < gioi_han_uu_tien" trong
  # xu_ly_toan_bo_hai_hang_doi) -- da CHAY THAT qua kiemAst(): tong so lan
  # "and" VA tong so lan goi sum tren TOAN BO solution DEU KHONG DOI (van
  # dung 1 va 1, chi doi VI TRI) -- static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': ben trong lay_request_tiep_theo (tham
  # so la hang_doi_tra_phi, hang_doi_mien_phi, so_lan_da_uu_tien_lien_tiep,
  # gioi_han_uu_tien -- KHONG CO "thu_tu_phuc_vu" nao trong scope nay), bieu
  # thuc moi doc ten "thu_tu_phuc_vu" CHUA TON TAI -- da tu chay THAT qua
  # python3, xac nhan NameError "name 'thu_tu_phuc_vu' is not defined" ngay
  # khi lay_request_tiep_theo duoc goi lan dau (tu vong while cua
  # xu_ly_toan_bo_hai_hang_doi). Loi nem RA TRUOC KHI kip chay toi dong
  # so_mien_phi_da_phuc_vu = ... o ben kia, nen ca hai nhanh deu bi chan boi
  # CUNG mot loi runtime nay, doc lap voi static.
  # Da tu ra soat GOTCHA #6: "thu_tu_phuc_vu" chi ton tai trong pham vi
  # xu_ly_toan_bo_hai_hang_doi, khong trung ten voi tham so nao cua
  # lay_request_tiep_theo -- khong co rui ro nham lan tinh co ve HINH DANG.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\['P1', 'P2', 'P3', 'F1', 'P4', 'P5', 'P6', 'F2', 'P7'\\]\\n2\\n3\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`F1` chen vào ngay sau `P3`, không phải chờ tới khi cả `7` request trả phí
xong. Cả hai request miễn phí đều được phục vụ — `SO_MIEN_PHI_3 = 2`. Bài
tiếp theo: khi hệ thống THẬT SỰ gặp sự cố (GPU hết bộ nhớ, chi phí tăng bất
thường...), làm sao TRA CỨU ngay ra cách xử lý?
::::

::::reflect{#nghi-lai}
`lay_request_tiep_theo` không hề "công bằng" theo nghĩa chia đều — nó vẫn
THIÊN VỊ trả phí rõ ràng (`gioi_han_uu_tien = 3` nghĩa là trả phí được ưu
tiên gấp nhiều lần miễn phí). Điều nó bảo đảm KHÔNG PHẢI công bằng, mà là
một CẬN TRÊN cho thời gian chờ: dù tải trả phí cao tới đâu, một request
miễn phí không bao giờ chờ QUÁ `gioi_han_uu_tien` request trả phí liên
tiếp. Đo được cụ thể trên `9` request: đổi `gioi_han_uu_tien` từ `3` xuống
`1` đổi VỊ TRÍ mà request miễn phí đầu tiên xuất hiện (từ `3` xuống `1`),
nhưng KHÔNG đổi tổng số request miễn phí cuối cùng được phục vụ (`2` cả
hai lần) — vì hàm chạy tới khi MỌI request (cả hai hàng đợi) đều xong.
Ba bài đầu của `q8.6d` đã mô phỏng: sự kiện tác tử (bài `1`), semantic
cache (bài `2`), hàng đợi ưu tiên (bài này). Bài tiếp theo — bài `4` — mô
phỏng mảnh CUỐI của `Chương 39`: bảng tra cứu sự cố, để khi một trong ba
cơ chế trên "báo động", hệ thống biết NGAY phải làm gì.
::::

::::checkpoint{mastery=0.8}
::::
