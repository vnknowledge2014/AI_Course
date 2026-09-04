---
id: thiet-ke-he-thong.noi-dung-va-kham-pha.fan-out-khi-doc
title: "Fan-out-khi-đọc: gộp bài NGAY lúc xem"
summary: "dangBaiFanOutDoc luôn ghi đúng 1 lần (outbox riêng của tác giả), bất kể tác giả có 0 hay 5 follower -- khác hẳn fan-out-khi-ghi (bài trước, cùng kịch bản 'an' 3 follower tốn 3 lần ghi/bài, 2 bài tốn 6 lần). Đổi lại, xemFeed phải GỘP outbox của MỌI tác giả đang theo dõi tại thời điểm đọc -- và vì outbox không lưu 'lúc nào bắt đầu theo dõi', người đọc MỚI (theo dõi sau khi cả 2 bài đã đăng) vẫn thấy ĐỦ CẢ 2 bài, khác hẳn hành vi inbox ở bài trước (nơi follower mới chỉ thấy bài đăng sau khi theo dõi)."
locale: vi
track: thiet-ke-he-thong
module: noi-dung-va-kham-pha
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [sd.fan-out-khi-doc]
requires: [sd.fan-out-khi-ghi]
concepts: [sd.fan-out-khi-doc]
gradingMatrix:
  web-chrome: [run, tests, output]
  web-firefox: [run, tests, output]
  macos: [run, tests, output]
  windows: [run, tests, output]
  linux: [run, tests, output]
  android: [run, tests, output]
  ios: [run, tests, output]
provenance:
  authoredBy: llm-assisted
  reviewed: false
---

::::byte{trigger=enter mood=curious pose=lean-in}
Bài trước đẩy bài viết đi NGAY lúc đăng — đọc RẺ, ghi ĐẮT theo số
follower. Có một cách LÀM NGƯỢC HẲN: đừng đẩy đi đâu CẢ, cứ để MỖI
người đọc TỰ gộp lúc họ MỞ feed lên.
::::

