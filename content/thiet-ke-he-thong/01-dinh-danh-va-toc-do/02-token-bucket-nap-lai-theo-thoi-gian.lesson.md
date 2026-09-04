---
id: thiet-ke-he-thong.dinh-danh-va-toc-do.token-bucket-nap-lai-theo-thoi-gian
title: "Token bucket: nạp lại theo thời gian"
summary: "napTheoThoiGian tính soTokenNap = (soMsTroiQua/1000) × tocDoNapMoiGiay rồi cộng dồn CÓ giới hạn ở soTokenToiDa (Math.min) — nạp 'lười', chỉ tính khi bucket được chạm tới (tieuThuTokenTG gọi nó mỗi lần), không dùng interval/setTimeout. Bucket dung lượng 10, tốc độ 5 token/giây: đợi 1000ms nạp đúng 5 token; đợi thêm 3000ms (lẽ ra +15) bị CHẶN ở trần 10 (Math.min phát huy tác dụng)."
locale: vi
track: thiet-ke-he-thong
module: dinh-danh-va-toc-do
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [sd.token-bucket-nap-lai-theo-thoi-gian]
requires: [sd.token-bucket-tieu-thu]
concepts: [sd.token-bucket-nap-lai-theo-thoi-gian]
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
Bài trước dùng `napDayLai` — một CÁI nút "nạp đầy" thủ công, không
THẬT. Rate limiter thật nạp token LIÊN tục, tỉ lệ THUẬN với thời gian
trôi qua — không CẦN ai đó bấm nút.
::::

