---
id: tri-tue-nhan-tao.token-hoa-bpe.dem-cap-lien-ke
title: "Đếm cặp liền kề: bước lõi đầu tiên của BPE"
summary: "Cài dem_cap_lien_ke: quét toàn bộ corpus (dạng danh sách ký tự), đếm tần suất MỌI cặp ký tự liền kề. Trên corpus 45 ký tự (con meo an ca, con cho an com, con ga an thoc) có 23 cặp khác nhau; cặp phổ biến nhất là ('n', ' ') với tần suất 6 -- ba lần từ 'con' và ba lần từ 'an' đều kết thúc bằng 'n' theo sau khoảng trắng, cộng dồn đúng 6. Đối chiếu tự tay bằng Python thật, không suy luận."
locale: vi
track: tri-tue-nhan-tao
module: token-hoa-bpe
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: [ai.dem-cap-lien-ke]
requires: [ai.token-hoa-ky-tu]
concepts: [ai.dem-cap-lien-ke]
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

::::byte{trigger=enter mood=curious pose=point-stage}
Token hoá theo ký tự xong rồi. Giờ tới bước LÕI của BPE — bước mà mọi vòng
"ghép" sau này đều bắt đầu từ nó: đếm xem cặp ký tự nào đứng CẠNH NHAU
thường xuyên nhất.
::::

