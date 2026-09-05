---
id: tri-tue-nhan-tao.dong-co-tensor.vi-sao-can-tensor
title: "Vì sao cần Tensor: Value quá chậm cho ma trận"
summary: "Value (T8.2) bọc MỘT số — một bảng embedding 50 từ vựng × 16 chiều cần 50 × 16 = 800 Value riêng biệt, đếm THẬT bằng vòng lặp lồng ba tầng. Một phép nhân ma trận (12,16)@(16,8) như trong attention cần 12 × 8 × 16 = 1.536 lần nhân vô hướng nếu làm từng Value một. Tensor (bài sau) bọc CẢ MỘT mảng numpy trong một object, giữ nguyên ý tưởng _prev/_backward của Value."
locale: vi
track: tri-tue-nhan-tao
module: dong-co-tensor
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 9
teaches: [ai.vi-sao-can-tensor]
requires: [ai.boss-token-hoa-bpe]
concepts: [ai.vi-sao-can-tensor]
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
Token đã thành số (quest trước). Nhưng biến MỖI token thành một vector nghĩa
— cho CẢ một từ vựng cùng lúc — có còn dùng nổi cách đếm-object của `Value`
không?
::::

::::explain{#value_qua_cham}
`Value` (T8.2) bọc đúng MỘT số vô hướng: `self.data` là một `float`, và mỗi
phép toán (`+`, `*`, `tanh`...) tạo ra một `Value` MỚI, riêng biệt. Cách này
chạy tốt cho một neuron hay một MLP nhỏ — vài chục `Value` cho cả mạng.

Nhưng token hoá xong, bước kế tiếp của một mô hình ngôn ngữ là **embedding**:
mỗi token ID cần một vector số thực đại diện cho "nghĩa" của nó — và một
**bảng embedding** phải giữ một vector RIÊNG cho MỌI token trong từ vựng
cùng lúc. Với từ vựng `50` token, mỗi vector `16` chiều, bảng đó cần
`50 × 16` số thực. Nếu giữ ĐÚNG cách `Value` đã làm — mỗi số một object
riêng — bảng embedding nhỏ bé này đã cần tới `800` object `Value`, TRƯỚC KHI
một phép tính nào chạy.

Bước sau embedding còn tốn hơn: **attention** (cơ chế trung tâm của
Transformer, quest sau sẽ cài) so sánh MỌI cặp vị trí trong một câu bằng
phép nhân ma trận `Q · Kᵀ`. Nhân hai ma trận hình dạng `(m, k)` và `(k, n)`
cần đúng `m × n × k` phép nhân vô hướng (mỗi trong `m × n` phần tử của kết
quả là tổng của `k` số hạng, mỗi số hạng một phép nhân). Với `12` token, mỗi
token chiếu qua một phép biến đổi `16` chiều xuống còn `8` chiều — hình dạng
`(12, 16)` nhân `(16, 8)` — con số đó đã là `12 × 8 × 16 = 1.536` phép nhân
VÔ HƯỚNG, nếu mỗi phép nhân là một lời gọi `Value.__mul__` tạo ra một object
`Value` mới.

`Tensor` (bài sau) giải quyết đúng vấn đề này: bọc CẢ MỘT mảng `numpy`
(`self.data` là toàn bộ ma trận, không phải một số) trong MỘT object duy
nhất. Một phép `Tensor.__mul__` gọi `numpy` tính hết `800` số (hay `1.536`
phép nhân) trong MỘT lời gọi, không tạo ra hàng trăm object Python riêng lẻ.
Ý tưởng `_prev` (node cha), `_backward` (closure lan gradient cục bộ), và
`.grad` (đạo hàm tích luỹ) của `Value` được GIỮ NGUYÊN — chỉ đổi kiểu dữ liệu
bên trong từ một số sang một mảng.
::::

::::example{#dem_that_so_value_can}
Đếm THẬT (không suy luận suông) số `Value` cần cho một bảng embedding
`50 × 16`, và số phép nhân vô hướng cần cho một phép nhân ma trận
`(12, 16) @ (16, 8)`:

```python title=readonly
class Value:
    def __init__(self, data):
        self.data = data
        self.grad = 0.0


vocab, dim = 50, 16
bang_embedding = [[Value(0.0) for _ in range(dim)] for _ in range(vocab)]
so_value_embedding = sum(len(hang) for hang in bang_embedding)

m, k, n = 12, 16, 8
so_phep_nhan = 0
for i in range(m):
    for j in range(n):
        for t in range(k):
            so_phep_nhan += 1

print(so_value_embedding)
print(so_phep_nhan)
```

```text title=readonly
800
1536
```

`bang_embedding` thật sự TẠO ra `50 × 16 = 800` object `Value` — đếm bằng
`len()` trên chính cấu trúc đã dựng, không phải nhân tay rồi tin. Vòng lặp ba
tầng (`i`, `j`, `t`) mô phỏng ĐÚNG số lần một phép nhân vô hướng phải chạy để
tính xong phép nhân ma trận `(12, 16) @ (16, 8)` theo định nghĩa (mỗi phần tử
kết quả là tổng `16` số hạng) — đếm ra `1536`, khớp với `12 × 8 × 16`. Với
`Tensor`, TOÀN BỘ con số này gói trong một lời gọi `numpy` duy nhất — không
`800` object Python, không `1536` lần gọi `__mul__`.
::::

::::predict{#doan_vocab_tang_gap_doi commitOnce}
Bảng embedding ở trên (`vocab=50, dim=16`) cần `800` `Value` nếu vẫn giữ
cách MỖI SỐ MỘT OBJECT.

**Trước khi tính lại**, bạn đoán: nếu từ vựng tăng GẤP ĐÔI lên `vocab=100`
(giữ nguyên `dim=16`), số `Value` cần cho bảng embedding sẽ là bao nhiêu?

:::opt{correct}
`1600` — số `Value` cần luôn bằng `vocab × dim`; tăng `vocab` gấp đôi (giữ
`dim` cố định) làm tích đó tăng gấp đôi theo đúng tỉ lệ, từ `800` lên `1600`
:::

:::opt
Vẫn `800` — vì mỗi token chỉ cần một vector RIÊNG, số `Value` không phụ
thuộc vào việc từ vựng có bao nhiêu token
::why
Gần đúng ở việc mỗi token đúng là chỉ cần MỘT vector riêng — quan sát đó
không sai.

Chỗ lệch: "một vector riêng" không có nghĩa là "không tốn thêm object". Mỗi
vector đó vẫn gồm `dim=16` số thực, mỗi số một `Value`. Thêm MỘT token mới
vào từ vựng là thêm `16` `Value` mới (một vector đầy đủ) — không phải thêm
`0`. Gấp đôi số token thì tổng số `Value` cũng gấp đôi.
::
:::

:::opt
`3200` — vì tăng vocab gấp đôi làm cả số hàng LẪN số cột của bảng đều tăng,
nên tích tăng gấp bốn
::why
Gần đúng ở việc nghĩ tới một phép tăng "theo cả hai chiều" — đúng kiểu suy
luận khi CẢ HAI kích thước của một ma trận cùng tăng (như trường hợp nhân ma
trận ở ví dụ trên, nơi tăng cả `m` và `n` sẽ nhân bốn tích `m × n`).

Chỗ lệch: bảng embedding chỉ có MỘT kích thước thay đổi ở đây — `vocab`
(số hàng) tăng gấp đôi, còn `dim` (số cột, `16`) giữ NGUYÊN. Tích `vocab ×
dim` chỉ tăng theo tỉ lệ của phần đã đổi — gấp đôi, không phải gấp bốn.
::
:::
::::

::::code{#viet_dem_scalar}
Hoàn thiện hai hàm đếm: `so_scalar_embedding` (số `Value` cho một bảng
embedding) và `so_scalar_nhan_ma_tran` (số phép nhân vô hướng cho một phép
nhân ma trận).

```python title=starter
def so_scalar_embedding(vocab, dim):
    return ___                          # vocab * dim


def so_scalar_nhan_ma_tran(m, n, k):
    return ___                          # m * n * k


so_value_embedding = so_scalar_embedding(50, 16)
so_phep_nhan = so_scalar_nhan_ma_tran(12, 8, 16)

print(so_value_embedding)
print(so_phep_nhan)
```

```python title=solution
def so_scalar_embedding(vocab, dim):
    return vocab * dim


def so_scalar_nhan_ma_tran(m, n, k):
    return m * n * k


so_value_embedding = so_scalar_embedding(50, 16)
so_phep_nhan = so_scalar_nhan_ma_tran(12, 8, 16)

print(so_value_embedding)
print(so_phep_nhan)
```

```python title=test
assert so_value_embedding == 800, f"so_value_embedding sai -- dang ra {so_value_embedding}"
assert so_phep_nhan == 1536, f"so_phep_nhan sai -- dang ra {so_phep_nhan}"

# rieng kiem tra CONG THUC TONG QUAT tren nhung kich thuoc KHAC, chan cheat
# chep san 800/1536 lam hang so co dinh.
assert so_scalar_embedding(10, 4) == 40, f"so_scalar_embedding(10,4) sai -- dang ra {so_scalar_embedding(10, 4)}"
assert so_scalar_embedding(1, 1) == 1, f"so_scalar_embedding(1,1) sai -- dang ra {so_scalar_embedding(1, 1)}"
assert so_scalar_nhan_ma_tran(2, 3, 4) == 24, f"so_scalar_nhan_ma_tran(2,3,4) sai -- dang ra {so_scalar_nhan_ma_tran(2, 3, 4)}"
assert so_scalar_nhan_ma_tran(5, 1, 1) == 5, f"so_scalar_nhan_ma_tran(5,1,1) sai -- dang ra {so_scalar_nhan_ma_tran(5, 1, 1)}"
```

:::hints
- kind: attention
  body: Hai chỗ trống, cả hai đều là MỘT phép nhân. `so_scalar_embedding`: số `Value` cần cho bảng embedding là `vocab` nhân `dim` (mỗi token một vector `dim` chiều). `so_scalar_nhan_ma_tran`: số phép nhân vô hướng cho một phép nhân ma trận `(m,k)@(k,n)` là `m` nhân `n` nhân `k` (mỗi trong `m×n` phần tử kết quả tốn `k` phép nhân).
- kind: strategy
  body: 'so_scalar_embedding: `vocab * dim`. so_scalar_nhan_ma_tran: `m * n * k`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `vocab * dim` và `m * n * k`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: so_scalar_embedding phai tra ve THAT vocab*dim (mot phep nhan); so_scalar_nhan_ma_tran phai tra ve THAT m*n*k (hai phep nhan) -- khong duoc chep san hang so 800/1536 hay dung phep cong thay phep nhan
  requireAst:
  - kind: uses-operator, target: "*", min: 3
  - kind: uses-name, target: vocab, min: 1
  - kind: uses-name, target: dim, min: 1
  - kind: uses-name, target: k, min: 1
  forbidAst:
  - kind: has-literal, target: "800"
  - kind: has-literal, target: "1536"
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca bon requireAst qua sach ("*"=3: vocab*dim trong ham
  # dau, m*n roi (m*n)*k trong ham hai -- hai dau "*" tren cung mot bieu
  # thuc; vocab=1, dim=1, k=1: moi ten doc dung mot lan trong than ham cua
  # no) VA khong vi pham forbidAst nao (khong co literal 800/1536 nao trong
  # solution, vi ca hai deu tinh THAT bang phep nhan tham so). Cheat "return
  # 800"/"return 1536" (chep san dung cho hai lenh goi cu the o duoi) lam
  # "*" tut ve duoi 3 VA vi pham ca hai forbidAst has-literal -- bi chan KEP.
  # Cheat "return vocab + dim" (doi nhan thanh cong) lam "*" tut xuong 1 --
  # bi chan, va da tu kiem chung bang Python that: (50+16)=66 khac han 800,
  # bi bat ngay boi assert gia tri.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^800\\n1536\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`800` `Value` cho một bảng embedding nhỏ, `1.536` phép nhân cho một phép
chiếu — và đó mới chỉ là MỘT phép nhân ma trận nhỏ. Bài sau: lớp `Tensor`,
bọc cả một mảng trong một object, giữ nguyên `_prev`/`_backward` đã học.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cả hai con số vừa đếm (`800` và `1536`) đều tính bằng công thức TÍCH của các
kích thước liên quan. `Value` xử lý từng số một, nên số OBJECT nó cần cũng
tăng theo đúng tích đó. Nếu một object DUY NHẤT (`Tensor`) có thể ôm trọn cả
mảng `numpy` — nghĩa là cả phép cộng LẪN phép nhân của hàng trăm số chạy
trong một lời gọi hàm `numpy`, không phải hàng trăm lời gọi `__add__`/
`__mul__` riêng lẻ của Python — điều gì xảy ra với chi phí phải trả, khi kích
thước ma trận tăng lên hàng nghìn (như một mô hình ngôn ngữ thật)?
::::

::::checkpoint{mastery=0.8}
::::