::::explain{#nap-theo-thoi-gian}
`ThungTokenTG` nhớ THÊM `tocDoNapMoiGiay` (bao NHIÊU token mỗi giây)
VÀ `thoiDiemNapCuoi` (mốc THỜI gian ẢO lần nạp gần nhất). Mỗi lần
bucket được CHẠM tới, `napTheoThoiGian` tính số mili-giây đã trôi QUA
rồi cộng đúng tỉ LỆ đó — có giới hạn Ở `soTokenToiDa`:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface ThungTokenTG { soTokenHienTai: number; soTokenToiDa: number; tocDoNapMoiGiay: number; thoiDiemNapCuoi: number; }
function taoThungTokenTG(soTokenToiDa: number, tocDoNapMoiGiay: number, dh: DongHoMoPhong): ThungTokenTG {
  return { soTokenHienTai: soTokenToiDa, soTokenToiDa, tocDoNapMoiGiay, thoiDiemNapCuoi: dh.thoiGianHienTai };
}
function napTheoThoiGian(thung: ThungTokenTG, dh: DongHoMoPhong): void {
  const soMsTroiQua = dh.thoiGianHienTai - thung.thoiDiemNapCuoi;
  if (soMsTroiQua <= 0) return;
  const soTokenNap = (soMsTroiQua / 1000) * thung.tocDoNapMoiGiay;
  thung.soTokenHienTai = Math.min(thung.soTokenToiDa, thung.soTokenHienTai + soTokenNap);
  thung.thoiDiemNapCuoi = dh.thoiGianHienTai;
}
function tieuThuTokenTG(thung: ThungTokenTG, dh: DongHoMoPhong): boolean {
  napTheoThoiGian(thung, dh);
  if (thung.soTokenHienTai < 1) return false;
  thung.soTokenHienTai -= 1;
  return true;
}

const dh = taoDongHoMoPhong();
const thung = taoThungTokenTG(10, 5, dh);

const dot1: boolean[] = [];
for (let i = 0; i < 11; i++) dot1.push(tieuThuTokenTG(thung, dh));
console.log("dot 1 (t=0, 11 request lien tiep):", dot1.join(","));
console.log("con lai sau dot 1:", thung.soTokenHienTai);

tienThoiGian(dh, 1000);
console.log("sau khi tien 1000ms, chua tieu thu, con lai (chi tinh khi goi ham):", thung.soTokenHienTai);
const dot2: boolean[] = [];
for (let i = 0; i < 6; i++) dot2.push(tieuThuTokenTG(thung, dh));
console.log("dot 2 (t=1000, 6 request):", dot2.join(","));

tienThoiGian(dh, 3000);
tieuThuTokenTG(thung, dh);
console.log("sau khi tien them 3000ms roi tieu thu 1, con lai (phai bi CHAN o soTokenToiDa=10):", thung.soTokenHienTai);
```

```text title=readonly
dot 1 (t=0, 11 request lien tiep): true,true,true,true,true,true,true,true,true,true,false
con lai sau dot 1: 0
sau khi tien 1000ms, chua tieu thu, con lai (chi tinh khi goi ham): 0
dot 2 (t=1000, 6 request): true,true,true,true,true,false
sau khi tien them 3000ms roi tieu thu 1, con lai (phai bi CHAN o soTokenToiDa=10): 9
```

Chú ý dòng "chua tieu thu, con lai... 0" — `napTheoThoiGian` KHÔNG chạy
NGẦM theo thời gian thực; nó chỉ tính TOÁN khi CÓ ai đó gọi
`tieuThuTokenTG`. Đây LÀ nạp "lười" (lazy): không CẦN interval chạy
nền, chỉ cần biết CHÍNH XÁC bao nhiêu mili-giây đã trôi qua tại đúng
THỜI điểm cần dùng. Ở dot 2, `1000ms × 5 token/giây = 5` token — khớp
đúng SỐ request được cho qua. Cuối cùng, `3000ms × 5 = 15` token lẽ
ra được NẠP, nhưng `Math.min` chặn Ở `soTokenToiDa = 10`.
::::

::::example{#tich-luy-tung-phan}
Nạp KHÔNG chỉ hoạt động theo số nguyên giây — MỘT khoảng thời gian
LẺ (dưới 1 giây) vẫn tích luỹ ĐÚNG tỉ lệ, không bị làm tròn VỀ 0:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface ThungTokenTG { soTokenHienTai: number; soTokenToiDa: number; tocDoNapMoiGiay: number; thoiDiemNapCuoi: number; }
function napTheoThoiGian(thung: ThungTokenTG, dh: DongHoMoPhong): void {
  const soMsTroiQua = dh.thoiGianHienTai - thung.thoiDiemNapCuoi;
  if (soMsTroiQua <= 0) return;
  const soTokenNap = (soMsTroiQua / 1000) * thung.tocDoNapMoiGiay;
  thung.soTokenHienTai = Math.min(thung.soTokenToiDa, thung.soTokenHienTai + soTokenNap);
  thung.thoiDiemNapCuoi = dh.thoiGianHienTai;
}
function tieuThuTokenTG(thung: ThungTokenTG, dh: DongHoMoPhong): boolean {
  napTheoThoiGian(thung, dh);
  if (thung.soTokenHienTai < 1) return false;
  thung.soTokenHienTai -= 1;
  return true;
}

const dh = taoDongHoMoPhong();
// bat dau RONG (0 token), toc do 2 token/giay -- moi 250ms tich luy dung 0.5 token
const thung: ThungTokenTG = { soTokenHienTai: 0, soTokenToiDa: 5, tocDoNapMoiGiay: 2, thoiDiemNapCuoi: dh.thoiGianHienTai };

for (let i = 0; i < 4; i++) {
  tienThoiGian(dh, 250);
  const duocPhep = tieuThuTokenTG(thung, dh);
  console.log(`t=${dh.thoiGianHienTai}ms: duoc phep=${duocPhep}, con lai=${thung.soTokenHienTai}`);
}
```

```text title=readonly
t=250ms: duoc phep=false, con lai=0.5
t=500ms: duoc phep=true, con lai=0
t=750ms: duoc phep=false, con lai=0.5
t=1000ms: duoc phep=true, con lai=0
```

Tốc độ `2` token/giây nghĩa LÀ đúng `250ms` tích luỹ `0.5` token —
CHƯA đủ `1` nên bị từ chối. Sau `500ms` (hai LẦN `250ms`), tích luỹ
đủ `1.0` token NGUYÊN, được cho qua. Mẫu LẶP lại đúng chu kỳ: cứ
`500ms` LÀ có đúng một request được phép — khớp CHÍNH XÁC với "2
token/giây = 1 token mỗi 500ms".
::::

::::predict{#doan-nghi-lau-nap-nhieu commitOnce}
Một bucket dung lượng `10`, tốc độ `3` token/giây, ĐANG có `0` token.
Nếu KHÔNG ai gọi `tieuThuTokenTG` (hay bất kỳ hàm nào) trong SUỐT
`10000ms` (`10` giây) rồi mới gọi `tieuThuTokenTG` đúng MỘT lần, kết
quả gọi đó VÀ số token còn lại sau đó là gì?

:::opt{correct}
`true`, còn lại `9` — `10000ms × 3/giây = 30` token LẼ ra tích luỹ,
nhưng bị `Math.min` CHẶN Ở `soTokenToiDa = 10`, rồi tiêu thụ MẤT `1`
:::
:::opt
`true`, còn lại `29` — bucket tích luỹ ĐỦ nguyên `30` token (không có
giới hạn NÀO áp dụng khi tính TOÁN qua một khoảng thời gian dài), trừ
đi `1` cho request VỪA gọi
::why
Nhầm "công thức tính SỐ token nạp" (`(soMsTroiQua/1000) ×
tocDoNapMoiGiay`, không giới hạn) VỚI "số token THỰC SỰ được LƯU vào
bucket" (LUÔN đi qua `Math.min` với `soTokenToiDa` NGAY sau đó).

Chỗ lệch: `napTheoThoiGian` tính `soTokenNap = 30` đúng, nhưng dòng
KẾ tiếp LÀ `thung.soTokenHienTai = Math.min(thung.soTokenToiDa,
thung.soTokenHienTai + soTokenNap)` — GIÁ trị LƯU vào không bao giờ
vượt `soTokenToiDa`. Dù có ĐỢI bao lâu đi nữa, bucket dung lượng `10`
CHỈ có thể chứa TỐI ĐA `10` token, không bao giờ `29` hay `30`.
::
:::
::::

::::code{#viet_nap_theo_thoi_gian}
Hoàn thiện `napTheoThoiGian` — cộng `soTokenNap` VÀO số token hiện
có, nhưng KHÔNG được vượt quá `soTokenToiDa` (dùng `Math.min`).

```typescript title=starter
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface ThungTokenTG { soTokenHienTai: number; soTokenToiDa: number; tocDoNapMoiGiay: number; thoiDiemNapCuoi: number; }
function napTheoThoiGian(thung: ThungTokenTG, dh: DongHoMoPhong): void {
  const soMsTroiQua = dh.thoiGianHienTai - thung.thoiDiemNapCuoi;
  if (soMsTroiQua <= 0) return;
  const soTokenNap = (soMsTroiQua / 1000) * thung.tocDoNapMoiGiay;
  ___
  thung.thoiDiemNapCuoi = dh.thoiGianHienTai;
}
function tieuThuTokenTG(thung: ThungTokenTG, dh: DongHoMoPhong): boolean {
  napTheoThoiGian(thung, dh);
  if (thung.soTokenHienTai < 1) return false;
  thung.soTokenHienTai -= 1;
  return true;
}

const dh = taoDongHoMoPhong();
const thung: ThungTokenTG = { soTokenHienTai: 0, soTokenToiDa: 3, tocDoNapMoiGiay: 1, thoiDiemNapCuoi: dh.thoiGianHienTai };
tienThoiGian(dh, 2000);
console.log(tieuThuTokenTG(thung, dh), thung.soTokenHienTai);
```

```typescript title=solution
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface ThungTokenTG { soTokenHienTai: number; soTokenToiDa: number; tocDoNapMoiGiay: number; thoiDiemNapCuoi: number; }
function napTheoThoiGian(thung: ThungTokenTG, dh: DongHoMoPhong): void {
  const soMsTroiQua = dh.thoiGianHienTai - thung.thoiDiemNapCuoi;
  if (soMsTroiQua <= 0) return;
  const soTokenNap = (soMsTroiQua / 1000) * thung.tocDoNapMoiGiay;
  thung.soTokenHienTai = Math.min(thung.soTokenToiDa, thung.soTokenHienTai + soTokenNap);
  thung.thoiDiemNapCuoi = dh.thoiGianHienTai;
}
function tieuThuTokenTG(thung: ThungTokenTG, dh: DongHoMoPhong): boolean {
  napTheoThoiGian(thung, dh);
  if (thung.soTokenHienTai < 1) return false;
  thung.soTokenHienTai -= 1;
  return true;
}

const dh = taoDongHoMoPhong();
const thung: ThungTokenTG = { soTokenHienTai: 0, soTokenToiDa: 3, tocDoNapMoiGiay: 1, thoiDiemNapCuoi: dh.thoiGianHienTai };
tienThoiGian(dh, 2000);
console.log(tieuThuTokenTG(thung, dh), thung.soTokenHienTai);
```

```typescript title=test
function laySoTokenConLaiTG(t: ThungTokenTG): number { return t.soTokenHienTai; }

const dhT = taoDongHoMoPhong();
const thungT: ThungTokenTG = { soTokenHienTai: 0, soTokenToiDa: 3, tocDoNapMoiGiay: 1, thoiDiemNapCuoi: dhT.thoiGianHienTai };
tienThoiGian(dhT, 5000);
const ketQuaT: boolean[] = [];
for (let i = 0; i < 4; i++) ketQuaT.push(tieuThuTokenTG(thungT, dhT));
if (ketQuaT.join(",") !== "true,true,true,false") throw new Error("5000ms voi rate=1/s tich luy 5 token nhung PHAI bi CHAN o soTokenToiDa=3");
if (laySoTokenConLaiTG(thungT) !== 0) throw new Error("sau 3 lan tieu thu tu muc tran 3, con lai phai la 0");

const dhT2 = taoDongHoMoPhong();
const thungT2: ThungTokenTG = { soTokenHienTai: 2, soTokenToiDa: 10, tocDoNapMoiGiay: 1, thoiDiemNapCuoi: dhT2.thoiGianHienTai };
tienThoiGian(dhT2, 0);
tieuThuTokenTG(thungT2, dhT2);
if (laySoTokenConLaiTG(thungT2) !== 1) throw new Error("khong troi qua thoi gian nao (0ms) thi khong nap them token nao");
```

:::hints
- kind: attention
  body: "Gan lai thung.soTokenHienTai bang Math.min giua soTokenToiDa va (soTokenHienTai + soTokenNap) -- mot dong."
- kind: strategy
  body: "thung.soTokenHienTai = Math.min(thung.soTokenToiDa, thung.soTokenHienTai + soTokenNap);"
- kind: one-line
  body: "thung.soTokenHienTai = Math.min(thung.soTokenToiDa, thung.soTokenHienTai + soTokenNap);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Token bucket giờ TỰ nạp theo thời gian, không cần bấm nút. Nhưng nó
vẫn cho phép BURST — dùng dồn hết token tích luỹ trong MỘT khoảnh
khắc. Có thuật toán NÀO buộc luồng ra đều đặn, không burst?
::::

::::reflect{#nghi-lai}
`napTheoThoiGian` chỉ LÀ một phép tính TỈ lệ VÀ một `Math.min` — nhưng
tính "lười" (chỉ tính KHI cần, không chạy nền) LÀ lý do token bucket
mô phỏng được HÀNG giờ trong MỘT tích tắc: không CẦN interval, không
CẦN chờ thời gian thật, chỉ cần biết CHÍNH XÁC bao nhiêu mili-giây đã
trôi qua.
::::

::::checkpoint{mastery=0.75}
::::
