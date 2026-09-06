---
id: ky-nghe-ung-dung-ai.ky-thuat-do-thi.boss-do-thi-production-grade-vs-don-gian
title: "BOSS q9.5b — graph runner production-grade vs runner đơn giản"
summary: "taoDoThiChinhBoss(giaTri, coTheThuLai) ráp MỘT đồ thị DUY NHẤT chứa CẢ BA hình dạng: chuẩn bị tuyến tính (buoc1-4, bài 1), rẽ nhánh HAI TẦNG (kiemTra/xuLyLoi, bài 2-3) VÀ một cạnh quay lại tiềm ẩn (thuLai->kiemTra, bài 10) — CỘNG một điểm fan-out ('phanNhanh', bài 7) khi giaTri>=0, dẫn sang HAI nhánh A/B độc lập (11 cạnh CÓ THỂ tổng cộng). Trên bộ 4 kịch bản (giaTri=5 x{B ok, B lỗi}; giaTri=-3,coTheThuLai=false; giaTri=-3,coTheThuLai=true [chu trình]): runner 'don-gian' (chỉ chayDoThi node+edge cơ bản, bài 1 — KHÔNG biết fan-out nên báo lỗi node_khong_ton_tai tại phanNhanh, KHÔNG phát hiện chu trình nên chạy tới hetBuoc rồi BÁO THÀNH CÔNG SAI) và runner 'production-grade' (cycle detection bài 10 + fan-out THẬT bài 7 + checkpoint/resume bài 4) đều cho tyLeHoanThanh=0.5 (2/4) — CÙNG một con số NHƯNG khác NHAU HOÀN TOÀN về kịch bản nào đúng: production đúng ở KB1 (fan-out) và KB4 (chu trình) mà don-gian sai; edge coverage lệch RÕ: don-gian=8/11 (in ra 0.7272727272727273, thiếu 3 cạnh fan-out), production=10/11 (in ra 0.9090909090909091, chỉ thiếu 1 cạnh quay lại); tongLuotGoi: don-gian=37, production=33 (tiết kiệm 4, dù production CHẠY THÊM fan-out). Phần B — checkpoint/resume (bài 4) trên taoDoThiChinhBoss(-3,false): chạy từ đầu tốn 7 lượt gọi, resume từ checkpoint (sau 4 bước chuẩn bị) chỉ tốn 3 — tiết kiệm 4. Đóng track T9.5 tại 12/12 (6 bài q9.5a + 6 bài q9.5b)."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-do-thi
order: 12
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [kna.boss-do-thi-production-grade-vs-don-gian]
requires: [kna.so-sanh-cau-hinh-do-thi-qua-bang-danh-gia]
concepts: [kna.boss-do-thi-production-grade-vs-don-gian]
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

::::byte{trigger=enter mood=happy pose=jump}
Sáu bài Ở q9.5b: fan-out/fan-in, audit trail, test một nhánh, phát hiện
chu trình, VÀ một bảng so sánh cấu hình. BOSS này ráp NGUYÊN VĂN mọi cơ
chế của CẢ q9.5a (node+edge, rẽ nhánh, edge coverage, checkpoint/resume)
VÀ q9.5b (fan-out/fan-in, phát hiện chu trình) thành MỘT graph runner
"production-grade" DUY NHẤT — so VỚI một runner ĐƠN GIẢN chỉ biết
node+edge — trên MỘT đồ thị chứa CẢ BA cạm bẫy CÙNG lúc.
::::

