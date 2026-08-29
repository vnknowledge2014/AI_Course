---
id: toan.dai-so-va-ham-so.tach-o-trong-ra-mot-minh
title: Tách ô trống ra đứng một mình
summary: Gỡ một phương trình là cởi ngược thứ tự mặc — thứ được làm sau cùng thì gỡ trước — cho tới khi ô trống còn lại một mình trên một đĩa.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 15
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.solve-linear-equation]
requires: [math.equation-multiply-both-sides, math.equation-add-both-sides, math.equation, math.order-of-operations, math.multiply-distributive, math.additive-inverse, math.negative-number, math.multiplication, core.variable, core.arithmetic, core.division, core.float, core.number-literal, core.print-variable, core.output, core.boolean, ctrl.comparison]
concepts: [math.phuong-trinh-bac-nhat, math.go-nguoc-thu-tu, math.giu-nguyen-nghiem]
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
  reviewed: true
---

::::byte{trigger=enter mood=thinking pose=lean-in}
Hai thứ đang bám vào ô trống. Mình gỡ cái nào trước đây?
::::

::::explain{#hai-thu-bam-vao-o-trong}
Câu hỏi cuối buổi ở xe bánh mì, viết lại từ bài trước:

*Mỗi ổ bán 15 000 đồng. Mỗi buổi trả 30 000 đồng tiền thuê chỗ. Dọn hàng về,
trong túi còn 195 000 đồng. Byte bán được bao nhiêu ổ?*

`15000 × n − 30000 = 195000`

Byte có sẵn hai loại phép giữ nghiệm — cộng/trừ cả hai đĩa (bài 13), nhân/chia
cả hai đĩa cho một số khác 0 (bài 14). Thiếu đúng một thứ: **thứ tự dùng
chúng**.

Nhìn kỹ đĩa trái. Nó không phải một cục liền; nó là một cách tính, và cách tính
ấy có **thứ tự**. Nếu ai đó đưa cho bạn một con số để điền vào `n`, bạn làm gì
trước? T2.1 bài 42 đã chốt: **nhân trước, cộng trừ sau.** Nhân `15000 × n` gói
số ổ thành một khối tiền, rồi mới lấy khối ấy trừ đi tiền thuê.

Vậy trên đường đi xuôi, `− 30000` là việc làm **sau cùng**.

Và đây là chỗ cả bài xoay quanh. Sáng ra Byte đi tất rồi mới xỏ giày. Tối về,
Byte cởi **giày** trước, rồi mới cởi tất. Không ai lôi được đôi tất ra khi giày
vẫn còn trên chân.

Gỡ một phương trình cũng đúng thế: **cái được mặc vào sau cùng thì cởi ra
trước.** Trên đĩa trái, thứ mặc sau cùng là phép trừ 30 000. Nên gỡ nó trước,
bằng phép cộng cả hai đĩa. Xong rồi mới tới phép nhân, gỡ bằng phép chia cả hai
đĩa.
::::

::::example{#go-tung-buoc}
Đi hết một lượt, mỗi dòng là một câu mới nhưng cùng một tập nghiệm:

```text
   15000 × n − 30000  =  195000
        ↓ cộng 30000 vào CẢ HAI đĩa
   15000 × n          =  225000
        ↓ chia CẢ HAI đĩa cho 15000
           n          =  15
```

Byte bán 15 ổ. Đọc thẳng ra khỏi dòng cuối, không phải thử số nào.

Bây giờ tới phần thú vị: **nếu gỡ ngược thứ tự thì sao?** Thử chia trước.

Chia cả hai đĩa cho 15 000. Đĩa phải thì dễ: `195000 : 15000 = 13`. Đĩa trái
thì phải cẩn thận — nó có **hai cụm**.

Đi đường vòng qua đồ đã có, thay vì mượn một luật chưa ai dựng. Đĩa trái là
`15000 × n − 30000`; rút cái chung ra như bài 8 thì nó thành
`15000 × (n − 2)`. Giờ chia cho 15 000 chỉ là bỏ đúng cái thừa số vừa rút ra —
không cần luật nào mới:

- `15000 × n` chia cho 15 000 còn `n`
- `30000` chia cho 15 000 còn `2`

```text
   15000 × n − 30000  =  195000
        ↓ chia CẢ HAI đĩa cho 15000 — chia CẢ HAI cụm bên trái
           n − 2      =  13
        ↓ cộng 2 vào CẢ HAI đĩa
           n          =  15
```

Cũng ra 15. Vậy thứ tự kia **không sai** — nó vẫn là hai phép giữ nghiệm, nối
đúng luật.

Nhưng để ý cái giá phải trả: gỡ phép nhân trước thì con số 30 000 bị lôi theo,
và bạn phải nhớ chia **cả nó nữa**. Ở đây 30 000 chia 15 000 vừa vặn ra 2 nên
nhìn còn dễ chịu; đổi tiền thuê thành 25 000 thì bạn đang cầm `n − 5/3 = 13` và
phải kéo một phân số qua suốt phần còn lại.

Nên thứ tự ở trên là thứ tự **nên dùng**, không phải thứ tự duy nhất **được
dùng**:

> Cái mặc vào **sau cùng** thì cởi ra **trước**.

Với câu dạng `a × n + b = c` thì cái mặc sau cùng là phép cộng trừ, nên ở dạng
ấy: gỡ cộng trừ trước, gỡ nhân chia sau. Nhưng đó là hệ quả của cái luật trên,
không phải bản thân cái luật. Gặp `15000 × (n − 2) = 195000` thì cái mặc sau
cùng lại là phép **nhân** — cả cụm `(n − 2)` mới được đem nhân — nên ở đó phải
gỡ nhân trước. Cứ hỏi "nếu điền một số vào, mình làm việc gì sau cùng?" rồi gỡ
đúng việc ấy trước; câu hỏi ấy đúng cho mọi dạng.
::::

::::predict{#doan-quen-mot-cum commitOnce}
Đây là chỗ trượt chân hay gặp nhất khi ai đó gỡ phép nhân trước: chia đĩa trái
cho 15 000 nhưng chỉ chia **một** cụm, để con số 30 000 nằm nguyên.

Byte lấy `n = 15` — số vừa tìm được — rồi hỏi máy hai câu.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
n = 15
print(15000 * n - 30000 == 195000)
print(n - 30000 == 195000 / 15000)
```

:::opt{correct}
True rồi False
:::

:::opt
True rồi True
::why
Gần đúng ở chỗ bạn đang bám đúng luật bài 14: *chia cả hai vế cho cùng một số
khác 0 thì tập nghiệm không đổi*. Luật ấy đúng, và nó đúng cả ở đây — nếu nó
được dùng đủ.

Chỗ lệch nằm ở chữ "cả hai **vế**". Vế trái không phải là cụm `15000 × n`; vế
trái là nguyên cả lượng `15000 × n − 30000`. Chia lượng ấy cho 15 000 nghĩa là
chia **mọi cụm** trong nó — luật phân phối ở bài 7. Dòng thứ hai chỉ chia cụm
đầu, còn 30 000 thì để nguyên, nên đĩa trái đã bị đối xử khác đĩa phải. Với
`n = 15` thì vế trái ra −29 985, vế phải ra 13,0 — máy trả `False`.
::
:::

:::opt
False rồi False
::why
Gần đúng ở chỗ bạn thấy dòng thứ hai hỏng, và nó hỏng thật.

Chỗ lệch: quy tắc *sai một bước thì hỏng cả chuỗi* đúng khi bạn đang viết một
chuỗi biến đổi trên giấy, vì dòng sau được sinh ra từ dòng trước. Ở đây thì
không: hai lệnh `print` chạy độc lập, mỗi lệnh tự đọc `n` và tự tính lại hai vế
từ đầu. Dòng thứ nhất vẫn là `15000 × 15 − 30000 == 195000`, tức
`195000 == 195000`.
::
:::

:::opt
Máy báo lỗi ở dòng thứ hai, vì `15 − 30000` ra số âm mà số ổ thì không âm được
::why
Gần đúng ở chỗ bạn kiểm lại kết quả bằng ý nghĩa ngoài đời — đó là thói quen
tốt, và "số ổ bánh mì không thể âm" là một nhận xét đúng về **cái quán**.

Chỗ lệch là nhận xét ấy nói về quán, không nói về máy. Với máy thì −29 985 là
một con số bình thường, có chỗ đứng đàng hoàng ở bên trái mốc 0 (T2.1 bài 16).
Máy tính ra nó, so với 13,0, thấy khác nhau, rồi trả `False` — không có lỗi nào
cả. Chỗ hỏng nằm sớm hơn: dòng ấy đã **không còn là câu hỏi cũ** kể từ lúc một
cụm bị bỏ quên, nên nó ra số gì cũng chẳng nói lên điều gì về số ổ nữa.
::
:::
::::

::::explain{#dat-ten-cho-cach-lam}
Đặt tên cho việc vừa làm:

> **Giải một phương trình** là nối liên tiếp những phép giữ nghiệm cho tới khi
> ô trống còn đứng **một mình** trên một đĩa. Lúc đó dòng cuối có dạng
> `n = một số`, và con số ấy đọc thẳng ra được.

Ba điều đáng cất đi:

- **Không có bước nào "chuyển vế đổi dấu".** Cả chuỗi chỉ gồm hai việc bạn đã
  học: làm cùng một phép cộng trừ ở hai đĩa, làm cùng một phép nhân chia ở hai
  đĩa. Nếu sau này bạn nghe ai đó nói "bê số sang vế kia rồi đổi dấu", đó chỉ
  là cái tên gọi tắt của việc cộng cùng một lượng vào cả hai đĩa.
- **Mỗi dòng là một câu mới, không phải câu cũ viết gọn.** `n = 15` là một lời
  khẳng định khác hẳn `15000 × n − 30000 = 195000`. Thứ nối chúng lại với nhau
  không phải hình dạng, mà là **cùng một tập nghiệm** — lời hứa của bài 13 và
  bài 14.
- **Thứ tự gỡ là ngược thứ tự tính.** Muốn biết gỡ cái nào trước, cứ hỏi: *nếu
  điền một số vào, mình sẽ làm việc gì sau cùng?* Việc ấy gỡ trước.

Câu nào **đưa được về** dạng `một số × ô trống = một số` bằng đúng hai loại
phép của bài 13 và 14 thì gọi là **phương trình bậc nhất** — ô trống chỉ đứng
một mình, không nhân với chính nó, không nằm dưới mẫu. Định nghĩa theo *đưa
được về* chứ không theo hình dạng chữ viết, vì `n + 200 = 500` (bài 12) và
`2n = 12` (bài 14) đều là bậc nhất dù trông chẳng giống cái khuôn nào cả. Cả phần còn lại của khối này chỉ là những
kiểu bậc nhất khác nhau — chữ ở cả hai đĩa, chữ biến mất hẳn — nhưng đồ nghề
thì vẫn đúng hai món bạn đang cầm.
::::

::::code{#hai-loi-mot-dich}
Byte muốn máy đi hộ cả hai con đường, để thấy chúng cùng về một chỗ.

Điền ba chỗ trống. Đừng tính nhẩm rồi gõ thẳng con số vào — hãy để mỗi bước
được dựng từ những cái tên ở phía trên nó, vì chính chuỗi ấy mới là thứ bài này
đi dạy.

Bài chấm bằng cả ba chỗ, và nó khai thác đúng chỗ ba con số ấy quan hệ với
nhau: chỗ đầu phải ra một số **khác** đĩa phải ban đầu (không khác thì bạn mới
động vào một đĩa), còn hai chỗ sau phải ra **cùng** một số (hai con đường, một
cái đích). Gõ cứng một con số vào cả ba chỗ thì hỏng ngay ở câu kiểm thứ hai.

Một lưu ý về màn hình: máy sẽ in `15.0` chứ không phải `15`. Dấu `/` của Python
luôn trả về số lẻ được, đúng như Realm 0 đã báo — vẫn là mười lăm ổ.

```python title=starter
gia_mot_o = 15000
tien_thue = 30000
ve_phai_goc = 195000        # tiền còn trong túi lúc dọn hàng

# ── Cách 1: gỡ cộng trừ trước, nhân chia sau ──
# Bước 1 — cộng tiền thuê vào CẢ HAI đĩa. Đĩa trái còn lại `15000 × n`.
ve_phai_1 = ___
# Bước 2 — chia CẢ HAI đĩa cho giá một ổ. Đĩa trái còn lại `n`.
so_o = ___

# ── Cách 2: gỡ nhân chia trước ──
# Chia CẢ HAI đĩa cho giá một ổ. Đĩa trái có hai cụm, nên chia cả hai cụm:
ve_phai_2 = ve_phai_goc / gia_mot_o        # đĩa phải còn 13,0
tien_thue_theo_o = tien_thue / gia_mot_o   # cụm bị trừ, tính theo ổ: 2,0
# Câu giờ là `n − 2 = 13`. Cộng cụm ấy vào CẢ HAI đĩa:
so_o_cach_2 = ___

print(ve_phai_1)
print(so_o)
print(so_o_cach_2)
```

```python title=solution
gia_mot_o = 15000
tien_thue = 30000
ve_phai_goc = 195000        # tiền còn trong túi lúc dọn hàng

# ── Cách 1: gỡ cộng trừ trước, nhân chia sau ──
# Bước 1 — cộng tiền thuê vào CẢ HAI đĩa. Đĩa trái còn lại `15000 × n`.
ve_phai_1 = ve_phai_goc + tien_thue
# Bước 2 — chia CẢ HAI đĩa cho giá một ổ. Đĩa trái còn lại `n`.
so_o = ve_phai_1 / gia_mot_o

# ── Cách 2: gỡ nhân chia trước ──
# Chia CẢ HAI đĩa cho giá một ổ. Đĩa trái có hai cụm, nên chia cả hai cụm:
ve_phai_2 = ve_phai_goc / gia_mot_o        # đĩa phải còn 13,0
tien_thue_theo_o = tien_thue / gia_mot_o   # cụm bị trừ, tính theo ổ: 2,0
# Câu giờ là `n − 2 = 13`. Cộng cụm ấy vào CẢ HAI đĩa:
so_o_cach_2 = ve_phai_2 + tien_thue_theo_o

print(ve_phai_1)
print(so_o)
print(so_o_cach_2)
```

```python title=test
# Câu `!=` đứng trước: nó canh cái bẫy "để nguyên đĩa phải". Xếp nó sau các câu
# `==` thì nó không bao giờ chạy tới, và cái bẫy không bao giờ sập.
assert ve_phai_1 != ve_phai_goc, "cộng tiền thuê vào CẢ HAI đĩa thì đĩa phải phải ĐỔI — nó vẫn đúng bằng 195000 nghĩa là bạn mới chỉ động vào một đĩa"
assert ve_phai_1 == 225000, "gỡ phép trừ trước: 195 000 cộng 30 000 tiền thuê, đĩa phải thành 225 000"
assert so_o == 15, "chia cả hai đĩa cho 15 000 thì đĩa trái còn n, đĩa phải còn 15 — Byte bán 15 ổ"
assert so_o_cach_2 == so_o, "gỡ theo thứ tự ngược phải ra đúng con số ấy; lệch nghĩa là có một cụm ở đĩa trái chưa được chia"
```

:::hints
- kind: attention
  body: Ba dòng đầu đặt tên sẵn cho ba con số của đề. Mỗi chỗ trống chỉ cần hai cái tên đã có ở phía trên nó, nối bằng một dấu phép tính — không chỗ nào cần bạn gõ thêm một con số mới.
- kind: strategy
  body: Đọc lại câu bình luận ngay trên mỗi chỗ trống, nó nói đúng phép phải làm. "Cộng tiền thuê vào cả hai đĩa" thì đĩa phải là đĩa phải cũ cộng tiền thuê. "Chia cả hai đĩa cho giá một ổ" thì đĩa phải là đĩa phải vừa có, đem chia cho giá một ổ. Chỗ cuối cũng vậy, chỉ khác là hai cái tên của nó nằm ngay hai dòng trên.
- kind: one-line
  body: "Ba chỗ lần lượt là `ve_phai_goc + tien_thue`, `ve_phai_1 / gia_mot_o`, `ve_phai_2 + tien_thue_theo_o`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải được dựng từ những cái TÊN đứng phía trên nó, không phải từ con số bạn tính nhẩm ra — gõ thẳng `225000` hay `15` thì chuỗi biến đổi không xuất hiện ở đâu cả, và máy chẳng gỡ giúp bạn bước nào
  requireAst:
  # Khung khởi đầu chưa có dấu cộng nào. Hai bước "cộng vào cả hai đĩa" phải
  # thêm đúng hai dấu, nên luật này chặn được đáp án chép cứng con số.
  - kind: uses-operator, target: +, min: 2
  # Khung khởi đầu có hai dấu chia (hai dòng của cách 2). Bước 2 của cách 1
  # phải thêm một dấu nữa.
  - kind: uses-operator, target: /, min: 3
  # Sáu luật tên dưới đây buộc từng chỗ trống phải ĐỌC đúng những cái tên
  # đứng trên nó. Con số ghi sau `min:` là số lần khung khởi đầu đã đọc tên
  # ấy, cộng thêm số lần lời giải phải đọc nó.
  - kind: uses-name, target: ve_phai_goc, min: 2
  - kind: uses-name, target: tien_thue, min: 2
  - kind: uses-name, target: ve_phai_1, min: 2
  - kind: uses-name, target: gia_mot_o, min: 3
  - kind: uses-name, target: ve_phai_2, min: 1
  - kind: uses-name, target: tien_thue_theo_o, min: 1
  forbidAst:
  # Lưới thứ hai: bốn con số là KẾT QUẢ của các bước gỡ. Lời giải thật không
  # chứa nguyên văn cái nào, nên bốn luật này không cản ai làm thật.
  - kind: has-literal, target: 225000
  - kind: has-literal, target: 15
  - kind: has-literal, target: 15.0
  - kind: has-literal, target: 13
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^225000\n15\.0\n15\.0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai đường khác nhau, cùng về vạch 15. Ô trống hết chỗ trốn rồi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đếm lại chuỗi vừa đi: câu gốc, một dòng cộng, câu mới, một dòng chia, câu cuối
— năm dòng cho một bài toán khá hiền. Bài khó hơn thì bảy, tám dòng.

Giờ tưởng tượng ở dòng thứ ba Byte viết nhầm `225000` thành `255000`. Chuỗi vẫn
chạy tiếp trơn tru: chia cho 15 000 ra 17, dòng cuối thành `n = 17`. Sạch sẽ,
gọn gàng, đúng hình dạng của một đáp án.

Và đó mới là chỗ đáng sợ: **một đáp án sai trông y hệt một đáp án đúng.** Nó
cũng là một con số, cũng đứng sau dấu `=`, cũng đứng một mình. Không có dấu hiệu
nào trên mặt giấy tố cáo nó.

Byte không muốn nhờ người khác chấm hộ mỗi lần. Vậy:

- Có cách nào **tự** biết con số vừa tìm là đúng không?
- Nếu đem 17 đi thử, thì thử ở dòng nào — dòng cuối cùng vừa viết ra, hay dòng
  nào khác?

Bài sau trả lời, và câu trả lời nằm ở chỗ ít ai nghĩ tới.
::::

::::checkpoint{mastery=0.8}
::::
