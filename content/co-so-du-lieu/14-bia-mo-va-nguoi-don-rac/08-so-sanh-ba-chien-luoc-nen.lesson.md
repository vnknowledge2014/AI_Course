---
id: co-so-du-lieu.bia-mo-va-nguoi-don-rac.so-sanh-ba-chien-luoc-nen
title: "So sánh ba chiến lược nén"
summary: "soSanhBaChienLuoc chạy CÙNG một khối lượng (16 lần flush) qua ba cách tổ chức: không nén (16 SSTable, đọc tối đa 16), STCS nguongMin=4 (dùng lại moPhongSTCS bài 5 -- cuối cùng gộp về 1 bảng, nhưng đọc tối đa 6 trong quá trình), LCS 2 tầng (dùng lại chiaThanhCacManh bài 6 -- 5 bảng, đọc tối đa CHỈ 2). Không chiến lược nào thắng tuyệt đối trên MỌI chỉ số cùng lúc."
locale: vi
track: co-so-du-lieu
module: bia-mo-va-nguoi-don-rac
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [db.so-sanh-ba-chien-luoc-nen]
requires: [db.nen-theo-cua-so-thoi-gian-twcs]
concepts: [db.so-sanh-ba-chien-luoc-nen]
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
STCS giữ số bảng THẤP (bài 5). LCS giữ số bảng CẦN đọc THẤP hơn nữa
(bài 6). TWCS drop ĐƯỢC nguyên cửa sổ (bài 7). Đo CẢ ba trên CÙNG một
khối lượng — chiến lược NÀO thật sự "tốt nhất"?
::::

