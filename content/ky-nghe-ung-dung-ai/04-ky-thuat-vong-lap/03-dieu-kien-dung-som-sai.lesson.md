---
id: ky-nghe-ung-dung-ai.ky-thuat-vong-lap.dieu-kien-dung-som-sai
title: "Điều kiện dừng sớm SAI — dừng thành công trước khi tác vụ thật sự xong"
summary: "TacVuTinHieu{demBuoc; chayMotBuoc(): KetQuaBuocTinHieu} mỗi bước trả CẢ trangThai ('xong'/'chua_xong'/'loi') LẪN tinHieuOn (một tín hiệu 'có vẻ ổn' KHÔNG đáng tin, tách biệt khỏi trangThai thật). chayVongLapDungSomNgayTho(tacVu, cap) dừng NGAY khi tinHieuOn=true (bug: kiểm tín hiệu thay vì trangThai==='xong') -- trên taoTacVuCanBaBuocVoiTinHieuSom() (cần ĐÚNG 3 bước để trangThai thật sự 'xong', NHƯNG tinHieuOn=true đã bật từ bước 1): dừng SAU 1 bước, báo 'thanhCong' với giaTri='chua_co_gia_tri' (SAI -- tác vụ chưa xong thật). Đối chứng chayVongLapDungDung (kiểm đúng trangThai==='xong') trên CÙNG tác vụ: chạy đủ 3 bước, giaTri='hoan_thanh_that_su' (ĐÚNG). Đối chứng thứ hai: taoTacVuTinHieuDangTinCay() (tinHieuOn chỉ bật CHÍNH LÚC trangThai thật sự 'xong', ở bước 2) -- cả hai hàm dừng số bước NHƯ NHAU (2 bước, giaTri='gia_tri_that') vì tín hiệu TÌNH CỜ trùng khớp thực tế lần này -- chứng minh điều kiện dừng ngây thơ KHÔNG PHẢI LUÔN sai, nó CHỈ KHÔNG ĐÁNG TIN, và sự không đáng tin đó ẩn mình cho tới khi gặp đúng tác vụ mà tín hiệu bật SỚM hơn thực tế."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-vong-lap
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [kna.dieu-kien-dung-som-sai]
requires: [kna.rang-buoc-so-buoc-toi-da]
concepts: [kna.dieu-kien-dung-som-sai]
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
Hai bài trước dạy vòng lặp KHÔNG DỪNG ĐỦ SỚM — chạy mãi (bài `1`) hoặc chạm
step cap (bài `2`). Có một chế độ hỏng NGƯỢC LẠI, thường nguy hiểm hơn: vòng
lặp dừng QUÁ SỚM, TỰ TIN báo "thành công" trong khi tác vụ THẬT SỰ chưa xong.
Bài này dựng đúng cái bẫy đó — một điều kiện dừng dựa trên tín hiệu "có vẻ
ổn", KHÔNG phải trạng thái thật.
::::

