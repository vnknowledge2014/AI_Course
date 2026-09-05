---
id: tri-tue-nhan-tao.dao-ham-tu-dong.dao-ham-nguoc-chuoi
title: "Đạo hàm ngược theo chuỗi"
summary: "Tính tay ∂f/∂a, ∂f/∂b, ∂f/∂c, ∂f/∂d cho ĐÚNG đồ thị f=(a·b+c)·d của bài trước bằng chain rule: df_da=b·d, df_db=a·d, df_dc=d, df_dd=a·b+c. Xác nhận bằng hai bộ số khác nhau (a=2,b=-3,c=10,d=-2 cho 6,-4,-2,4; a=1,b=2,c=3,d=4 cho 8,4,4,5) — mỗi biến cần một lượt tính riêng, quy mô kém khi biểu thức dài ra."
locale: vi
track: tri-tue-nhan-tao
module: dao-ham-tu-dong
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: [ai.dao-ham-nguoc-chuoi]
requires: [ai.do-thi-tinh-toan]
concepts: [ai.dao-ham-nguoc-chuoi]
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
Đồ thị xong rồi. Câu hỏi thật sự chưa được hỏi: nếu đổi `a` một chút, `f`
đổi theo bao nhiêu? Đó là câu hỏi mà GRADIENT trả lời.
::::

