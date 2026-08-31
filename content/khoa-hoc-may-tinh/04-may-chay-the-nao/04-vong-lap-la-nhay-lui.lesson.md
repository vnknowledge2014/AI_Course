---
id: khoa-hoc-may-tinh.may-chay-the-nao.vong-lap-la-nhay-lui
title: "Vòng lặp là một lệnh NHẢY LÙI"
summary: "dis.dis() một vòng for cho thấy JUMP_BACKWARD quay đúng về nhãn L1 ở đầu vòng — chạy thật trên CPython 3.13, nhãn L1/L2 in tường minh. 'Lặp lại' không phải khái niệm ma thuật, nó CHÍNH LÀ máy nhảy ngược lại một chỗ đã đi qua."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [may.loop-is-jump]
requires: [may.eval-stack]
concepts: [may.loop-is-jump]
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
Nó nhảy lùi — về đúng chỗ đã đi qua. Không có gì thần bí hơn thế.
::::

::::explain{#lap-lai-la-nhay-nguoc}
Mọi lệnh bài 1-3 đã học đều đi TỚI: lấy giá trị, tính, đẩy kết quả, rồi
sang lệnh kế tiếp. Không lệnh nào quay lại chỗ cũ.

`for` thì khác — nó chạy đi chạy lại đúng một khối lệnh nhiều lần. Chữ
"lặp lại" nghe như một khả năng riêng, đặc biệt của từ khoá `for`. Nó
không phải vậy. `dis.dis()` cho thấy đúng cơ chế: có một lệnh tên
`JUMP_BACKWARD` — nhảy lùi. Vị trí đang chạy trong dãy lệnh bị đưa ngược
về một chỗ ĐÃ đi qua rồi, và chạy tiếp từ đó — y hệt bạn tua lại một
đoạn băng về đúng một điểm đã xem.

CPython 3.13 còn in kèm một chỉ dẫn tiện lợi: những chỗ có thể là đích
nhảy tới được đánh dấu bằng nhãn — `L1:`, `L2:` — và mỗi lệnh nhảy ghi
rõ "to L1" ngay trong ngoặc. Đích nhảy hiện ra bằng chữ, thay vì một con
số offset thô bạn phải tự dò ngược lại trong danh sách lệnh.
::::

::::example{#dem-toi-n}
```python title=readonly
import dis

def dem_toi(n):
    tong = 0
    for i in range(n):
        tong += i
    return tong

dis.dis(dem_toi)
```

```text title=readonly
  3           RESUME                   0

  4           LOAD_CONST               1 (0)
              STORE_FAST               1 (tong)

  5           LOAD_GLOBAL              1 (range + NULL)
              LOAD_FAST                0 (n)
              CALL                     1
              GET_ITER
      L1:     FOR_ITER                 7 (to L2)
              STORE_FAST               2 (i)

  6           LOAD_FAST_LOAD_FAST     18 (tong, i)
              BINARY_OP               13 (+=)
              STORE_FAST               1 (tong)
              JUMP_BACKWARD            9 (to L1)

  5   L2:     END_FOR
              POP_TOP

  7           LOAD_FAST                1 (tong)
              RETURN_VALUE
```

Nhãn `L1:` đứng ngay trước `FOR_ITER` — điểm bắt đầu của mỗi vòng lặp
(lấy phần tử kế tiếp, hoặc báo hết). Cuối khối lệnh của thân vòng,
`JUMP_BACKWARD 9 (to L1)` đưa vị trí đang chạy NHẢY LÙI đúng về `L1`,
để `FOR_ITER` được gọi lại, kiểm tra còn phần tử nào không.

Khi hết phần tử, `FOR_ITER` không nhảy lùi nữa — nó để dòng chạy rơi
xuống `L2`, nơi `END_FOR` dọn dẹp rồi hàm chạy tiếp xuống `return tong`.
Vòng lặp kết thúc không phải vì `JUMP_BACKWARD` ngừng được gọi — mà vì
`FOR_ITER` không còn gì để đưa vào thân vòng nữa.
::::

::::predict{#du-doan-nhan-doi commitOnce}
```python
def nhan_doi_moi_phan_tu(mang):
    ket_qua = []
    for x in mang:
        ket_qua.append(x * 2)
    return ket_qua
```

Bốn lệnh dưới đây đều THẬT SỰ xuất hiện trong `dis.dis(nhan_doi_moi_phan_tu)`.
Lệnh nào là lệnh đưa vị trí đang chạy NHẢY LÙI về đầu vòng lặp?

:::opt{correct}
JUMP_BACKWARD
:::

:::opt
FOR_ITER
::why
Gần đúng ở chỗ `FOR_ITER` cũng đứng ngay đầu vòng và cũng liên quan mật
thiết tới việc lặp — không sai khi nghĩ tới nó đầu tiên.

Chỗ lệch: `FOR_ITER` không NHẢY LÙI. Việc của nó là lấy phần tử kế
tiếp; khi còn phần tử, nó để dòng chạy RƠI XUỐNG (đi tới) thân vòng —
không nhảy đi đâu cả. Khi hết phần tử, nó mới nhảy, nhưng nhảy TỚI
(forward), ra khỏi vòng — ngược hướng với "nhảy lùi".
::
:::

:::opt
GET_ITER
::why
Gần đúng ở việc bạn nhận ra nó có mặt trong phần dựng vòng lặp — đúng,
nó có.

Chỗ lệch: `GET_ITER` chỉ chạy đúng MỘT LẦN, ngay lúc vào vòng — nó lấy
ra "bộ đếm vị trí duyệt" từ `mang`, chuẩn bị cho `FOR_ITER` dùng. Nó
không lặp lại ở mỗi lượt, nên không thể là lệnh "nhảy lùi mỗi vòng".
::
:::

:::opt
STORE_FAST
::why
Gần đúng ở việc bạn nhận ra nó CHẠY LẠI ở mỗi lượt lặp thật — đúng,
`STORE_FAST` (cất `x`) chạy mỗi vòng.

Chỗ lệch: việc nó làm là CẤT một giá trị vào một cái tên — không di
chuyển vị trí đang chạy đi đâu cả. Chạy lại mỗi vòng không đồng nghĩa
với việc là lệnh nhảy.
::
:::
::::

::::code{#cong-don-mang}
Hàm dưới đây cộng dồn từng phần tử của `mang` vào `tong`. Vòng lặp đã
viết sẵn — việc của bạn chỉ là điền đúng phép cộng dồn.

```python title=starter
import dis

def cong_don(mang):
    tong = 0
    for so in mang:
        tong = ___                     # cộng dồn so vào tong
    return tong

ket_qua = cong_don([10, 20, 30])
print(ket_qua)
dis.dis(cong_don)
```

```python title=solution
import dis

def cong_don(mang):
    tong = 0
    for so in mang:
        tong = tong + so
    return tong

ket_qua = cong_don([10, 20, 30])
print(ket_qua)
dis.dis(cong_don)
```

```python title=test
assert ket_qua == 60, f"10 + 20 + 30 phải ra 60 — đang ra {ket_qua}"
assert cong_don([1, 2, 3, 4]) == 10, f"1+2+3+4 phải ra 10 — đang ra {cong_don([1, 2, 3, 4])}"
```

:::hints
- kind: attention
  body: Vòng lặp đã có sẵn — for so in mang chạy đúng số lần bằng độ dài mang. Việc của bạn chỉ là cộng dồn so vào tong ở mỗi lượt, không phải viết lại vòng lặp.
- kind: strategy
  body: Lấy giá trị tong đang có, cộng thêm so của lượt này, gán ngược lại tong — đúng lối cộng dồn đã quen.
- kind: one-line
  body: 'Chỗ trống là: tong + so'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải cộng dồn — dùng cả tong lẫn so, không gõ cứng kết quả và không thay đổi cấu trúc vòng lặp
  requireAst:
  # Đã thử lời giải khác `so + tong` (đổi thứ tự, phép cộng giao hoán) —
  # vẫn qua cả static lẫn tests. Đúng luật 2.
  #
  # ĐÃ THỬ BA CÁCH ĐIỀN HỤT (True/1/0) trên khối này (khối có `for`, vòng
  # lặp KHÔNG bị blank hoá — chỉ thân vòng bị blank): cả ba đều DỪNG AN
  # TOÀN (mang cố định 3 phần tử, không phụ thuộc chỗ trống), và cả ba ra
  # kết quả sai (True/1/0), khác hẳn 60 — trượt tests. Không ca nào chạy
  # vô thời hạn.
  - kind: uses-name, target: tong, min: 1
  - kind: uses-name, target: so, min: 1
  - kind: uses-operator, target: "+", min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^60\\n"
- tier: output
  expect: "JUMP_BACKWARD"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`JUMP_BACKWARD 9 (to L1)` — con số đó không phải trang trí, nó là địa
chỉ thật mà vị trí đang chạy quay về, đúng ba lần rồi dừng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nhảy LÙI để lặp lại — bạn vừa thấy. Nhưng `if`/`else` không lặp lại gì
cả — nó CHỌN một trong hai nhánh rồi đi tiếp, không bao giờ quay đầu.
Nó có dùng lệnh nhảy không? Nếu có, nhảy kiểu gì, và máy dựa vào đâu để
biết chọn nhánh nào?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
