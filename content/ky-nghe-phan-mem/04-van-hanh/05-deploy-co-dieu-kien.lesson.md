---
id: ky-nghe-phan-mem.van-hanh.deploy-co-dieu-kien
title: "Deploy CÓ ĐIỀU KIỆN — chỉ khi pipeline QUA VÀ đúng nhánh"
summary: "Deploy KHÔNG xảy ra chỉ VÌ code đã đẩy lên — nó cần HAI điều kiện CÙNG đúng: (1) pipeline CI đã qua hết (chayPipelineCI trả qua:true), (2) nhánh ĐÚNG là nhánh triển khai (thường là \"main\"). xetDuyetDeploy(ketQuaPipeline, tenNhanh): boolean kết hợp hai điều kiện bằng &&, tái hiện chuỗi phán quyết needs+if của GitHub Actions."
locale: vi
track: ky-nghe-phan-mem
module: van-hanh
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [vh.conditional-deploy-gate]
requires: [vh.gate-boss-ci-pipeline]
concepts: [vh.conditional-deploy-gate]
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
Cụm mới. Pipeline CI (cụm 1) đã QUA — có nên deploy NGAY LẬP TỨC
không? Còn ĐIỀU KIỆN nào khác cần kiểm không?
::::

::::explain{#deploy-can-hai-dieu-kien}
Deploy KHÔNG xảy ra CHỈ VÌ code ĐÃ đẩy lên — nó CẦN **HAI** điều kiện
CÙNG đúng: (1) pipeline CI ĐÃ qua HẾT (`chayPipelineCI` trả
`qua:true`, cụm 1), (2) nhánh ĐÚNG LÀ nhánh triển khai (thường LÀ
`"main"`):

```typescript title=readonly
type KetQuaPipeline = { qua: boolean; buocHong: string | null };

function xetDuyetDeploy(ketQuaPipeline: KetQuaPipeline, tenNhanh: string): boolean {
  return ketQuaPipeline.qua && tenNhanh === "main";
}

console.log(xetDuyetDeploy({ qua: true, buocHong: null }, "main"));
console.log(xetDuyetDeploy({ qua: true, buocHong: null }, "feature/x"));
console.log(xetDuyetDeploy({ qua: false, buocHong: "test" }, "main"));
```

```text title=readonly
true
false
false
```

`xetDuyetDeploy` LÀ MỘT hàm THUẦN kết hợp HAI điều kiện `&&` — tái
hiện chuỗi PHÁN QUYẾT `needs: test` + `if: github.ref ==
'refs/heads/main'` (dạng comment YAML ĐÃ THẤY Ở nguồn chương 43).
CHỈ khi CẢ HAI ĐÚNG mới deploy.
::::

::::example{#nhanh-tinh-nang-khong-bao-gio-deploy}
MỘT nhánh TÍNH NĂNG (KHÔNG PHẢI `"main"`) — DÙ pipeline QUA HOÀN
TOÀN — VẪN KHÔNG BAO GIỜ được duyệt DEPLOY:

```typescript title=readonly
type KetQuaPipeline = { qua: boolean; buocHong: string | null };
function xetDuyetDeploy(ketQuaPipeline: KetQuaPipeline, tenNhanh: string): boolean {
  return ketQuaPipeline.qua && tenNhanh === "main";
}
console.log(xetDuyetDeploy({ qua: true, buocHong: null }, "feature/thanh-toan"));
```

```text title=readonly
false
```

`{ qua: true, buocHong: null }` LÀ kết quả PIPELINE TỐT NHẤT CÓ THỂ
— NHƯNG `"feature/thanh-toan"` KHÁC `"main"`, VẾ THỨ HAI của `&&`
LÀ `false`, TOÀN BỘ biểu thức LÀ `false`. Đây LÀ hành vi ĐÚNG: hàng
CHỤC nhánh TÍNH NĂNG được đẩy code MỖI ngày, pipeline QUA thường
xuyên — TUYỆT ĐỐI KHÔNG được deploy TỰ ĐỘNG TỪ đó.
::::

::::predict{#doan-bang-su-that-bon-truong-hop commitOnce}
```typescript
type KetQuaPipeline = { qua: boolean; buocHong: string | null };
function xetDuyetDeploy(ketQuaPipeline: KetQuaPipeline, tenNhanh: string): boolean {
  return ketQuaPipeline.qua && tenNhanh === "main";
}

const cacTruongHop: Array<[KetQuaPipeline, string]> = [
  [{ qua: true, buocHong: null }, "main"],
  [{ qua: false, buocHong: "build" }, "main"],
  [{ qua: true, buocHong: null }, "develop"],
  [{ qua: false, buocHong: "lint" }, "develop"],
];

const ketQua = cacTruongHop.map(([kq, nhanh]) => xetDuyetDeploy(kq, nhanh));
console.log(ketQua);
```

Dòng cuối in ra gì?

:::opt{correct}
`[true,false,false,false]`
:::

:::opt
`[true,true,false,false]` — vì trường hợp THỨ HAI ĐANG Ở nhánh
`"main"` (nhánh TRIỂN KHAI CHÍNH THỨC), VÀ nhánh `"main"` LUÔN được
COI LÀ ĐÁNG TIN CẬY — dù pipeline BÁO hỏng (`qua: false`), deploy
VẪN được PHÉP vì ĐÂY LÀ nhánh QUAN TRỌNG nhất
::why
Gần đúng ở việc bạn nhớ ĐÚNG trường hợp THỨ HAI Ở NHÁNH `"main"` —
quan sát ĐÓ về VỊ TRÍ nhánh chính XÁC.

Chỗ lệch: "Ở nhánh main" KHÔNG PHẢI đặc quyền BỎ QUA điều kiện CÒN
LẠI — `&&` đòi **CẢ HAI** vế ĐỀU `true`. Trường hợp THỨ HAI có
`ketQuaPipeline.qua === false` (pipeline THẬT SỰ hỏng, Ở BƯỚC
`"build"`) — VẾ ĐẦU CỦA `&&` ĐÃ LÀ `false`, TOÀN BỘ biểu thức LÀ
`false`, BẤT KỂ nhánh LÀ GÌ. `main` KHÔNG "MIỄN TRỪ" pipeline hỏng —
NGƯỢC LẠI, `main` LÀ NƠI **CÀNG CẦN** pipeline PHẢI qua TRƯỚC khi
deploy, VÌ đây LÀ code sẽ tới TAY người DÙNG THẬT.
::
:::

:::opt
Máy báo lỗi biên dịch — `xetDuyetDeploy` trả về kiểu `boolean`,
NHƯNG `ketQuaPipeline.qua && tenNhanh === "main"` LÀ MỘT biểu thức
KẾT HỢP `boolean` VÀ `string`, TypeScript CẤM dùng `&&` giữa HAI
kiểu KHÁC NHAU
::why
Gần đúng ở việc bạn để ý `ketQuaPipeline.qua` LÀ `boolean` CÒN
`tenNhanh` LÀ `string` — HAI kiểu KHÁC nhau XUẤT HIỆN trong CÙNG một
biểu thức, một quan sát ĐÚNG về SỰ ĐA DẠNG kiểu.

Chỗ lệch: `&&` KHÔNG kết hợp `ketQuaPipeline.qua` TRỰC TIẾP với
`tenNhanh` — nó kết hợp `ketQuaPipeline.qua` (MỘT `boolean`) VỚI
`tenNhanh === "main"` (MỘT PHÉP SO SÁNH, **KẾT QUẢ CỦA NÓ CŨNG LÀ
`boolean`**). `boolean && boolean` HOÀN TOÀN hợp lệ, trả về
`boolean` — biên dịch SẠCH.
::
:::
::::

::::code{#viet_xet_duyet_deploy}
Viết `xetDuyetDeploy` — kết hợp ĐÚNG hai điều kiện: pipeline QUA VÀ
nhánh đúng LÀ `"main"`.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type KetQuaPipeline = { qua: boolean; buocHong: string | null };

function xetDuyetDeploy(ketQuaPipeline: KetQuaPipeline, tenNhanh: string): boolean {
  return ___;
}

assertEqual(xetDuyetDeploy({ qua: true, buocHong: null }, "main"), true, "pipeline qua tren main -- duyet");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type KetQuaPipeline = { qua: boolean; buocHong: string | null };

function xetDuyetDeploy(ketQuaPipeline: KetQuaPipeline, tenNhanh: string): boolean {
  return ketQuaPipeline.qua && tenNhanh === "main";
}

assertEqual(xetDuyetDeploy({ qua: true, buocHong: null }, "main"), true, "pipeline qua tren main -- duyet");
```

```typescript title=test
assertEqual(xetDuyetDeploy({ qua: true, buocHong: null }, "feature/x"), false, "pipeline qua nhung SAI nhanh -- tu choi");
assertEqual(xetDuyetDeploy({ qua: false, buocHong: "test" }, "main"), false, "dung nhanh nhung pipeline HONG -- tu choi");
assertEqual(xetDuyetDeploy({ qua: false, buocHong: "lint" }, "feature/y"), false, "ca hai deu sai -- tu choi");
```

:::hints
- kind: attention
  body: "Kết hợp ĐÚNG hai điều kiện bằng &&: ketQuaPipeline.qua (pipeline có qua không), VÀ tenNhanh có đúng bằng \"main\" không."
- kind: strategy
  body: 'ketQuaPipeline.qua && tenNhanh === "main"'
- kind: one-line
  body: '___ = ketQuaPipeline.qua && tenNhanh === "main"'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "PASS"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Deploy = pipeline qua VÀ đúng nhánh, cả hai. Bài tiếp theo: nhánh
KHÁC nhau nên đi TỚI môi trường KHÁC nhau như thế nào?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`xetDuyetDeploy` CHỈ trả `true`/`false` — nó chưa NÓI deploy TỚI ĐÂU.
Nếu nhánh `"develop"` NÊN đi TỚI môi trường thử NGHIỆM (staging),
CÒN `"main"` đi TỚI người DÙNG THẬT (production) — LÀM SAO PHÂN
BIỆT?
::::

::::checkpoint{mastery=0.8}
::::
