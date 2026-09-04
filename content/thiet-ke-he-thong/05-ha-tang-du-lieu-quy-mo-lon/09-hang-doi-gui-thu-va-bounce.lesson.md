---
id: thiet-ke-he-thong.ha-tang-du-lieu-quy-mo-lon.hang-doi-gui-thu-va-bounce
title: "Gửi thư và bounce: hai loại lỗi, hai cách xử lý"
summary: "guiThu(thu, dh, ketQua) phan biet RO hai loai loi: 'loi_tam_thoi' (hop thu day) tang soLanThuLai va dat do tre THU LAI tang dan (backoff nhan doi moi lan, tuc la 1000ms roi 2000ms), giu trangThai 'cho_gui'; 'loi_vinh_vien' (dia chi khong ton tai) chuyen NGAY sang 'bounce', KHONG tang soLanThuLai, KHONG con co hoi thu lai nua. Hai duong xu ly tach biet hoan toan, khong co duong nao roi vao duong con lai."
locale: vi
track: thiet-ke-he-thong
module: ha-tang-du-lieu-quy-mo-lon
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [sd.hang-doi-gui-thu-va-bounce]
requires: [sd.hop-thu-nhieu-nhan]
concepts: [sd.hang-doi-gui-thu-va-bounce]
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
Nhãn giờ chỉ là tham chiếu. Nhưng gửi một email đi lại là chuyện KHÁC
— nó có thể thất bại. Có loại thất bại đáng để thử LẠI (hộp thư người
nhận tạm thời đầy), và có loại KHÔNG bao giờ nên thử lại (địa chỉ đó
chưa từng tồn tại). Trộn hai loại đó làm MỘT là lãng phí công sức thử
lại vô ích.
::::

