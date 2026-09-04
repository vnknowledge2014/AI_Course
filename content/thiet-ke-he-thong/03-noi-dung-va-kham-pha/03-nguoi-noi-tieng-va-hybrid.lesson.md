---
id: thiet-ke-he-thong.noi-dung-va-kham-pha.nguoi-noi-tieng-va-hybrid
title: "Người nổi tiếng và chiến lược hybrid"
summary: "dangBaiHybrid dùng NGUONG_NOI_TIENG=3: tác giả có <=3 follower vẫn fan-out-khi-ghi (ghi N lần vào inbox); tác giả >3 follower (kể cả 1000) hoàn toàn bỏ qua fan-out, ghi 0 lần vào inbox, chỉ ghi 1 lần vào outbox để đọc-khi-cần. Ranh giới đúng TẠI ngưỡng (dùng <=): tác giả có ĐÚNG 3 follower vẫn được fan-out đủ 3 lần, không bị đẩy sang nhánh 'nổi tiếng' -- chỉ 4 follower trở lên mới chuyển sang doc-khi-can."
locale: vi
track: thiet-ke-he-thong
module: noi-dung-va-kham-pha
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [sd.nguoi-noi-tieng-va-hybrid]
requires: [sd.fan-out-khi-doc]
concepts: [sd.nguoi-noi-tieng-va-hybrid]
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
Hai bài trước LÀ hai thái cực: đẩy hết lúc GHI, hay gộp hết lúc ĐỌC. Cả
hai đều CÓ một điểm yếu chung — khi MỘT tác giả có hàng triệu follower,
"đẩy hết" nghĩa LÀ hàng triệu lần ghi cho MỘT bài đăng.
::::

