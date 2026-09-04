---
id: thiet-ke-he-thong.trien-khai-an-toan.canary-tang-dan-theo-nguong-loi
title: "Canary rollout: tăng tỷ lệ dần, dừng khi vượt ngưỡng lỗi"
summary: "quyetDinhTangCanary(trangThai, soLoiQuanSat, soRequestQuanSat, nguongTyLeLoi, buocTang) tang tyLeHienTai them buocTang neu tyLeLoi <= nguong, dung VA giu nguyen neu vuot -- vi du tu 0%, 10 loi/1000 request (ty le 1%, duoi nguong 5%) tang len 10%; 60 loi/1000 (6%, VUOT nguong) tra ve dung_vi_vuot_nguong, ty le GIU NGUYEN o 10%; tai 100%, buoc tang tiep tra ve giu_nguyen_da_toi_da. Dieu kien la > (nghiem ngat) khong phai >=, nen ty le loi DUNG BANG nguong VAN duoc tang tiep."
locale: vi
track: thiet-ke-he-thong
module: trien-khai-an-toan
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [sd.canary-tang-dan-theo-nguong-loi]
requires: [sd.ba-chien-luoc-trien-khai]
concepts: [sd.canary-tang-dan-theo-nguong-loi]
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
Bài trước đo được: canary không hề tốn thêm tài nguyên như blue-green
— nó chỉ đưa MỘT tỷ lệ nhỏ traffic trước. Nhưng "tăng dần" không phải
LÀ một con số cố định chạy sẵn — nó LÀ một QUYẾT ĐỊNH lặp lại nhiều
lần: quan sát lỗi, RỒI mới quyết có tăng tiếp hay không.
::::

