---
id: thiet-ke-he-thong.noi-dung-va-kham-pha.fan-out-khi-ghi
title: "Fan-out-khi-ghi: đẩy bài NGAY lúc đăng"
summary: "dangBaiFanOutGhi(ht, inbox, tacGia, noiDung) đẩy NGAY bài viết vào inbox của MỌI follower hiện có -- tác giả 'an' có 3 follower (binh, chi, dung) tốn đúng 3 lần ghi cho 1 bài; tác giả 0 follower tốn 0 lần ghi. Follower MỚI theo dõi SAU khi một bài đã đăng KHÔNG thấy bài đó qua fan-out -- 'em' theo dõi 'an' sau bài 'bai dau tien', chỉ nhận đúng 1 bài ('bai thu hai') trong khi 'binh' (theo dõi từ đầu) nhận đủ cả 2 bài."
locale: vi
track: thiet-ke-he-thong
module: noi-dung-va-kham-pha
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [sd.fan-out-khi-ghi]
requires: [sd.boss-url-rut-gon-va-thu-thap-web]
concepts: [sd.fan-out-khi-ghi]
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
Bốn quest T7.1 ráp XONG một dịch vụ nhỏ: gateway, định danh, rút gọn URL,
crawler. Track MỚI "Thiết kế thực chiến" bắt đầu VỚI một câu hỏi khác hẳn:
làm sao PHỤC vụ nội dung cho HÀNG triệu người ĐỌC cùng lúc? Mảnh đầu tiên —
News Feed — mở đầu bằng một quyết định: khi đăng bài, LÀM gì NGAY lúc đó?
::::

