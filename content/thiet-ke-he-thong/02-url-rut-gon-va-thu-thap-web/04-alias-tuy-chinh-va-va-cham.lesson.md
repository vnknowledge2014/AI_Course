---
id: thiet-ke-he-thong.url-rut-gon-va-thu-thap-web.alias-tuy-chinh-va-va-cham
title: "Alias tuỳ chỉnh: nơi va chạm thật sự xảy ra"
summary: "dangKyAliasTuyChinh(kho, alias) khác hẳn bộ đếm bài 3: người dùng TỰ chọn chuỗi (vd \"my-brand\") nên hai người khác nhau có thể gõ trùng. KhoAlias{daCap: Set<string>} kiểm tra has() TRƯỚC khi add() — trùng thì từ chối (false), mới thì cấp (true). Set phân biệt hoa/thường tuyệt đối: [\"vidu-a\",\"VIDU-A\",\"vidu-a\"] cho kết quả [true,true,false] — 2 alias khác nhau được cấp, lần lặp thứ 3 (y hệt lần đầu) mới bị từ chối."
locale: vi
track: thiet-ke-he-thong
module: url-rut-gon-va-thu-thap-web
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [sd.alias-tuy-chinh-va-va-cham]
requires: [sd.bo-dem-toi-ma-ngan]
concepts: [sd.alias-tuy-chinh-va-va-cham]
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
Bộ đếm (bài trước) không bao giờ cấp mã trùng — VÌ nó tự CHỌN số tiếp
theo, không ai can THIỆP được. Nhưng nhiều dịch vụ rút gọn URL còn
cho người dùng TỰ đặt tên riêng, kiểu `vidu.com/my-brand`. Đó LÀ một
bài toán khác HẲN.
::::