::::explain{#hai-duong-loi}
`guiThu` rẽ theo ĐÚNG ba nhánh: thành công thì chuyển `"da_gui"`;
lỗi TẠM thời (`"loi_tam_thoi"`) thì tăng `soLanThuLai` VÀ đặt một độ
trễ trước lần thử tiếp theo — độ trễ đó NHÂN đôi sau MỖI lần lỗi liên
tiếp (tức LÀ `1000ms`, rồi `2000ms`, rồi `4000ms`); còn lỗi VĨNH viễn
(`"loi_vinh_vien"`) chuyển NGAY sang `"bounce"`, không hề tăng
`soLanThuLai`:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

type TrangThaiThu = "cho_gui" | "da_gui" | "bounce";
type KetQuaGui = "thanh_cong" | "loi_tam_thoi" | "loi_vinh_vien";
interface ThuTrongHangDoi {
  id: string;
  nguoiNhan: string;
  trangThai: TrangThaiThu;
  soLanThuLai: number;
  thoiDiemThuLaiTiepTheo: number;
}
function taoThu(id: string, nguoiNhan: string): ThuTrongHangDoi {
  return { id, nguoiNhan, trangThai: "cho_gui", soLanThuLai: 0, thoiDiemThuLaiTiepTheo: 0 };
}
const BASE_BACKOFF_MS = 1000;
function guiThu(thu: ThuTrongHangDoi, dh: DongHoMoPhong, ketQua: KetQuaGui): void {
  if (ketQua === "thanh_cong") {
    thu.trangThai = "da_gui";
    return;
  }
  if (ketQua === "loi_vinh_vien") {
    thu.trangThai = "bounce";
    return;
  }
  thu.soLanThuLai += 1;
  const doTre = BASE_BACKOFF_MS * 2 ** (thu.soLanThuLai - 1);
  thu.thoiDiemThuLaiTiepTheo = dh.thoiGianHienTai + doTre;
}
function sanSangThuLai(thu: ThuTrongHangDoi, dh: DongHoMoPhong): boolean {
  return thu.trangThai === "cho_gui" && dh.thoiGianHienTai >= thu.thoiDiemThuLaiTiepTheo;
}

const dh = taoDongHoMoPhong();
const thu = taoThu("thu-1", "an@vidu.com");

guiThu(thu, dh, "loi_tam_thoi");
console.log("lan 1, hop thu day (loi tam thoi):", thu.trangThai, "so lan thu lai:", thu.soLanThuLai, "thoi diem thu lai tiep theo:", thu.thoiDiemThuLaiTiepTheo);

tienThoiGian(dh, thu.thoiDiemThuLaiTiepTheo - dh.thoiGianHienTai);
console.log("san sang thu lai?", sanSangThuLai(thu, dh));

guiThu(thu, dh, "loi_tam_thoi");
console.log("lan 2, van day (loi tam thoi):", thu.trangThai, "so lan thu lai:", thu.soLanThuLai, "thoi diem thu lai tiep theo:", thu.thoiDiemThuLaiTiepTheo);

tienThoiGian(dh, thu.thoiDiemThuLaiTiepTheo - dh.thoiGianHienTai);
guiThu(thu, dh, "thanh_cong");
console.log("lan 3, thanh cong:", thu.trangThai, "so lan thu lai (khong doi):", thu.soLanThuLai);
```

```text title=readonly
lan 1, hop thu day (loi tam thoi): cho_gui so lan thu lai: 1 thoi diem thu lai tiep theo: 1000
san sang thu lai? true
lan 2, van day (loi tam thoi): cho_gui so lan thu lai: 2 thoi diem thu lai tiep theo: 3000
lan 3, thanh cong: da_gui so lan thu lai (khong doi): 2
```

Sau lỗi tạm thời LẦN đầu, `thu` vẫn Ở `"cho_gui"` — chỉ LÀ phải chờ
tới `t=1000` mới `sanSangThuLai`. Lần lỗi THỨ hai đặt độ trễ dài GẤP
đôi lần trước, tính TỪ mốc `t=1000` — thời điểm thử lại tiếp theo LÀ
`3000`. Khi cuối cùng gửi thành CÔNG, `soLanThuLai` giữ nguyên giá
trị đã tích luỹ (`2`) — nó LÀ một bản GHI lịch sử, không bị xoá đi.
::::

::::example{#loi-vinh-vien-bounce-ngay}
Lỗi VĨNH viễn không hề đi qua đường thử lại — nó rẽ SANG một nhánh
hoàn toàn khác, kết THÚC ngay lập tức:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

type TrangThaiThu = "cho_gui" | "da_gui" | "bounce";
type KetQuaGui = "thanh_cong" | "loi_tam_thoi" | "loi_vinh_vien";
interface ThuTrongHangDoi {
  id: string;
  nguoiNhan: string;
  trangThai: TrangThaiThu;
  soLanThuLai: number;
  thoiDiemThuLaiTiepTheo: number;
}
function taoThu(id: string, nguoiNhan: string): ThuTrongHangDoi {
  return { id, nguoiNhan, trangThai: "cho_gui", soLanThuLai: 0, thoiDiemThuLaiTiepTheo: 0 };
}
const BASE_BACKOFF_MS = 1000;
function guiThu(thu: ThuTrongHangDoi, dh: DongHoMoPhong, ketQua: KetQuaGui): void {
  if (ketQua === "thanh_cong") {
    thu.trangThai = "da_gui";
    return;
  }
  if (ketQua === "loi_vinh_vien") {
    thu.trangThai = "bounce";
    return;
  }
  thu.soLanThuLai += 1;
  const doTre = BASE_BACKOFF_MS * 2 ** (thu.soLanThuLai - 1);
  thu.thoiDiemThuLaiTiepTheo = dh.thoiGianHienTai + doTre;
}
function sanSangThuLai(thu: ThuTrongHangDoi, dh: DongHoMoPhong): boolean {
  return thu.trangThai === "cho_gui" && dh.thoiGianHienTai >= thu.thoiDiemThuLaiTiepTheo;
}

const dh = taoDongHoMoPhong();
const thu = taoThu("thu-2", "khong-ton-tai@vidu.com");

guiThu(thu, dh, "loi_vinh_vien");
console.log("dia chi khong ton tai (loi vinh vien):", thu.trangThai, "so lan thu lai:", thu.soLanThuLai);
console.log("san sang thu lai?", sanSangThuLai(thu, dh));

tienThoiGian(dh, 999999);
console.log("du da troi qua rat lau, van san sang thu lai?", sanSangThuLai(thu, dh));
```

```text title=readonly
dia chi khong ton tai (loi vinh vien): bounce so lan thu lai: 0
san sang thu lai? false
du da troi qua rat lau, van san sang thu lai? false
```

Chỉ MỘT lần gọi `guiThu` VỚI `"loi_vinh_vien"` LÀ đủ để `trangThai`
nhảy thẳng sang `"bounce"` — `soLanThuLai` không hề bị đụng tới, VẪN
LÀ `0`. Dù đồng hồ trôi xa bao NHIÊU, `sanSangThuLai` vẫn LÀ `false`
— vì hàm đó chỉ xét thư đang `"cho_gui"`, VÀ `"bounce"` LÀ một trạng
thái kết THÚC, không có đường quay lại.
::::

::::predict{#doan-bounce-khong-cong-them-so-lan commitOnce}
Một thư đã lỗi TẠM thời hai lần liên tiếp (`soLanThuLai = 2`, vẫn Ở
`"cho_gui"`). Lần GỌI `guiThu` thứ ba nhận kết quả `"loi_vinh_vien"`.
Ngay SAU lần gọi đó, `trangThai` VÀ `soLanThuLai` LÀ gì?

:::opt{correct}
`trangThai` LÀ `"bounce"`, `soLanThuLai` vẫn LÀ `2` — nhánh
`"loi_vinh_vien"` return NGAY sau khi đặt `trangThai`, không hề chạm
tới dòng `thu.soLanThuLai += 1`
:::
:::opt
`trangThai` LÀ `"bounce"`, `soLanThuLai` LÀ `3` — lần gọi này VẪN tính
LÀ một lần thử, nên bộ đếm phải tăng lên TRƯỚC khi hàm quyết định
bounce
::why
Nhầm "một lần GỌI hàm" VỚI "một lần thử lại được ĐẾM" — nhưng
`soLanThuLai` chỉ tăng Ở nhánh `"loi_tam_thoi"`, không tăng Ở bất kỳ
nhánh NÀO khác.

Chỗ lệch: nhánh `if (ketQua === "loi_vinh_vien") { thu.trangThai =
"bounce"; return; }` đứng TRƯỚC dòng `thu.soLanThuLai += 1;` VÀ có
`return` riêng của NÓ — luồng thực thi thoát khỏi hàm NGAY tại đó,
chưa bao giờ chạy tới dòng tăng bộ đếm. `soLanThuLai` giữ nguyên giá
trị `2` đã tích luỹ TỪ trước.
::
:::
::::

::::code{#viet_gui_thu}
Hoàn thiện `guiThu` — nhánh thành công đã có sẵn. Còn thiếu: nhánh lỗi
vĩnh viễn (chuyển `"bounce"` NGAY, không thử lại) VÀ nhánh lỗi tạm
thời (tăng `soLanThuLai`, đặt độ trễ thử lại tiếp theo gấp đôi lần
trước).

```typescript title=starter
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

type TrangThaiThu = "cho_gui" | "da_gui" | "bounce";
type KetQuaGui = "thanh_cong" | "loi_tam_thoi" | "loi_vinh_vien";
interface ThuTrongHangDoi {
  id: string;
  nguoiNhan: string;
  trangThai: TrangThaiThu;
  soLanThuLai: number;
  thoiDiemThuLaiTiepTheo: number;
}
function taoThu(id: string, nguoiNhan: string): ThuTrongHangDoi {
  return { id, nguoiNhan, trangThai: "cho_gui", soLanThuLai: 0, thoiDiemThuLaiTiepTheo: 0 };
}
const BASE_BACKOFF_MS = 1000;
function guiThu(thu: ThuTrongHangDoi, dh: DongHoMoPhong, ketQua: KetQuaGui): void {
  if (ketQua === "thanh_cong") {
    thu.trangThai = "da_gui";
    return;
  }
  ___
}
function sanSangThuLai(thu: ThuTrongHangDoi, dh: DongHoMoPhong): boolean {
  return thu.trangThai === "cho_gui" && dh.thoiGianHienTai >= thu.thoiDiemThuLaiTiepTheo;
}

const dhX = taoDongHoMoPhong();
const thuX = taoThu("tx", "y@vidu.com");
guiThu(thuX, dhX, "loi_tam_thoi");
console.log(thuX.trangThai, thuX.soLanThuLai, thuX.thoiDiemThuLaiTiepTheo);
```

```typescript title=solution
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

type TrangThaiThu = "cho_gui" | "da_gui" | "bounce";
type KetQuaGui = "thanh_cong" | "loi_tam_thoi" | "loi_vinh_vien";
interface ThuTrongHangDoi {
  id: string;
  nguoiNhan: string;
  trangThai: TrangThaiThu;
  soLanThuLai: number;
  thoiDiemThuLaiTiepTheo: number;
}
function taoThu(id: string, nguoiNhan: string): ThuTrongHangDoi {
  return { id, nguoiNhan, trangThai: "cho_gui", soLanThuLai: 0, thoiDiemThuLaiTiepTheo: 0 };
}
const BASE_BACKOFF_MS = 1000;
function guiThu(thu: ThuTrongHangDoi, dh: DongHoMoPhong, ketQua: KetQuaGui): void {
  if (ketQua === "thanh_cong") {
    thu.trangThai = "da_gui";
    return;
  }
  if (ketQua === "loi_vinh_vien") {
    thu.trangThai = "bounce";
    return;
  }
  thu.soLanThuLai += 1;
  const doTre = BASE_BACKOFF_MS * 2 ** (thu.soLanThuLai - 1);
  thu.thoiDiemThuLaiTiepTheo = dh.thoiGianHienTai + doTre;
}
function sanSangThuLai(thu: ThuTrongHangDoi, dh: DongHoMoPhong): boolean {
  return thu.trangThai === "cho_gui" && dh.thoiGianHienTai >= thu.thoiDiemThuLaiTiepTheo;
}

const dhX = taoDongHoMoPhong();
const thuX = taoThu("tx", "y@vidu.com");
guiThu(thuX, dhX, "loi_tam_thoi");
console.log(thuX.trangThai, thuX.soLanThuLai, thuX.thoiDiemThuLaiTiepTheo);
```

```typescript title=test
const dhT = taoDongHoMoPhong();
const thuT = taoThu("thu-t1", "an@vidu.com");
guiThu(thuT, dhT, "loi_tam_thoi");
const trangThaiSauLan1 = thuT.trangThai;
if (trangThaiSauLan1 !== "cho_gui") throw new Error("loi tam thoi phai giu trangThai cho_gui (khong bi bounce)");
const soLanThuLaiSauLan1 = thuT.soLanThuLai;
if (soLanThuLaiSauLan1 !== 1) throw new Error("lan loi tam thoi dau tien phai tang soLanThuLai len 1");
const doTreSauLan1 = thuT.thoiDiemThuLaiTiepTheo;
if (doTreSauLan1 !== 1000) throw new Error("do tre lan dau phai la BASE_BACKOFF_MS (1000ms)");

tienThoiGian(dhT, 1000);
guiThu(thuT, dhT, "loi_tam_thoi");
const soLanThuLaiSauLan2 = thuT.soLanThuLai;
if (soLanThuLaiSauLan2 !== 2) throw new Error("lan loi tam thoi thu hai phai tang soLanThuLai len 2");
const doTreSauLan2 = thuT.thoiDiemThuLaiTiepTheo;
if (doTreSauLan2 !== 3000) throw new Error("do tre lan hai phai gap doi, tinh tu t=1000");

tienThoiGian(dhT, 2000);
guiThu(thuT, dhT, "thanh_cong");
const trangThaiSauThanhCong = thuT.trangThai;
if (trangThaiSauThanhCong !== "da_gui") throw new Error("thanh cong phai chuyen trangThai sang da_gui");
const soLanThuLaiSauThanhCong = thuT.soLanThuLai;
if (soLanThuLaiSauThanhCong !== 2) throw new Error("thanh cong khong duoc thay doi soLanThuLai da tich luy");

const dhT2 = taoDongHoMoPhong();
const thuT2 = taoThu("thu-t2", "khong-ton-tai@vidu.com");
guiThu(thuT2, dhT2, "loi_vinh_vien");
if (thuT2.trangThai !== "bounce") throw new Error("loi vinh vien phai chuyen trangThai sang bounce NGAY");
if (thuT2.soLanThuLai !== 0) throw new Error("loi vinh vien khong duoc tang soLanThuLai (khong he thu lai)");
if (sanSangThuLai(thuT2, dhT2) !== false) throw new Error("thu da bounce khong bao gio san sang thu lai");

const dhT3 = taoDongHoMoPhong();
const thuT3 = taoThu("thu-t3", "day@vidu.com");
guiThu(thuT3, dhT3, "loi_tam_thoi");
guiThu(thuT3, dhT3, "loi_tam_thoi");
if (thuT3.soLanThuLai !== 2) throw new Error("hai lan loi tam thoi lien tiep phai la 2");
guiThu(thuT3, dhT3, "loi_vinh_vien");
if (thuT3.trangThai !== "bounce") throw new Error("loi vinh vien sau do van phai bounce du truoc do da thu lai");
if (thuT3.soLanThuLai !== 2) throw new Error("loi vinh vien KHONG duoc cong them vao soLanThuLai da tich luy tu truoc");
```

:::hints
- kind: attention
  body: "Con thieu hai nhanh: neu ketQua la 'loi_vinh_vien' thi dat trangThai = 'bounce' roi return NGAY; con lai (loi_tam_thoi) thi tang soLanThuLai, tinh doTre = BASE_BACKOFF_MS nhan doi theo soLanThuLai, roi cong vao dh.thoiGianHienTai."
- kind: strategy
  body: "if (ketQua === 'loi_vinh_vien') { thu.trangThai = 'bounce'; return; } thu.soLanThuLai += 1; const doTre = BASE_BACKOFF_MS * 2 ** (thu.soLanThuLai - 1); thu.thoiDiemThuLaiTiepTheo = dh.thoiGianHienTai + doTre;"
- kind: one-line
  body: "if (ketQua === \"loi_vinh_vien\") { thu.trangThai = \"bounce\"; return; } thu.soLanThuLai += 1; const doTre = BASE_BACKOFF_MS * 2 ** (thu.soLanThuLai - 1); thu.thoiDiemThuLaiTiepTheo = dh.thoiGianHienTai + doTre;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "cho_gui 1 1000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chín mảnh hạ tầng đã xong — phân vùng, ACK, gom metric, cảnh báo, chia
khối, khử trùng, sao lưu, nhãn, gửi thư. Giờ ráp bốn mảnh CUỐI thành
một luồng upload hoàn chỉnh.
::::

::::reflect{#nghi-lai}
`guiThu` không hề coi "lỗi" LÀ một khối duy nhất — nó buộc người viết
PHẢI trả lời câu hỏi "lỗi này có đáng thử LẠI hay không?" ngay tại chỗ
xảy ra, thay VÌ xử lý mọi lỗi giống hệt nhau RỒI hối tiếc VỀ sau. Backoff
tăng dần chỉ có Ý nghĩa cho loại lỗi CÓ cơ hội tự khỏi theo thời gian.
::::

::::checkpoint{mastery=0.82}
::::