::::explain{#dem_cap_lien_ke_the_nao}
BPE (Byte Pair Encoding) quyết định "ghép cái gì thành token mới" bằng
đúng một tiêu chí: **cặp hai token LIỀN KỀ xuất hiện thường xuyên nhất
trong toàn bộ corpus**. Trước khi ghép được gì, phải ĐẾM được tần suất của
MỌI cặp liền kề trước đã.

Với corpus đã là một danh sách ký tự (`['c', 'o', 'n', ' ', 'm', ...]`),
"cặp liền kề" ở vị trí `i` là `(danh_sach[i], danh_sach[i + 1])` — hai
phần tử ĐỨNG CẠNH NHAU. Quét từ `i = 0` tới `i = độ dài − 2` (dừng trước
phần tử cuối, vì phần tử cuối không có ai đứng SAU nó để ghép cặp), cộng
dồn một bộ đếm cho mỗi cặp xuất hiện:

> `for i in range(len(danh_sach) - 1): dem[(danh_sach[i], danh_sach[i+1])] += 1`

Kết quả là một bảng tần suất — mỗi cặp (ký tự, ký tự) ánh xạ tới số lần nó
xuất hiện liền kề nhau trong TOÀN BỘ corpus. Cặp có tần suất CAO NHẤT
trong bảng này chính là ứng viên đầu tiên để GHÉP thành một token mới —
việc đó là của bài sau.
::::

::::example{#dem_cap_tren_corpus_that}
Đếm mọi cặp liền kề trên đúng corpus `45` ký tự đã dùng ở hai bài trước:

```python title=readonly
def dem_cap_lien_ke(danh_sach):
    dem = {}
    for i in range(len(danh_sach) - 1):
        cap = (danh_sach[i], danh_sach[i + 1])
        dem[cap] = dem.get(cap, 0) + 1
    return dem

corpus = "con meo an ca, con cho an com, con ga an thoc"
danh_sach = list(corpus)
dem = dem_cap_lien_ke(danh_sach)

cap_pho_bien = max(dem, key=dem.get)
tan_suat = dem[cap_pho_bien]

print("so cap khac nhau:", len(dem))
print("cap pho bien nhat:", cap_pho_bien)
print("tan suat:", tan_suat)
```

```text title=readonly
so cap khac nhau: 23
cap pho bien nhat: ('n', ' ')
tan suat: 6
```

Có `23` cặp KHÁC NHAU trong corpus này. Cặp phổ biến nhất là `('n', ' ')`
— chữ `n` theo ngay sau bởi một khoảng trắng — xuất hiện đúng `6` lần.
Điều này khớp với việc corpus có `3` từ `"con"` và `3` từ `"an"` đứng độc
lập (không dính liền từ khác) — CẢ SÁU từ đó đều kết thúc bằng `n` rồi
một khoảng trắng, cộng dồn đúng `6` lần xuất hiện của cặp `('n', ' ')`.
::::

::::predict{#doan_cap_pho_bien commitOnce}
**Trước khi chạy `dem_cap_lien_ke`**, bạn đoán: trong ba cặp ứng viên dưới
đây, cặp nào có tần suất CAO NHẤT trong corpus `"con meo an ca, con cho an
com, con ga an thoc"`?

:::opt{correct}
`('n', ' ')` — vì CẢ hai từ ngắn phổ biến trong corpus này, `"con"` (xuất
hiện `3` lần) và `"an"` (xuất hiện `3` lần, đứng độc lập), đều kết thúc
bằng chữ `n` theo sau bởi khoảng trắng — cộng dồn đúng `6` lần, nhiều hơn
bất kỳ cặp nào chỉ xuất hiện qua MỘT từ duy nhất
:::

:::opt
`('c', 'o')` — vì `"con"`, `"cho"`, `"com"` đều bắt đầu bằng `c` rồi `o`,
và có vẻ như "co" xuất hiện ở rất nhiều từ trong corpus
::why
Gần đúng ở việc bạn để ý đúng: `('c', 'o')` THẬT SỰ xuất hiện nhiều lần
(`4` lần — từ `"con"` × `3` và `"cho"`, `"com"` mỗi từ một lần chứa `c` rồi
`o`... nhưng chỉ những từ BẮT ĐẦU bằng `co` mới đóng góp).

Chỗ lệch: `4` lần vẫn ÍT hơn `6` lần của `('n', ' ')`. Cặp `('n', ' ')`
được "tiếp sức" bởi HAI từ khác nhau kết thúc cùng kiểu (`"con"` VÀ
`"an"`), trong khi `('c', 'o')` chỉ được đóng góp bởi những từ bắt đầu
đúng bằng `co` — ít từ hơn tưởng tượng ban đầu.
::
:::

:::opt
Không thể biết trước — tần suất cặp phụ thuộc vào thứ tự các ký tự xuất
hiện một cách ngẫu nhiên, không có quy luật nào đoán được trước khi chạy
code
::why
Gần đúng ở việc nhấn mạnh KHÔNG nên đoán mò mà không tính — tinh thần "đo
bằng số thật" đúng là nguyên tắc xuyên suốt các bài học này.

Chỗ lệch: câu hỏi không đòi hỏi đoán một con số CHÍNH XÁC mà không có cơ
sở nào — nó đòi hỏi NHẬN RA MỘT QUY LUẬT có thể suy luận được TRƯỚC khi
chạy: đếm xem từ NÀO trong corpus kết thúc bằng ký tự nào, rồi cộng dồn.
Đó chính xác là cách `dem_cap_lien_ke` hoạt động bên trong — hoàn toàn có
thể lập luận trước, và bài học sau đó xác nhận lại bằng code thật, không
phải đoán mò không căn cứ.
::
:::
::::

::::code{#viet_dem_cap_lien_ke}
Hoàn thiện `dem_cap_lien_ke`: xây cặp `(ký tự tại vị trí i, ký tự liền
sau nó)`, rồi cộng dồn số đếm cho cặp đó.

```python title=starter
def dem_cap_lien_ke(danh_sach):
    dem = {}
    for i in range(len(danh_sach) - 1):
        cap = (danh_sach[i], ___)          # danh_sach[i + 1]
        dem[cap] = ___                      # dem.get(cap, 0) + 1
    return dem

corpus = "con meo an ca, con cho an com, con ga an thoc"
danh_sach = list(corpus)
dem = dem_cap_lien_ke(danh_sach)

cap_pho_bien = max(dem, key=dem.get)
tan_suat = dem[cap_pho_bien]

print(len(dem))
print(cap_pho_bien)
print(tan_suat)
```

```python title=solution
def dem_cap_lien_ke(danh_sach):
    dem = {}
    for i in range(len(danh_sach) - 1):
        cap = (danh_sach[i], danh_sach[i + 1])
        dem[cap] = dem.get(cap, 0) + 1
    return dem

corpus = "con meo an ca, con cho an com, con ga an thoc"
danh_sach = list(corpus)
dem = dem_cap_lien_ke(danh_sach)

cap_pho_bien = max(dem, key=dem.get)
tan_suat = dem[cap_pho_bien]

print(len(dem))
print(cap_pho_bien)
print(tan_suat)
```

```python title=test
assert len(dem) == 23, f"so cap khac nhau phai la 23 -- dang ra {len(dem)}"
assert cap_pho_bien == ('n', ' '), f"cap pho bien nhat phai la ('n', ' ') -- dang ra {cap_pho_bien}"
assert tan_suat == 6, f"tan suat cap pho bien phai la 6 -- dang ra {tan_suat}"
assert dem[('c', 'o')] == 4, f"tan suat ('c','o') phai la 4 -- dang ra {dem[('c', 'o')]}"
assert dem[(' ', 'a')] == 3, f"tan suat (' ','a') phai la 3 -- dang ra {dem[(' ', 'a')]}"

# kiem tra ham tren mot danh sach nho, tu tay doi chieu duoc: "aabb" chi co
# 3 cap lien ke: ('a','a')=1, ('a','b')=1, ('b','b')=1
dem_nho = dem_cap_lien_ke(list("aabb"))
assert dem_nho == {('a', 'a'): 1, ('a', 'b'): 1, ('b', 'b'): 1}, f"dem tren 'aabb' sai -- dang ra {dem_nho}"

# bien: danh sach chi co DUNG MOT phan tu -- khong co cap lien ke nao ca,
# phai tra ve dict RONG, khong duoc loi vi truy cap danh_sach[1] khong ton tai
assert dem_cap_lien_ke(['x']) == {}, "danh sach chi 1 phan tu phai tra ve dict RONG (khong co cap lien ke nao)"
assert dem_cap_lien_ke([]) == {}, "danh sach RONG phai tra ve dict RONG"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ 1 là ký tự LIỀN SAU vị trí `i` trong `danh_sach` — chỉ số `i + 1`. Chỗ 2 là bước CỘNG DỒN số đếm cho `cap` — lấy giá trị hiện tại (hoặc `0` nếu `cap` chưa từng gặp, dùng `dict.get`) rồi cộng thêm `1`.
- kind: strategy
  body: 'Chỗ 1: `danh_sach[i + 1]`. Chỗ 2: `dem.get(cap, 0) + 1`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `danh_sach[i + 1]` và `dem.get(cap, 0) + 1`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: dem_cap_lien_ke phai tinh dung chi so i+1 (khong duoc chep lai i hay mot hang so co dinh), VA phai dung dict.get de cong don so dem (khong duoc gan cung mot gia tri hang so)
  requireAst:
  - kind: uses-operator, target: "+", min: 2
  - kind: uses-call, target: get, min: 1
  - kind: has-literal, target: "0", min: 1
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca ba luat qua sach. "+"=2: mot lan trong "i + 1" (cho
  # trong 1), mot lan trong "dem.get(cap, 0) + 1" (cho trong 2) -- neu cho
  # trong 1 bi thay bang "danh_sach[i]" (thieu +1), dem "+" tut xuong con 1,
  # bi chan. get=1: chi mot lan goi dict.get, dung de phat hien cheat "dem[cap]
  # = 1" (gan cung hang so, khong cong don) lam get tut ve 0. has-literal
  # "0"=1: gia tri mac dinh trong get(cap, 0) -- neu doi thanh get(cap, 1)
  # (mac dinh sai), has-literal "0" tut ve 0, bi chan boi static. Da tu kiem
  # chung bang Python that: cheat get(cap, 1) lam MOI tan suat trong dem bi
  # lech dung +1 so voi dung (lan dem dau tien cua moi cap da cong tu 1 thay
  # vi 0) -- cap_pho_bien VAN la ('n', ' ') (thu tu tuong doi khong doi),
  # nhung tan_suat tra ve 7 thay vi 6 -- bi chan CA boi static LAN boi
  # assertion tan_suat == 6 (tests/output), phong thu kep.
  #
  # Cheat "cap = (danh_sach[i], danh_sach[i])" (dung sai chi so, lap lai
  # chinh no) lam "+"=1 -- bi chan; da tu kiem chung: cheat nay cho
  # so_cap_khac_nhau va cap_pho_bien hoan toan khac (vi du 'n','n' se khong
  # bao gio la cap pho bien tren corpus nay), bi bat boi ca output/tests.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^23\\n\\('n', ' '\\)\\n6\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`('n', ' ')`, tần suất `6` — cặp phổ biến nhất, đã đếm bằng Python thật.
Bài sau dùng đúng con số này: ghép cặp đó thành MỘT token mới, vòng "merge"
đầu tiên của BPE.
::::

::::reflect{#nghi-lai}
Đếm cặp liền kề chỉ là bước NHÌN — nó cho biết cặp nào nên ghép, nhưng
chưa ghép gì cả. Bài sau THỰC HIỆN việc ghép: thay MỌI lần xuất hiện của
cặp phổ biến nhất bằng một token mới duy nhất, và xác nhận số lượng token
trong corpus giảm đi sau đó.
::::

::::checkpoint{mastery=0.85}
::::