::::explain{#quyet-dinh-tang}
`quyetDinhTangCanary` nhận trạng thái hiện tại (`tyLeHienTai`) VÀ số
liệu quan sát được (`soLoiQuanSat`/`soRequestQuanSat`). Nếu tỷ lệ lỗi
tính RA vượt `nguongTyLeLoi`, hàm MUTATE trạng thái để đánh dấu đã
dừng VÀ trả về kết quả tương ứng — KHÔNG tăng tỷ lệ. Ngược lại, tăng
`tyLeHienTai` thêm `buocTang`, chặn Ở `100`:

```typescript title=readonly
interface TrangThaiCanary { tyLeHienTai: number; daDung: boolean; }
function taoTrangThaiCanary(): TrangThaiCanary { return { tyLeHienTai: 0, daDung: false }; }

type KetQuaQuyetDinh = "da_tang" | "giu_nguyen_da_toi_da" | "dung_vi_vuot_nguong";

function quyetDinhTangCanary(
  trangThai: TrangThaiCanary,
  soLoiQuanSat: number,
  soRequestQuanSat: number,
  nguongTyLeLoi: number,
  buocTang: number
): KetQuaQuyetDinh {
  const tyLeLoi = soRequestQuanSat === 0 ? 0 : soLoiQuanSat / soRequestQuanSat;
  if (tyLeLoi > nguongTyLeLoi) {
    trangThai.daDung = true;
    return "dung_vi_vuot_nguong";
  }
  if (trangThai.tyLeHienTai >= 100) return "giu_nguyen_da_toi_da";
  trangThai.tyLeHienTai = Math.min(100, trangThai.tyLeHienTai + buocTang);
  return "da_tang";
}

const ts = taoTrangThaiCanary();
console.log("ty le ban dau:", ts.tyLeHienTai);
console.log("buoc 1 (10 loi / 1000 request, nguong 5%):", quyetDinhTangCanary(ts, 10, 1000, 0.05, 10));
console.log("ty le sau buoc 1:", ts.tyLeHienTai);
console.log("buoc 2 (60 loi / 1000 request, VUOT nguong 5%):", quyetDinhTangCanary(ts, 60, 1000, 0.05, 10));
console.log("ty le sau buoc 2 (KHONG doi):", ts.tyLeHienTai);
console.log("da dung:", ts.daDung);
```

```text title=readonly
ty le ban dau: 0
buoc 1 (10 loi / 1000 request, nguong 5%): da_tang
ty le sau buoc 1: 10
buoc 2 (60 loi / 1000 request, VUOT nguong 5%): dung_vi_vuot_nguong
ty le sau buoc 2 (KHONG doi): 10
da dung: true
```

`10` lỗi trên `1000` request LÀ tỷ lệ `0.01` (`1%`), dưới ngưỡng `5%`
— canary được TĂNG từ `0` lên `10`. Nhưng `60` lỗi trên CÙNG `1000`
request LÀ tỷ lệ `0.06` (`6%`), VƯỢT ngưỡng — hàm dừng NGAY, `tyLeHienTai`
GIỮ NGUYÊN Ở `10`, không hề tăng thêm dù được gọi.
::::

::::example{#chan-o-100}
Khi `tyLeHienTai` ĐÃ đạt `100`, gọi tiếp `quyetDinhTangCanary` với số
liệu TỐT không hề làm nó vượt quá `100` — hàm nhận ra đã tối đa VÀ trả
về một kết quả RIÊNG, khác với "tăng":

```typescript title=readonly
interface TrangThaiCanary { tyLeHienTai: number; daDung: boolean; }
function taoTrangThaiCanary(): TrangThaiCanary { return { tyLeHienTai: 0, daDung: false }; }

type KetQuaQuyetDinh = "da_tang" | "giu_nguyen_da_toi_da" | "dung_vi_vuot_nguong";

function quyetDinhTangCanary(
  trangThai: TrangThaiCanary,
  soLoiQuanSat: number,
  soRequestQuanSat: number,
  nguongTyLeLoi: number,
  buocTang: number
): KetQuaQuyetDinh {
  const tyLeLoi = soRequestQuanSat === 0 ? 0 : soLoiQuanSat / soRequestQuanSat;
  if (tyLeLoi > nguongTyLeLoi) {
    trangThai.daDung = true;
    return "dung_vi_vuot_nguong";
  }
  if (trangThai.tyLeHienTai >= 100) return "giu_nguyen_da_toi_da";
  trangThai.tyLeHienTai = Math.min(100, trangThai.tyLeHienTai + buocTang);
  return "da_tang";
}

// tai lap trang thai: ty le da o 90% tu truoc, chua bao gio vuot nguong
const ts = taoTrangThaiCanary();
ts.tyLeHienTai = 90;

console.log("ty le truoc buoc nay:", ts.tyLeHienTai);
console.log("buoc (5 loi / 1000, duoi nguong), tang tiep:", quyetDinhTangCanary(ts, 5, 1000, 0.05, 20));
console.log("ty le sau buoc (CHAN o 100, khong vuot):", ts.tyLeHienTai);
console.log("goi THEM mot lan nua, van it loi:", quyetDinhTangCanary(ts, 5, 1000, 0.05, 20));
console.log("ty le KHONG doi, da toi da:", ts.tyLeHienTai);
```

```text title=readonly
ty le truoc buoc nay: 90
buoc (5 loi / 1000, duoi nguong), tang tiep: da_tang
ty le sau buoc (CHAN o 100, khong vuot): 100
goi THEM mot lan nua, van it loi: giu_nguyen_da_toi_da
ty le KHONG doi, da toi da: 100
```

Từ `90`, cộng thêm `buocTang=20` LẼ ra ra `110` — nhưng `Math.min(100,
...)` chặn LẠI Ở đúng `100`. Lần gọi TIẾP theo, dù số liệu vẫn tốt, hàm
kiểm tra `trangThai.tyLeHienTai >= 100` NGAY từ đầu VÀ trả về
`"giu_nguyen_da_toi_da"` — một trạng thái CUỐI cùng, khác hẳn "tăng
thành công" hay "bị dừng vì lỗi".
::::

::::predict{#doan-bien-nguong commitOnce}
Trạng thái đang Ở `tyLeHienTai=50`. Gọi `quyetDinhTangCanary(ts, 5,
100, 0.05, 10)` — `5` lỗi trên `100` request LÀ tỷ lệ ĐÚNG BẰNG ngưỡng
`0.05`. Kết quả LÀ gì, VÀ `tyLeHienTai` sau đó bằng bao nhiêu?

:::opt{correct}
`"da_tang"`, `tyLeHienTai` thành `60` — điều kiện dừng dùng `>` (nghiêm
ngặt), `0.05 > 0.05` LÀ `false`, nên hàm KHÔNG coi tỷ lệ lỗi bằng đúng
ngưỡng LÀ vượt ngưỡng, VÀ tiếp tục tăng bình thường
:::
:::opt
`"dung_vi_vuot_nguong"`, `tyLeHienTai` giữ nguyên `50` — tỷ lệ lỗi
CHẠM đúng ngưỡng cũng nên bị coi LÀ nguy hiểm, phải dừng NGAY để an
toàn
::why
Nhầm "chạm đúng ngưỡng" VỚI "vượt ngưỡng" — nhưng điều kiện trong code
LÀ `tyLeLoi > nguongTyLeLoi`, dùng phép so sánh nghiêm ngặt.

Chỗ lệch: `5 / 100 = 0.05`, VÀ `nguongTyLeLoi` cũng LÀ `0.05` — hai giá
trị bằng nhau NGHĨA LÀ `tyLeLoi > nguongTyLeLoi` cho ra `false`. Nhánh
`if` không chạy, hàm ĐI tiếp xuống nhánh tăng, cộng `buocTang=10` vào
`50` thành `60`, VÀ trả về `"da_tang"`. Muốn dừng NGAY tại đúng ngưỡng,
code sẽ phải viết `>=` thay vì `>` — đây LÀ một lựa chọn thiết kế có
chủ đích, không phải một lỗi.
::
:::
::::

::::code{#viet_quyet_dinh_tang_canary}
Hoàn thiện `quyetDinhTangCanary` — tính `tyLeLoi`, kiểm tra VƯỢT
ngưỡng (đánh dấu `daDung` VÀ dừng nếu vượt), kiểm tra ĐÃ tối đa (`100`)
để giữ nguyên, RỒI mới tăng `tyLeHienTai` thêm `buocTang`, chặn Ở
`100` bằng `Math.min`.

```typescript title=starter
interface TrangThaiCanary { tyLeHienTai: number; daDung: boolean; }
function taoTrangThaiCanary(): TrangThaiCanary { return { tyLeHienTai: 0, daDung: false }; }

type KetQuaQuyetDinh = "da_tang" | "giu_nguyen_da_toi_da" | "dung_vi_vuot_nguong";

function quyetDinhTangCanary(
  trangThai: TrangThaiCanary,
  soLoiQuanSat: number,
  soRequestQuanSat: number,
  nguongTyLeLoi: number,
  buocTang: number
): KetQuaQuyetDinh {
  ___
}

const tsX = taoTrangThaiCanary();
console.log(quyetDinhTangCanary(tsX, 1, 100, 0.05, 15), tsX.tyLeHienTai);
```

```typescript title=solution
interface TrangThaiCanary { tyLeHienTai: number; daDung: boolean; }
function taoTrangThaiCanary(): TrangThaiCanary { return { tyLeHienTai: 0, daDung: false }; }

type KetQuaQuyetDinh = "da_tang" | "giu_nguyen_da_toi_da" | "dung_vi_vuot_nguong";

function quyetDinhTangCanary(
  trangThai: TrangThaiCanary,
  soLoiQuanSat: number,
  soRequestQuanSat: number,
  nguongTyLeLoi: number,
  buocTang: number
): KetQuaQuyetDinh {
  const tyLeLoi = soRequestQuanSat === 0 ? 0 : soLoiQuanSat / soRequestQuanSat;
  if (tyLeLoi > nguongTyLeLoi) {
    trangThai.daDung = true;
    return "dung_vi_vuot_nguong";
  }
  if (trangThai.tyLeHienTai >= 100) return "giu_nguyen_da_toi_da";
  trangThai.tyLeHienTai = Math.min(100, trangThai.tyLeHienTai + buocTang);
  return "da_tang";
}

const tsX = taoTrangThaiCanary();
console.log(quyetDinhTangCanary(tsX, 1, 100, 0.05, 15), tsX.tyLeHienTai);
```

```typescript title=test
const tsT = taoTrangThaiCanary();
const k1 = quyetDinhTangCanary(tsT, 2, 200, 0.05, 25);
if (k1 !== "da_tang") throw new Error("ty le loi thap phai duoc tang");
if (tsT.tyLeHienTai !== 25) throw new Error("tang dung buocTang tu 0");

const k2 = quyetDinhTangCanary(tsT, 100, 200, 0.05, 25);
if (k2 !== "dung_vi_vuot_nguong") throw new Error("ty le loi 50% vuot nguong 5% phai DUNG");
if (tsT.tyLeHienTai !== 25) throw new Error("khi dung, ty le KHONG duoc tang them");
if (tsT.daDung !== true) throw new Error("co Dung phai duoc danh dau true");

const tsBienT = taoTrangThaiCanary();
tsBienT.tyLeHienTai = 30;
const k3 = quyetDinhTangCanary(tsBienT, 5, 100, 0.05, 20);
if (k3 !== "da_tang") throw new Error("ty le loi DUNG BANG nguong (0.05 = 0.05) khong duoc tinh la vuot -- dieu kien phai la > khong phai >=");
if (tsBienT.tyLeHienTai !== 50) throw new Error("truong hop bien phai VAN tang binh thuong");

const tsMaxT = taoTrangThaiCanary();
tsMaxT.tyLeHienTai = 95;
const k4 = quyetDinhTangCanary(tsMaxT, 0, 500, 0.05, 20);
if (k4 !== "da_tang") throw new Error("con duoi 100 phai tang, du buocTang vuot qua 100");
if (tsMaxT.tyLeHienTai !== 100) throw new Error("ty le phai bi CHAN o dung 100, khong vuot");

const k5 = quyetDinhTangCanary(tsMaxT, 0, 500, 0.05, 20);
if (k5 !== "giu_nguyen_da_toi_da") throw new Error("da o 100 roi thi giu nguyen, khong tang them");
if (tsMaxT.tyLeHienTai !== 100) throw new Error("ty le van la 100");
```

:::hints
- kind: attention
  body: "Ba buoc theo dung thu tu: (1) tinh tyLeLoi = soLoiQuanSat/soRequestQuanSat (tru truong hop soRequestQuanSat=0 thi tyLeLoi=0); (2) neu tyLeLoi > nguongTyLeLoi thi danh dau daDung=true va tra ve dung_vi_vuot_nguong; (3) neu da >= 100 thi tra ve giu_nguyen_da_toi_da; (4) nguoc lai tang tyLeHienTai bang Math.min(100, ...) va tra ve da_tang."
- kind: strategy
  body: "const tyLeLoi = soRequestQuanSat === 0 ? 0 : soLoiQuanSat / soRequestQuanSat; if (tyLeLoi > nguongTyLeLoi) { trangThai.daDung = true; return 'dung_vi_vuot_nguong'; } if (trangThai.tyLeHienTai >= 100) return 'giu_nguyen_da_toi_da'; trangThai.tyLeHienTai = Math.min(100, trangThai.tyLeHienTai + buocTang); return 'da_tang';"
- kind: one-line
  body: "Sao chep dung logic o phan Strategy, doi dau nhay don thanh dau nhay kep."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "da_tang 15"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một quyết định, gọi lại nhiều lần, luôn dựa trên số liệu MỚI nhất —
không phải một kịch bản viết sẵn. Nhưng canary chỉ LÀ MỘT trục kiểm
soát. Có những thay đổi cần bật tắt NGAY LẬP TỨC, không chờ deploy lại
— VÀ không liên quan gì tới tỷ lệ traffic.
::::

::::reflect{#nghi-lai}
`quyetDinhTangCanary` không hề tách RIÊNG "đo lỗi" khỏi "quyết định" —
nó LÀM cả hai trong một hàm, VÀ mutate trạng thái NGAY khi biết kết
quả. Ba nhánh trả về (`da_tang`, `giu_nguyen_da_toi_da`,
`dung_vi_vuot_nguong`) không phải LÀ ba cách diễn đạt của "thành công/
thất bại" — chúng LÀ ba tình huống VẬN HÀNH khác nhau, mỗi tình huống
đòi hỏi người vận hành phản ứng khác nhau.
::::

::::checkpoint{mastery=0.74}
::::