::::explain{#mot_do_thi_rap_ca_ba_cam_bay}
`taoDoThiChinhBoss(giaTri, coTheThuLai)` nối `4` node CHUẨN BỊ tuyến
tính (`buoc1→buoc2→buoc3→buoc4→kiemTra`, bài `1`), rồi TỚI `"kiemTra"` —
NẾU `giaTri >= 0`, đi tới `"phanNhanh"` (một điểm fan-out, bài `7`, dẫn
sang HAI nhánh A/B ĐỘC LẬP); NẾU `giaTri < 0`, đi `"xuLyLoi"` — rẽ TIẾP
`"thuLai"`/`"boCuoc"` tuỳ `coTheThuLai`, VÀ `"thuLai"` trỏ NGƯỢC VỀ
`"kiemTra"` (một cạnh QUAY LẠI tiềm ẩn, bài `10` — chỉ trở thành CHU
TRÌNH thật nếu `giaTri` mãi mãi âm). Runner "đơn giản" CHỈ dùng `chayDoThi`
(bài `1`, KHÔNG sửa gì); runner "production-grade" DUY TRÌ một `Set` các
node đã ghé (dừng SỚM tại chu trình, bài `10`) VÀ chuyển sang
`chayFanOutFanIn` THẬT (bài `7`) khi chạm `"phanNhanh"`:

```typescript title=readonly
type KetQuaNode = { trangThai: "xong" | "loi"; tenNodeTiepTheo: string | null };

interface NodeXuLy {
  chay(): KetQuaNode;
}

type DoThi = Record<string, NodeXuLy>;

function taoNodeThanhCong(tenNodeTiepTheo: string | null): NodeXuLy {
  return {
    chay(): KetQuaNode {
      return { trangThai: "xong", tenNodeTiepTheo };
    },
  };
}

function taoNodeLoi(): NodeXuLy {
  return {
    chay(): KetQuaNode {
      return { trangThai: "loi", tenNodeTiepTheo: null };
    },
  };
}

const SO_BUOC_TOI_DA_AN_TOAN = 20;

type KetQuaChayDoThi =
  | { thanhCong: true; dsNodeDaDi: string[] }
  | { thanhCong: false; loi: string; dsNodeDaDi: string[] };

function chayDoThi(doThi: DoThi, tenNodeGoc: string): KetQuaChayDoThi {
  const dsNodeDaDi: string[] = [];
  let tenHienTai: string | null = tenNodeGoc;
  let soBuoc = 0;
  while (tenHienTai !== null && soBuoc < SO_BUOC_TOI_DA_AN_TOAN) {
    soBuoc++;
    const node: NodeXuLy | undefined = doThi[tenHienTai];
    if (!node) return { thanhCong: false, loi: "node_khong_ton_tai", dsNodeDaDi };
    dsNodeDaDi.push(tenHienTai);
    const kq: KetQuaNode = node.chay();
    if (kq.trangThai === "loi") return { thanhCong: false, loi: "loi_tai_node", dsNodeDaDi };
    tenHienTai = kq.tenNodeTiepTheo;
  }
  return { thanhCong: true, dsNodeDaDi };
}

function taoDoThiChinhBoss(giaTri: number, coTheThuLai: boolean): DoThi {
  return {
    buoc1: taoNodeThanhCong("buoc2"),
    buoc2: taoNodeThanhCong("buoc3"),
    buoc3: taoNodeThanhCong("buoc4"),
    buoc4: taoNodeThanhCong("kiemTra"),
    kiemTra: {
      chay(): KetQuaNode {
        if (giaTri >= 0) return { trangThai: "xong", tenNodeTiepTheo: "phanNhanh" };
        return { trangThai: "xong", tenNodeTiepTheo: "xuLyLoi" };
      },
    },
    xuLyLoi: {
      chay(): KetQuaNode {
        if (coTheThuLai) return { trangThai: "xong", tenNodeTiepTheo: "thuLai" };
        return { trangThai: "xong", tenNodeTiepTheo: "boCuoc" };
      },
    },
    thuLai: taoNodeThanhCong("kiemTra"),
    boCuoc: taoNodeThanhCong(null),
  };
}

const TOAN_BO_CANH_CO_THE = [
  "buoc1->buoc2", "buoc2->buoc3", "buoc3->buoc4", "buoc4->kiemTra",
  "kiemTra->phanNhanh", "kiemTra->xuLyLoi", "xuLyLoi->thuLai", "xuLyLoi->boCuoc", "thuLai->kiemTra",
  "nhanhA1->nhanhA2", "nhanhB1->nhanhB2",
];
console.log(TOAN_BO_CANH_CO_THE.length);
```

```text title=readonly
11
```

`"phanNhanh"` KHÔNG hề LÀ một khoá trong `taoDoThiChinhBoss` — nó CHỈ LÀ
một cái TÊN mà `"kiemTra"` trả về LÀM `tenNodeTiepTheo`. Runner "đơn
giản" (`chayDoThi` nguyên bản) sẽ tra `doThi["phanNhanh"]`, KHÔNG tìm
thấy, VÀ báo `"node_khong_ton_tai"` — nó KHÔNG có cơ chế NÀO để hiểu
"đây LÀ một điểm fan-out cần xử lý ĐẶC BIỆT".
::::

::::example{#phan_a_bang_so_sanh_va_phan_b_checkpoint}
**Phần A** — chạy `4` kịch bản (`giaTri=5` với B hợp lệ/lỗi; `giaTri=-3`
với `coTheThuLai=false`/`true`) qua CẢ HAI runner. **Phần B** — trên MỘT
kịch bản (`giaTri=-3, coTheThuLai=false`), so sánh chạy TỪ ĐẦU VỚI
checkpoint/resume (bài `4`), checkpoint lưu SAU `4` bước chuẩn bị:

```typescript title=readonly
// (xem khoi `solution` o buoc ::::code de doc TOAN VAN chayCoreCoPhatHienChuTrinh,
// chayProductionGrade, chayDonGian, doThiNhanhA/B, chayFanOutFanIn, checkpoint)

type KichBanBoss = { giaTri: number; coTheThuLai: boolean; nhanhBLoi: boolean };

const boKichBanBoss: KichBanBoss[] = [
  { giaTri: 5, coTheThuLai: true, nhanhBLoi: false },
  { giaTri: 5, coTheThuLai: true, nhanhBLoi: true },
  { giaTri: -3, coTheThuLai: false, nhanhBLoi: false },
  { giaTri: -3, coTheThuLai: true, nhanhBLoi: false },
];

// bangSoSanhBoss[0] = hang "don-gian", bangSoSanhBoss[1] = hang "production-grade"
// console.log(JSON.stringify(bangSoSanhBoss));
```

```text title=readonly
[{"ten":"don-gian","tyLeHoanThanh":0.5,"tyLeEdgeCoverage":0.7272727272727273,"tongLuotGoi":37},{"ten":"production-grade","tyLeHoanThanh":0.5,"tyLeEdgeCoverage":0.9090909090909091,"tongLuotGoi":33}]
```

`tyLeHoanThanh` GIỐNG NHAU HỆT (`0.5` — `2`/`4`) Ở CẢ HAI runner — NHƯNG
đúng Ở HAI kịch bản KHÁC NHAU: "đơn giản" đúng Ở `giaTri=-3,coTheThuLai=
false` (kịch bản `3`, không chạm fan-out hay chu trình) VÀ SAI Ở kịch
bản `4` (`coTheThuLai=true`, chu trình — nó chạy tới `hetBuoc` rồi BÁO
THÀNH CÔNG SAI, đúng cạm bẫy bài `10` đã dạy); "production-grade" đúng
Ở kịch bản `1` (fan-out THẬT thành công) VÀ kịch bản `4` (phát hiện chu
trình, báo THẤT BẠI ĐÚNG), NHƯNG "thất bại" Ở kịch bản `2` (B lỗi — ĐÚNG,
vì fan-out THẬT phát hiện được). Edge coverage LỘ rõ khác biệt: "đơn
giản" chỉ `8`/`11` (in ra `0.7272727272727273` — THIẾU cả `3` cạnh
fan-out, vì chưa từng chạm `"phanNhanh"`); "production-grade" đạt `10`/
`11` (in ra `0.9090909090909091` — chỉ THIẾU `"thuLai->kiemTra"`, vì
cycle detection dừng NGAY TRƯỚC khi đi lại cạnh đó). `tongLuotGoi`:
`37` so VỚI `33` — production-grade TIẾT KIỆM đúng `4` lệnh gọi TỔNG
THỂ, DÙ nó phải TRẢ THÊM chi phí chạy fan-out THẬT (mà "đơn giản" bỏ
qua hoàn toàn).

Phần B (checkpoint/resume, VẪN cùng cơ chế bài `4`) trên
`taoDoThiChinhBoss(-3, false)`: chạy TỪ ĐẦU tốn `7` lệnh gọi
(`buoc1..buoc4,kiemTra,xuLyLoi,boCuoc`); resume TỪ checkpoint (lưu SAU
`4` bước chuẩn bị) chỉ tốn `3` lệnh gọi MỚI — tiết kiệm ĐÚNG `4`.
::::

::::predict{#doan-doi-nhanhbloi-kich-ban-2 commitOnce}
Nếu đổi kịch bản `2` (`giaTri=5, coTheThuLai=true, nhanhBLoi=true`)
thành `nhanhBLoi=false` (CẢ `4` kịch bản giờ có nhánh B hợp lệ) —
`tyLeHoanThanh` của runner "production-grade" đổi ra sao, VÀ của runner
"đơn giản" có đổi THEO không?

:::opt{correct}
"production-grade" TĂNG từ `0.5` lên `3`/`4` (`0.75`) — vì kịch bản `2`
giờ THÀNH CÔNG THẬT (B không còn lỗi); "đơn giản" GIỮ NGUYÊN `0.5` —
`nhanhBLoi` KHÔNG hề ảnh hưởng tới nó, vì nó chưa BAO GIỜ chạy TỚI nhánh
B (đã báo lỗi `"node_khong_ton_tai"` NGAY tại `"phanNhanh"`, TRƯỚC KHI
biết B có lỗi hay không)
:::
:::opt
CẢ HAI runner đều TĂNG `tyLeHoanThanh`, vì `nhanhBLoi=false` LÀ một cải
thiện CHUNG cho TOÀN BỘ bộ kịch bản
::why
Nhầm rằng thay đổi MỘT tham số ảnh hưởng ĐỀU tới MỌI cách chạy — nhưng
runner "đơn giản" (`chayDonGian`) gọi THẲNG `chayDoThi` trên
`taoDoThiChinhBoss`, hàm này KHÔNG hề nhận `nhanhBLoi` LÀM tham số — nó
không có CÁCH NÀO để "biết" B có lỗi hay không, vì nó chưa từng thử chạm
tới B.

Chỗ lệch: `chayDonGian` chỉ nhận `(giaTri, coTheThuLai)` — hai tham số
đó KHÔNG đổi Ở kịch bản `2`; CHỈ `nhanhBLoi` đổi, VÀ tham số đó CHỈ được
`chayProductionGrade` sử dụng.
::
:::
:::opt
"đơn giản" cũng TĂNG, vì fan-out giờ "thành công" nên `"phanNhanh"` được
xem LÀ một node hợp lệ
::why
Nhầm rằng `nhanhBLoi=false` khiến `"phanNhanh"` "trở thành" một khoá
HỢP LỆ trong `doThi` — nhưng `taoDoThiChinhBoss` KHÔNG hề đổi cấu trúc
dựa trên `nhanhBLoi`; `"phanNhanh"` VẪN LÀ một tên KHÔNG tồn tại trong
`Record` đó, bất kể nhánh B có lỗi hay không.

Chỗ lệch: `nhanhBLoi` CHỈ LÀ một tham số của `chayProductionGrade` (chọn
`doThiNhanhBLoi` hay `doThiNhanhBHopLe`) — nó không hề được TRUYỀN VÀO
`taoDoThiChinhBoss` hay `chayDoThi`.
::
:::
::::

::::code{#viet_boss_do_thi_production_grade}
Hoàn thiện `chayCoreCoPhatHienChuTrinh` — vòng `while` (KHÔNG giới hạn
bằng điều kiện, chỉ `tenHienTai !== null`): NẾU `tenHienTai ===
"phanNhanh"`, đẩy vào `dsNodeDaDi` rồi trả `{ trangThaiDung:
"toi_phan_nhanh", dsNodeDaDi }` NGAY (KHÔNG tra cứu); NẾU hết bước an
toàn, trả `"het_buoc"`; NẾU node ĐÃ ghé qua (`daGhePhamVi`), trả
`"chu_trinh"`; NGƯỢC LẠI tra node (lỗi nếu không tồn tại), THÊM VÀO
`daGhePhamVi` VÀ `dsNodeDaDi`, gọi `chay()`, xử lý `"loi"`/đi tới
`tenNodeTiepTheo`; hết vòng lặp (`tenHienTai` LÀ `null`) trả
`"ket_thuc"`. Hoàn thiện `chayProductionGrade` — chạy
`chayCoreCoPhatHienChuTrinh` từ `"buoc1"`; NẾU KHÔNG dừng Ở
`"toi_phan_nhanh"`, trả kết quả dựa TRÊN phần lõi (`thanhCong` LÀ `true`
CHỈ KHI `trangThaiDung === "ket_thuc"`); NGƯỢC LẠI, chạy
`chayFanOutFanIn` (CHỌN `doThiNhanhBLoi`/`doThiNhanhBHopLe` theo
`nhanhBLoi`), GỘP cạnh VÀ lệnh gọi của CẢ phần lõi LẪN fan-out.

```typescript title=starter
type KetQuaNode = { trangThai: "xong" | "loi"; tenNodeTiepTheo: string | null };

interface NodeXuLy {
  chay(): KetQuaNode;
}

type DoThi = Record<string, NodeXuLy>;

function taoNodeThanhCong(tenNodeTiepTheo: string | null): NodeXuLy {
  return {
    chay(): KetQuaNode {
      return { trangThai: "xong", tenNodeTiepTheo };
    },
  };
}

function taoNodeLoi(): NodeXuLy {
  return {
    chay(): KetQuaNode {
      return { trangThai: "loi", tenNodeTiepTheo: null };
    },
  };
}

const SO_BUOC_TOI_DA_AN_TOAN = 20;

type KetQuaChayDoThi =
  | { thanhCong: true; dsNodeDaDi: string[] }
  | { thanhCong: false; loi: string; dsNodeDaDi: string[] };

function chayDoThi(doThi: DoThi, tenNodeGoc: string): KetQuaChayDoThi {
  const dsNodeDaDi: string[] = [];
  let tenHienTai: string | null = tenNodeGoc;
  let soBuoc = 0;
  while (tenHienTai !== null && soBuoc < SO_BUOC_TOI_DA_AN_TOAN) {
    soBuoc++;
    const node: NodeXuLy | undefined = doThi[tenHienTai];
    if (!node) return { thanhCong: false, loi: "node_khong_ton_tai", dsNodeDaDi };
    dsNodeDaDi.push(tenHienTai);
    const kq: KetQuaNode = node.chay();
    if (kq.trangThai === "loi") return { thanhCong: false, loi: "loi_tai_node", dsNodeDaDi };
    tenHienTai = kq.tenNodeTiepTheo;
  }
  return { thanhCong: true, dsNodeDaDi };
}

function layCacCanhDaDi(dsNodeDaDi: string[]): string[] {
  const canh: string[] = [];
  for (let i = 0; i < dsNodeDaDi.length - 1; i++) {
    canh.push(`${dsNodeDaDi[i]}->${dsNodeDaDi[i + 1]}`);
  }
  return canh;
}

function tinhTyLeHoanThanh(dsThanhCong: boolean[]): number {
  if (dsThanhCong.length === 0) return 0;
  return dsThanhCong.filter((tc) => tc).length / dsThanhCong.length;
}

function tinhTyLeEdgeCoverage(canhDaDi: Set<string>, toanBoCanh: string[]): number {
  if (toanBoCanh.length === 0) return 0;
  const soCanhDaDi = toanBoCanh.filter((c) => canhDaDi.has(c)).length;
  return soCanhDaDi / toanBoCanh.length;
}

type Checkpoint = { tenNodeTiepTheo: string; dsNodeDaDiTruocDo: string[] };

function chayDoThiCoDemGoi(
  doThi: DoThi,
  tenNodeGoc: string,
  demGoiMoiNode: Record<string, number>,
): KetQuaChayDoThi {
  const dsNodeDaDi: string[] = [];
  let tenHienTai: string | null = tenNodeGoc;
  let soBuoc = 0;
  while (tenHienTai !== null && soBuoc < SO_BUOC_TOI_DA_AN_TOAN) {
    soBuoc++;
    const node: NodeXuLy | undefined = doThi[tenHienTai];
    if (!node) return { thanhCong: false, loi: "node_khong_ton_tai", dsNodeDaDi };
    demGoiMoiNode[tenHienTai] = (demGoiMoiNode[tenHienTai] ?? 0) + 1;
    dsNodeDaDi.push(tenHienTai);
    const kq: KetQuaNode = node.chay();
    if (kq.trangThai === "loi") return { thanhCong: false, loi: "loi_tai_node", dsNodeDaDi };
    tenHienTai = kq.tenNodeTiepTheo;
  }
  return { thanhCong: true, dsNodeDaDi };
}

function chayDoThiTuCheckpoint(
  doThi: DoThi,
  checkpoint: Checkpoint,
  demGoiMoiNode: Record<string, number>,
): KetQuaChayDoThi {
  const ketQuaMoi = chayDoThiCoDemGoi(doThi, checkpoint.tenNodeTiepTheo, demGoiMoiNode);
  return { ...ketQuaMoi, dsNodeDaDi: [...checkpoint.dsNodeDaDiTruocDo, ...ketQuaMoi.dsNodeDaDi] };
}

function tinhTongSoLuotGoi(demGoiMoiNode: Record<string, number>): number {
  return Object.values(demGoiMoiNode).reduce((acc, n) => acc + n, 0);
}

type KetQuaFanOutFanIn =
  | { thanhCong: true; dsNodeDaDiNhanhA: string[]; dsNodeDaDiNhanhB: string[] }
  | { thanhCong: false; nhanhLoi: ("A" | "B")[]; dsNodeDaDiNhanhA: string[]; dsNodeDaDiNhanhB: string[] };

function chayFanOutFanIn(
  doThiNhanhA: DoThi, tenGocNhanhA: string,
  doThiNhanhB: DoThi, tenGocNhanhB: string,
): KetQuaFanOutFanIn {
  const ketQuaA = chayDoThi(doThiNhanhA, tenGocNhanhA);
  const ketQuaB = chayDoThi(doThiNhanhB, tenGocNhanhB);
  if (ketQuaA.thanhCong && ketQuaB.thanhCong) {
    return { thanhCong: true, dsNodeDaDiNhanhA: ketQuaA.dsNodeDaDi, dsNodeDaDiNhanhB: ketQuaB.dsNodeDaDi };
  }
  const nhanhLoi: ("A" | "B")[] = [];
  if (!ketQuaA.thanhCong) nhanhLoi.push("A");
  if (!ketQuaB.thanhCong) nhanhLoi.push("B");
  return { thanhCong: false, nhanhLoi, dsNodeDaDiNhanhA: ketQuaA.dsNodeDaDi, dsNodeDaDiNhanhB: ketQuaB.dsNodeDaDi };
}

const doThiNhanhA: DoThi = { nhanhA1: taoNodeThanhCong("nhanhA2"), nhanhA2: taoNodeThanhCong(null) };
const doThiNhanhBHopLe: DoThi = { nhanhB1: taoNodeThanhCong("nhanhB2"), nhanhB2: taoNodeThanhCong(null) };
const doThiNhanhBLoi: DoThi = { nhanhB1: taoNodeLoi() };

function taoDoThiChinhBoss(giaTri: number, coTheThuLai: boolean): DoThi {
  return {
    buoc1: taoNodeThanhCong("buoc2"),
    buoc2: taoNodeThanhCong("buoc3"),
    buoc3: taoNodeThanhCong("buoc4"),
    buoc4: taoNodeThanhCong("kiemTra"),
    kiemTra: {
      chay(): KetQuaNode {
        if (giaTri >= 0) return { trangThai: "xong", tenNodeTiepTheo: "phanNhanh" };
        return { trangThai: "xong", tenNodeTiepTheo: "xuLyLoi" };
      },
    },
    xuLyLoi: {
      chay(): KetQuaNode {
        if (coTheThuLai) return { trangThai: "xong", tenNodeTiepTheo: "thuLai" };
        return { trangThai: "xong", tenNodeTiepTheo: "boCuoc" };
      },
    },
    thuLai: taoNodeThanhCong("kiemTra"),
    boCuoc: taoNodeThanhCong(null),
  };
}

const TOAN_BO_CANH_CO_THE = [
  "buoc1->buoc2", "buoc2->buoc3", "buoc3->buoc4", "buoc4->kiemTra",
  "kiemTra->phanNhanh", "kiemTra->xuLyLoi", "xuLyLoi->thuLai", "xuLyLoi->boCuoc", "thuLai->kiemTra",
  "nhanhA1->nhanhA2", "nhanhB1->nhanhB2",
];

type KetQuaChayCore =
  { trangThaiDung: "toi_phan_nhanh" | "ket_thuc" | "chu_trinh" | "loi" | "het_buoc"; dsNodeDaDi: string[] };

function chayCoreCoPhatHienChuTrinh(doThi: DoThi, tenNodeGoc: string): KetQuaChayCore {
  ___
}

type KetQuaBoss = { thanhCong: boolean; dsCanhDaDi: string[]; soLuotGoi: number };

function chayProductionGrade(giaTri: number, coTheThuLai: boolean, nhanhBLoi: boolean): KetQuaBoss {
  ___
}

function chayDonGian(giaTri: number, coTheThuLai: boolean): KetQuaBoss {
  const doThiChinh = taoDoThiChinhBoss(giaTri, coTheThuLai);
  const kt = chayDoThi(doThiChinh, "buoc1");
  return { thanhCong: kt.thanhCong, dsCanhDaDi: layCacCanhDaDi(kt.dsNodeDaDi), soLuotGoi: kt.dsNodeDaDi.length };
}

type KichBanBoss = { giaTri: number; coTheThuLai: boolean; nhanhBLoi: boolean };

function chayBangSoSanhBoss(dsKichBan: KichBanBoss[]) {
  const ketQuaProd = dsKichBan.map((kb) => chayProductionGrade(kb.giaTri, kb.coTheThuLai, kb.nhanhBLoi));
  const ketQuaDonGian = dsKichBan.map((kb) => chayDonGian(kb.giaTri, kb.coTheThuLai));

  const tapCanhProd = new Set<string>();
  for (const k of ketQuaProd) for (const c of k.dsCanhDaDi) tapCanhProd.add(c);
  const tapCanhDonGian = new Set<string>();
  for (const k of ketQuaDonGian) for (const c of k.dsCanhDaDi) tapCanhDonGian.add(c);

  return [
    {
      ten: "don-gian",
      tyLeHoanThanh: tinhTyLeHoanThanh(ketQuaDonGian.map((k) => k.thanhCong)),
      tyLeEdgeCoverage: tinhTyLeEdgeCoverage(tapCanhDonGian, TOAN_BO_CANH_CO_THE),
      tongLuotGoi: ketQuaDonGian.reduce((t, k) => t + k.soLuotGoi, 0),
    },
    {
      ten: "production-grade",
      tyLeHoanThanh: tinhTyLeHoanThanh(ketQuaProd.map((k) => k.thanhCong)),
      tyLeEdgeCoverage: tinhTyLeEdgeCoverage(tapCanhProd, TOAN_BO_CANH_CO_THE),
      tongLuotGoi: ketQuaProd.reduce((t, k) => t + k.soLuotGoi, 0),
    },
  ];
}

const boKichBanBoss: KichBanBoss[] = [
  { giaTri: 5, coTheThuLai: true, nhanhBLoi: false },
  { giaTri: 5, coTheThuLai: true, nhanhBLoi: true },
  { giaTri: -3, coTheThuLai: false, nhanhBLoi: false },
  { giaTri: -3, coTheThuLai: true, nhanhBLoi: false },
];
const bangSoSanhBoss = chayBangSoSanhBoss(boKichBanBoss);

const doThiKichBanDon = taoDoThiChinhBoss(-3, false);
const demGoiTuDau: Record<string, number> = {};
const ketQuaTuDau = chayDoThiCoDemGoi(doThiKichBanDon, "buoc1", demGoiTuDau);
const checkpointGiuaChung: Checkpoint = { tenNodeTiepTheo: "kiemTra", dsNodeDaDiTruocDo: ["buoc1", "buoc2", "buoc3", "buoc4"] };
const demGoiTuCheckpoint: Record<string, number> = {};
const ketQuaTuCheckpoint = chayDoThiTuCheckpoint(doThiKichBanDon, checkpointGiuaChung, demGoiTuCheckpoint);
const tongLuotGoiTuDau = tinhTongSoLuotGoi(demGoiTuDau);
const tongLuotGoiTuCheckpoint = tinhTongSoLuotGoi(demGoiTuCheckpoint);
const soLuotGoiTietKiemCheckpoint = tongLuotGoiTuDau - tongLuotGoiTuCheckpoint;

console.log(
  bangSoSanhBoss[0]!.tyLeHoanThanh, bangSoSanhBoss[1]!.tyLeHoanThanh,
  bangSoSanhBoss[0]!.tyLeEdgeCoverage, bangSoSanhBoss[1]!.tyLeEdgeCoverage,
  bangSoSanhBoss[0]!.tongLuotGoi, bangSoSanhBoss[1]!.tongLuotGoi,
  tongLuotGoiTuDau, tongLuotGoiTuCheckpoint, soLuotGoiTietKiemCheckpoint,
);
```

```typescript title=solution
type KetQuaNode = { trangThai: "xong" | "loi"; tenNodeTiepTheo: string | null };

interface NodeXuLy {
  chay(): KetQuaNode;
}

type DoThi = Record<string, NodeXuLy>;

function taoNodeThanhCong(tenNodeTiepTheo: string | null): NodeXuLy {
  return {
    chay(): KetQuaNode {
      return { trangThai: "xong", tenNodeTiepTheo };
    },
  };
}

function taoNodeLoi(): NodeXuLy {
  return {
    chay(): KetQuaNode {
      return { trangThai: "loi", tenNodeTiepTheo: null };
    },
  };
}

const SO_BUOC_TOI_DA_AN_TOAN = 20;

type KetQuaChayDoThi =
  | { thanhCong: true; dsNodeDaDi: string[] }
  | { thanhCong: false; loi: string; dsNodeDaDi: string[] };

function chayDoThi(doThi: DoThi, tenNodeGoc: string): KetQuaChayDoThi {
  const dsNodeDaDi: string[] = [];
  let tenHienTai: string | null = tenNodeGoc;
  let soBuoc = 0;
  while (tenHienTai !== null && soBuoc < SO_BUOC_TOI_DA_AN_TOAN) {
    soBuoc++;
    const node: NodeXuLy | undefined = doThi[tenHienTai];
    if (!node) return { thanhCong: false, loi: "node_khong_ton_tai", dsNodeDaDi };
    dsNodeDaDi.push(tenHienTai);
    const kq: KetQuaNode = node.chay();
    if (kq.trangThai === "loi") return { thanhCong: false, loi: "loi_tai_node", dsNodeDaDi };
    tenHienTai = kq.tenNodeTiepTheo;
  }
  return { thanhCong: true, dsNodeDaDi };
}

function layCacCanhDaDi(dsNodeDaDi: string[]): string[] {
  const canh: string[] = [];
  for (let i = 0; i < dsNodeDaDi.length - 1; i++) {
    canh.push(`${dsNodeDaDi[i]}->${dsNodeDaDi[i + 1]}`);
  }
  return canh;
}

function tinhTyLeHoanThanh(dsThanhCong: boolean[]): number {
  if (dsThanhCong.length === 0) return 0;
  return dsThanhCong.filter((tc) => tc).length / dsThanhCong.length;
}

function tinhTyLeEdgeCoverage(canhDaDi: Set<string>, toanBoCanh: string[]): number {
  if (toanBoCanh.length === 0) return 0;
  const soCanhDaDi = toanBoCanh.filter((c) => canhDaDi.has(c)).length;
  return soCanhDaDi / toanBoCanh.length;
}

type Checkpoint = { tenNodeTiepTheo: string; dsNodeDaDiTruocDo: string[] };

function chayDoThiCoDemGoi(
  doThi: DoThi,
  tenNodeGoc: string,
  demGoiMoiNode: Record<string, number>,
): KetQuaChayDoThi {
  const dsNodeDaDi: string[] = [];
  let tenHienTai: string | null = tenNodeGoc;
  let soBuoc = 0;
  while (tenHienTai !== null && soBuoc < SO_BUOC_TOI_DA_AN_TOAN) {
    soBuoc++;
    const node: NodeXuLy | undefined = doThi[tenHienTai];
    if (!node) return { thanhCong: false, loi: "node_khong_ton_tai", dsNodeDaDi };
    demGoiMoiNode[tenHienTai] = (demGoiMoiNode[tenHienTai] ?? 0) + 1;
    dsNodeDaDi.push(tenHienTai);
    const kq: KetQuaNode = node.chay();
    if (kq.trangThai === "loi") return { thanhCong: false, loi: "loi_tai_node", dsNodeDaDi };
    tenHienTai = kq.tenNodeTiepTheo;
  }
  return { thanhCong: true, dsNodeDaDi };
}

function chayDoThiTuCheckpoint(
  doThi: DoThi,
  checkpoint: Checkpoint,
  demGoiMoiNode: Record<string, number>,
): KetQuaChayDoThi {
  const ketQuaMoi = chayDoThiCoDemGoi(doThi, checkpoint.tenNodeTiepTheo, demGoiMoiNode);
  return { ...ketQuaMoi, dsNodeDaDi: [...checkpoint.dsNodeDaDiTruocDo, ...ketQuaMoi.dsNodeDaDi] };
}

function tinhTongSoLuotGoi(demGoiMoiNode: Record<string, number>): number {
  return Object.values(demGoiMoiNode).reduce((acc, n) => acc + n, 0);
}

type KetQuaFanOutFanIn =
  | { thanhCong: true; dsNodeDaDiNhanhA: string[]; dsNodeDaDiNhanhB: string[] }
  | { thanhCong: false; nhanhLoi: ("A" | "B")[]; dsNodeDaDiNhanhA: string[]; dsNodeDaDiNhanhB: string[] };

function chayFanOutFanIn(
  doThiNhanhA: DoThi, tenGocNhanhA: string,
  doThiNhanhB: DoThi, tenGocNhanhB: string,
): KetQuaFanOutFanIn {
  const ketQuaA = chayDoThi(doThiNhanhA, tenGocNhanhA);
  const ketQuaB = chayDoThi(doThiNhanhB, tenGocNhanhB);
  if (ketQuaA.thanhCong && ketQuaB.thanhCong) {
    return { thanhCong: true, dsNodeDaDiNhanhA: ketQuaA.dsNodeDaDi, dsNodeDaDiNhanhB: ketQuaB.dsNodeDaDi };
  }
  const nhanhLoi: ("A" | "B")[] = [];
  if (!ketQuaA.thanhCong) nhanhLoi.push("A");
  if (!ketQuaB.thanhCong) nhanhLoi.push("B");
  return { thanhCong: false, nhanhLoi, dsNodeDaDiNhanhA: ketQuaA.dsNodeDaDi, dsNodeDaDiNhanhB: ketQuaB.dsNodeDaDi };
}

const doThiNhanhA: DoThi = { nhanhA1: taoNodeThanhCong("nhanhA2"), nhanhA2: taoNodeThanhCong(null) };
const doThiNhanhBHopLe: DoThi = { nhanhB1: taoNodeThanhCong("nhanhB2"), nhanhB2: taoNodeThanhCong(null) };
const doThiNhanhBLoi: DoThi = { nhanhB1: taoNodeLoi() };

function taoDoThiChinhBoss(giaTri: number, coTheThuLai: boolean): DoThi {
  return {
    buoc1: taoNodeThanhCong("buoc2"),
    buoc2: taoNodeThanhCong("buoc3"),
    buoc3: taoNodeThanhCong("buoc4"),
    buoc4: taoNodeThanhCong("kiemTra"),
    kiemTra: {
      chay(): KetQuaNode {
        if (giaTri >= 0) return { trangThai: "xong", tenNodeTiepTheo: "phanNhanh" };
        return { trangThai: "xong", tenNodeTiepTheo: "xuLyLoi" };
      },
    },
    xuLyLoi: {
      chay(): KetQuaNode {
        if (coTheThuLai) return { trangThai: "xong", tenNodeTiepTheo: "thuLai" };
        return { trangThai: "xong", tenNodeTiepTheo: "boCuoc" };
      },
    },
    thuLai: taoNodeThanhCong("kiemTra"),
    boCuoc: taoNodeThanhCong(null),
  };
}

const TOAN_BO_CANH_CO_THE = [
  "buoc1->buoc2", "buoc2->buoc3", "buoc3->buoc4", "buoc4->kiemTra",
  "kiemTra->phanNhanh", "kiemTra->xuLyLoi", "xuLyLoi->thuLai", "xuLyLoi->boCuoc", "thuLai->kiemTra",
  "nhanhA1->nhanhA2", "nhanhB1->nhanhB2",
];

type KetQuaChayCore =
  { trangThaiDung: "toi_phan_nhanh" | "ket_thuc" | "chu_trinh" | "loi" | "het_buoc"; dsNodeDaDi: string[] };

function chayCoreCoPhatHienChuTrinh(doThi: DoThi, tenNodeGoc: string): KetQuaChayCore {
  const dsNodeDaDi: string[] = [];
  const daGhePhamVi = new Set<string>();
  let tenHienTai: string | null = tenNodeGoc;
  let soBuoc = 0;
  while (tenHienTai !== null) {
    if (tenHienTai === "phanNhanh") {
      dsNodeDaDi.push(tenHienTai);
      return { trangThaiDung: "toi_phan_nhanh", dsNodeDaDi };
    }
    if (soBuoc >= SO_BUOC_TOI_DA_AN_TOAN) return { trangThaiDung: "het_buoc", dsNodeDaDi };
    if (daGhePhamVi.has(tenHienTai)) return { trangThaiDung: "chu_trinh", dsNodeDaDi };
    soBuoc++;
    const node: NodeXuLy | undefined = doThi[tenHienTai];
    if (!node) return { trangThaiDung: "loi", dsNodeDaDi };
    daGhePhamVi.add(tenHienTai);
    dsNodeDaDi.push(tenHienTai);
    const kq: KetQuaNode = node.chay();
    if (kq.trangThai === "loi") return { trangThaiDung: "loi", dsNodeDaDi };
    tenHienTai = kq.tenNodeTiepTheo;
  }
  return { trangThaiDung: "ket_thuc", dsNodeDaDi };
}

type KetQuaBoss = { thanhCong: boolean; dsCanhDaDi: string[]; soLuotGoi: number };

function chayProductionGrade(giaTri: number, coTheThuLai: boolean, nhanhBLoi: boolean): KetQuaBoss {
  const doThiChinh = taoDoThiChinhBoss(giaTri, coTheThuLai);
  const ktCore = chayCoreCoPhatHienChuTrinh(doThiChinh, "buoc1");
  if (ktCore.trangThaiDung !== "toi_phan_nhanh") {
    return {
      thanhCong: ktCore.trangThaiDung === "ket_thuc",
      dsCanhDaDi: layCacCanhDaDi(ktCore.dsNodeDaDi),
      soLuotGoi: ktCore.dsNodeDaDi.length,
    };
  }
  const doThiB = nhanhBLoi ? doThiNhanhBLoi : doThiNhanhBHopLe;
  const ktFan = chayFanOutFanIn(doThiNhanhA, "nhanhA1", doThiB, "nhanhB1");
  const canhCore = layCacCanhDaDi(ktCore.dsNodeDaDi);
  const canhFan = [...layCacCanhDaDi(ktFan.dsNodeDaDiNhanhA), ...layCacCanhDaDi(ktFan.dsNodeDaDiNhanhB)];
  return {
    thanhCong: ktFan.thanhCong,
    dsCanhDaDi: [...canhCore, ...canhFan],
    soLuotGoi: ktCore.dsNodeDaDi.length + ktFan.dsNodeDaDiNhanhA.length + ktFan.dsNodeDaDiNhanhB.length,
  };
}

function chayDonGian(giaTri: number, coTheThuLai: boolean): KetQuaBoss {
  const doThiChinh = taoDoThiChinhBoss(giaTri, coTheThuLai);
  const kt = chayDoThi(doThiChinh, "buoc1");
  return { thanhCong: kt.thanhCong, dsCanhDaDi: layCacCanhDaDi(kt.dsNodeDaDi), soLuotGoi: kt.dsNodeDaDi.length };
}

type KichBanBoss = { giaTri: number; coTheThuLai: boolean; nhanhBLoi: boolean };

function chayBangSoSanhBoss(dsKichBan: KichBanBoss[]) {
  const ketQuaProd = dsKichBan.map((kb) => chayProductionGrade(kb.giaTri, kb.coTheThuLai, kb.nhanhBLoi));
  const ketQuaDonGian = dsKichBan.map((kb) => chayDonGian(kb.giaTri, kb.coTheThuLai));

  const tapCanhProd = new Set<string>();
  for (const k of ketQuaProd) for (const c of k.dsCanhDaDi) tapCanhProd.add(c);
  const tapCanhDonGian = new Set<string>();
  for (const k of ketQuaDonGian) for (const c of k.dsCanhDaDi) tapCanhDonGian.add(c);

  return [
    {
      ten: "don-gian",
      tyLeHoanThanh: tinhTyLeHoanThanh(ketQuaDonGian.map((k) => k.thanhCong)),
      tyLeEdgeCoverage: tinhTyLeEdgeCoverage(tapCanhDonGian, TOAN_BO_CANH_CO_THE),
      tongLuotGoi: ketQuaDonGian.reduce((t, k) => t + k.soLuotGoi, 0),
    },
    {
      ten: "production-grade",
      tyLeHoanThanh: tinhTyLeHoanThanh(ketQuaProd.map((k) => k.thanhCong)),
      tyLeEdgeCoverage: tinhTyLeEdgeCoverage(tapCanhProd, TOAN_BO_CANH_CO_THE),
      tongLuotGoi: ketQuaProd.reduce((t, k) => t + k.soLuotGoi, 0),
    },
  ];
}

const boKichBanBoss: KichBanBoss[] = [
  { giaTri: 5, coTheThuLai: true, nhanhBLoi: false },
  { giaTri: 5, coTheThuLai: true, nhanhBLoi: true },
  { giaTri: -3, coTheThuLai: false, nhanhBLoi: false },
  { giaTri: -3, coTheThuLai: true, nhanhBLoi: false },
];
const bangSoSanhBoss = chayBangSoSanhBoss(boKichBanBoss);

const doThiKichBanDon = taoDoThiChinhBoss(-3, false);
const demGoiTuDau: Record<string, number> = {};
const ketQuaTuDau = chayDoThiCoDemGoi(doThiKichBanDon, "buoc1", demGoiTuDau);
const checkpointGiuaChung: Checkpoint = { tenNodeTiepTheo: "kiemTra", dsNodeDaDiTruocDo: ["buoc1", "buoc2", "buoc3", "buoc4"] };
const demGoiTuCheckpoint: Record<string, number> = {};
const ketQuaTuCheckpoint = chayDoThiTuCheckpoint(doThiKichBanDon, checkpointGiuaChung, demGoiTuCheckpoint);
const tongLuotGoiTuDau = tinhTongSoLuotGoi(demGoiTuDau);
const tongLuotGoiTuCheckpoint = tinhTongSoLuotGoi(demGoiTuCheckpoint);
const soLuotGoiTietKiemCheckpoint = tongLuotGoiTuDau - tongLuotGoiTuCheckpoint;

console.log(
  bangSoSanhBoss[0]!.tyLeHoanThanh, bangSoSanhBoss[1]!.tyLeHoanThanh,
  bangSoSanhBoss[0]!.tyLeEdgeCoverage, bangSoSanhBoss[1]!.tyLeEdgeCoverage,
  bangSoSanhBoss[0]!.tongLuotGoi, bangSoSanhBoss[1]!.tongLuotGoi,
  tongLuotGoiTuDau, tongLuotGoiTuCheckpoint, soLuotGoiTietKiemCheckpoint,
);
```

```typescript title=test
const donGian = bangSoSanhBoss[0]!;
const prod = bangSoSanhBoss[1]!;

if (Math.abs(donGian.tyLeHoanThanh - 0.5) > 1e-9) throw new Error("runner don-gian phai co tyLeHoanThanh=0.5 (2/4)");
if (Math.abs(prod.tyLeHoanThanh - 0.5) > 1e-9) throw new Error("runner production-grade phai CUNG co tyLeHoanThanh=0.5 (2/4, nhung khac kich ban)");

if (donGian.tongLuotGoi !== 37) throw new Error("runner don-gian phai ton DUNG 37 luot goi tren 4 kich ban");
if (prod.tongLuotGoi !== 33) throw new Error("runner production-grade phai ton DUNG 33 luot goi -- tiet kiem 4 so voi don-gian");

if (Math.abs(donGian.tyLeEdgeCoverage - 8 / 11) > 1e-9) throw new Error("runner don-gian phai dat edge coverage 8/11 (thieu 3 canh fan-out)");
if (Math.abs(prod.tyLeEdgeCoverage - 10 / 11) > 1e-9) throw new Error("runner production-grade phai dat edge coverage 10/11 (chi thieu canh quay lai)");

const ktKB1DonGian = chayDonGian(5, true);
if (ktKB1DonGian.thanhCong !== false) throw new Error("don-gian tren giaTri=5 (can fan-out) phai that bai (node_khong_ton_tai tai phanNhanh)");
const ktKB1Prod = chayProductionGrade(5, true, false);
if (ktKB1Prod.thanhCong !== true) throw new Error("production-grade tren giaTri=5, nhanh B hop le phai THANH CONG");

const ktKB4DonGian = chayDonGian(-3, true);
if (ktKB4DonGian.thanhCong !== true) throw new Error("don-gian tren chu trinh (giaTri=-3,coTheThuLai=true) phai BAO THANH CONG SAI (het buoc an toan)");
if (ktKB4DonGian.soLuotGoi !== 20) throw new Error("don-gian tren chu trinh phai ton dung 20 luot goi (het buoc an toan)");
const ktKB4Prod = chayProductionGrade(-3, true, false);
if (ktKB4Prod.thanhCong !== false) throw new Error("production-grade tren chu trinh phai BAO THAT BAI DUNG (phat hien chu trinh)");
if (ktKB4Prod.soLuotGoi !== 7) throw new Error("production-grade tren chu trinh phai ton dung 7 luot goi (4 chuan bi + kiemTra + xuLyLoi + thuLai)");

if (JSON.stringify(ketQuaTuDau.dsNodeDaDi) !== JSON.stringify(["buoc1", "buoc2", "buoc3", "buoc4", "kiemTra", "xuLyLoi", "boCuoc"])) {
  throw new Error("chay tu dau tren kich ban don (giaTri=-3,coTheThuLai=false) phai di qua dung 7 node");
}
if (tongLuotGoiTuDau !== 7) throw new Error("chay tu dau phai ton DUNG 7 luot goi");
if (tongLuotGoiTuCheckpoint !== 3) throw new Error("chay tu checkpoint (bo qua 4 buoc chuan bi) chi duoc ton DUNG 3 luot goi MOI");
if (soLuotGoiTietKiemCheckpoint !== 4) throw new Error("so luot goi tiet kiem nho checkpoint/resume phai la 4 (7 - 3)");

const ktBLoiProd = chayProductionGrade(5, true, true);
if (ktBLoiProd.thanhCong !== false) throw new Error("production-grade tren giaTri=5, nhanh B LOI phai THAT BAI");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (chayCoreCoPhatHienChuTrinh): mot vong while(tenHienTai !== null) -- kiem 'phanNhanh' TRUOC TIEN (return toi_phan_nhanh NGAY, KHONG tra cuu), roi kiem het buoc an toan, roi kiem chu trinh (Set), CHI KHI qua ca ba moi tang soBuoc/tra node/them vao Set+dsNodeDaDi/goi chay()/xu ly loi hoac di tiep; het vong lap (tenHienTai la null) tra ket_thuc. Cho hai (chayProductionGrade): goi chayCoreCoPhatHienChuTrinh tu 'buoc1'; neu KHONG dung o toi_phan_nhanh, tra thanhCong = (trangThaiDung === 'ket_thuc') kem canh/soLuotGoi cua phan loi; nguoc lai chon doThiB theo nhanhBLoi, goi chayFanOutFanIn, GOP canh+soLuotGoi cua CA phan loi LAN fan-out."
- kind: strategy
  body: "Cho dau: const dsNodeDaDi: string[] = []; const daGhePhamVi = new Set<string>(); let tenHienTai: string | null = tenNodeGoc; let soBuoc = 0; while (tenHienTai !== null) { if (tenHienTai === 'phanNhanh') { dsNodeDaDi.push(tenHienTai); return { trangThaiDung: 'toi_phan_nhanh', dsNodeDaDi }; } if (soBuoc >= SO_BUOC_TOI_DA_AN_TOAN) return { trangThaiDung: 'het_buoc', dsNodeDaDi }; if (daGhePhamVi.has(tenHienTai)) return { trangThaiDung: 'chu_trinh', dsNodeDaDi }; soBuoc++; const node: NodeXuLy | undefined = doThi[tenHienTai]; if (!node) return { trangThaiDung: 'loi', dsNodeDaDi }; daGhePhamVi.add(tenHienTai); dsNodeDaDi.push(tenHienTai); const kq: KetQuaNode = node.chay(); if (kq.trangThai === 'loi') return { trangThaiDung: 'loi', dsNodeDaDi }; tenHienTai = kq.tenNodeTiepTheo; } return { trangThaiDung: 'ket_thuc', dsNodeDaDi }; Cho hai: const doThiChinh = taoDoThiChinhBoss(giaTri, coTheThuLai); const ktCore = chayCoreCoPhatHienChuTrinh(doThiChinh, 'buoc1'); if (ktCore.trangThaiDung !== 'toi_phan_nhanh') { return { thanhCong: ktCore.trangThaiDung === 'ket_thuc', dsCanhDaDi: layCacCanhDaDi(ktCore.dsNodeDaDi), soLuotGoi: ktCore.dsNodeDaDi.length }; } const doThiB = nhanhBLoi ? doThiNhanhBLoi : doThiNhanhBHopLe; const ktFan = chayFanOutFanIn(doThiNhanhA, 'nhanhA1', doThiB, 'nhanhB1'); const canhCore = layCacCanhDaDi(ktCore.dsNodeDaDi); const canhFan = [...layCacCanhDaDi(ktFan.dsNodeDaDiNhanhA), ...layCacCanhDaDi(ktFan.dsNodeDaDiNhanhB)]; return { thanhCong: ktFan.thanhCong, dsCanhDaDi: [...canhCore, ...canhFan], soLuotGoi: ktCore.dsNodeDaDi.length + ktFan.dsNodeDaDiNhanhA.length + ktFan.dsNodeDaDiNhanhB.length };"
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung vi tri tuong ung -- CHU Y kiem 'phanNhanh' TRUOC ca het-buoc va chu-trinh."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 10000
- tier: output
  match: contains
  expect: "0.5 0.5 0.7272727272727273 0.9090909090909091 37 33 7 3 4"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Completion rate GIỐNG NHAU (`0.5` cả hai), NHƯNG đúng Ở hai bộ kịch bản
KHÁC nhau hoàn toàn — production-grade bắt được lỗi fan-out THẬT VÀ chu
trình THẬT, mà runner đơn giản hoặc bỏ sót hoàn toàn (fan-out) hoặc BÁO
SAI (chu trình). Edge coverage (`0.727` so VỚI `0.909`) VÀ tổng lệnh gọi
(`37` so VỚI `33`) LÀ hai con số KHÔNG nói dối được. Track `T9.5` —
"Kỹ thuật Đồ thị": node+edge, rẽ nhánh, edge coverage, checkpoint/resume,
catamorphism, fan-out/fan-in, audit trail, test từng nhánh, phát hiện
chu trình, VÀ so sánh cấu hình — khép lại tại `12/12`.
::::

::::reflect{#nghi-lai}
Con số gây bất ngờ NHẤT Ở BOSS này KHÔNG phải edge coverage hay tổng
lệnh gọi — mà LÀ `tyLeHoanThanh` GIỐNG HỆT nhau (`0.5`) Ở CẢ HAI runner,
dù chúng đúng SAI hoàn toàn KHÁC nhau Ở TỪNG kịch bản cụ thể. Đây LÀ bài
học lớn nhất của CẢ track `T9.5`: một con số TỔNG HỢP (completion rate)
có thể NGUỴ TRANG cho hai hệ thống hoàn toàn KHÁC NHAU về chất lượng —
CHỈ khi tách RA theo TỪNG trục (edge coverage, tổng lệnh gọi, VÀ — muốn
biết đúng SAI Ở ĐÂU — audit trail cùng test-từng-nhánh) mới lộ ra runner
NÀO thật sự đáng tin. MASTERPLAN định nghĩa GRAPH bằng bốn chế độ hỏng
("không resume/không song song/không audit/không test từng nhánh") VÀ
hai cách đo ("edge coverage, replay từ checkpoint") — CẢ `12` bài của
track NÀY, từ node+edge tường minh (bài `1`) tới BOSS NÀY, đều LÀ một
câu trả lời CHO ĐÚNG những chế độ hỏng VÀ cách đo đó, KHÔNG câu nào thừa.
::::

::::checkpoint{mastery=0.95}
::::