::::explain{#fan-out-khi-doc}
Fan-out-khi-đọc (fan-out-on-read) không hề CÓ inbox — mỗi tác giả chỉ
giữ MỘT `outbox` chứa TOÀN bộ bài của chính mình. Đăng bài chỉ LÀ một
lần ghi VÀO đúng outbox ĐÓ. Việc "xem feed" mới thật sự TỐN công: phải
LẶP qua từng tác giả đang theo dõi VÀ gộp outbox của HỌ lại:

```typescript title=readonly
interface Bai { tacGia: string; noiDung: string; }
interface HeThongOutbox { outbox: Map<string, Bai[]>; }
function taoHeThongOutbox(): HeThongOutbox { return { outbox: new Map() }; }

function dangBaiFanOutDoc(ht: HeThongOutbox, tacGia: string, noiDung: string): number {
  const dsBai = ht.outbox.get(tacGia) ?? [];
  dsBai.push({ tacGia, noiDung });
  ht.outbox.set(tacGia, dsBai);
  return 1;
}

function xemFeed(ht: HeThongOutbox, dsDangTheoDoi: string[]): Bai[] {
  const ketQua: Bai[] = [];
  for (const tacGia of dsDangTheoDoi) {
    const baiCuaHo = ht.outbox.get(tacGia) ?? [];
    for (const b of baiCuaHo) ketQua.push(b);
  }
  return ketQua;
}

const ht = taoHeThongOutbox();
console.log("so lan ghi khi an dang bai:", dangBaiFanOutDoc(ht, "an", "xin chao"));
console.log("feed cua binh (theo doi an):", JSON.stringify(xemFeed(ht, ["an"])));
```

```text title=readonly
so lan ghi khi an dang bai: 1
feed cua binh (theo doi an): [{"tacGia":"an","noiDung":"xin chao"}]
```

`dangBaiFanOutDoc` LUÔN trả về `1` — chỉ MỘT lần ghi, bất kể "an" có
BAO nhiêu follower, vì hàm không hề BIẾT tới danh sách follower. Toàn bộ
công việc "ai theo dõi ai" bị DỜI sang `xemFeed`, chạy MỖI khi một người
MỞ ứng dụng lên.
::::

::::example{#chi-phi-ghi-khong-doi}
So VỚI bài trước — cùng kịch bản "an" có 3 follower, đăng 2 bài — fan-out-
khi-ghi tốn `3 + 3 = 6` lần ghi. Ở đây, chi phí GHI không hề phụ thuộc số
follower, dù tác giả đó có 0 hay HÀNG nghìn follower Ở một hệ thống theo
dõi khác:

```typescript title=readonly
interface Bai { tacGia: string; noiDung: string; }
interface HeThongOutbox { outbox: Map<string, Bai[]>; }
function taoHeThongOutbox(): HeThongOutbox { return { outbox: new Map() }; }

function dangBaiFanOutDoc(ht: HeThongOutbox, tacGia: string, noiDung: string): number {
  const dsBai = ht.outbox.get(tacGia) ?? [];
  dsBai.push({ tacGia, noiDung });
  ht.outbox.set(tacGia, dsBai);
  return 1;
}

const ht1 = taoHeThongOutbox();
console.log("ghi bai 1 cua an:", dangBaiFanOutDoc(ht1, "an", "bai 1"));
console.log("ghi bai 2 cua an:", dangBaiFanOutDoc(ht1, "an", "bai 2"));
console.log("tong so lan ghi cho 2 bai: 2 (so voi fan-out-khi-ghi cung kich ban: 3+3=6)");

const ht2 = taoHeThongOutbox();
for (let i = 0; i < 5000; i++) dangBaiFanOutDoc(ht2, "nguoi-noi-tieng", `bai-${i}`);
console.log("nguoi-noi-tieng dang 1 bai MOI, du co hang nghin follower o he thong khac, van chi ghi:", dangBaiFanOutDoc(ht2, "nguoi-noi-tieng", "bai moi"));
```

```text title=readonly
ghi bai 1 cua an: 1
ghi bai 2 cua an: 1
tong so lan ghi cho 2 bai: 2 (so voi fan-out-khi-ghi cung kich ban: 3+3=6)
nguoi-noi-tieng dang 1 bai MOI, du co hang nghin follower o he thong khac, van chi ghi: 1
```

Ngay CẢ khi `"nguoi-noi-tieng"` đã có sẵn `5000` bài trong outbox, lần
GHI thứ `5001` vẫn tốn ĐÚNG `1` — vì `dangBaiFanOutDoc` chỉ chạm ĐÚNG
một outbox, không hề động TỚI follower nào cả. Đây LÀ điểm ngược hẳn với
bài trước, nơi "nổi tiếng" đồng nghĩa VỚI ghi đắt.
::::

::::predict{#doan-gop-toan-bo-lich-su commitOnce}
"an" đăng `"bai dau tien"` rồi `"bai thu hai"`. SAU khi cả hai đã đăng
xong, `"em"` MỚI bắt đầu theo dõi "an" (đưa `"an"` vào `dsDangTheoDoi`
của `"em"`). Gọi `xemFeed` cho `"em"` NGAY bây giờ — kết quả có bao
nhiêu bài?

:::opt{correct}
Cả 2 bài — `xemFeed` chỉ đọc TOÀN bộ `outbox` của tác giả TẠI thời điểm
gọi, nó không hề "nhớ" ai bắt đầu theo dõi TỪ lúc nào, nên người đọc MỚI
vẫn thấy đủ LỊCH sử của tác giả họ theo dõi
:::
:::opt
Chỉ 1 bài — giống hệt bài TRƯỚC (fan-out-khi-ghi), người theo dõi MỚI
chỉ thấy bài đăng SAU khi họ bắt đầu theo dõi
::why
Nhầm hành vi của outbox (fan-out-khi-đọc) VỚI hành vi của inbox (fan-out-
khi-ghi, bài trước) — nhưng hai cấu trúc dữ liệu NÀY ghi nhớ những thứ
khác nhau hoàn toàn.

Chỗ lệch: inbox (bài trước) chỉ nhận bài LÚC nó được fan-out, nên bỏ lỡ
mọi bài ĐĂNG trước khi theo dõi. Outbox Ở đây không hề có khái niệm "lúc
nào theo dõi" — `xemFeed` chỉ đơn giản LẶP qua TOÀN bộ mảng
`ht.outbox.get("an")` mỗi lần được gọi, và mảng ĐÓ chứa cả `"bai dau
tien"` lẫn `"bai thu hai"`, bất kể `"em"` theo dõi từ LÚC nào.
::
:::
::::

::::code{#viet_xem_feed}
Hoàn thiện `xemFeed` — với MỖI tác giả trong `dsDangTheoDoi`, gộp TOÀN
bộ bài của họ (đọc từ `baiCuaHo`) vào mảng kết quả.

```typescript title=starter
interface Bai { tacGia: string; noiDung: string; }
interface HeThongOutbox { outbox: Map<string, Bai[]>; }
function taoHeThongOutbox(): HeThongOutbox { return { outbox: new Map() }; }

function dangBaiFanOutDoc(ht: HeThongOutbox, tacGia: string, noiDung: string): number {
  const dsBai = ht.outbox.get(tacGia) ?? [];
  dsBai.push({ tacGia, noiDung });
  ht.outbox.set(tacGia, dsBai);
  return 1;
}

function xemFeed(ht: HeThongOutbox, dsDangTheoDoi: string[]): Bai[] {
  const ketQua: Bai[] = [];
  for (const tacGia of dsDangTheoDoi) {
    const baiCuaHo = ht.outbox.get(tacGia) ?? [];
    ___
  }
  return ketQua;
}

const ht = taoHeThongOutbox();
dangBaiFanOutDoc(ht, "an", "bai dau tien");
dangBaiFanOutDoc(ht, "an", "bai thu hai");
console.log(JSON.stringify(xemFeed(ht, ["an"])));
```

```typescript title=solution
interface Bai { tacGia: string; noiDung: string; }
interface HeThongOutbox { outbox: Map<string, Bai[]>; }
function taoHeThongOutbox(): HeThongOutbox { return { outbox: new Map() }; }

function dangBaiFanOutDoc(ht: HeThongOutbox, tacGia: string, noiDung: string): number {
  const dsBai = ht.outbox.get(tacGia) ?? [];
  dsBai.push({ tacGia, noiDung });
  ht.outbox.set(tacGia, dsBai);
  return 1;
}

function xemFeed(ht: HeThongOutbox, dsDangTheoDoi: string[]): Bai[] {
  const ketQua: Bai[] = [];
  for (const tacGia of dsDangTheoDoi) {
    const baiCuaHo = ht.outbox.get(tacGia) ?? [];
    for (const b of baiCuaHo) ketQua.push(b);
  }
  return ketQua;
}

const ht = taoHeThongOutbox();
dangBaiFanOutDoc(ht, "an", "bai dau tien");
dangBaiFanOutDoc(ht, "an", "bai thu hai");
console.log(JSON.stringify(xemFeed(ht, ["an"])));
```

```typescript title=test
const htT = taoHeThongOutbox();
if (dangBaiFanOutDoc(htT, "an", "xin chao") !== 1) throw new Error("dangBaiFanOutDoc phai luon tra ve 1, bat ke so follower");

const htNhieuFollower = taoHeThongOutbox();
if (dangBaiFanOutDoc(htNhieuFollower, "nguoi-noi-tieng", "bai moi") !== 1) throw new Error("chi phi ghi khong phu thuoc so follower, van la 1");

const ht2 = taoHeThongOutbox();
dangBaiFanOutDoc(ht2, "an", "bai dau tien");
dangBaiFanOutDoc(ht2, "an", "bai thu hai");
const feedBinh = xemFeed(ht2, ["an"]);
if (feedBinh.length !== 2) throw new Error("feed phai gom du 2 bai cua an");
if (feedBinh[0]?.noiDung !== "bai dau tien" || feedBinh[1]?.noiDung !== "bai thu hai") throw new Error("thu tu bai trong feed phai dung thu tu dang");

const feedEm = xemFeed(ht2, ["an"]);
if (feedEm.length !== 2) throw new Error("nguoi doc moi van thay TOAN BO lich su cua tac gia ho theo doi, khong chi bai moi");

const ht3 = taoHeThongOutbox();
dangBaiFanOutDoc(ht3, "an", "an-1");
dangBaiFanOutDoc(ht3, "binh", "binh-1");
const feedGop = xemFeed(ht3, ["an", "binh"]);
if (feedGop.length !== 2) throw new Error("feed phai gop bai tu CA HAI tac gia duoc theo doi");
if (feedGop[0]?.tacGia !== "an" || feedGop[1]?.tacGia !== "binh") throw new Error("thu tu gop phai theo thu tu danh sach dsDangTheoDoi");

if (xemFeed(ht3, ["chua-dang-bai-nao"]).length !== 0) throw new Error("tac gia chua dang bai nao phai cho feed rong");
if (xemFeed(ht3, []).length !== 0) throw new Error("danh sach theo doi rong phai cho feed rong");
```

:::hints
- kind: attention
  body: "Voi tung tac gia dang xet, day TAT CA bai trong baiCuaHo vao mang ketQua -- mot vong lap for long ben trong."
- kind: strategy
  body: "for (const b of baiCuaHo) ketQua.push(b); -- lap qua mang bai cua tac gia nay va them tung bai vao ketQua."
- kind: one-line
  body: "for (const b of baiCuaHo) ketQua.push(b);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 5000
- tier: output
  match: contains
  expect: "bai dau tien"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ghi rẻ, đọc phải GỘP — đúng NGƯỢC với bài trước. Nhưng CÓ một trường
hợp làm cách nào CŨNG khổ: một tác giả có HÀNG triệu follower.
::::

::::reflect{#nghi-lai}
`dangBaiFanOutDoc` VÀ `dangBaiFanOutGhi` (bài trước) giải quyết CÙNG một
bài toán bằng cách DỜI chi phí sang hai THỜI điểm khác nhau — lúc GHI hay
lúc ĐỌC. Không có cách NÀO "đúng" tuyệt đối; chọn cách nào phụ thuộc vào
việc hệ thống của bạn có nhiều LƯỢT đọc hơn lượt GHI hay ngược lại.
::::

::::checkpoint{mastery=0.68}
::::