::::explain{#van-de-nguoi-noi-tieng}
Đây gọi LÀ "celebrity problem": fan-out-khi-ghi (bài 1) cực NHANH khi
đọc, nhưng một tác giả có `1000` follower sẽ khiến MỘT lần đăng tốn
`1000` lần ghi — CÀNG nổi tiếng, càng CHẬM đăng bài. Giải pháp hybrid
chọn chiến lược THEO từng tác giả: follower ÍT (dưới ngưỡng) vẫn
fan-out-khi-ghi NHƯ cũ; follower NHIỀU (vượt ngưỡng) chuyển hẳn sang
fan-out-khi-đọc (bài 2) — chỉ ghi VÀO outbox, để follower TỰ gộp lúc đọc:

```typescript title=readonly
interface Bai { tacGia: string; noiDung: string; }
interface HeThongHybrid {
  nguoiTheoDoi: Map<string, string[]>;
  inbox: Map<string, Bai[]>;
  outbox: Map<string, Bai[]>;
}
function taoHeThongHybrid(): HeThongHybrid {
  return { nguoiTheoDoi: new Map(), inbox: new Map(), outbox: new Map() };
}
function themTheoDoiHybrid(ht: HeThongHybrid, tacGia: string, nguoiMoi: string): void {
  const ds = ht.nguoiTheoDoi.get(tacGia) ?? [];
  ds.push(nguoiMoi);
  ht.nguoiTheoDoi.set(tacGia, ds);
}

const NGUONG_NOI_TIENG = 3;

function dangBaiHybrid(ht: HeThongHybrid, tacGia: string, noiDung: string): number {
  const bai: Bai = { tacGia, noiDung };
  const dsOutbox = ht.outbox.get(tacGia) ?? [];
  dsOutbox.push(bai);
  ht.outbox.set(tacGia, dsOutbox);

  const dsFollower = ht.nguoiTheoDoi.get(tacGia) ?? [];
  if (dsFollower.length <= NGUONG_NOI_TIENG) {
    for (const nguoi of dsFollower) {
      const feed = ht.inbox.get(nguoi) ?? [];
      feed.push(bai);
      ht.inbox.set(nguoi, feed);
    }
    return dsFollower.length;
  }
  return 0;
}

const ht1 = taoHeThongHybrid();
themTheoDoiHybrid(ht1, "an", "binh");
themTheoDoiHybrid(ht1, "an", "chi");
themTheoDoiHybrid(ht1, "an", "dung");
console.log("an co 3 follower (<=nguong), so lan ghi inbox:", dangBaiHybrid(ht1, "an", "xin chao"));

const ht2 = taoHeThongHybrid();
themTheoDoiHybrid(ht2, "sao", "binh");
themTheoDoiHybrid(ht2, "sao", "chi");
themTheoDoiHybrid(ht2, "sao", "dung");
themTheoDoiHybrid(ht2, "sao", "em");
themTheoDoiHybrid(ht2, "sao", "phong");
console.log("sao co 5 follower (>nguong), so lan ghi inbox:", dangBaiHybrid(ht2, "sao", "toi la sao"));
console.log("outbox cua sao sau khi dang:", JSON.stringify(ht2.outbox.get("sao")));
```

```text title=readonly
an co 3 follower (<=nguong), so lan ghi inbox: 3
sao co 5 follower (>nguong), so lan ghi inbox: 0
outbox cua sao sau khi dang: [{"tacGia":"sao","noiDung":"toi la sao"}]
```

"an" (3 follower, KHÔNG vượt ngưỡng) vẫn nhận đủ `3` lần ghi inbox —
GIỐNG hệt bài 1. "sao" (5 follower, VƯỢT ngưỡng) ghi `0` lần vào inbox
NHƯNG vẫn ghi vào `outbox` — follower của "sao" sẽ phải TỰ gộp lúc đọc,
giống bài 2.
::::

::::example{#chi-phi-bi-chan-lai}
Cái LỢI của hybrid: chi phí ghi KHÔNG BAO GIỜ vượt quá `NGUONG_NOI_TIENG`
— dù follower tăng TỪ `3` lên `1000`, số lần ghi inbox VẪN CHỈ LÀ `0`
(chuyển hẳn sang outbox), không hề tăng THEO:

```typescript title=readonly
interface Bai { tacGia: string; noiDung: string; }
interface HeThongHybrid {
  nguoiTheoDoi: Map<string, string[]>;
  inbox: Map<string, Bai[]>;
  outbox: Map<string, Bai[]>;
}
function taoHeThongHybrid(): HeThongHybrid {
  return { nguoiTheoDoi: new Map(), inbox: new Map(), outbox: new Map() };
}
function themTheoDoiHybrid(ht: HeThongHybrid, tacGia: string, nguoiMoi: string): void {
  const ds = ht.nguoiTheoDoi.get(tacGia) ?? [];
  ds.push(nguoiMoi);
  ht.nguoiTheoDoi.set(tacGia, ds);
}

const NGUONG_NOI_TIENG = 3;

function dangBaiHybrid(ht: HeThongHybrid, tacGia: string, noiDung: string): number {
  const bai: Bai = { tacGia, noiDung };
  const dsOutbox = ht.outbox.get(tacGia) ?? [];
  dsOutbox.push(bai);
  ht.outbox.set(tacGia, dsOutbox);

  const dsFollower = ht.nguoiTheoDoi.get(tacGia) ?? [];
  if (dsFollower.length <= NGUONG_NOI_TIENG) {
    for (const nguoi of dsFollower) {
      const feed = ht.inbox.get(nguoi) ?? [];
      feed.push(bai);
      ht.inbox.set(nguoi, feed);
    }
    return dsFollower.length;
  }
  return 0;
}

const ht3 = taoHeThongHybrid();
for (let i = 0; i < 3; i++) themTheoDoiHybrid(ht3, "vua-du-nguong", `f${i}`);
console.log("vua-du-nguong co 3 follower, ghi inbox:", dangBaiHybrid(ht3, "vua-du-nguong", "bai"));

for (let i = 0; i < 1000; i++) themTheoDoiHybrid(ht3, "sieu-sao", `f${i}`);
console.log("sieu-sao co 1000 follower, ghi inbox:", dangBaiHybrid(ht3, "sieu-sao", "bai"));
```

```text title=readonly
vua-du-nguong co 3 follower, ghi inbox: 3
sieu-sao co 1000 follower, ghi inbox: 0
```

Từ `3` follower LÊN `1000` follower, chi phí ghi inbox KHÔNG hề tăng dần
— nó NHẢY thẳng từ `3` xuống `0` NGAY khi vượt ngưỡng, RỒI giữ nguyên Ở
`0` dù follower có tăng thêm bao NHIÊU đi nữa. Đây chính LÀ cách hybrid
"chặn TRẦN" chi phí ghi.
::::

::::predict{#doan-ranh-gioi-nguong commitOnce}
`NGUONG_NOI_TIENG = 3`. Một tác giả có ĐÚNG `3` follower (không hơn,
không kém) đăng một bài. Bài đó có được fan-out-khi-ghi (đẩy vào inbox
của cả 3 follower) không?

:::opt{correct}
CÓ — điều kiện dùng `dsFollower.length <= NGUONG_NOI_TIENG`, VÀ `3 <= 3`
LÀ đúng, nên tác giả nàyVẪN được coi LÀ "chưa nổi tiếng", fan-out-khi-ghi
diễn ra bình thường
:::
:::opt
KHÔNG — "ĐẠT đúng ngưỡng" nghĩa LÀ đã chạm mức giới hạn, nên hệ thống
phải chuyển sang chế độ "nổi tiếng" NGAY tại đó, không đợi vượt qua
::why
Nhầm "chạm ĐÚNG ngưỡng" VỚI "đã VƯỢT ngưỡng" — hai điều kiện NÀY chỉ
giống nhau NẾU code dùng `<` thay VÌ `<=`.

Chỗ lệch: `dangBaiHybrid` viết `if (dsFollower.length <= NGUONG_NOI_TIENG)`
— dùng `<=`, KHÔNG phải `<`. Với `dsFollower.length === 3` VÀ
`NGUONG_NOI_TIENG === 3`, biểu thức `3 <= 3` LÀ `true`, nên nhánh
fan-out-khi-ghi VẪN chạy, trả về `3`. Chỉ khi follower thứ `4` xuất
hiện (`4 <= 3` LÀ `false`) hệ thống mới chuyển sang nhánh "nổi tiếng".
::
:::
::::

::::code{#viet_dang_bai_hybrid}
Hoàn thiện điều kiện quyết định trong `dangBaiHybrid` — chọn fan-out-khi-
ghi (nhánh CÓ vòng lặp) khi số follower CÒN nằm trong ngưỡng cho phép.

```typescript title=starter
interface Bai { tacGia: string; noiDung: string; }
interface HeThongHybrid {
  nguoiTheoDoi: Map<string, string[]>;
  inbox: Map<string, Bai[]>;
  outbox: Map<string, Bai[]>;
}
function taoHeThongHybrid(): HeThongHybrid {
  return { nguoiTheoDoi: new Map(), inbox: new Map(), outbox: new Map() };
}
function themTheoDoiHybrid(ht: HeThongHybrid, tacGia: string, nguoiMoi: string): void {
  const ds = ht.nguoiTheoDoi.get(tacGia) ?? [];
  ds.push(nguoiMoi);
  ht.nguoiTheoDoi.set(tacGia, ds);
}

const NGUONG_NOI_TIENG = 3;

function dangBaiHybrid(ht: HeThongHybrid, tacGia: string, noiDung: string): number {
  const bai: Bai = { tacGia, noiDung };
  const dsOutbox = ht.outbox.get(tacGia) ?? [];
  dsOutbox.push(bai);
  ht.outbox.set(tacGia, dsOutbox);

  const dsFollower = ht.nguoiTheoDoi.get(tacGia) ?? [];
  if (___) {
    for (const nguoi of dsFollower) {
      const feed = ht.inbox.get(nguoi) ?? [];
      feed.push(bai);
      ht.inbox.set(nguoi, feed);
    }
    return dsFollower.length;
  }
  return 0;
}

const ht = taoHeThongHybrid();
themTheoDoiHybrid(ht, "an", "binh");
console.log(dangBaiHybrid(ht, "an", "xin chao"));
```

```typescript title=solution
interface Bai { tacGia: string; noiDung: string; }
interface HeThongHybrid {
  nguoiTheoDoi: Map<string, string[]>;
  inbox: Map<string, Bai[]>;
  outbox: Map<string, Bai[]>;
}
function taoHeThongHybrid(): HeThongHybrid {
  return { nguoiTheoDoi: new Map(), inbox: new Map(), outbox: new Map() };
}
function themTheoDoiHybrid(ht: HeThongHybrid, tacGia: string, nguoiMoi: string): void {
  const ds = ht.nguoiTheoDoi.get(tacGia) ?? [];
  ds.push(nguoiMoi);
  ht.nguoiTheoDoi.set(tacGia, ds);
}

const NGUONG_NOI_TIENG = 3;

function dangBaiHybrid(ht: HeThongHybrid, tacGia: string, noiDung: string): number {
  const bai: Bai = { tacGia, noiDung };
  const dsOutbox = ht.outbox.get(tacGia) ?? [];
  dsOutbox.push(bai);
  ht.outbox.set(tacGia, dsOutbox);

  const dsFollower = ht.nguoiTheoDoi.get(tacGia) ?? [];
  if (dsFollower.length <= NGUONG_NOI_TIENG) {
    for (const nguoi of dsFollower) {
      const feed = ht.inbox.get(nguoi) ?? [];
      feed.push(bai);
      ht.inbox.set(nguoi, feed);
    }
    return dsFollower.length;
  }
  return 0;
}

const ht = taoHeThongHybrid();
themTheoDoiHybrid(ht, "an", "binh");
console.log(dangBaiHybrid(ht, "an", "xin chao"));
```

```typescript title=test
function xemFeedHybridT(ht: HeThongHybrid, nguoiDoc: string, dsTacGiaNoiTieng: string[]): Bai[] {
  const tuInbox = ht.inbox.get(nguoiDoc) ?? [];
  const ketQua = [...tuInbox];
  for (const tacGia of dsTacGiaNoiTieng) {
    const baiCuaHo = ht.outbox.get(tacGia) ?? [];
    for (const b of baiCuaHo) ketQua.push(b);
  }
  return ketQua;
}

const htT = taoHeThongHybrid();
themTheoDoiHybrid(htT, "an", "binh");
themTheoDoiHybrid(htT, "an", "chi");
themTheoDoiHybrid(htT, "an", "dung");
if (dangBaiHybrid(htT, "an", "xin chao") !== 3) throw new Error("an co 3 follower (<=nguong) phai fan-out-khi-ghi, ghi dung 3 lan");
if ((htT.inbox.get("binh") ?? []).length !== 1) throw new Error("binh phai nhan bai qua inbox");

const htSao = taoHeThongHybrid();
for (let i = 0; i < 5; i++) themTheoDoiHybrid(htSao, "sao", `f${i}`);
if (dangBaiHybrid(htSao, "sao", "toi la sao") !== 0) throw new Error("sao co 5 follower (>nguong) KHONG duoc fan-out-khi-ghi, phai ghi 0 lan inbox");
if ((htSao.inbox.get("f0") ?? []).length !== 0) throw new Error("follower cua nguoi noi tieng KHONG duoc nhan bai qua inbox");
if ((htSao.outbox.get("sao") ?? []).length !== 1) throw new Error("nguoi noi tieng VAN phai ghi vao outbox de doc-khi-can");

const docF0 = xemFeedHybridT(htSao, "f0", ["sao"]);
if (docF0.length !== 1) throw new Error("doc feed cua f0 (gop tu outbox cua sao) phai thay 1 bai");
if (docF0[0]?.noiDung !== "toi la sao") throw new Error("noi dung bai doc duoc tu outbox phai khop");

const htRanh = taoHeThongHybrid();
themTheoDoiHybrid(htRanh, "dung-nguong", "f1");
themTheoDoiHybrid(htRanh, "dung-nguong", "f2");
themTheoDoiHybrid(htRanh, "dung-nguong", "f3");
if (dangBaiHybrid(htRanh, "dung-nguong", "bai") !== 3) throw new Error("dung 3 follower (bang nguong) VAN phai fan-out-khi-ghi, ghi 3 lan");

const htVuot = taoHeThongHybrid();
for (let i = 0; i < 4; i++) themTheoDoiHybrid(htVuot, "vuot-nguong", `f${i}`);
if (dangBaiHybrid(htVuot, "vuot-nguong", "bai") !== 0) throw new Error("4 follower (vuot nguong 3) phai chuyen sang doc-khi-can, ghi 0 lan inbox");
```

:::hints
- kind: attention
  body: "Dieu kien if() quyet dinh co chay vong lap fan-out-khi-ghi hay khong, dua tren dsFollower.length so voi NGUONG_NOI_TIENG."
- kind: strategy
  body: "Con <= nguong thi VAN fan-out-khi-ghi (nhu bai 1); vuot qua nguong thi bo qua nhanh nay, roi ve return 0."
- kind: one-line
  body: "dsFollower.length <= NGUONG_NOI_TIENG"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chi phí ghi giờ có TRẦN — hybrid ráp đúng hai chiến lược đã học VÀO một
quyết định DUY nhất dựa trên số follower. News Feed xong. Sang Search
Autocomplete.
::::

::::reflect{#nghi-lai}
`dangBaiHybrid` không PHÁT minh chiến lược mới — nó chỉ CHỌN giữa hai
chiến lược ĐÃ có (bài 1 VÀ bài 2) dựa trên MỘT con số đo được (số
follower). Đây LÀ một khuôn mẫu sẽ lặp lại xuyên suốt Realm 7: khi hai
đánh đổi đối NGHỊCH nhau đều có trường hợp THẮNG, đo đạc để CHỌN đúng
chiến lược theo TỪNG trường hợp thường tốt hơn chọn MỘT chiến lược duy
nhất cho TẤT cả.
::::

::::checkpoint{mastery=0.70}
::::