::::explain{#kho-alias}
Khi người dùng TỰ chọn alias, HAI người khác nhau hoàn TOÀN có thể gõ
đúng CÙNG một chuỗi. `KhoAlias` giữ một `Set` các alias ĐÃ cấp;
`dangKyAliasTuyChinh` kiểm tra CÓ trong Set chưa TRƯỚC khi thêm — có
rồi thì TỪ chối, chưa có thì CẤP luôn:

```typescript title=readonly
interface KhoAlias { daCap: Set<string>; }
function taoKhoAlias(): KhoAlias {
  return { daCap: new Set() };
}
function dangKyAliasTuyChinh(kho: KhoAlias, alias: string): boolean {
  if (kho.daCap.has(alias)) return false;
  kho.daCap.add(alias);
  return true;
}

const kho = taoKhoAlias();
console.log('dang ky "quan-bun-cha":', dangKyAliasTuyChinh(kho, "quan-bun-cha"));
console.log('dang ky "quan-bun-cha" LAN 2:', dangKyAliasTuyChinh(kho, "quan-bun-cha"));
console.log('dang ky "quan-pho-24":', dangKyAliasTuyChinh(kho, "quan-pho-24"));
console.log("so alias da cap:", kho.daCap.size);
```

```text title=readonly
dang ky "quan-bun-cha": true
dang ky "quan-bun-cha" LAN 2: false
dang ky "quan-pho-24": true
so alias da cap: 2
```

Lần ĐẦU đăng ký `"quan-bun-cha"` thành CÔNG (`true`) — nó được thêm
VÀO `daCap`. Lần HAI, y hệt chuỗi đó ĐÃ nằm trong Set, nên bị TỪ chối
(`false`), VÀ kích thước Set không hề thay đổi.
::::

::::example{#khac-han-bo-dem}
Đây LÀ điểm khác biệt CỐT lõi so với bộ đếm (bài trước): bộ đếm
KHÔNG BAO GIỜ cần kiểm tra trùng VÌ nó tự sinh số tiếp theo, còn alias
tuỳ chỉnh thì NGƯỜI dùng tự gõ chuỗi — nên và chạm THẬT sự có thể xảy
ra, VÀ code phải xử LÝ nó:

```typescript title=readonly
interface KhoAlias { daCap: Set<string>; }
function taoKhoAlias(): KhoAlias {
  return { daCap: new Set() };
}
function dangKyAliasTuyChinh(kho: KhoAlias, alias: string): boolean {
  if (kho.daCap.has(alias)) return false;
  kho.daCap.add(alias);
  return true;
}

const kho = taoKhoAlias();
const cacYeuCau = ["quan-tra-da", "quan-tra-da", "banh-mi-co-ba", "quan-tra-da", "che-thap-cam"];
const ketQua = cacYeuCau.map((alias) => dangKyAliasTuyChinh(kho, alias));
console.log("cac yeu cau:", cacYeuCau.join(","));
console.log("ket qua dang ky:", ketQua.join(","));
console.log("so alias DUY NHAT da cap thanh cong:", kho.daCap.size);
```

```text title=readonly
cac yeu cau: quan-tra-da,quan-tra-da,banh-mi-co-ba,quan-tra-da,che-thap-cam
ket qua dang ky: true,false,true,false,true
so alias DUY NHAT da cap thanh cong: 3
```

Năm YÊU cầu, nhưng chỉ BA alias DUY nhất — `"quan-tra-da"` chỉ được
CẤP ở lần XUẤT hiện đầu tiên, hai lần SAU đều bị từ chối. Khác HẲN
`laMaNganTiepTheo` (bài 3), hàm NÀY thật sự CẦN nhánh từ chối vì đầu
vào ĐẾN từ người dùng, không phải TỪ một bộ đếm nội bộ luôn duy nhất.
::::

::::predict{#doan-hoa-thuong-alias commitOnce}
Ba yêu CẦU đăng ký theo đúng thứ tự: `"vidu-a"`, `"VIDU-A"`,
`"vidu-a"`. Có bao NHIÊU yêu cầu được `dangKyAliasTuyChinh` CHẤP
nhận (trả về `true`)?

:::opt{correct}
Hai — `"vidu-a"` VÀ `"VIDU-A"` LÀ hai chuỗi khác nhau (Set phân biệt
hoa/thường), lần thứ BA lặp lại y hệt lần ĐẦU nên bị từ chối
:::
:::opt
Một — hệ thống coi `"VIDU-A"` VÀ `"vidu-a"` LÀ "cùng một thương
hiệu" về mặt Ý nghĩa, nên tự động từ chối luôn LẦN thứ hai
::why
Nhầm "ý nghĩa thương hiệu trong đời THẬT" với "cách `Set<string>` so
khớp chuỗi trong CODE" — nhưng `dangKyAliasTuyChinh` không hề chuẩn
hoá hoa/thường trước khi kiểm TRA.

Chỗ lệch: `kho.daCap.has(alias)` so khớp CHÍNH XÁC từng ký tự, kể cả
hoa/thường — `"vidu-a"` VÀ `"VIDU-A"` LÀ hai giá trị `string` khác
nhau về mặt kỹ thuật. Chỉ khi chuỗi thứ BA (`"vidu-a"`, y hệt yêu
cầu ĐẦU tiên) tới, `has()` mới trả `true` VÀ bị từ chối.
::
:::
::::

::::code{#viet_dang_ky_alias_tuy_chinh}
Hoàn thiện `dangKyAliasTuyChinh` — SAU khi đã xác nhận alias CHƯA
từng cấp (nhánh từ chối đã xử LÝ Ở trên), thêm nó vào kho VÀ báo
thành công.

```typescript title=starter
interface KhoAlias { daCap: Set<string>; }
function taoKhoAlias(): KhoAlias {
  return { daCap: new Set() };
}
function dangKyAliasTuyChinh(kho: KhoAlias, alias: string): boolean {
  if (kho.daCap.has(alias)) return false;
  ___
}

const kho = taoKhoAlias();
console.log(dangKyAliasTuyChinh(kho, "my-brand"), dangKyAliasTuyChinh(kho, "my-brand"));
```

```typescript title=solution
interface KhoAlias { daCap: Set<string>; }
function taoKhoAlias(): KhoAlias {
  return { daCap: new Set() };
}
function dangKyAliasTuyChinh(kho: KhoAlias, alias: string): boolean {
  if (kho.daCap.has(alias)) return false;
  kho.daCap.add(alias);
  return true;
}

const kho = taoKhoAlias();
console.log(dangKyAliasTuyChinh(kho, "my-brand"), dangKyAliasTuyChinh(kho, "my-brand"));
```

```typescript title=test
const khoT = taoKhoAlias();
if (dangKyAliasTuyChinh(khoT, "cua-hang-a") !== true) throw new Error("alias chua tung cap phai dang ky THANH CONG (true)");
if (dangKyAliasTuyChinh(khoT, "cua-hang-a") !== false) throw new Error("alias DA cap roi phai bi TU CHOI (false)");
if (dangKyAliasTuyChinh(khoT, "cua-hang-b") !== true) throw new Error("alias khac phai dang ky THANH CONG (true)");
if (khoT.daCap.size !== 2) throw new Error("chi 2 alias DUY NHAT duoc cap thanh cong, du co 3 lan goi");
if (!khoT.daCap.has("cua-hang-a") || !khoT.daCap.has("cua-hang-b")) throw new Error("kho phai chua dung hai alias da cap");
if (dangKyAliasTuyChinh(khoT, "cua-hang-A") !== true) throw new Error('"cua-hang-A" (hoa) phai KHAC "cua-hang-a" (thuong), duoc cap binh thuong');
```

:::hints
- kind: attention
  body: "Nhanh 'da cap' o tren da xu ly xong -- gio la nhanh 'chua cap', can THEM alias vao kho roi bao thanh cong."
- kind: strategy
  body: "kho.daCap.add(alias); roi return true;"
- kind: one-line
  body: "kho.daCap.add(alias); return true;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "false"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Alias tuỳ chỉnh giờ đã CHỐNG được va chạm. Nhưng mọi mã đã cấp (dù tự
sinh hay tuỳ chỉnh) đều CẦN một hạn dùng — không thì kho chứa cứ PHÌNH
to mãi.
::::

::::reflect{#nghi-lai}
`dangKyAliasTuyChinh` VÀ `laMaNganTiepTheo` (bài 3) giải quyết đúng
MỘT câu hỏi — "làm sao có mã KHÔNG trùng" — theo hai cách hoàn TOÀN
khác nhau, vì nguồn GỐC của chuỗi khác nhau: một BÊN do hệ thống tự
sinh (không CẦN kiểm), một bên do NGƯỜI dùng tự chọn (BẮT buộc phải
kiểm). Chọn đúng chiến LƯỢC phụ thuộc vào việc AI kiểm soát giá trị
đầu vào.
::::

::::checkpoint{mastery=0.71}
::::