::::explain{#tin_hieu_khong_dang_tin_va_trang_thai_that}
Một `TacVuTinHieu` mỗi bước trả về HAI thứ TÁCH BIỆT: `trangThai` (sự thật —
`"xong"`/`"chua_xong"`/`"loi"`, đúng như bài `1`-`2`) VÀ `tinHieuOn` (một tín
hiệu phụ, kiểu "log không báo lỗi" hay "output trông hợp lý" — thứ một agent
THẬT có thể quan sát được nhưng KHÔNG đảm bảo tác vụ đã xong). `taoTacVuCanBaBuocVoiTinHieuSom`
mô phỏng đúng cái bẫy: cần THẬT SỰ `3` bước để `trangThai` thành `"xong"`,
NHƯNG `tinHieuOn` LUÔN `true` — kể cả Ở bước ĐẦU TIÊN, khi tác vụ còn lâu mới
xong:

```typescript title=readonly
type DemBuoc = { soBuocDaChay: number };
type KetQuaBuocTinHieu = { trangThai: "xong" | "chua_xong" | "loi"; tinHieuOn: boolean; giaTri: string };

interface TacVuTinHieu {
  demBuoc: DemBuoc;
  chayMotBuoc(): KetQuaBuocTinHieu;
}

function taoTacVuCanBaBuocVoiTinHieuSom(): TacVuTinHieu {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuocTinHieu {
      demBuoc.soBuocDaChay++;
      const xong = demBuoc.soBuocDaChay >= 3;
      return {
        trangThai: xong ? "xong" : "chua_xong",
        tinHieuOn: true,
        giaTri: xong ? "hoan_thanh_that_su" : "chua_co_gia_tri",
      };
    },
  };
}

const tv = taoTacVuCanBaBuocVoiTinHieuSom();
console.log(JSON.stringify(tv.chayMotBuoc()));
console.log(JSON.stringify(tv.chayMotBuoc()));
console.log(JSON.stringify(tv.chayMotBuoc()));
```

```text title=readonly
{"trangThai":"chua_xong","tinHieuOn":true,"giaTri":"chua_co_gia_tri"}
{"trangThai":"chua_xong","tinHieuOn":true,"giaTri":"chua_co_gia_tri"}
{"trangThai":"xong","tinHieuOn":true,"giaTri":"hoan_thanh_that_su"}
```

Chú ý: `tinHieuOn` LÀ `true` Ở CẢ BA bước — kể cả hai bước ĐẦU, khi `trangThai`
vẫn LÀ `"chua_xong"`. Một điều kiện dừng đọc NHẦM cột này thay vì cột
`trangThai` sẽ dừng NGAY Ở bước đầu tiên, mang theo `giaTri: "chua_co_gia_tri"`
— một placeholder, KHÔNG phải kết quả thật.
::::

::::example{#dung_som_ngay_tho_vs_dung_dung}
`chayVongLapDungSomNgayTho` mắc đúng lỗi đó — nó kiểm `kq.tinHieuOn` thay vì
`kq.trangThai === "xong"`. So sánh với `chayVongLapDungDung` (kiểm ĐÚNG cột
`trangThai`, y hệt bài `2`) trên CÙNG một tác vụ:

```typescript title=readonly
type DemBuoc = { soBuocDaChay: number };
type KetQuaBuocTinHieu = { trangThai: "xong" | "chua_xong" | "loi"; tinHieuOn: boolean; giaTri: string };

interface TacVuTinHieu {
  demBuoc: DemBuoc;
  chayMotBuoc(): KetQuaBuocTinHieu;
}

function taoTacVuCanBaBuocVoiTinHieuSom(): TacVuTinHieu {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuocTinHieu {
      demBuoc.soBuocDaChay++;
      const xong = demBuoc.soBuocDaChay >= 3;
      return {
        trangThai: xong ? "xong" : "chua_xong",
        tinHieuOn: true,
        giaTri: xong ? "hoan_thanh_that_su" : "chua_co_gia_tri",
      };
    },
  };
}

type KetQuaVongLapCoCap =
  | { trangThai: "thanhCong"; giaTri: string }
  | { trangThai: "loi"; loi: string }
  | { trangThai: "hetBuoc" };

function chayVongLapDungSomNgayTho(tacVu: TacVuTinHieu, soBuocToiDa: number): KetQuaVongLapCoCap {
  let soBuoc = 0;
  while (soBuoc < soBuocToiDa) {
    const kq = tacVu.chayMotBuoc();
    soBuoc++;
    if (kq.trangThai === "loi") return { trangThai: "loi", loi: "loi_tac_vu" };
    if (kq.tinHieuOn) return { trangThai: "thanhCong", giaTri: kq.giaTri };
  }
  return { trangThai: "hetBuoc" };
}

function chayVongLapDungDung(tacVu: TacVuTinHieu, soBuocToiDa: number): KetQuaVongLapCoCap {
  let soBuoc = 0;
  while (soBuoc < soBuocToiDa) {
    const kq = tacVu.chayMotBuoc();
    soBuoc++;
    if (kq.trangThai === "xong") return { trangThai: "thanhCong", giaTri: kq.giaTri };
    if (kq.trangThai === "loi") return { trangThai: "loi", loi: "loi_tac_vu" };
  }
  return { trangThai: "hetBuoc" };
}

const tvNgayTho = taoTacVuCanBaBuocVoiTinHieuSom();
console.log("ngay tho:", JSON.stringify(chayVongLapDungSomNgayTho(tvNgayTho, 5)), tvNgayTho.demBuoc.soBuocDaChay);

const tvDung = taoTacVuCanBaBuocVoiTinHieuSom();
console.log("dung:    ", JSON.stringify(chayVongLapDungDung(tvDung, 5)), tvDung.demBuoc.soBuocDaChay);
```

```text title=readonly
ngay tho: {"trangThai":"thanhCong","giaTri":"chua_co_gia_tri"} 1
dung:     {"trangThai":"thanhCong","giaTri":"hoan_thanh_that_su"} 3
```

CẢ HAI đều báo `"thanhCong"` — nhưng đó chính LÀ mối nguy hiểm: nhìn vào MỘT
mình `trangThai: "thanhCong"`, không thể phân biệt được đâu LÀ thật. Chỉ khi
nhìn VÀO `giaTri` (`"chua_co_gia_tri"` LÀ một placeholder, KHÔNG phải kết
quả) VÀ số bước THẬT SỰ chạy (`1` so với `3`) mới lộ ra: điều kiện ngây thơ
đã dừng lại SAU ĐÚNG một bước, trong khi tác vụ cần `3` bước để THẬT SỰ xong.
::::

::::predict{#doan-tin-hieu-dang-tin commitOnce}
Nếu tác vụ có `tinHieuOn` chỉ bật `true` CHÍNH XÁC lúc `trangThai` thật sự
chuyển thành `"xong"` (không bật sớm hơn) — `chayVongLapDungSomNgayTho` chạy
trên tác vụ ĐÓ có còn dừng SAI như trên không?

:::opt{correct}
Không — LẦN NÀY nó dừng ĐÚNG NGAY THỜI ĐIỂM `trangThai` thật sự `"xong"`, vì
`tinHieuOn` (điều kiện nó kiểm) VÀ `trangThai === "xong"` (điều kiện ĐÚNG)
XẢY RA CÙNG một bước — tín hiệu KHÔNG SAI trong trường hợp NÀY, nó chỉ TÌNH
CỜ đáng tin; hàm vẫn dùng đúng LOGIC cũ (kiểm sai cột), chỉ LÀ dữ liệu đầu
vào không lộ ra lỗi đó
:::
:::opt
Vẫn dừng sai — hàm `chayVongLapDungSomNgayTho` LUÔN dừng Ở bước đầu tiên bất
kể tác vụ nào, vì bug nằm trong CHÍNH code của nó
::why
Nhầm rằng bug LÀ "luôn dừng Ở bước 1" — nhưng bug thật LÀ "dừng khi
`tinHieuOn` LÀ `true`, bất kể `trangThai`", và ĐIỀU ĐÓ trùng khớp với bước
xong THẬT nếu bản thân tác vụ chỉ bật tín hiệu ĐÚNG lúc xong thật.

Chỗ lệch: hàm không hề có một hằng số "luôn dừng Ở bước 1" — nó dừng Ở BƯỚC
ĐẦU TIÊN mà `kq.tinHieuOn` LÀ `true`, và bước đó phụ thuộc HOÀN TOÀN vào
CÁCH tác vụ cụ thể gán giá trị cho trường đó.
::
:::
:::opt
Không thể biết được — kết quả phụ thuộc vào việc chạy nhiều lần rồi lấy
trung bình, vì tín hiệu không đáng tin về bản chất LÀ ngẫu nhiên
::why
Nhầm "không đáng tin" VỚI "ngẫu nhiên" — MỌI tác vụ Ở quest này hoàn toàn
tất định, `tinHieuOn` LÀ một giá trị CỐ ĐỊNH do CHÍNH tác vụ gán Ở MỖI bước,
không có `Math.random()` nào tham gia.

Chỗ lệch: với MỘT tác vụ CỤ THỂ (đã cố định luật gán `tinHieuOn`), kết quả
của `chayVongLapDungSomNgayTho` LÀ một hàm số HOÀN TOÀN xác định của luật
đó — chạy lại bao nhiêu lần cũng ra CÙNG một kết quả.
::
:::
::::

::::code{#viet_hai_dieu_kien_dung}
Hoàn thiện `chayVongLapDungSomNgayTho` — lặp `while` tối đa `soBuocToiDa`
lần; NẾU `trangThai` LÀ `"loi"`, trả `{ trangThai: "loi", loi: "loi_tac_vu" }`
NGAY; NẾU `tinHieuOn` LÀ `true` (BẤT KỂ `trangThai` LÀ gì), trả
`{ trangThai: "thanhCong", giaTri: kq.giaTri }` NGAY; hết vòng lặp thì trả
`{ trangThai: "hetBuoc" }`. Hoàn thiện `chayVongLapDungDung` — giống hệt
nhưng kiểm `trangThai === "xong"` (KHÔNG đụng tới `tinHieuOn`) để quyết định
thành công.

```typescript title=starter
type DemBuoc = { soBuocDaChay: number };
type KetQuaBuocTinHieu = { trangThai: "xong" | "chua_xong" | "loi"; tinHieuOn: boolean; giaTri: string };

interface TacVuTinHieu {
  demBuoc: DemBuoc;
  chayMotBuoc(): KetQuaBuocTinHieu;
}

function taoTacVuCanBaBuocVoiTinHieuSom(): TacVuTinHieu {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuocTinHieu {
      demBuoc.soBuocDaChay++;
      const xong = demBuoc.soBuocDaChay >= 3;
      return {
        trangThai: xong ? "xong" : "chua_xong",
        tinHieuOn: true,
        giaTri: xong ? "hoan_thanh_that_su" : "chua_co_gia_tri",
      };
    },
  };
}

function taoTacVuTinHieuDangTinCay(): TacVuTinHieu {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuocTinHieu {
      demBuoc.soBuocDaChay++;
      const xong = demBuoc.soBuocDaChay >= 2;
      return {
        trangThai: xong ? "xong" : "chua_xong",
        tinHieuOn: xong,
        giaTri: xong ? "gia_tri_that" : "chua_co_gia_tri",
      };
    },
  };
}

type KetQuaVongLapCoCap =
  | { trangThai: "thanhCong"; giaTri: string }
  | { trangThai: "loi"; loi: string }
  | { trangThai: "hetBuoc" };

function chayVongLapDungSomNgayTho(tacVu: TacVuTinHieu, soBuocToiDa: number): KetQuaVongLapCoCap {
  ___
}

function chayVongLapDungDung(tacVu: TacVuTinHieu, soBuocToiDa: number): KetQuaVongLapCoCap {
  ___
}

const tvA = taoTacVuCanBaBuocVoiTinHieuSom();
const ngayTho = chayVongLapDungSomNgayTho(tvA, 5);
console.log(JSON.stringify(ngayTho), tvA.demBuoc.soBuocDaChay);

const tvB = taoTacVuCanBaBuocVoiTinHieuSom();
const dung = chayVongLapDungDung(tvB, 5);
console.log(JSON.stringify(dung), tvB.demBuoc.soBuocDaChay);
```

```typescript title=solution
type DemBuoc = { soBuocDaChay: number };
type KetQuaBuocTinHieu = { trangThai: "xong" | "chua_xong" | "loi"; tinHieuOn: boolean; giaTri: string };

interface TacVuTinHieu {
  demBuoc: DemBuoc;
  chayMotBuoc(): KetQuaBuocTinHieu;
}

function taoTacVuCanBaBuocVoiTinHieuSom(): TacVuTinHieu {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuocTinHieu {
      demBuoc.soBuocDaChay++;
      const xong = demBuoc.soBuocDaChay >= 3;
      return {
        trangThai: xong ? "xong" : "chua_xong",
        tinHieuOn: true,
        giaTri: xong ? "hoan_thanh_that_su" : "chua_co_gia_tri",
      };
    },
  };
}

function taoTacVuTinHieuDangTinCay(): TacVuTinHieu {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuocTinHieu {
      demBuoc.soBuocDaChay++;
      const xong = demBuoc.soBuocDaChay >= 2;
      return {
        trangThai: xong ? "xong" : "chua_xong",
        tinHieuOn: xong,
        giaTri: xong ? "gia_tri_that" : "chua_co_gia_tri",
      };
    },
  };
}

type KetQuaVongLapCoCap =
  | { trangThai: "thanhCong"; giaTri: string }
  | { trangThai: "loi"; loi: string }
  | { trangThai: "hetBuoc" };

function chayVongLapDungSomNgayTho(tacVu: TacVuTinHieu, soBuocToiDa: number): KetQuaVongLapCoCap {
  let soBuoc = 0;
  while (soBuoc < soBuocToiDa) {
    const kq = tacVu.chayMotBuoc();
    soBuoc++;
    if (kq.trangThai === "loi") return { trangThai: "loi", loi: "loi_tac_vu" };
    if (kq.tinHieuOn) return { trangThai: "thanhCong", giaTri: kq.giaTri };
  }
  return { trangThai: "hetBuoc" };
}

function chayVongLapDungDung(tacVu: TacVuTinHieu, soBuocToiDa: number): KetQuaVongLapCoCap {
  let soBuoc = 0;
  while (soBuoc < soBuocToiDa) {
    const kq = tacVu.chayMotBuoc();
    soBuoc++;
    if (kq.trangThai === "xong") return { trangThai: "thanhCong", giaTri: kq.giaTri };
    if (kq.trangThai === "loi") return { trangThai: "loi", loi: "loi_tac_vu" };
  }
  return { trangThai: "hetBuoc" };
}

const tvA = taoTacVuCanBaBuocVoiTinHieuSom();
const ngayTho = chayVongLapDungSomNgayTho(tvA, 5);
console.log(JSON.stringify(ngayTho), tvA.demBuoc.soBuocDaChay);

const tvB = taoTacVuCanBaBuocVoiTinHieuSom();
const dung = chayVongLapDungDung(tvB, 5);
console.log(JSON.stringify(dung), tvB.demBuoc.soBuocDaChay);
```

```typescript title=test
if (ngayTho.trangThai !== "thanhCong") throw new Error("dieu kien ngay tho van bao thanhCong (SAI ve ban chat, nhung dung logic bug)");
if (ngayTho.trangThai === "thanhCong" && ngayTho.giaTri !== "chua_co_gia_tri") {
  throw new Error("dieu kien ngay tho phai tra ve gia tri placeholder chua_co_gia_tri, KHONG phai ket qua that");
}
if (tvA.demBuoc.soBuocDaChay !== 1) throw new Error("dieu kien ngay tho phai dung lai SAU DUNG 1 buoc (tin hieu bat tu dau)");

if (dung.trangThai !== "thanhCong") throw new Error("dieu kien dung dung phai bao thanhCong");
if (dung.trangThai === "thanhCong" && dung.giaTri !== "hoan_thanh_that_su") {
  throw new Error("dieu kien dung dung phai tra ve gia tri THAT hoan_thanh_that_su");
}
if (tvB.demBuoc.soBuocDaChay !== 3) throw new Error("dieu kien dung dung phai chay DU 3 buoc that su");

const tvTinCay = taoTacVuTinHieuDangTinCay();
const kqTinCay = chayVongLapDungSomNgayTho(tvTinCay, 5);
if (kqTinCay.trangThai !== "thanhCong") throw new Error("tren tac vu tin hieu dang tin cay, dieu kien ngay tho van phai bao thanhCong");
if (kqTinCay.trangThai === "thanhCong" && kqTinCay.giaTri !== "gia_tri_that") {
  throw new Error("tren tac vu tin hieu dang tin cay, gia tri tra ve phai la gia_tri_that (KHONG phai placeholder)");
}
if (tvTinCay.demBuoc.soBuocDaChay !== 2) throw new Error("tren tac vu tin hieu dang tin cay, dieu kien ngay tho phai dung DUNG luc trangThai that su xong (buoc 2)");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (chayVongLapDungSomNgayTho): mot vong while, moi lan goi chayMotBuoc(); neu trangThai la 'loi' thi tra loi NGAY; neu kq.tinHieuOn la true (BAT KE trangThai) thi tra thanhCong voi giaTri = kq.giaTri; het vong lap thi tra hetBuoc. Cho hai (chayVongLapDungDung): giong het cau truc nhung doi dieu kien thanh cong thanh kq.trangThai === \"xong\" (KHONG dung tinHieuOn)."
- kind: strategy
  body: "Cho dau: let soBuoc = 0; while (soBuoc < soBuocToiDa) { const kq = tacVu.chayMotBuoc(); soBuoc++; if (kq.trangThai === \"loi\") return { trangThai: \"loi\", loi: \"loi_tac_vu\" }; if (kq.tinHieuOn) return { trangThai: \"thanhCong\", giaTri: kq.giaTri }; } return { trangThai: \"hetBuoc\" }; Cho hai: thay dieu kien tinHieuOn bang kq.trangThai === \"xong\", giu nguyen phan con lai (ca thu tu kiem loi truoc)."
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung vi tri tuong ung -- CHI KHAC nhau o mot dieu kien duy nhat."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "{\"trangThai\":\"thanhCong\",\"giaTri\":\"chua_co_gia_tri\"} 1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng MỘT tác vụ, hai điều kiện dừng khác nhau — một dừng Ở bước `1` với một
placeholder, một dừng Ở bước `3` với kết quả thật. Nguy hiểm nhất LÀ khi tín
hiệu ngây thơ TÌNH CỜ đúng (`taoTacVuTinHieuDangTinCay`) — nó ẩn mất bug cho
tới khi gặp đúng tác vụ mà tín hiệu bật SỚM hơn thực tế. Bài sau xử lý một
chế độ hỏng KHÁC: vòng lặp KHÔNG dừng sai, KHÔNG chạm step cap, nhưng CŨNG
không tiến triển — lặp lại đúng một lỗi mãi mãi.
::::

::::reflect{#nghi-lai}
Bài `1` dạy "không dừng"; bài này dạy đối lập của nó — "dừng quá sớm". Cả
hai đều LÀ lỗi Ở ĐIỀU KIỆN DỪNG, nhưng SAI theo hai hướng khác nhau: một bên
không có điều kiện dừng nào cả, một bên có điều kiện dừng nhưng điều kiện đó
ĐỌC NHẦM tín hiệu. Bài học quan trọng nhất không nằm Ở việc "tín hiệu ngây
thơ luôn sai" — `taoTacVuTinHieuDangTinCay` chứng minh nó CÓ THỂ đúng, tình
cờ. Vấn đề LÀ: một điều kiện dừng ĐÚNG đắn phải kiểm tra TRẠNG THÁI THẬT của
tác vụ (`trangThai === "xong"`), không phải một tín hiệu PHỤ trông có vẻ liên
quan — vì tín hiệu phụ có thể đúng Ở tác vụ này VÀ sai Ở tác vụ khác, mà
không có cách nào biết trước.
::::

::::checkpoint{mastery=0.85}
::::
