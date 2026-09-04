---
id: thiet-ke-he-thong.noi-dung-va-kham-pha.boss-noi-dung-va-kham-pha
title: "BOSS — đăng video, fan-out, tìm lại qua autocomplete"
summary: "dangVideo rap dung 3 mang: transcode (chayTranscodeDenKhiXong tien dong ho 300ms, tra hoan_tat) -> fan-out hybrid (<=3 follower thi day vao inbox, >3 thi bo qua) -> chen tieu de vao trie (LUON chay, doc lap voi fan-out). Video cua tac gia thuong (1 follower) VA video cua tac gia 'noi tieng' (5 follower, fan-out bi bo qua) deu duoc autocomplete tim thay nhu nhau -- tim 'may' sau khi ca 2 video da dang tra ve dung 2 ket qua, du chi 1 trong 2 video co mat trong inbox cua follower."
locale: vi
track: thiet-ke-he-thong
module: noi-dung-va-kham-pha
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [sd.boss-noi-dung-va-kham-pha]
requires: [sd.cdn-cache-video-pho-bien]
concepts: [sd.boss-noi-dung-va-kham-pha]
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
Chín bài — fan-out-khi-ghi, fan-out-khi-đọc, hybrid (News Feed); trie,
top-K, cập nhật định kỳ (Search); hàng đợi transcode, đa độ phân giải,
CDN (YouTube). Giờ ráp CẢ ba mảng vào MỘT luồng "đăng video" duy nhất.
::::

