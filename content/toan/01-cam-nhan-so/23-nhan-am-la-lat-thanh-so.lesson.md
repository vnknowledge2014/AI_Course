---
id: toan.cam-nhan-so.nhan-am-la-lat-thanh-so
title: Nhân số âm là lật thanh số
summary: Nhân với −1 không kéo giãn gì cả — nó lật cả thanh số quanh mốc 0. Lật hai lần thì mọi chỗ về chỗ cũ, nên (−1) × (−1) bằng 1 là thứ nhìn thấy được.
locale: vi
track: toan
module: cam-nhan-so
order: 23
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.multiply-by-negative]
requires: [math.multiply-as-scaling, math.negative-number, math.multiply-distributive, math.multiplication, core.variable, core.assignment, core.arithmetic, core.number-literal, core.print-variable, core.output]
concepts: [math.thanh-so, math.so-am, math.so-doi, math.phep-nhan]
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
Rễ của mình đang ở dưới mặt đất. Nhân nó với −1 thì nó chui lên à?
::::

::::explain{#mot-phep-khong-phai-keo-gian}
Bài trước để lại đúng câu hỏi này: nhân với `−1` làm gì với thanh số?

Trước hết, loại trừ một khả năng. Nó **không** phải một phép kéo giãn. Kéo giãn
một lần là để nguyên thanh số — mà `a × 1` bằng `a` còn `a × (−1)` thì không thể
cũng bằng `a`, nếu không thì `1` và `−1` là một số. Vậy `−1` bắt thanh số làm
một việc khác.

Bài trước cũng cho bạn ba điều mà **mọi** phép nhân đều tuân theo, và ba điều ấy
gần như trói chặt câu trả lời:

1. Nhân là một phép biến đổi **cả thanh số** — nó đối xử với mọi cái vạch theo
   cùng một luật, không ưu ái vạch nào.
2. **Mốc 0 đứng yên.** Không có số nào nhân với gì mà lôi được 0 đi chỗ khác.
3. Muốn biết phép ấy làm gì, chỉ cần hỏi **vạch 1 rơi vào đâu** — vì bài 19 nói
   "một lần lấy một lô" thì được đúng một lô, nên `1 × (−1)` là `−1`. (Giao
   hoán của bài 20, nên `1 × (−1)` hay `(−1) × 1` viết kiểu nào cũng thế.)

Ghép ba điều lại: vạch 1 phải đi từ chỗ 1 sang chỗ `−1`. Bài 18 gọi `−1` là
**số đối** của `1` — cùng khoảng cách tới mốc 0, khác phía.

Một phép biến đổi giữ mốc 0 nằm im, không làm khoảng nào dài ra hay ngắn lại, và
đưa vạch 1 sang đúng phía bên kia — chỉ có một phép như thế: **lật cả thanh số
quanh mốc 0**, như gấp một tờ giấy theo nếp gấp đặt tại 0.
::::

::::explain{#lat-quanh-moc-0}
Vườn của Byte lấy mặt đất làm mốc 0: mầm ở trên là số dương, rễ ở dưới là số âm.
Lật thanh số quanh 0 thì mầm xuống thành rễ và rễ lên thành mầm, sâu bao nhiêu
thì cao đúng bấy nhiêu.

| vạch | ở đâu trước khi lật | sau khi lật quanh mốc 0 |
|---|---|---|
| mốc 0 | ngay trên mặt đất | nằm nguyên tại 0 — nó là nếp gấp |
| vạch 5 | mầm cao 5 phân | rễ sâu 5 phân → chỗ `−5` |
| vạch `−2` | rễ sâu 2 phân | mầm cao 2 phân → chỗ `2` |
| vạch `−7` | rễ sâu 7 phân | mầm cao 7 phân → chỗ `7` |

Nhìn cột phải: chưa vạch nào lại gần mốc 0 hơn hay ra xa hơn. Khoảng cách giữ
nguyên hết, chỉ có **phía** là đổi. Đó là chỗ khác nhau giữa lật và kéo giãn.

Bây giờ tới chỗ đáng tiền nhất của bài. **Lật hai lần thì sao?**

Vạch 5 lật một cái xuống `−5`, lật cái nữa lên lại `5`. Vạch `−2` lật xuống `2`,
lật lại về `−2`. Cái nào cũng về đúng chỗ cũ — vì lật quanh cùng một nếp gấp hai
lần thì tờ giấy nằm y như lúc đầu.

Mà "để nguyên thanh số" chính là phép nhân với `1` của bài trước. Vậy hai lần
nhân `−1` gộp lại phải bằng một lần nhân `1`:

`(−1) × (−1) = 1`

Câu ấy thường được dạy như một luật phải học thuộc, kèm câu vè "âm nhân âm ra
dương". Ở đây nó không phải luật: nó là thứ bàn tay bạn vừa làm khi lật tờ giấy
hai lần.

> **Cách kiểm thứ hai cho cùng một lựa chọn.** Trước khi tính, nói thẳng một
> chuyện: bài 21 dựng luật phân phối bằng cách cắt một mảng chữ nhật **cây**,
> tức là dựng nó trên số dương. Ở đây ta **chọn** cho luật ấy chạy tiếp sang số
> âm — chọn, vì ta muốn luật cũ đừng gãy khi thanh số dài ra bên trái.
>
> Đã chọn rồi thì đi tiếp. Bài 18: `1 + (−1)` bằng `0`. Bài 21: cắt một tổng ra
> thì phép nhân tách theo. Nhân cả cụm ấy với `−1` rồi tách: `(−1) × 1` cộng
> `(−1) × (−1)` phải bằng `(−1) × 0`, tức bằng `0`. Số hạng đầu là `−1`. Vậy số
> hạng sau phải là thứ cộng với `−1` ra `0` — tức là `1`.
>
> Điều đáng nói là: sau khi đã chọn như thế, ta **không còn tự do nào nữa**.
> `(−1) × (−1)` buộc phải bằng `1`, không có lựa chọn thứ hai. Bức tranh lật tờ
> giấy ở trên cũng đứng trên đúng cái lựa chọn ấy — nên đây là hai cách kiểm
> cùng một lựa chọn, không phải hai đường độc lập. Chỗ chắc nằm ở chỗ đó: không
> phải vì không có quy ước nào, mà vì quy ước ấy chỉ mở đúng một cửa.

Và một hệ quả gộp lại từ hai bài, không phải luật mới: nhân với một số âm bất
kỳ là **lật rồi kéo giãn** — hai động tác bạn đã có, làm liên tiếp. Mầm ở `5`
nhân với `−2` thì lật xuống `−5` rồi giãn gấp đôi thành `−10`. Bài tập dưới đây
sẽ để bạn tự tay làm một ca như vậy.
::::

::::byte{trigger=enter mood=thinking pose=point-editor}
Lật một cái thì rễ thành mầm. Lật thêm cái nữa thì nó về lại chỗ cũ chứ?
::::

::::predict{#doan-lat-mot-lan-va-hai-lan commitOnce}
Rễ của Byte ở độ sâu `−2`. Dòng đầu lật thanh số **một lần**, dòng sau lật
**hai lần**.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
do_sau_re = -2
print(do_sau_re * -1)
print(do_sau_re * -1 * -1)
```

:::opt{correct}
2, rồi −2
:::

:::opt
2, rồi 2
::why
Gần đúng ở chỗ bạn dùng một quy tắc có thật, và dùng nó chuẩn ở dòng đầu: âm
nhân âm ra dương, nên `−2` nhân `−1` ra `2`. Dòng đó bạn không sai chỗ nào.

Phạm vi bị vượt ở dòng thứ hai. Quy tắc ấy nói về **hai thừa số âm**; nó không
nói "hễ trong phép tính có dấu âm thì kết quả dương". Tới lúc nhân lần thứ hai,
con số đang bị nhân không còn là `−2` nữa — nó đã bị lật thành `2` rồi. Nên dòng
hai là dương nhân âm, và bức tranh nói y hệt: lật hai lần thì về chỗ cũ, mà chỗ
cũ của rễ là `−2`.
::
:::

:::opt
−2, rồi −2
::why
Gần đúng ở chỗ bạn đọc dấu trừ trong `−1` như một cái nhãn "âm" đem dán lên kết
quả. Cái nhãn ấy có thật: bài 16 nói dấu trừ trước một con số cho biết nó nằm
phía **trái** mốc 0, và độ sâu của rễ đúng là nằm phía đó.

Chỗ lệch: dán nhãn thì dán một lần là xong, dán thêm lần nữa cũng thế. Còn nhân
là một phép **biến đổi** — mỗi lần nhân là một lần cả thanh số bị lật. Rễ đang ở
bên trái, lật một cái là nó sang bên phải thành `2`, chứ không nằm nguyên.
::
:::

:::opt
−2, rồi 2
::why
Gần đúng ở chỗ bạn thấy hai dòng phải cho hai kết quả khác nhau — dòng sau có
thêm một phép nhân nữa, nên nó không thể trùng dòng trước. Nhận xét ấy đúng.

Chỗ lệch là thứ tự. Bạn đang để dòng đầu giữ nguyên `−2` rồi mới đổi phía ở dòng
sau, tức là lần lật thứ nhất không làm gì cả. Nhưng lần nào cũng là một lần lật:
lần đầu đưa rễ lên `2`, lần thứ hai đưa nó về `−2`. Hai kết quả vẫn khác nhau,
chỉ là ngược lại với thứ tự bạn đoán.
::
:::
::::

::::example{#may-tra-loi}
Chạy lên, máy in ra:

```text
2
-2
```

Một lần lật: rễ sâu 2 phân thành mầm cao 2 phân. Hai lần lật: về đúng chỗ cũ,
dưới mặt đất 2 phân.

Để ý dòng thứ hai đang nói cùng một chuyện với `(−1) × (−1) = 1` — hai phép lật
gộp lại thành "không đổi gì", nên `−2` đi một vòng rồi trở lại `−2`.
::::

::::code{#lat-ca-vuon}
Vườn Byte lấy mặt đất làm mốc 0. Rễ cây rau muống ở `−2` phân, mầm của nó nhô
lên `5` phân. Byte muốn máy xác nhận cả bốn điều bài này vừa nói.

Điền bốn chỗ trống: chỗ mới của rễ sau **một** lần lật, chỗ mới của mầm sau
**một** lần lật, chỗ của mầm sau **hai** lần lật, và chỗ của rễ sau khi
**nhân với `−3`** — tức lật một cái rồi kéo giãn ba lần.

Bài chấm bằng bốn tình huống lệch nhau — một vạch âm, một vạch dương, một vạch
bị lật hai lượt, và một vạch vừa lật vừa kéo giãn. Một con số chép cứng vào cả
bốn chỗ trống chỉ đúng được nhiều nhất một dòng.

```python title=starter
re_cay = -2
mam_cay = 5
lat = -1
keo = 3

re_sau_mot_lan_lat = ___
mam_sau_mot_lan_lat = ___
mam_sau_hai_lan_lat = ___
re_sau_lat_va_keo = ___

print(re_sau_mot_lan_lat)
print(mam_sau_mot_lan_lat)
print(mam_sau_hai_lan_lat)
print(re_sau_lat_va_keo)
```

```python title=solution
re_cay = -2
mam_cay = 5
lat = -1
keo = 3

re_sau_mot_lan_lat = re_cay * lat
mam_sau_mot_lan_lat = mam_cay * lat
mam_sau_hai_lan_lat = mam_cay * lat * lat
re_sau_lat_va_keo = re_cay * lat * keo

print(re_sau_mot_lan_lat)
print(mam_sau_mot_lan_lat)
print(mam_sau_hai_lan_lat)
print(re_sau_lat_va_keo)
```

```python title=test
# Bốn câu, bốn chỗ trống — câu nào cũng đọc thẳng thứ bạn vừa viết, nên gõ sai
# là đỏ ngay. Câu áp chót là ca tổng quát: nhân với −3, không phải chỉ −1. Câu
# cuối chốt lại rằng nó không phải luật mới — nó đúng bằng phép lật của bài này
# nối tiếp phép kéo giãn của bài 22.
assert re_sau_mot_lan_lat == 2, "rễ sâu 2 phân, lật quanh mốc 0 thì thành mầm cao 2 phân"
assert mam_sau_mot_lan_lat == -5, "mầm cao 5 phân, lật một cái là xuống −5 — cùng khoảng cách, khác phía"
assert mam_sau_hai_lan_lat == 5, "lật hai lần thì mọi vạch về đúng chỗ cũ — đó chính là (−1) × (−1) = 1"
assert re_sau_lat_va_keo == 6, "rễ ở −2 nhân với −3: lật lên 2 rồi kéo giãn ba lần thành 6"
assert re_sau_lat_va_keo == re_sau_mot_lan_lat * keo, \
    "lật xong mới tới kéo giãn — hai động tác nối tiếp, không phải một luật mới phải học thuộc"
```

:::hints
- kind: attention
  body: Bốn cái tên đã có sẵn ở đầu bài — `re_cay`, `mam_cay`, `lat`, `keo`. Cái tên `lat` đang giữ đúng con số làm nên phép lật, `keo` giữ số lần kéo giãn, nên không chỗ trống nào cần gõ thêm số mới.
- kind: strategy
  body: Một lần lật là một lần nhân với `lat`. Vậy hai lần lật thì nhân với `lat` hai lượt trên cùng một dòng. Dòng cuối cũng dài hai phép nhân, nhưng phép thứ hai không phải lật nữa — nó là phép kéo giãn của bài 22.
- kind: one-line
  body: "Bốn chỗ trống lần lượt là `re_cay * lat`, `mam_cay * lat`, `mam_cay * lat * lat`, và `re_cay * lat * keo`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi lần lật phải là một phép nhân với `lat`, không phải chép cứng kết quả — dòng lật hai lượt cần đúng hai phép nhân, và dòng cuối cần một phép lật rồi một phép kéo giãn qua `keo`
  requireAst:
  # Năm phép nhân trở lên và bốn chỗ ĐỌC tên `lat`: một cho rễ, một cho mầm,
  # hai cho dòng lật hai lượt, một cho dòng lật-rồi-kéo. `keo` phải xuất hiện
  # ít nhất một lần, nếu không thì dòng cuối chỉ là con số 6 chép cứng. Điền
  # bừa vào chỗ trống thì không có dấu nhân nào.
  - kind: uses-operator, target: *, min: 5
  - kind: uses-name, target: lat, min: 4
  - kind: uses-name, target: keo, min: 1
- tier: output
  match: regex
  expect: ^2\n-5\n5\n6\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Lật một cái rễ thành mầm. Lật cái nữa nó về chỗ cũ. Nhân `−3` chỉ là lật rồi kéo.
::::

::::sandbox{#lat-thu}
Byte đang đứng ở mốc 0. Thử tự tay lật thanh số xem.

Đây là sân chơi — không có đáp án đúng, không ai chấm. Chạy lại bao nhiêu lần
cũng được, và hãy thử cả những thứ bạn nghĩ sẽ lạ: lật ba lần thì sao, nhân
`-2` khác gì nhân `-1` rồi nhân `2`, và `nhan(0)` đưa bạn về đâu.

```python title=starter
di(3)
nhan(-1)
nhan(-1)
noi(dang_o())
```

:::world{number-line}
{ "tu": -10, "den": 10, "bat_dau": 0, "buoc_vach": 1 }
:::
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Giờ bạn có hai động tác trên thanh số, và cả hai đều viết bằng đúng một dấu —
dấu nhân. Kéo giãn ra xa mốc 0, và lật quanh mốc 0.

Byte quay về chỗ đóng bó ở bài 6. Mười hạt gom thành một bó: kéo giãn 10 lần,
viết `10`. Mười bó gom thành một bó lớn: kéo giãn 10 lần **thêm một lượt nữa**,
viết `10 × 10`. Bó của bó của bó thì `10 × 10 × 10`.

Byte đóng bó tới bậc năm thì phải viết năm con 10 cạnh nhau. Tới bậc hai mươi
thì hai mươi con 10, và tay mỏi trước khi đếm xong xem đã đủ chưa.

Cách viết ấy gọn lại được không? Và trong cách viết gọn ấy, con số nào nói **kéo
giãn mấy lần một lượt**, con số nào đếm **làm bao nhiêu lượt**?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
