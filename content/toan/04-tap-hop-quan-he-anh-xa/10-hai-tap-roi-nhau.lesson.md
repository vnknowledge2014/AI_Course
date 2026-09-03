---
id: toan.tap-hop-quan-he-anh-xa.hai-tap-roi-nhau
title: Hai tập hợp rời nhau
summary: "A ∩ B = ∅ — hai tập KHÔNG chung phần tử nào gọi là rời nhau. Một tập hợp CHỈ rời nhau với chính nó khi nó LÀ tập rỗng — chung với chính mình luôn CHỨA chính mình, trừ khi không có gì để chứa."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 10
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.set-disjoint]
requires: [math.set-intersection, logic.not]
concepts: [math.roi-nhau]
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
Luống 4 chỉ có bí đỏ, luống 6 chỉ có khoai lang. `∩` giữa chúng ra `∅`
— quan hệ ĐÓ có tên riêng không?
::::

::::explain{#roi-nhau-la-gi}
Có. **Rời nhau** — `A ∩ B = ∅` — hai tập hợp KHÔNG chung PHẦN TỬ nào
gọi LÀ rời nhau:

```python title=readonly
luong_4 = {"bí đỏ"}
luong_6 = {"khoai lang"}

print((luong_4 & luong_6) == set())
```

```text title=readonly
True
```

`luống_4` VÀ `luống_6` rời nhau — bí đỏ VÀ khoai lang chẳng chung LOẠI
rau nào. "Rời nhau" KHÔNG PHẢI "không liên quan gì" — chúng CÙNG nằm
trong MỘT vườn, CÙNG được Byte chăm — chỉ riêng LOẠI RAU thì KHÔNG có
gì trùng.
::::

::::example{#khong-phai-cap-nao-cung-roi}
Không PHẢI cặp luống NÀO cũng rời nhau — CÒN TUỲ chúng có chung LOẠI
rau nào không:

```python title=readonly
luong_1 = {"cà chua", "xà lách"}
luong_4 = {"bí đỏ"}
luong_3 = {"xà lách", "cà rốt"}

print((luong_1 & luong_4) == set())
print((luong_1 & luong_3) == set())
```

```text title=readonly
True
False
```

`luống_1` VÀ `luống_4` rời nhau. `luống_1` VÀ `luống_3` KHÔNG rời nhau
— cả hai CÙNG trồng `xà lách`.
::::

::::predict{#doan-tap-hop-roi-voi-chinh-no commitOnce}
Byte thử một câu hỏi lạ: MỘT tập hợp có "rời nhau" VỚI CHÍNH NÓ không?

```python
luong_rong = set()
luong_1 = {"cà chua", "xà lách"}

print((luong_rong & luong_rong) == set())
print((luong_1 & luong_1) == set())
```

Hai dòng cuối in ra gì?

:::opt{correct}
`True`, rồi `False`
:::

:::opt
`False`, rồi `False` — vì MỘT tập hợp GIAO với CHÍNH NÓ luôn cho RA
lại chính nó, VÀ tập hợp nào cũng có Ý NGHĨA riêng của nó (khác `∅`),
nên phép giao CỦA nó với chính mình KHÔNG BAO GIỜ rỗng, dù nó rỗng hay
không
::why
Gần đúng ở việc bạn suy luận đúng "MỘT tập hợp giao với CHÍNH NÓ luôn
RA lại chính nó" (`A ∩ A = A`, một quy luật ĐÚNG) — quan sát ĐÓ chính
xác.

Chỗ lệch: bạn dừng lại TRƯỚC bước cuối — `A ∩ A = A`, nhưng câu HỎI
LÀ "`A ∩ A` có BẰNG `∅` không", tức LÀ hỏi "chính `A` có BẰNG `∅`
không". `luống_rong` chính LÀ `∅` — nên `luống_rong ∩ luống_rong` LÀ
`luống_rong`, MÀ `luống_rong` LÀ `∅`, nên câu hỏi ĐÚNG. Còn `luống_1`
KHÔNG PHẢI `∅` (có hai loại rau) — nên `luống_1 ∩ luống_1` LÀ
`luống_1`, KHÔNG rỗng, câu hỏi SAI.
::
:::

:::opt
Máy báo lỗi biên dịch — `luong_rong & luong_rong` viết CÙNG một biến
Ở CẢ HAI vế của toán tử `&` là một thao tác dư thừa, Python cấm giao
một tập hợp VỚI CHÍNH NÓ
::why
Gần đúng ở việc bạn để ý `luong_rong` xuất hiện Ở CẢ HAI vế — một
quan sát đúng về CÚ PHÁP có vẻ "thừa".

Chỗ lệch: Python KHÔNG cấm điều này — `&` (VÀ mọi toán tử tập hợp
khác) hoàn toàn chấp nhận CÙNG một biến Ở cả hai vế, kết quả đơn giản
LÀ chính tập hợp ĐÓ (`A ∩ A = A`, đúng NHƯ vừa nói Ở TRÊN). Biên dịch
sạch, chạy sạch — KHÔNG có luật "cấm dư thừa" nào trong Python cả.
::
:::
::::

::::code{#viet_nhung_luong_roi_nhau_voi}
Viết `nhung_luong_roi_nhau_voi(muc_tieu, vuon)` — trả về danh sách tên
những luống RỜI NHAU với `muc_tieu` (KHÔNG chung loại rau nào).

```python title=starter
def nhung_luong_roi_nhau_voi(muc_tieu, vuon):
    ket_qua = []
    for ten_luong in vuon:
        if ___:
            ket_qua.append(ten_luong)
    return ket_qua


vuon = {
    "luong_1": {"cà chua", "xà lách"},
    "luong_2": {"xà lách", "cải bó xôi"},
    "luong_3": {"khoai lang"},
}

print(sorted(nhung_luong_roi_nhau_voi({"khoai lang"}, vuon)))
```

```python title=solution
def nhung_luong_roi_nhau_voi(muc_tieu, vuon):
    ket_qua = []
    for ten_luong in vuon:
        if not (muc_tieu & vuon[ten_luong]):
            ket_qua.append(ten_luong)
    return ket_qua


vuon = {
    "luong_1": {"cà chua", "xà lách"},
    "luong_2": {"xà lách", "cải bó xôi"},
    "luong_3": {"khoai lang"},
}

print(sorted(nhung_luong_roi_nhau_voi({"khoai lang"}, vuon)))
```

```python title=test
assert nhung_luong_roi_nhau_voi({"xà lách"}, vuon) == ["luong_3"], "chi luong_3 roi nhau voi xa lach"
assert sorted(nhung_luong_roi_nhau_voi(set(), vuon)) == ["luong_1", "luong_2", "luong_3"], "muc tieu rong -- roi nhau voi moi luong"
assert nhung_luong_roi_nhau_voi({"xà lách", "khoai lang"}, vuon) == [], "moi luong deu chung it nhat mot trong hai loai -- khong luong nao roi"
```

:::hints
- kind: attention
  body: "Dieu kien if phai kiem KHONG chung (dung not (...) boc quanh phep giao &)."
- kind: strategy
  body: "not (muc_tieu & vuon[ten_luong])"
- kind: one-line
  body: "___ = not (muc_tieu & vuon[ten_luong])"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: dieu kien phai KIEM KHONG CHUNG -- boc not(...) quanh phep giao & giua muc_tieu va tap hop cua luong dang xet
  requireAst:
  - kind: uses-operator, target: not, min: 1
  - kind: uses-name, target: muc_tieu, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\['luong_1', 'luong_2'\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Rời nhau = giao ra rỗng. Bài sau: giữ phần RIÊNG của một luống, bỏ
phần chung.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte muốn biết luống nào trồng cà chua mà KHÔNG trồng xà lách. `∪`
gộp, `∩` giữ chung — phép NÀO lấy đúng "có cái này, không cái kia"?
::::

::::checkpoint{mastery=0.8}
::::