::::explain{#fan-out-khi-ghi}
Một news feed cần trả lời: "bài của những người TÔI theo dõi đâu RỒI?"
Cách thứ NHẤT — fan-out-khi-ghi (fan-out-on-write) — trả lời câu hỏi ĐÓ
NGAY lúc đăng: mỗi follower có MỘT `inbox` riêng, VÀ việc đăng bài lập
tức đẩy bản SAO bài viết vào TỪNG inbox đó. Đọc feed SAU này chỉ còn LÀ
đọc thẳng từ inbox — không cần TÍNH toán gì thêm:

```typescript title=readonly
interface HeThongTheoDoi { nguoiTheoDoi: Map<string, string[]>; }
function taoHeThongTheoDoi(): HeThongTheoDoi { return { nguoiTheoDoi: new Map() }; }
function themTheoDoi(ht: HeThongTheoDoi, tacGia: string, nguoiMoi: string): void {
  const ds = ht.nguoiTheoDoi.get(tacGia) ?? [];
  ds.push(nguoiMoi);
  ht.nguoiTheoDoi.set(tacGia, ds);
}

interface Bai { tacGia: string; noiDung: string; }
type Inbox = Map<string, Bai[]>;
function taoInbox(): Inbox { return new Map(); }

function dangBaiFanOutGhi(ht: HeThongTheoDoi, inbox: Inbox, tacGia: string, noiDung: string): number {
  const dsFollower = ht.nguoiTheoDoi.get(tacGia) ?? [];
  const bai: Bai = { tacGia, noiDung };
  let soLanGhi = 0;
  for (const nguoi of dsFollower) {
    const feedCuaHo = inbox.get(nguoi) ?? [];
    feedCuaHo.push(bai);
    inbox.set(nguoi, feedCuaHo);
    soLanGhi += 1;
  }
  return soLanGhi;
}

const ht = taoHeThongTheoDoi();
themTheoDoi(ht, "an", "binh");
themTheoDoi(ht, "an", "chi");
themTheoDoi(ht, "an", "dung");
const inbox = taoInbox();
const soLanGhi = dangBaiFanOutGhi(ht, inbox, "an", "xin chao");
console.log("so lan ghi (an co 3 follower):", soLanGhi);
console.log("inbox cua binh:", JSON.stringify(inbox.get("binh")));
console.log("inbox cua dung:", JSON.stringify(inbox.get("dung")));
```

```text title=readonly
so lan ghi (an co 3 follower): 3
inbox cua binh: [{"tacGia":"an","noiDung":"xin chao"}]
inbox cua dung: [{"tacGia":"an","noiDung":"xin chao"}]
```

`dangBaiFanOutGhi` KHÔNG hề "lưu một bản" rồi CHỜ ai đó đọc — nó LẶP qua
TỪNG follower VÀ ghi trực tiếp vào inbox của HỌ. "an" có 3 follower NÊN
tốn ĐÚNG 3 lần ghi cho một bài — chi phí GHI tỷ lệ THUẬN với số follower,
đổi LẠI chi phí ĐỌC sau này gần như bằng KHÔNG (chỉ đọc thẳng inbox).
::::

::::example{#chi-phi-ghi-ty-le-follower}
Chi phí GHI của fan-out-khi-ghi phụ thuộc HOÀN TOÀN vào số follower, KHÔNG
phải vào độ dài bài viết HAY bất cứ điều gì khác. Tác giả 0 follower tốn
0 lần ghi; tác giả 5 follower tốn ĐÚNG 5 lần — số lần ghi LUÔN khớp số
follower:

```typescript title=readonly
interface HeThongTheoDoi { nguoiTheoDoi: Map<string, string[]>; }
function taoHeThongTheoDoi(): HeThongTheoDoi { return { nguoiTheoDoi: new Map() }; }
function themTheoDoi(ht: HeThongTheoDoi, tacGia: string, nguoiMoi: string): void {
  const ds = ht.nguoiTheoDoi.get(tacGia) ?? [];
  ds.push(nguoiMoi);
  ht.nguoiTheoDoi.set(tacGia, ds);
}

interface Bai { tacGia: string; noiDung: string; }
type Inbox = Map<string, Bai[]>;
function taoInbox(): Inbox { return new Map(); }

function dangBaiFanOutGhi(ht: HeThongTheoDoi, inbox: Inbox, tacGia: string, noiDung: string): number {
  const dsFollower = ht.nguoiTheoDoi.get(tacGia) ?? [];
  const bai: Bai = { tacGia, noiDung };
  let soLanGhi = 0;
  for (const nguoi of dsFollower) {
    const feedCuaHo = inbox.get(nguoi) ?? [];
    feedCuaHo.push(bai);
    inbox.set(nguoi, feedCuaHo);
    soLanGhi += 1;
  }
  return soLanGhi;
}

const ht = taoHeThongTheoDoi();
themTheoDoi(ht, "nam-follower", "x1");
themTheoDoi(ht, "nam-follower", "x2");
themTheoDoi(ht, "nam-follower", "x3");
themTheoDoi(ht, "nam-follower", "x4");
themTheoDoi(ht, "nam-follower", "x5");
const inbox = taoInbox();

console.log("ghi cho tac gia 0 follower:", dangBaiFanOutGhi(ht, inbox, "khong-follower", "noi dung"));
console.log("ghi cho tac gia 5 follower:", dangBaiFanOutGhi(ht, inbox, "nam-follower", "noi dung"));
```

```text title=readonly
ghi cho tac gia 0 follower: 0
ghi cho tac gia 5 follower: 5
```

Tác giả `"khong-follower"` chưa từng xuất hiện TRONG `nguoiTheoDoi` NÊN
`dsFollower` LÀ mảng rỗng — vòng lặp `for` không chạy LẦN nào, VÀ hàm trả
về `0` mà không hề NÉM lỗi. Đây chính LÀ điều làm fan-out-khi-ghi ĐẮT dần
khi một tài khoản CÀNG có nhiều follower — bài SAU (T7.2a q3) sẽ chỉ ra
tại sao con số NÀY có thể trở thành vấn đề THẬT sự.
::::

::::predict{#doan-follower-moi-khong-thay-bai-cu commitOnce}
"an" có 2 follower: `"binh"` VÀ `"chi"`. "an" đăng bài `"bai dau tien"`
(fan-out NGAY tới cả hai). SAU đó, `"em"` MỚI bắt đầu theo dõi "an". "an"
đăng tiếp `"bai thu hai"`. Ngay LÚC này, inbox của `"em"` chứa bao nhiêu
bài?

:::opt{correct}
Đúng 1 bài (`"bai thu hai"`) — `"em"` theo dõi SAU khi `"bai dau tien"`
đã fan-out xong, NÊN không hề có mặt trong `dsFollower` lúc bài đó được
đẩy đi; chỉ bài đăng SAU khi theo dõi mới tới được inbox của `"em"`
:::
:::opt
Đủ 2 bài, giống hệt `"binh"` — MIỄN LÀ đang theo dõi tại thời điểm ĐỌC
feed, hệ thống sẽ tự BÙ lại mọi bài cũ của tác giả đó
::why
Nhầm fan-out-khi-ghi VỚI một hệ thống "gộp lúc ĐỌC" (đọc feed lúc nào,
lấy đủ lịch sử lúc ĐÓ) — nhưng `dangBaiFanOutGhi` không hề hoạt động theo
kiểu ĐÓ.

Chỗ lệch: `dangBaiFanOutGhi` chỉ ghi vào inbox của những NGƯỜI có mặt
TRONG `ht.nguoiTheoDoi.get(tacGia)` ngay LÚC hàm chạy. Khi `"bai dau
tien"` fan-out, `"em"` CHƯA có trong danh sách đó NÊN không hề nhận được
gì — VÀ không có bước NÀO sau này "bù" lại bài cũ cho follower mới. Chỉ
`"bai thu hai"` (đăng SAU khi `"em"` đã theo dõi) mới tới được inbox của
`"em"`.
::
:::
::::

::::code{#viet_dang_bai_fan_out_ghi}
Hoàn thiện `dangBaiFanOutGhi` — bên TRONG vòng lặp, đẩy bài VÀO feed của
từng follower (tạo mảng MỚI nếu follower đó chưa có inbox).

```typescript title=starter
interface HeThongTheoDoi { nguoiTheoDoi: Map<string, string[]>; }
function taoHeThongTheoDoi(): HeThongTheoDoi { return { nguoiTheoDoi: new Map() }; }
function themTheoDoi(ht: HeThongTheoDoi, tacGia: string, nguoiMoi: string): void {
  const ds = ht.nguoiTheoDoi.get(tacGia) ?? [];
  ds.push(nguoiMoi);
  ht.nguoiTheoDoi.set(tacGia, ds);
}

interface Bai { tacGia: string; noiDung: string; }
type Inbox = Map<string, Bai[]>;
function taoInbox(): Inbox { return new Map(); }

function dangBaiFanOutGhi(ht: HeThongTheoDoi, inbox: Inbox, tacGia: string, noiDung: string): number {
  const dsFollower = ht.nguoiTheoDoi.get(tacGia) ?? [];
  const bai: Bai = { tacGia, noiDung };
  let soLanGhi = 0;
  for (const nguoi of dsFollower) {
    const feedCuaHo = inbox.get(nguoi) ?? [];
    ___
    soLanGhi += 1;
  }
  return soLanGhi;
}

const ht = taoHeThongTheoDoi();
themTheoDoi(ht, "an", "binh");
themTheoDoi(ht, "an", "chi");
const inbox = taoInbox();
console.log(dangBaiFanOutGhi(ht, inbox, "an", "xin chao"), JSON.stringify(inbox.get("binh")));
```

```typescript title=solution
interface HeThongTheoDoi { nguoiTheoDoi: Map<string, string[]>; }
function taoHeThongTheoDoi(): HeThongTheoDoi { return { nguoiTheoDoi: new Map() }; }
function themTheoDoi(ht: HeThongTheoDoi, tacGia: string, nguoiMoi: string): void {
  const ds = ht.nguoiTheoDoi.get(tacGia) ?? [];
  ds.push(nguoiMoi);
  ht.nguoiTheoDoi.set(tacGia, ds);
}

interface Bai { tacGia: string; noiDung: string; }
type Inbox = Map<string, Bai[]>;
function taoInbox(): Inbox { return new Map(); }

function dangBaiFanOutGhi(ht: HeThongTheoDoi, inbox: Inbox, tacGia: string, noiDung: string): number {
  const dsFollower = ht.nguoiTheoDoi.get(tacGia) ?? [];
  const bai: Bai = { tacGia, noiDung };
  let soLanGhi = 0;
  for (const nguoi of dsFollower) {
    const feedCuaHo = inbox.get(nguoi) ?? [];
    feedCuaHo.push(bai);
    inbox.set(nguoi, feedCuaHo);
    soLanGhi += 1;
  }
  return soLanGhi;
}

const ht = taoHeThongTheoDoi();
themTheoDoi(ht, "an", "binh");
themTheoDoi(ht, "an", "chi");
const inbox = taoInbox();
console.log(dangBaiFanOutGhi(ht, inbox, "an", "xin chao"), JSON.stringify(inbox.get("binh")));
```

```typescript title=test
const htT = taoHeThongTheoDoi();
themTheoDoi(htT, "an", "binh");
themTheoDoi(htT, "an", "chi");
themTheoDoi(htT, "an", "dung");
const inboxT = taoInbox();
const soLanGhi = dangBaiFanOutGhi(htT, inboxT, "an", "xin chao");
if (soLanGhi !== 3) throw new Error("an co 3 follower, phai ghi dung 3 lan");
if ((inboxT.get("binh") ?? []).length !== 1) throw new Error("binh phai nhan dung 1 bai");
if (inboxT.get("binh")?.[0]?.noiDung !== "xin chao") throw new Error("noi dung bai trong inbox binh phai khop");
if (inboxT.get("binh")?.[0]?.tacGia !== "an") throw new Error("tac gia bai trong inbox binh phai la an");

const inboxRong = taoInbox();
if (dangBaiFanOutGhi(htT, inboxRong, "khong-ai-theo-doi", "test") !== 0) throw new Error("tac gia 0 follower phai ghi 0 lan");
if (inboxRong.size !== 0) throw new Error("khong follower nao duoc them, inbox phai rong");

const ht2 = taoHeThongTheoDoi();
themTheoDoi(ht2, "an", "binh");
themTheoDoi(ht2, "an", "chi");
const inbox2 = taoInbox();
dangBaiFanOutGhi(ht2, inbox2, "an", "bai dau tien");
themTheoDoi(ht2, "an", "em");
dangBaiFanOutGhi(ht2, inbox2, "an", "bai thu hai");
if ((inbox2.get("binh") ?? []).length !== 2) throw new Error("binh theo doi tu dau phai thay ca 2 bai");
if ((inbox2.get("em") ?? []).length !== 1) throw new Error("em chi theo doi sau bai 1, phai chi thay 1 bai (bai 2)");
if (inbox2.get("em")?.[0]?.noiDung !== "bai thu hai") throw new Error("bai em thay phai la bai thu hai");
```

:::hints
- kind: attention
  body: "Voi tung follower trong vong lap, day 'bai' vao mang feedCuaHo (da lay/tao o dong tren) roi ghi mang do TRO LAI vao inbox."
- kind: strategy
  body: "feedCuaHo.push(bai) them bai vao mang; inbox.set(nguoi, feedCuaHo) ghi mang do vao dung khoa nguoi trong Map."
- kind: one-line
  body: "feedCuaHo.push(bai); inbox.set(nguoi, feedCuaHo);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 5000
- tier: output
  match: contains
  expect: "2"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Fan-out-khi-ghi làm ĐỌC nhanh gần như tức thì — đổi lại GHI tốn công
đúng bằng số follower. Cách còn LẠI làm ngược hẳn: ghi RẺ, đọc mới TÍNH.
::::

::::reflect{#nghi-lai}
`dangBaiFanOutGhi` không hề PHỨC tạp — một vòng lặp VÀ một phép ghi Map.
Điều ĐÁNG nhớ nằm Ở chỗ khác: nó khoá CHẶT một bài viết vào đúng tập
follower TẠI thời điểm đăng. Ai theo dõi SAU đó không hề được "bù" lại —
đây LÀ cái giá thật của việc dời TOÀN bộ công sức sang lúc GHI.
::::

::::checkpoint{mastery=0.66}
::::