::::explain{#rap_dang_video}
Đăng một video đi qua ĐÚNG ba trạm, theo thứ TỰ: TRẠM một — transcode
(đơn giản hoá TỪ bài 7: một bước tiến đồng hồ, LUÔN xong ngay). Trạm
hai — fan-out hybrid (bài 3): follower ÍT thì đẩy NGAY vào inbox,
follower NHIỀU (nổi tiếng) thì bỏ QUA, không đẩy. Trạm ba — chèn tiêu đề
VÀO trie (bài 4) để autocomplete tìm LẠI được, chạy ĐỘC LẬP với kết quả
của trạm hai:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

type TrangThaiVideo = "cho_xu_ly" | "dang_xu_ly" | "hoan_tat";
const THOI_GIAN_XU_LY_MS = 300;

function chayTranscodeDenKhiXong(dh: DongHoMoPhong): TrangThaiVideo {
  tienThoiGian(dh, THOI_GIAN_XU_LY_MS);
  return "hoan_tat";
}

const NGUONG_NOI_TIENG = 3;

interface NutTrie { con: Map<string, NutTrie>; laTuHoanChinh: boolean; }
function taoNutTrie(): NutTrie { return { con: new Map(), laTuHoanChinh: false }; }
function chenTuVaoTrie(goc: NutTrie, tu: string): void {
  let hienTai = goc;
  for (const kyTu of tu) {
    let con = hienTai.con.get(kyTu);
    if (con === undefined) { con = taoNutTrie(); hienTai.con.set(kyTu, con); }
    hienTai = con;
  }
  hienTai.laTuHoanChinh = true;
}
function timNutTheoTienTo(goc: NutTrie, tienTo: string): NutTrie | undefined {
  let hienTai = goc;
  for (const kyTu of tienTo) {
    const con = hienTai.con.get(kyTu);
    if (con === undefined) return undefined;
    hienTai = con;
  }
  return hienTai;
}
function thuThapTatCaTu(nut: NutTrie, tienToHienTai: string, ketQua: string[]): void {
  if (nut.laTuHoanChinh) ketQua.push(tienToHienTai);
  for (const [kyTu, con] of nut.con) thuThapTatCaTu(con, tienToHienTai + kyTu, ketQua);
}
function timMoiTuTheoTienTo(goc: NutTrie, tienTo: string): string[] {
  const nutGoc = timNutTheoTienTo(goc, tienTo);
  if (nutGoc === undefined) return [];
  const ketQua: string[] = [];
  thuThapTatCaTu(nutGoc, tienTo, ketQua);
  return ketQua;
}

interface HeThongDangVideo {
  trangThaiVideo: Map<string, TrangThaiVideo>;
  inbox: Map<string, string[]>;
  trieTuaDe: NutTrie;
}
function taoHeThongDangVideo(): HeThongDangVideo {
  return { trangThaiVideo: new Map(), inbox: new Map(), trieTuaDe: taoNutTrie() };
}

function dangVideo(
  ht: HeThongDangVideo,
  dh: DongHoMoPhong,
  idVideo: string,
  tuaDe: string,
  dsFollower: string[],
): TrangThaiVideo {
  ht.trangThaiVideo.set(idVideo, "cho_xu_ly");
  ht.trangThaiVideo.set(idVideo, "dang_xu_ly");
  const trangThaiCuoi = chayTranscodeDenKhiXong(dh);
  ht.trangThaiVideo.set(idVideo, trangThaiCuoi);

  if (dsFollower.length <= NGUONG_NOI_TIENG) {
    for (const nguoi of dsFollower) {
      const feed = ht.inbox.get(nguoi) ?? [];
      feed.push(tuaDe);
      ht.inbox.set(nguoi, feed);
    }
  }

  chenTuVaoTrie(ht.trieTuaDe, tuaDe);
  return trangThaiCuoi;
}

const dh = taoDongHoMoPhong();
const ht = taoHeThongDangVideo();
const trangThai = dangVideo(ht, dh, "vid-1", "may tinh moi 2026", ["binh", "chi", "dung"]);
console.log("trang thai video sau khi dangVideo tra ve:", trangThai);
console.log("dong ho da tien toi:", dh.thoiGianHienTai, "ms");
console.log("inbox cua binh (follower, <=nguong nen duoc fan-out):", JSON.stringify(ht.inbox.get("binh")));
console.log('follower tim "may" qua autocomplete:', JSON.stringify(timMoiTuTheoTienTo(ht.trieTuaDe, "may")));
```

```text title=readonly
trang thai video sau khi dangVideo tra ve: hoan_tat
dong ho da tien toi: 300 ms
inbox cua binh (follower, <=nguong nen duoc fan-out): ["may tinh moi 2026"]
follower tim "may" qua autocomplete: ["may tinh moi 2026"]
```

`dangVideo` không hề PHÁT minh gì mới — nó chỉ GỌI đúng thứ tự BA mảnh
đã xây riêng lẻ Ở chín bài trước: transcode XONG mới xét fan-out, RỒI
luôn chèn vào trie Ở bước CUỐI, bất kể fan-out đã LÀM gì.
::::

::::example{#nguoi_noi_tieng_van_tim_duoc}
Với tác giả CÓ nhiều follower (vượt `NGUONG_NOI_TIENG`), trạm HAI (fan-
out) bị bỏ QUA hoàn toàn — nhưng trạm BA (chèn vào trie) vẫn CHẠY bình
thường, vì hai trạm này ĐỘC LẬP với nhau:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

type TrangThaiVideo = "cho_xu_ly" | "dang_xu_ly" | "hoan_tat";
const THOI_GIAN_XU_LY_MS = 300;

function chayTranscodeDenKhiXong(dh: DongHoMoPhong): TrangThaiVideo {
  tienThoiGian(dh, THOI_GIAN_XU_LY_MS);
  return "hoan_tat";
}

const NGUONG_NOI_TIENG = 3;

interface NutTrie { con: Map<string, NutTrie>; laTuHoanChinh: boolean; }
function taoNutTrie(): NutTrie { return { con: new Map(), laTuHoanChinh: false }; }
function chenTuVaoTrie(goc: NutTrie, tu: string): void {
  let hienTai = goc;
  for (const kyTu of tu) {
    let con = hienTai.con.get(kyTu);
    if (con === undefined) { con = taoNutTrie(); hienTai.con.set(kyTu, con); }
    hienTai = con;
  }
  hienTai.laTuHoanChinh = true;
}
function timNutTheoTienTo(goc: NutTrie, tienTo: string): NutTrie | undefined {
  let hienTai = goc;
  for (const kyTu of tienTo) {
    const con = hienTai.con.get(kyTu);
    if (con === undefined) return undefined;
    hienTai = con;
  }
  return hienTai;
}
function thuThapTatCaTu(nut: NutTrie, tienToHienTai: string, ketQua: string[]): void {
  if (nut.laTuHoanChinh) ketQua.push(tienToHienTai);
  for (const [kyTu, con] of nut.con) thuThapTatCaTu(con, tienToHienTai + kyTu, ketQua);
}
function timMoiTuTheoTienTo(goc: NutTrie, tienTo: string): string[] {
  const nutGoc = timNutTheoTienTo(goc, tienTo);
  if (nutGoc === undefined) return [];
  const ketQua: string[] = [];
  thuThapTatCaTu(nutGoc, tienTo, ketQua);
  return ketQua;
}

interface HeThongDangVideo {
  trangThaiVideo: Map<string, TrangThaiVideo>;
  inbox: Map<string, string[]>;
  trieTuaDe: NutTrie;
}
function taoHeThongDangVideo(): HeThongDangVideo {
  return { trangThaiVideo: new Map(), inbox: new Map(), trieTuaDe: taoNutTrie() };
}

function dangVideo(
  ht: HeThongDangVideo,
  dh: DongHoMoPhong,
  idVideo: string,
  tuaDe: string,
  dsFollower: string[],
): TrangThaiVideo {
  ht.trangThaiVideo.set(idVideo, "cho_xu_ly");
  ht.trangThaiVideo.set(idVideo, "dang_xu_ly");
  const trangThaiCuoi = chayTranscodeDenKhiXong(dh);
  ht.trangThaiVideo.set(idVideo, trangThaiCuoi);

  if (dsFollower.length <= NGUONG_NOI_TIENG) {
    for (const nguoi of dsFollower) {
      const feed = ht.inbox.get(nguoi) ?? [];
      feed.push(tuaDe);
      ht.inbox.set(nguoi, feed);
    }
  }

  chenTuVaoTrie(ht.trieTuaDe, tuaDe);
  return trangThaiCuoi;
}

const dh = taoDongHoMoPhong();
const ht = taoHeThongDangVideo();
const dsNoiTieng = ["f1", "f2", "f3", "f4", "f5"];
const trangThai = dangVideo(ht, dh, "vid-2", "may anh xin", dsNoiTieng);
console.log("video cua nguoi noi tieng (5 follower, >nguong), trang thai:", trangThai);
console.log("inbox cua f1 (follower nguoi noi tieng):", JSON.stringify(ht.inbox.get("f1")));
console.log('nhung van tim duoc qua autocomplete, tim "may":', JSON.stringify(timMoiTuTheoTienTo(ht.trieTuaDe, "may")));
```

```text title=readonly
video cua nguoi noi tieng (5 follower, >nguong), trang thai: hoan_tat
inbox cua f1 (follower nguoi noi tieng): undefined
nhung van tim duoc qua autocomplete, tim "may": ["may anh xin"]
```

`f1` KHÔNG hề nhận video qua inbox (`undefined` — không CÓ mục nào cho
`f1` trong `Map`) vì `5 > NGUONG_NOI_TIENG (3)`. NHƯNG `"may anh xin"`
vẫn XUẤT hiện trong autocomplete — trạm BA không hề "hỏi" trạm hai đã
LÀM gì trước khi CHẠY.
::::

::::predict{#doan_autocomplete_khong_phan_biet_chien_luoc commitOnce}
Hai video được đăng: video A của tác giả THƯỜNG (`1` follower, tiêu đề
`"may tinh moi 2026"`), VÀ video B của tác giả NỔI TIẾNG (`5` follower,
tiêu đề `"may anh xin"`). Follower tìm `"may"` qua autocomplete SAU khi
CẢ hai video đã đăng xong. Kết quả có mấy video?

:::opt{correct}
Cả `2` — autocomplete (trạm ba) chèn tiêu đề vào trie NGAY sau bước
transcode, KHÔNG hề kiểm tra video đó có được fan-out hay không; cả A
lẫn B đều đi qua ĐÚNG bước chèn trie đó
:::
:::opt
Chỉ `1` (video A) — video B của tác giả nổi tiếng KHÔNG được fan-out
tới follower, nên nó cũng không nên xuất hiện trong kết quả tìm kiếm
CỦA follower đó
::why
Nhầm "không xuất hiện trong INBOX của follower" (kết quả của trạm hai)
VỚI "không xuất hiện trong CHỈ MỤC tìm kiếm" (kết quả của trạm ba) —
nhưng `dangVideo` chạy HAI trạm này HOÀN TOÀN tách biệt.

Chỗ lệch: trong thân `dangVideo`, dòng `chenTuVaoTrie(ht.trieTuaDe,
tuaDe)` nằm NGOÀI khối `if (dsFollower.length <= NGUONG_NOI_TIENG) {
... }` — nó LUÔN chạy, bất kể nhánh fan-out CÓ thực thi hay không.
Chiến lược phân phối (đẩy NGAY hay không đẩy) VÀ khả năng được tìm THẤY
(có mặt trong chỉ mục hay KHÔNG) là hai câu hỏi khác nhau HOÀN toàn — hệ
thống này tách BIỆT chúng có chủ đích.
::
:::
::::

::::code{#viet_dang_video}
Hoàn thiện nhánh fan-out TRONG `dangVideo` — khi số follower còn nằm
trong ngưỡng, đẩy tiêu đề VÀO inbox của TỪNG follower.

```typescript title=starter
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

type TrangThaiVideo = "cho_xu_ly" | "dang_xu_ly" | "hoan_tat";
const THOI_GIAN_XU_LY_MS = 300;

function chayTranscodeDenKhiXong(dh: DongHoMoPhong): TrangThaiVideo {
  tienThoiGian(dh, THOI_GIAN_XU_LY_MS);
  return "hoan_tat";
}

const NGUONG_NOI_TIENG = 3;

interface NutTrie { con: Map<string, NutTrie>; laTuHoanChinh: boolean; }
function taoNutTrie(): NutTrie { return { con: new Map(), laTuHoanChinh: false }; }
function chenTuVaoTrie(goc: NutTrie, tu: string): void {
  let hienTai = goc;
  for (const kyTu of tu) {
    let con = hienTai.con.get(kyTu);
    if (con === undefined) { con = taoNutTrie(); hienTai.con.set(kyTu, con); }
    hienTai = con;
  }
  hienTai.laTuHoanChinh = true;
}
function timNutTheoTienTo(goc: NutTrie, tienTo: string): NutTrie | undefined {
  let hienTai = goc;
  for (const kyTu of tienTo) {
    const con = hienTai.con.get(kyTu);
    if (con === undefined) return undefined;
    hienTai = con;
  }
  return hienTai;
}
function thuThapTatCaTu(nut: NutTrie, tienToHienTai: string, ketQua: string[]): void {
  if (nut.laTuHoanChinh) ketQua.push(tienToHienTai);
  for (const [kyTu, con] of nut.con) thuThapTatCaTu(con, tienToHienTai + kyTu, ketQua);
}
function timMoiTuTheoTienTo(goc: NutTrie, tienTo: string): string[] {
  const nutGoc = timNutTheoTienTo(goc, tienTo);
  if (nutGoc === undefined) return [];
  const ketQua: string[] = [];
  thuThapTatCaTu(nutGoc, tienTo, ketQua);
  return ketQua;
}

interface HeThongDangVideo {
  trangThaiVideo: Map<string, TrangThaiVideo>;
  inbox: Map<string, string[]>;
  trieTuaDe: NutTrie;
}
function taoHeThongDangVideo(): HeThongDangVideo {
  return { trangThaiVideo: new Map(), inbox: new Map(), trieTuaDe: taoNutTrie() };
}

function dangVideo(
  ht: HeThongDangVideo,
  dh: DongHoMoPhong,
  idVideo: string,
  tuaDe: string,
  dsFollower: string[],
): TrangThaiVideo {
  ht.trangThaiVideo.set(idVideo, "cho_xu_ly");
  ht.trangThaiVideo.set(idVideo, "dang_xu_ly");
  const trangThaiCuoi = chayTranscodeDenKhiXong(dh);
  ht.trangThaiVideo.set(idVideo, trangThaiCuoi);

  if (dsFollower.length <= NGUONG_NOI_TIENG) {
    ___
  }

  chenTuVaoTrie(ht.trieTuaDe, tuaDe);
  return trangThaiCuoi;
}

const dh = taoDongHoMoPhong();
const ht = taoHeThongDangVideo();
dangVideo(ht, dh, "vid-1", "may tinh moi 2026", ["binh"]);
console.log(JSON.stringify(ht.inbox.get("binh")));
```

```typescript title=solution
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

type TrangThaiVideo = "cho_xu_ly" | "dang_xu_ly" | "hoan_tat";
const THOI_GIAN_XU_LY_MS = 300;

function chayTranscodeDenKhiXong(dh: DongHoMoPhong): TrangThaiVideo {
  tienThoiGian(dh, THOI_GIAN_XU_LY_MS);
  return "hoan_tat";
}

const NGUONG_NOI_TIENG = 3;

interface NutTrie { con: Map<string, NutTrie>; laTuHoanChinh: boolean; }
function taoNutTrie(): NutTrie { return { con: new Map(), laTuHoanChinh: false }; }
function chenTuVaoTrie(goc: NutTrie, tu: string): void {
  let hienTai = goc;
  for (const kyTu of tu) {
    let con = hienTai.con.get(kyTu);
    if (con === undefined) { con = taoNutTrie(); hienTai.con.set(kyTu, con); }
    hienTai = con;
  }
  hienTai.laTuHoanChinh = true;
}
function timNutTheoTienTo(goc: NutTrie, tienTo: string): NutTrie | undefined {
  let hienTai = goc;
  for (const kyTu of tienTo) {
    const con = hienTai.con.get(kyTu);
    if (con === undefined) return undefined;
    hienTai = con;
  }
  return hienTai;
}
function thuThapTatCaTu(nut: NutTrie, tienToHienTai: string, ketQua: string[]): void {
  if (nut.laTuHoanChinh) ketQua.push(tienToHienTai);
  for (const [kyTu, con] of nut.con) thuThapTatCaTu(con, tienToHienTai + kyTu, ketQua);
}
function timMoiTuTheoTienTo(goc: NutTrie, tienTo: string): string[] {
  const nutGoc = timNutTheoTienTo(goc, tienTo);
  if (nutGoc === undefined) return [];
  const ketQua: string[] = [];
  thuThapTatCaTu(nutGoc, tienTo, ketQua);
  return ketQua;
}

interface HeThongDangVideo {
  trangThaiVideo: Map<string, TrangThaiVideo>;
  inbox: Map<string, string[]>;
  trieTuaDe: NutTrie;
}
function taoHeThongDangVideo(): HeThongDangVideo {
  return { trangThaiVideo: new Map(), inbox: new Map(), trieTuaDe: taoNutTrie() };
}

function dangVideo(
  ht: HeThongDangVideo,
  dh: DongHoMoPhong,
  idVideo: string,
  tuaDe: string,
  dsFollower: string[],
): TrangThaiVideo {
  ht.trangThaiVideo.set(idVideo, "cho_xu_ly");
  ht.trangThaiVideo.set(idVideo, "dang_xu_ly");
  const trangThaiCuoi = chayTranscodeDenKhiXong(dh);
  ht.trangThaiVideo.set(idVideo, trangThaiCuoi);

  if (dsFollower.length <= NGUONG_NOI_TIENG) {
    for (const nguoi of dsFollower) {
      const feed = ht.inbox.get(nguoi) ?? [];
      feed.push(tuaDe);
      ht.inbox.set(nguoi, feed);
    }
  }

  chenTuVaoTrie(ht.trieTuaDe, tuaDe);
  return trangThaiCuoi;
}

const dh = taoDongHoMoPhong();
const ht = taoHeThongDangVideo();
dangVideo(ht, dh, "vid-1", "may tinh moi 2026", ["binh"]);
console.log(JSON.stringify(ht.inbox.get("binh")));
```

```typescript title=test
const dhT = taoDongHoMoPhong();
const htT = taoHeThongDangVideo();

const tt1 = dangVideo(htT, dhT, "vid-1", "may tinh moi 2026", ["binh", "chi", "dung"]);
if (tt1 !== "hoan_tat") throw new Error("dangVideo phai tra ve trang thai hoan_tat sau khi transcode xong");
if (htT.trangThaiVideo.get("vid-1") !== "hoan_tat") throw new Error("trangThaiVideo phai ghi nhan hoan_tat");
if (dhT.thoiGianHienTai !== THOI_GIAN_XU_LY_MS) throw new Error("dong ho phai tien dung THOI_GIAN_XU_LY_MS sau 1 video");

if ((htT.inbox.get("binh") ?? []).length !== 1) throw new Error("binh (follower, <=nguong) phai nhan video qua fan-out-khi-ghi");
if (htT.inbox.get("binh")?.[0] !== "may tinh moi 2026") throw new Error("noi dung inbox phai la tieu de video");

const goiY1 = timMoiTuTheoTienTo(htT.trieTuaDe, "may");
if (goiY1.length !== 1 || goiY1[0] !== "may tinh moi 2026") throw new Error("autocomplete phai tim thay video vua dang qua tien to 'may'");

const dsNoiTiengT = ["f1", "f2", "f3", "f4", "f5"];
const tt2 = dangVideo(htT, dhT, "vid-2", "may anh xin", dsNoiTiengT);
if (tt2 !== "hoan_tat") throw new Error("video cua nguoi noi tieng cung phai transcode xong binh thuong");
if ((htT.inbox.get("f1") ?? []).length !== 0) throw new Error("f1 (follower nguoi noi tieng, >nguong) KHONG duoc nhan qua fan-out-khi-ghi");

const goiY2 = timMoiTuTheoTienTo(htT.trieTuaDe, "may").sort();
if (goiY2.length !== 2) throw new Error("autocomplete PHAI tim thay CA HAI video (thuong lan noi tieng)");
if (JSON.stringify(goiY2) !== JSON.stringify(["may anh xin", "may tinh moi 2026"])) throw new Error("noi dung goi y autocomplete phai khop ca hai tieu de");

if (dhT.thoiGianHienTai !== THOI_GIAN_XU_LY_MS * 2) throw new Error("dong ho phai tien them dung THOI_GIAN_XU_LY_MS cho video thu hai");

const dh2 = taoDongHoMoPhong();
const ht2 = taoHeThongDangVideo();
dangVideo(ht2, dh2, "vid-3", "meo con", ["a", "b", "c"]);
if ((ht2.inbox.get("a") ?? []).length !== 1) throw new Error("dung 3 follower (bang NGUONG_NOI_TIENG) van phai duoc fan-out-khi-ghi");
```

:::hints
- kind: attention
  body: "Ben trong khoi if (con nam trong nguong), voi TUNG follower trong dsFollower, day tuaDe vao mang feed cua ho roi ghi mang do TRO LAI vao ht.inbox."
- kind: strategy
  body: "Giong het cau truc bai 1 (dangBaiFanOutGhi): lay feed hien co (hoac mang rong), push tuaDe vao, roi set lai vao inbox theo khoa nguoi."
- kind: one-line
  body: "for (const nguoi of dsFollower) { const feed = ht.inbox.get(nguoi) ?? []; feed.push(tuaDe); ht.inbox.set(nguoi, feed); }"
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 7000
- tier: output
  match: contains
  expect: "may tinh moi 2026"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Transcode, fan-out theo ngưỡng, chỉ mục tìm kiếm — ba mảnh ráp ĐÚNG thứ
tự thành một luồng đăng NỘI dung hoàn chỉnh. Quest "Nội dung VÀ khám
phá" đã xong.
::::

::::reflect{#nghi-lai}
`dangVideo` không hề PHÁT minh gì mới — nó chỉ GỌI đúng thứ tự các
mảnh đã xây RIÊNG lẻ trong chín bài trước, VÀ giữ chúng tách BIỆT đúng
mức cần thiết: fan-out (ai NHẬN được đẩy tới) VÀ index tìm kiếm (ai TÌM
thấy được) LÀ hai mối quan tâm khác nhau, không nên GỘP làm một chỉ vì
chúng CÙNG chạy trong một hàm.
::::

::::checkpoint{mastery=0.85}
::::