::::explain{#chain-rule-tren-do-thi}
Bài trước dựng đồ thị của `f = (a·b + c)·d` — ba node: `n1 = a·b`,
`n2 = n1 + c`, `f = n2·d`. Muốn biết `f` nhạy tới đâu với mỗi lá (`a`, `b`,
`c`, `d`), cần đạo hàm riêng phần `∂f/∂a`, `∂f/∂b`, `∂f/∂c`, `∂f/∂d`.

Vì `f` không phụ thuộc TRỰC TIẾP vào các lá — nó phụ thuộc qua `n1`, `n2` —
mỗi đạo hàm phải đi NGƯỢC qua chuỗi node, nhân các đạo hàm cục bộ dọc đường.
Đây là **quy tắc chuỗi** (chain rule): muốn biết `f` nhạy tới đâu với một lá
`x` nằm sâu trong đồ thị, nhân đạo hàm cục bộ của TỪNG bước trên đường từ
`x` tới `f`.

> `∂f/∂d`: `f = n2·d` — `d` là toán hạng TRỰC TIẾP của bước cuối, nên
> `∂f/∂d = n2` (đạo hàm của một tích theo một thừa số là thừa số còn lại).
>
> `∂f/∂c`: `c` không phải toán hạng trực tiếp của `f` — nó là toán hạng của
> `n2`. Đường từ `c` tới `f` đi qua đúng một bước trung gian: `∂f/∂c =
> (∂f/∂n2)·(∂n2/∂c)`. `∂f/∂n2 = d` (từ `f = n2·d`), `∂n2/∂c = 1` (từ
> `n2 = n1 + c`, đạo hàm của một tổng theo một số hạng luôn là `1`). Vậy
> `∂f/∂c = d·1 = d`.
>
> `∂f/∂a`: đường từ `a` tới `f` đi qua HAI bước trung gian (`n1` rồi `n2`):
> `∂f/∂a = (∂f/∂n2)·(∂n2/∂n1)·(∂n1/∂a) = d·1·b = b·d`.
>
> `∂f/∂b`: đối xứng với `a` — `∂f/∂b = d·1·a = a·d`.

Bốn công thức: `∂f/∂a = b·d`, `∂f/∂b = a·d`, `∂f/∂c = d`,
`∂f/∂d = n2 = a·b + c`. Mỗi công thức là kết quả của MỘT LƯỢT đi ngược từ
`f` về đúng MỘT lá — bốn lá cần bốn lượt riêng, mỗi lượt tự nhân lại các
đạo hàm cục bộ dọc đường, không lượt nào dùng lại việc đã làm ở lượt khác.
::::

::::example{#doi_chieu_hai_bo_so}
Bốn công thức trên áp cho HAI bộ số khác nhau, cùng một đồ thị:

```python title=readonly
def df_da(a, b, c, d):
    return b * d

def df_db(a, b, c, d):
    return a * d

def df_dc(a, b, c, d):
    return d

def df_dd(a, b, c, d):
    return a * b + c

for (a, b, c, d) in [(2, -3, 10, -2), (1, 2, 3, 4)]:
    print(a, b, c, d, "->", df_da(a, b, c, d), df_db(a, b, c, d), df_dc(a, b, c, d), df_dd(a, b, c, d))
```

```text title=readonly
2 -3 10 -2 -> 6 -4 -2 4
1 2 3 4 -> 8 4 4 5
```

Bộ số đầu (`a=2, b=-3, c=10, d=-2` — đúng bộ đã dùng ở bài trước, nơi
`f=-8`) cho bốn đạo hàm `6, -4, -2, 4`. Bộ số thứ hai (`a=1, b=2, c=3, d=4`,
cho `f=20`) cho bốn đạo hàm KHÁC HẲN: `8, 4, 4, 5`. Bốn hàm này không hề đổi
— chỉ đầu vào đổi — nên kết quả đổi theo là đúng: đạo hàm phụ thuộc vào
ĐIỂM đang xét (giá trị cụ thể của `a, b, c, d`), không phải một hằng số cố
định của biểu thức.
::::

::::predict{#doan_quy_mo_kem commitOnce}
Bốn công thức trên — mỗi công thức là kết quả của MỘT lượt đi ngược riêng từ
`f` về một lá, không lượt nào tái sử dụng việc đã làm ở lượt khác.

**Trước khi đọc tiếp**, bạn đoán: nếu biểu thức có `20` lá thay vì `4`
(nhưng vẫn cùng kiểu — mỗi lá cách `f` một số bước hữu hạn), cách tính "mỗi
lá một lượt riêng" này cần khoảng bao nhiêu lượt đi ngược qua đồ thị?

:::opt{correct}
Khoảng `20` lượt — một lượt CHO MỖI lá, vì cách tính này không có cơ chế
nào chia sẻ công đã làm giữa các lá khác nhau
:::

:::opt
Vẫn chỉ cần một lượt — vì đồ thị chỉ có một hình dạng, tính một lần là biết
đạo hàm cho MỌI lá cùng lúc
::why
Gần đúng ở việc đồ thị đúng là CHỈ CÓ MỘT hình dạng, dùng chung cho mọi lá —
quan sát về cấu trúc không sai.

Chỗ lệch: cách tính "mỗi lá một lượt" ở bài này không hề khai thác việc dùng
chung đồ thị — mỗi hàm `df_d...` tự đi từ `f` ngược lại lá của MÌNH, không
đọc lại kết quả của hàm khác. Muốn "một lượt cho mọi lá" cần một thuật toán
KHÁC hẳn — đó chính là điều bài `sap-xep-to-po-va-lan-truyen-nguoc` sắp xây.
::
:::

:::opt
Khoảng `4` lượt — vì số lượt phụ thuộc vào số BƯỚC của biểu thức (ở đây là
`3` node trung gian), không phụ thuộc số lá
::why
Gần đúng ở việc số BƯỚC của biểu thức (số node trung gian) đúng là một đại
lượng có ảnh hưởng — nó quyết định CHUỖI nhân dài hay ngắn cho MỖI lá.

Chỗ lệch: số LƯỢT (số lần phải đi ngược từ đầu) trong cách tính này gắn với
số LÁ cần đạo hàm, không gắn với số bước của biểu thức. Một biểu thức có
`20` lá, dù chỉ `3` bước như đồ thị này, vẫn cần `20` lượt riêng — mỗi lượt
NGẮN (ít bước nhân), nhưng vẫn phải lặp lại đúng `20` lần.
::
:::
::::

::::code{#hoan_thien_bon_dao_ham}
Hoàn thiện bốn hàm đạo hàm riêng phần, đúng bốn công thức đã suy ra ở phần
giải thích.

```python title=starter
def df_da(a, b, c, d):
    return ___                # b * d

def df_db(a, b, c, d):
    return ___                # a * d

def df_dc(a, b, c, d):
    return ___                # d

def df_dd(a, b, c, d):
    return ___                # a * b + c

a, b, c, d = 2, -3, 10, -2

print(df_da(a, b, c, d))
print(df_db(a, b, c, d))
print(df_dc(a, b, c, d))
print(df_dd(a, b, c, d))
```

```python title=solution
def df_da(a, b, c, d):
    return b * d

def df_db(a, b, c, d):
    return a * d

def df_dc(a, b, c, d):
    return d

def df_dd(a, b, c, d):
    return a * b + c

a, b, c, d = 2, -3, 10, -2

print(df_da(a, b, c, d))
print(df_db(a, b, c, d))
print(df_dc(a, b, c, d))
print(df_dd(a, b, c, d))
```

```python title=test
assert df_da(2, -3, 10, -2) == 6, f"df_da(2,-3,10,-2) sai -- dang ra {df_da(2, -3, 10, -2)}"
assert df_db(2, -3, 10, -2) == -4, f"df_db(2,-3,10,-2) sai -- dang ra {df_db(2, -3, 10, -2)}"
assert df_dc(2, -3, 10, -2) == -2, f"df_dc(2,-3,10,-2) sai -- dang ra {df_dc(2, -3, 10, -2)}"
assert df_dd(2, -3, 10, -2) == 4, f"df_dd(2,-3,10,-2) sai -- dang ra {df_dd(2, -3, 10, -2)}"

# rieng kiem tra bang MOT bo so KHAC: chan cheat "chep san 6/-4/-2/4" ma
# khong tinh tu cong thuc that.
assert df_da(1, 2, 3, 4) == 8, f"df_da(1,2,3,4) sai -- dang ra {df_da(1, 2, 3, 4)}"
assert df_db(1, 2, 3, 4) == 4, f"df_db(1,2,3,4) sai -- dang ra {df_db(1, 2, 3, 4)}"
assert df_dc(1, 2, 3, 4) == 4, f"df_dc(1,2,3,4) sai -- dang ra {df_dc(1, 2, 3, 4)}"
assert df_dd(1, 2, 3, 4) == 5, f"df_dd(1,2,3,4) sai -- dang ra {df_dd(1, 2, 3, 4)}"
```

:::hints
- kind: attention
  body: Bốn chỗ trống, bốn công thức đã suy ra ở phần giải thích bằng chain rule. `df_da`: đường từ `a` tới `f` đi qua `n1` rồi `n2`, tích các đạo hàm cục bộ cho `b·d`. `df_db`: đối xứng, cho `a·d`. `df_dc`: đường ngắn hơn (qua `n2`), cho đúng `d`. `df_dd`: `d` là toán hạng trực tiếp của bước cuối, đạo hàm là thừa số còn lại `a·b + c` (giá trị của `n2`).
- kind: strategy
  body: 'df_da: `b * d`. df_db: `a * d`. df_dc: `d`. df_dd: `a * b + c`.'
- kind: one-line
  body: 'Bốn chỗ trống lần lượt là `b * d`, `a * d`, `d`, và `a * b + c`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: ca bon ham phai tinh THAT tu tham so a/b/c/d bang phep toan dung cong thuc chain rule -- khong duoc chep san con so 6/-4/-2/4 (bai kiem tra bang mot bo so KHAC se lo cheat nay ra ngay)
  requireAst:
  - kind: uses-name, target: a, min: 5
  - kind: uses-name, target: b, min: 5
  - kind: uses-name, target: c, min: 5
  - kind: uses-name, target: d, min: 5
  - kind: uses-operator, target: "*", min: 3
  - kind: uses-operator, target: "+", min: 1
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai dung
  # dat=true, sau luat qua sach (a/b/c/d: baseline 4 lan doc tu bon dong
  # print(...(a,b,c,d)), cong them 1-3 lan tu than ham -- tong 6/6/5/7; *: 3
  # lan tu df_da,df_db,df_dd; +: 1 lan tu df_dd). Cheat chep san ca bon ham
  # (df_da tra ve 6, df_db tra ve -4, v.v.) lam ca "*" va "+" ve 0 -- bi chan.
  # Ngay ca neu static khong bat het moi cheat rieng le, tier tests da co bo
  # so THU HAI (1,2,3,4) doc lap, bat DOC LAP moi ham chep san sai gia tri.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^6\\n-4\\n-2\\n4\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn công thức, bốn lượt riêng — đúng và đủ cho `4` lá. Nhưng cách này không
scale: `20` lá cần `20` lượt. Bài sau bắt đầu xây một class Python để KHÔNG
phải viết lại `df_d...` bằng tay cho mỗi biến nữa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bốn hàm `df_da`, `df_db`, `df_dc`, `df_dd` đều PHẢI VIẾT TAY, từng công thức
một — và nếu đồ thị đổi hình dạng (thêm một bước, đổi một phép toán), CẢ BỐN
công thức phải viết lại từ đầu. Điều gì cần đúng để một MÁY TÍNH — không
phải một người — có thể tự suy ra bốn công thức này chỉ từ việc biết đồ thị
được xây bằng những phép toán gì?
::::

::::checkpoint{mastery=0.8}
::::
