---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.khi-khong-con-the-nao-tro-toi
title: Khi không còn tấm thẻ nào trỏ tới
summary: "Mỗi giá trị mang theo một con số đếm bao nhiêu tấm thẻ đang trỏ vào nó — về lại mức không thẻ nào thì Python dọn nó đi."
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 25
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [mem.refcount]
requires: [mem.pass-by-reference]
concepts: [mem.dem-the, core.bien]
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
Bài trước, đưa một danh sách vào hàm là buộc thêm một tấm thẻ vào cùng cái
nồi. Hàm chạy xong, tấm thẻ ấy hết việc. Cái nồi thì sao?
::::

::::explain{#gia-tri-cu-coi-nhu-khong-con}
Nhớ lại câu R0 đã nói, hồi chị Hạnh chuyển tấm thẻ từ nồi bò sang nồi gà:
gán lại một cái tên thì giá trị cũ "không đi đâu cả" — chỉ có tấm thẻ là đi.
Và bài đó chốt thêm một câu: "Máy dọn nó đi lúc nào thì bạn không cần bận
tâm." Với chương trình, giá trị cũ coi như không còn.

Bài này trả lời đúng câu để lửng đó: **vì sao** máy biết lúc nào để dọn.

Câu trả lời không huyền bí. Mỗi giá trị nằm trong bộ nhớ mang theo một con số
nhỏ, đếm đúng một thứ: **đang có bao nhiêu tấm thẻ trỏ vào nó**. Buộc thêm
một thẻ thì con số tăng một. Gỡ một thẻ ra — vì gán lại tên đó sang chỗ khác,
hay vì hàm chứa cái tên đó vừa chạy xong — thì con số giảm một.

Con số ấy có một cái tên: người ta gọi nó là **đếm thẻ** (tiếng Anh:
*reference count*, viết tắt *refcount*). Về tới mức không còn thẻ nào trỏ
tới nữa, giá trị ấy không ai với ra được nữa — dù nó vẫn còn nằm đâu đó
trong bộ nhớ. Và một giá trị không ai với ra được thì Python coi là rác,
dọn đi để lấy chỗ.

Đây chính là máy đếm mà R0 giấu kín. Bài này mở nắp ra nhìn nó chạy.
::::

::::explain{#hoi-may-dem-the}
Python cho hỏi thẳng con số đó. Hàm `sys.getrefcount(gia_tri)` trả về đúng
con số thẻ đang trỏ vào một giá trị.

Nhưng có một chỗ phải nói ngay, kẻo con số này tự nó lừa bạn: **gọi hàm để
hỏi cũng là một cách trỏ tới giá trị đó** — trong lúc hàm đang chạy, chính
đối số bạn vừa truyền vào cũng tạm thời là một tấm thẻ nữa. Vì vậy con số
`sys.getrefcount` trả về LUÔN cao hơn số thẻ thật đúng một, do chính lượt
hỏi gây ra.

Nói cách khác: đừng nhìn con số ấy như một sự thật tuyệt đối. Nhìn nó **thay
đổi** ra sao giữa hai lần hỏi mới là điều đáng tin — y hệt cách bài 10 từng
dặn với `sys.getsizeof`: ghim quan hệ, đừng ghim con số.
::::

::::example{#khay-topping-cua-co-bay}
Cô Bảy có một khay topping cho bàn số 5:

```python title=readonly
import sys

topping = ["trứng cút", "chả", "hành phi"]
truoc = sys.getrefcount(topping)

phu_bep = topping          # buộc thêm một tấm thẻ vào CÙNG cái khay
giua = sys.getrefcount(topping)

phu_bep = "đã trả khay"     # gỡ tấm thẻ đó ra, không trỏ vào khay nữa
sau = sys.getrefcount(topping)

print(f"Cô Bảy đưa thêm một thẻ: refcount tăng {giua - truoc}")
print(f"Phụ bếp trả thẻ lại: refcount tăng {sau - truoc}")
```

```text title=readonly
Cô Bảy đưa thêm một thẻ: refcount tăng 1
Phụ bếp trả thẻ lại: refcount tăng 0
```

Đọc theo đúng cách vừa dặn — nhìn **hiệu số**, không nhìn `truoc`, `giua`,
`sau` riêng lẻ:

- Dòng `phu_bep = topping` không tạo một khay mới. Nó buộc thêm một cái tên
  vào CHÍNH cái khay `topping` đang trỏ tới — đúng thứ bài 21 gọi là "hai
  tấm thẻ một nồi". Refcount tăng đúng một.
- Dòng `phu_bep = "đã trả khay"` gán lại `phu_bep`, y hệt chị Hạnh chuyển
  thẻ ở R0 bài 12. `phu_bep` không còn trỏ vào khay topping nữa. Refcount
  trở lại đúng mức ban đầu — hiệu số quay về 0.

Suốt lúc đó, cái khay `topping` không hề bị đụng tới. Chỉ có SỐ THẺ trỏ vào
nó là đổi.
::::

::::predict{#doan-hai-hieu-so commitOnce}
Byte viết đoạn này và tạm giấu kết quả:

```python title=readonly
import sys

mon = ["bún", "phở"]
truoc = sys.getrefcount(mon)

ban_sao_ten = mon
giua = sys.getrefcount(mon)

ban_sao_ten = "đã xong việc"
sau = sys.getrefcount(mon)

print(giua - truoc)
print(sau - truoc)
```

**Trước khi xem đáp án**, bạn đoán hai dòng in ra là gì?

:::opt{correct}
1, rồi 0.
:::

:::opt
1, rồi 1.
::why
Gần đúng ở dòng đầu: bạn nhớ đúng rằng buộc thêm một cái tên vào cùng `mon`
làm refcount tăng một. Dòng đó bạn theo dõi rất sát.

Chỗ lệch nằm ở dòng gán lại `ban_sao_ten = "đã xong việc"`. Gán lại một cái
tên không CỘNG THÊM một tấm thẻ nữa — nó GỠ tấm thẻ cũ ra khỏi `mon` rồi
buộc sang chỗ khác, đúng như chị Hạnh chuyển thẻ ở R0 bài 12. Từ giây đó,
`ban_sao_ten` không còn trỏ vào `mon` nữa, nên refcount của `mon` phải tụt
về đúng mức ban đầu — hiệu số là 0, không phải 1.
::
:::

:::opt
0, rồi 0.
::why
Gần đúng ở cảm giác an toàn: nếu `mon` là một con số hay một chữ, thêm một
cái tên khác trỏ vào nó có vẻ không đổi gì đáng kể.

Chỗ lệch: bài 21 đã chỉ ra dòng `ban_sao_ten = mon` không sao chép cái
danh sách — nó buộc thêm một tấm thẻ vào CHÍNH cái danh sách đó. Từ lúc đó
có HAI cái tên cùng trỏ vào một `mon`, và refcount phải phản ánh đúng điều
đó: tăng một, không phải đứng yên.
::
:::

:::opt
2, rồi 1.
::why
Gần đúng ở phần bạn nhớ: đúng là lượt gọi `sys.getrefcount` tự nó cũng tạm
mượn một tấm thẻ, nên mỗi con số trả về đều cao hơn thực tế đúng một — điều
vừa được nhắc ở trên.

Chỗ lệch là bạn cộng thêm phần dôi đó một lần NỮA. Nhưng phần dôi ấy có mặt
ở CẢ HAI lần gọi — cả `truoc` lẫn `giua` đều bị cộng thêm y như nhau — nên
khi lấy hiệu (`giua - truoc`) nó tự triệt tiêu. Không cần trừ tay thêm lần
nào cả; đó chính là lý do bài này dạy nhìn hiệu số thay vì con số tuyệt đối.
::
:::
::::

::::code{#gan-mot-the-roi-go-ra}
Đến lượt bạn. Cô Bảy có một đơn hàng `don_hang`. Bạn cần buộc thêm một tấm
thẻ thứ hai vào **chính** danh sách đó — không phải một danh sách mới trông
giống hệt — rồi gỡ tấm thẻ ấy ra khi xong việc.

```python title=starter
import sys

don_hang = ["phở tái", "phở chín", "quẩy"]

truoc = sys.getrefcount(don_hang)

ban_sao = ___          # buộc thêm một tấm thẻ nữa vào CÙNG don_hang

giua = sys.getrefcount(don_hang)

ban_sao = ___          # gỡ tấm thẻ đó ra, không trỏ vào don_hang nữa

sau = sys.getrefcount(don_hang)

print(f"Thêm một thẻ: refcount tăng {giua - truoc}")
print(f"Gỡ thẻ đó ra: refcount tăng {sau - truoc}")
```

```python title=solution
import sys

don_hang = ["phở tái", "phở chín", "quẩy"]

truoc = sys.getrefcount(don_hang)

ban_sao = don_hang

giua = sys.getrefcount(don_hang)

ban_sao = None

sau = sys.getrefcount(don_hang)

print(f"Thêm một thẻ: refcount tăng {giua - truoc}")
print(f"Gỡ thẻ đó ra: refcount tăng {sau - truoc}")
```

```python title=test
# Chỗ trống thứ nhất: phải buộc thêm một thẻ vào CÙNG don_hang, không phải
# dựng một danh sách mới trông giống hệt. Chép bằng [:] cũng là một danh
# sách khác — cả hai cách hụt đều khiến refcount không hề tăng.
assert giua - truoc == 1, "sau khi gán ban_sao vào CÙNG don_hang, refcount phải tăng đúng 1 — nếu bạn dựng một danh sách mới (kể cả bằng don_hang[:]) thì đây sẽ là 0, vì đó là hai vật khác nhau"
# Chỗ trống thứ hai: phải gỡ đúng tấm thẻ ban_sao ra khỏi don_hang.
assert sau - truoc == 0, "sau khi gỡ ban_sao ra, refcount của don_hang phải quay lại đúng mức ban đầu — nếu chỗ trống thứ hai không thật sự gỡ ban_sao (ví dụ gán lại ban_sao = don_hang lần nữa), số này sẽ vẫn là 1"
# don_hang là thứ chỉ đọc, không phải khoản để sửa trong bài này.
assert don_hang == ["phở tái", "phở chín", "quẩy"], "don_hang phải còn nguyên ba món, bài này không đổi nội dung đơn hàng"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất nằm ở vế phải của phép gán cho `ban_sao`, ngay dưới dòng đo `truoc`. Chỗ trống thứ hai nằm ở dòng `ban_sao = ___` phía dưới, sau khi đã đo `giua`.
- kind: strategy
  body: Buộc thêm một tấm thẻ vào CÙNG một danh sách nghĩa là gõ đúng cái tên đang trỏ vào nó — đừng gõ lại `["phở tái", "phở chín", "quẩy"]` hay `don_hang[:]`, cả hai đều dựng ra một danh sách KHÁC, chỉ trông giống hệt. Gỡ tấm thẻ ra thì làm đúng như chị Hạnh chuyển thẻ ở R0 — gán `ban_sao` sang một giá trị không liên quan gì tới don_hang.
- kind: one-line
  body: 'Chỗ trống thứ nhất viết `don_hang`, chỗ trống thứ hai viết `None`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: static
  onFail: chỗ trống thứ nhất phải THAM CHIẾU đúng don_hang — không phải một danh sách mới dựng riêng, dù trông giống hệt
  requireAst:
  - kind: uses-name, target: don_hang, min: 4
- tier: output
  match: regex
  expect: ^Thêm một thẻ: refcount tăng 1\nGỡ thẻ đó ra: refcount tăng 0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Con số đó vừa lộ mặt. Nó tăng khi có thêm thẻ, giảm khi thẻ rời đi — và về
lại không thẻ nào là lúc Python coi giá trị ấy đã xong việc.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cách đếm thẻ vừa học chạy tốt khi các tấm thẻ chỉ trỏ MỘT CHIỀU: từ một cái
tên, ra một cái nồi. Gỡ hết các chiều ấy thì con số về không, và Python dọn.

Nhưng nếu không phải một cái tên trỏ vào một cái nồi, mà là **hai cái nồi
trỏ thẳng vào nhau** — nồi A giữ một tấm thẻ chỉ sang nồi B, và nồi B cũng
giữ một tấm thẻ chỉ ngược lại sang nồi A — thì sao? Bạn gỡ hết tên bên ngoài
khỏi cả hai. Con số đếm thẻ của A và B có bao giờ về không được không?

Đừng trả lời vội. Bài sau đi thẳng vào đúng cái bẫy đó.
::::

::::checkpoint{mastery=0.8}
::::