::::explain{#so-sanh-lab}
`soSanhBaChienLuoc` chạy CÙNG `16` lần flush QUA ba cách tổ chức khác
nhau, dùng LẠI đúng các hàm ĐÃ verify Ở bài 5 (`moPhongSTCS`) VÀ bài
6 (`chiaThanhCacManh`, `soManhChuaKhoa`):

```typescript title=readonly
function moPhongSTCS(soLanFlush: number, nguongMin: number): number[] {
  const cacBang: number[] = [];
  const dauSoBang: number[] = [];
  for (let lan = 0; lan < soLanFlush; lan++) {
    cacBang.push(1);
    let doiMoi = true;
    while (doiMoi) {
      doiMoi = false;
      const theoKichCo = new Map<number, number[]>();
      cacBang.forEach((kc, i) => {
        const ds = theoKichCo.get(kc) ?? [];
        ds.push(i);
        theoKichCo.set(kc, ds);
      });
      for (const [kc, ds] of theoKichCo) {
        if (ds.length >= nguongMin) {
          const idsGop = new Set(ds.slice(0, nguongMin));
          const conLai = cacBang.filter((_, i) => !idsGop.has(i));
          conLai.push(kc * nguongMin);
          cacBang.length = 0;
          cacBang.push(...conLai);
          doiMoi = true;
          break;
        }
      }
    }
    dauSoBang.push(cacBang.length);
  }
  return dauSoBang;
}

interface PhamVi { min: number; max: number; }
function chiaThanhCacManh(soManh: number, tongKichCo: number): PhamVi[] {
  const ketQua: PhamVi[] = [];
  const kichCoManh = tongKichCo / soManh;
  for (let i = 0; i < soManh; i++) ketQua.push({ min: i * kichCoManh, max: (i + 1) * kichCoManh - 1 });
  return ketQua;
}
function soManhChuaKhoa(cacManh: PhamVi[], khoa: number): number {
  return cacManh.filter((m) => khoa >= m.min && khoa <= m.max).length;
}

interface KetQuaChienLuoc { ten: string; soSSTable: number; docToiDa: number; }

function soSanhBaChienLuoc(soLanFlush: number): KetQuaChienLuoc[] {
  const khongNen: KetQuaChienLuoc = { ten: "khong-nen", soSSTable: soLanFlush, docToiDa: soLanFlush };

  const daySoBangSTCS = moPhongSTCS(soLanFlush, 4);
  const stcs: KetQuaChienLuoc = {
    ten: "stcs",
    soSSTable: daySoBangSTCS.at(-1)!,
    docToiDa: Math.max(...daySoBangSTCS),
  };

  const tang0 = chiaThanhCacManh(1, soLanFlush);
  const tang1 = chiaThanhCacManh(4, soLanFlush);
  const lcs: KetQuaChienLuoc = {
    ten: "lcs",
    soSSTable: tang0.length + tang1.length,
    docToiDa: soManhChuaKhoa(tang0, 0) + soManhChuaKhoa(tang1, 0),
  };

  return [khongNen, stcs, lcs];
}

console.log(soSanhBaChienLuoc(16));
```

```text title=readonly
[
  { ten: 'khong-nen', soSSTable: 16, docToiDa: 16 },
  { ten: 'stcs', soSSTable: 1, docToiDa: 6 },
  { ten: 'lcs', soSSTable: 5, docToiDa: 2 }
]
```

CÙNG `16` lần flush: KHÔNG nén giữ NGUYÊN `16` SSTable rời rạc, đọc
tối đa CHÍNH nó (`16`). STCS gộp DẦN xuống còn `1` bảng CUỐI cùng,
nhưng TRONG quá trình đã CÓ lúc phải đọc TỚI `6` bảng. LCS giữ `5`
bảng (nhiều HƠN STCS's `1`), nhưng đọc TỐI đa chỉ `2` — THẤP nhất
trong BA.
::::

::::example{#khong-thang-tuyet-doi}
KHÔNG chiến lược NÀO thắng Ở CẢ hai chỉ số CÙNG lúc: STCS có `soSSTable`
THẤP nhất (`1`) nhưng `docToiDa` KHÔNG phải thấp NHẤT (`6`); LCS có
`docToiDa` thấp NHẤT (`2`) nhưng `soSSTable` KHÔNG thấp nhất (`5`).
Đây chính LÀ đánh đổi ghi‑so‑với‑đọc: LCS phải viết LẠI nhiều hơn để
giữ CÁC tầng không chồng lấn (write amplification cao HƠN — khái
niệm ĐÃ đo Ở q04 bài 13), đổi lấy đọc DỰ đoán được hơn.
::::

::::predict{#doan-flush-nhieu-hon commitOnce}
Gọi `soSanhBaChienLuoc(64)` (nhiều lần flush HƠN). So VỚI `docToiDa`
Ở `16` lần flush (khong-nen:16, stcs:6, lcs:2), khi TĂNG lên `64`
lần, `docToiDa` của "khong-nen" VÀ "lcs" thay đổi thế NÀO?

:::opt{correct}
"khong-nen" TĂNG lên `64` (LUÔN bằng đúng số lần flush — không giới
hạn); "lcs" VẪN LÀ `2` (chỉ phụ thuộc SỐ tầng — CỐ định Ở 2 tầng,
không phụ thuộc tổng dữ liệu)
:::

:::opt
CẢ hai đều TĂNG theo cùng TỈ lệ VỚI số lần flush — nhiều dữ liệu HƠN
luôn nghĩa LÀ đọc chậm hơn, bất kể chiến lược NÀO
::why
Trực giác "nhiều dữ LIỆU hơn luôn đọc chậm HƠN" đúng cho "khong-nen"
(hoàn TOÀN không có cấu trúc để LOẠI trừ bảng nào) — nhưng SAI cho
LCS.

Chỗ lệch: `docLCS` (bài 6) LUÔN bằng ĐÚNG số tầng — MỖI tầng đóng góp
`0` hoặc `1` bảng CẦN đọc, bất kể tầng đó có bao nhiêu MẢNH hay tổng
dữ liệu lớn cỡ NÀO. Đây chính LÀ điểm mạnh CỐT lõi của thiết kế theo
tầng: đọc KHÔNG phụ thuộc và tổng khối LƯỢNG dữ liệu, chỉ phụ thuộc
số tầng.
::
:::
::::

::::code{#viet_so_sanh_ba_chien_luoc}
Hoàn thiện `soSanhBaChienLuoc` — tính `docToiDa` cho STCS bằng giá
trị LỚN nhất xuất hiện TRONG suốt quá trình (`daySoBangSTCS`).

```typescript title=starter
function moPhongSTCS(soLanFlush: number, nguongMin: number): number[] {
  const cacBang: number[] = [];
  const dauSoBang: number[] = [];
  for (let lan = 0; lan < soLanFlush; lan++) {
    cacBang.push(1);
    let doiMoi = true;
    while (doiMoi) {
      doiMoi = false;
      const theoKichCo = new Map<number, number[]>();
      cacBang.forEach((kc, i) => {
        const ds = theoKichCo.get(kc) ?? [];
        ds.push(i);
        theoKichCo.set(kc, ds);
      });
      for (const [kc, ds] of theoKichCo) {
        if (ds.length >= nguongMin) {
          const idsGop = new Set(ds.slice(0, nguongMin));
          const conLai = cacBang.filter((_, i) => !idsGop.has(i));
          conLai.push(kc * nguongMin);
          cacBang.length = 0;
          cacBang.push(...conLai);
          doiMoi = true;
          break;
        }
      }
    }
    dauSoBang.push(cacBang.length);
  }
  return dauSoBang;
}

interface PhamVi { min: number; max: number; }
function chiaThanhCacManh(soManh: number, tongKichCo: number): PhamVi[] {
  const ketQua: PhamVi[] = [];
  const kichCoManh = tongKichCo / soManh;
  for (let i = 0; i < soManh; i++) ketQua.push({ min: i * kichCoManh, max: (i + 1) * kichCoManh - 1 });
  return ketQua;
}
function soManhChuaKhoa(cacManh: PhamVi[], khoa: number): number {
  return cacManh.filter((m) => khoa >= m.min && khoa <= m.max).length;
}

interface KetQuaChienLuoc { ten: string; soSSTable: number; docToiDa: number; }

function soSanhBaChienLuoc(soLanFlush: number): KetQuaChienLuoc[] {
  const khongNen: KetQuaChienLuoc = { ten: "khong-nen", soSSTable: soLanFlush, docToiDa: soLanFlush };

  const daySoBangSTCS = moPhongSTCS(soLanFlush, 4);
  const stcs: KetQuaChienLuoc = {
    ten: "stcs",
    soSSTable: daySoBangSTCS.at(-1)!,
    docToiDa: ___,
  };

  const tang0 = chiaThanhCacManh(1, soLanFlush);
  const tang1 = chiaThanhCacManh(4, soLanFlush);
  const lcs: KetQuaChienLuoc = {
    ten: "lcs",
    soSSTable: tang0.length + tang1.length,
    docToiDa: soManhChuaKhoa(tang0, 0) + soManhChuaKhoa(tang1, 0),
  };

  return [khongNen, stcs, lcs];
}

console.log(soSanhBaChienLuoc(16).map((k) => k.docToiDa));
```

```typescript title=solution
function moPhongSTCS(soLanFlush: number, nguongMin: number): number[] {
  const cacBang: number[] = [];
  const dauSoBang: number[] = [];
  for (let lan = 0; lan < soLanFlush; lan++) {
    cacBang.push(1);
    let doiMoi = true;
    while (doiMoi) {
      doiMoi = false;
      const theoKichCo = new Map<number, number[]>();
      cacBang.forEach((kc, i) => {
        const ds = theoKichCo.get(kc) ?? [];
        ds.push(i);
        theoKichCo.set(kc, ds);
      });
      for (const [kc, ds] of theoKichCo) {
        if (ds.length >= nguongMin) {
          const idsGop = new Set(ds.slice(0, nguongMin));
          const conLai = cacBang.filter((_, i) => !idsGop.has(i));
          conLai.push(kc * nguongMin);
          cacBang.length = 0;
          cacBang.push(...conLai);
          doiMoi = true;
          break;
        }
      }
    }
    dauSoBang.push(cacBang.length);
  }
  return dauSoBang;
}

interface PhamVi { min: number; max: number; }
function chiaThanhCacManh(soManh: number, tongKichCo: number): PhamVi[] {
  const ketQua: PhamVi[] = [];
  const kichCoManh = tongKichCo / soManh;
  for (let i = 0; i < soManh; i++) ketQua.push({ min: i * kichCoManh, max: (i + 1) * kichCoManh - 1 });
  return ketQua;
}
function soManhChuaKhoa(cacManh: PhamVi[], khoa: number): number {
  return cacManh.filter((m) => khoa >= m.min && khoa <= m.max).length;
}

interface KetQuaChienLuoc { ten: string; soSSTable: number; docToiDa: number; }

function soSanhBaChienLuoc(soLanFlush: number): KetQuaChienLuoc[] {
  const khongNen: KetQuaChienLuoc = { ten: "khong-nen", soSSTable: soLanFlush, docToiDa: soLanFlush };

  const daySoBangSTCS = moPhongSTCS(soLanFlush, 4);
  const stcs: KetQuaChienLuoc = {
    ten: "stcs",
    soSSTable: daySoBangSTCS.at(-1)!,
    docToiDa: Math.max(...daySoBangSTCS),
  };

  const tang0 = chiaThanhCacManh(1, soLanFlush);
  const tang1 = chiaThanhCacManh(4, soLanFlush);
  const lcs: KetQuaChienLuoc = {
    ten: "lcs",
    soSSTable: tang0.length + tang1.length,
    docToiDa: soManhChuaKhoa(tang0, 0) + soManhChuaKhoa(tang1, 0),
  };

  return [khongNen, stcs, lcs];
}

console.log(soSanhBaChienLuoc(16).map((k) => k.docToiDa));
```

```typescript title=test
const kq16 = soSanhBaChienLuoc(16);
if (JSON.stringify(kq16.map((k) => k.docToiDa)) !== JSON.stringify([16, 6, 2])) throw new Error("doc toi da phai la [16, 6, 2] cho khong-nen/stcs/lcs voi 16 flush");
if (JSON.stringify(kq16.map((k) => k.soSSTable)) !== JSON.stringify([16, 1, 5])) throw new Error("so SSTable phai la [16, 1, 5]");

const kq4 = soSanhBaChienLuoc(4);
if (kq4[1]!.docToiDa !== 3) throw new Error("voi 4 flush, STCS dat toi da 3 bang truoc khi gop het ve 1 o flush thu tu (day so bang: [1,2,3,1])");

const kq64 = soSanhBaChienLuoc(64);
if (kq64[0]!.docToiDa !== 64) throw new Error("khong nen luon bang dung so lan flush");
if (kq64[2]!.docToiDa !== 2) throw new Error("LCS 2 tang luon doc toi da dung 2, khong phu thuoc so lan flush");
```

:::hints
- kind: attention
  body: "docToiDa cua STCS la gia tri LON NHAT xuat hien trong ca qua trinh (mang daySoBangSTCS) -- mot bieu thuc."
- kind: strategy
  body: "docToiDa: Math.max(...daySoBangSTCS),"
- kind: one-line
  body: "docToiDa: Math.max(...daySoBangSTCS),"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "16"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba chiến lược, ba đánh đổi khác nhau. Đủ mảnh ghép để RÁP một hệ dọn
rác hoàn chỉnh: xoá, gc_grace, VÀ nén đúng lúc — không zombie, không
tích luỹ mãi.
::::

::::reflect{#nghi-lai}
KHÔNG có "chiến lược tốt nhất" tuyệt đối — chỉ có "phù hợp NHẤT với
workload". STCS phù hợp GHI nhiều, đọc ÍT quan trọng (viết lại THẤP).
LCS phù hợp đọc NHIỀU, cần độ trễ ỔN định (write amplification CAO
hơn LÀ cái giá chấp nhận được). TWCS (bài 7) phù hợp dữ liệu THEO
thời gian có TTL — không chiến LƯỢC nào khác tận dụng được lợi thế
"drop nguyên cửa sổ" đó. Lựa chọn ĐÚNG phụ thuộc hoàn TOÀN vào workload
thật, không phải một con SỐ "tốt hơn" chung chung.
::::

::::checkpoint{mastery=0.85}
::::
