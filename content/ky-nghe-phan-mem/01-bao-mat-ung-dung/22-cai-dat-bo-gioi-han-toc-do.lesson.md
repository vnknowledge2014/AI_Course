---
id: ky-nghe-phan-mem.bao-mat-ung-dung.cai-dat-bo-gioi-han-toc-do
title: "Cài đặt bộ giới hạn tốc độ — Map trong closure, hoàn toàn đồng bộ"
summary: "createRateLimiter(maxRequests, windowMs) dùng Map<string, number[]> trong CLOSURE, mỗi khoá (IP) giữ mảng timestamp RIÊNG. check(ip) lọc timestamp cũ, TỪ CHỐI nếu ĐỦ ngưỡng (kèm retryAfterMs), CHO PHÉP và GHI THÊM nếu chưa đủ."
locale: vi
track: ky-nghe-phan-mem
module: bao-mat-ung-dung
order: 22
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [bmud.rate-limit-implement]
requires: [bmud.rate-limit-sliding-window]
concepts: [bmud.rate-limit-implement]
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
`locTrongCuaSo` (bài trước) CHỈ lọc — CHƯA quyết định "cho phép hay
từ chối". Ghép nó với `Map` thành một bộ giới hạn HOÀN CHỈNH?
::::

::::explain{#createratelimiter}
`createRateLimiter(maxRequests, windowMs)` — dùng `Map<string,
number[]>` trong **CLOSURE**, MỖI khoá (IP) giữ mảng timestamp
**RIÊNG**. `check(ip, gioHienTai)` — lọc timestamp CŨ, nếu **ĐỦ**
ngưỡng thì **TỪ CHỐI** (kèm `retryAfterMs`), nếu **CHƯA** đủ thì
**GHI THÊM** timestamp mới, **CHO PHÉP**:

```typescript
type RateLimiter = { check: (ip: string, gioHienTai: number) => { allowed: boolean; retryAfterMs?: number } };

function createRateLimiter(maxRequests: number, windowMs: number): RateLimiter {
  const windows = new Map<string, number[]>();
  return {
    check: (ip, gioHienTai) => {
      const timestamps = (windows.get(ip) ?? []).filter((t) => gioHienTai - t < windowMs);
      if (timestamps.length >= maxRequests) {
        const camNhat = timestamps[0] ?? gioHienTai;
        return { allowed: false, retryAfterMs: windowMs - (gioHienTai - camNhat) };
      }
      timestamps.push(gioHienTai);
      windows.set(ip, timestamps);
      return { allowed: true };
    },
  };
}

const limiter = createRateLimiter(3, 60000);
console.log(limiter.check("1.2.3.4", 0).allowed);
console.log(limiter.check("1.2.3.4", 100).allowed);
console.log(limiter.check("1.2.3.4", 200).allowed);
console.log(limiter.check("1.2.3.4", 300).allowed);
```

```text
true
true
true
false
```

BA request ĐẦU (`allowed: true`) — mỗi lần GHI THÊM timestamp vào
`timestamps`, `windows.set("1.2.3.4", timestamps)`. Request THỨ TƯ:
`timestamps.length` ĐÃ BẰNG `3` (`maxRequests`) — **TỪ CHỐI**, `retry
AfterMs` cho biết CÒN BAO LÂU nữa timestamp CŨ NHẤT (`camNhat`) mới
"trượt RA KHỎI" cửa sổ, GIẢI PHÓNG một CHỖ TRỐNG.
::::

::::example{#moi-ip-doc-lap}
`windows` (Map) giữ **RIÊNG** mảng timestamp cho MỖI IP — request từ
IP KHÁC **KHÔNG BỊ ẢNH HƯỞNG** bởi giới hạn của IP kia:

```typescript title=readonly
type RateLimiter = { check: (ip: string, gioHienTai: number) => { allowed: boolean; retryAfterMs?: number } };
function createRateLimiter(maxRequests: number, windowMs: number): RateLimiter {
  const windows = new Map<string, number[]>();
  return {
    check: (ip, gioHienTai) => {
      const timestamps = (windows.get(ip) ?? []).filter((t) => gioHienTai - t < windowMs);
      if (timestamps.length >= maxRequests) {
        const camNhat = timestamps[0] ?? gioHienTai;
        return { allowed: false, retryAfterMs: windowMs - (gioHienTai - camNhat) };
      }
      timestamps.push(gioHienTai);
      windows.set(ip, timestamps);
      return { allowed: true };
    },
  };
}

const limiter = createRateLimiter(3, 60000);
limiter.check("1.2.3.4", 0);
limiter.check("1.2.3.4", 100);
limiter.check("1.2.3.4", 200);
console.log(limiter.check("1.2.3.4", 300).allowed); // IP NÀY đã đủ 3 -- từ chối
console.log(limiter.check("5.6.7.8", 300).allowed); // IP KHÁC -- chưa hề gọi lần nào
```

```text title=readonly
false
true
```

`"5.6.7.8"` LÀ KHOÁ MỚI trong `windows` (`windows.get("5.6.7.8")` ra
`undefined` → `?? []` cho mảng RỖNG) — request ĐẦU TIÊN của IP ĐÓ
LUÔN được phép, HOÀN TOÀN ĐỘC LẬP với việc `"1.2.3.4"` đã bị GIỚI HẠN.
::::

::::predict{#doan-request-moi-mo-rong-cua-so commitOnce}
```typescript
type RateLimiter = { check: (ip: string, gioHienTai: number) => { allowed: boolean; retryAfterMs?: number } };
function createRateLimiter(maxRequests: number, windowMs: number): RateLimiter {
  const windows = new Map<string, number[]>();
  return {
    check: (ip, gioHienTai) => {
      const timestamps = (windows.get(ip) ?? []).filter((t) => gioHienTai - t < windowMs);
      if (timestamps.length >= maxRequests) {
        const camNhat = timestamps[0] ?? gioHienTai;
        return { allowed: false, retryAfterMs: windowMs - (gioHienTai - camNhat) };
      }
      timestamps.push(gioHienTai);
      windows.set(ip, timestamps);
      return { allowed: true };
    },
  };
}

const limiter = createRateLimiter(2, 1000);
console.log(limiter.check("x", 0).allowed);     // 1/2
console.log(limiter.check("x", 100).allowed);   // 2/2
console.log(limiter.check("x", 200).allowed);   // 3 request trong 1000ms -- vượt?
console.log(limiter.check("x", 1500).allowed);  // request lúc 0 đã TRƯỢT RA KHỎI cửa sổ (1500-0=1500 >= 1000)
```

Bốn dòng cuối in ra gì?

:::opt{correct}
`true`, `true`, `false`, `true`
:::

:::opt
`true`, `true`, `false`, `false` — vì MỘT khi ĐàvUỢT ngưỡng (lần thứ
ba), bộ giới hạn "khoá" IP đó LẠI, MỌI request SAU ĐÓ đều bị từ chối
cho tới khi gọi một hàm "mở khoá" riêng
::why
Gần đúng ở việc bạn nhớ ĐÚNG lần gọi THỨ BA (`gioHienTai: 200`) bị từ
chối — quan sát ĐÓ đúng.

Chỗ lệch: `createRateLimiter` KHÔNG có khái niệm "khoá IP vĩnh viễn"
— MỖI lần gọi `check`, nó LỌC LẠI TỪ ĐẦU dựa trên `gioHienTai` HIỆN
TẠI. Ở lần gọi THỨ TƯ (`gioHienTai: 1500`): `timestamps` (lúc đó
đang giữ `[0, 100]`, VÌ lần gọi thứ BA bị từ chối KHÔNG `push` gì cả)
được LỌC LẠI: `1500 - 0 = 1500`, KHÔNG `< 1000` → timestamp `0` bị
LOẠI; `1500 - 100 = 1400`, CŨNG KHÔNG `< 1000` → timestamp `100`
CŨNG bị LOẠI. `timestamps` SAU lọc LÀ MẢNG RỖNG (`length = 0`),
`0 >= 2` là `false` → request được **CHO PHÉP**, VÀ `1500` được GHI
THÊM. Cửa sổ TỰ "trượt" theo THỜI GIAN, KHÔNG CẦN reset thủ công.
::
:::

:::opt
Máy báo lỗi biên dịch — gọi `limiter.check("x", ...)` BỐN LẦN với
CÙNG `ip = "x"` không hợp lệ, `RateLimiter.check` chỉ cho phép gọi
TỐI ĐA `maxRequests` LẦN cho MỖI khoá theo kiểu khai báo
::why
Gần đúng ở việc bạn để ý CÓ giới hạn `maxRequests` (`= 2`) LIÊN QUAN
tới SỐ LẦN gọi — một quan sát ĐÚNG về Ý NGHĨA NGHIỆP VỤ của tham số
đó.

Chỗ lệch: `maxRequests` là ràng buộc **NGHIỆP VỤ**, kiểm **LÚC CHẠY**
(bên trong THÂN hàm `check`, so sánh `timestamps.length`), **KHÔNG
PHẢI** ràng buộc Ở TẦNG KIỂU. `check` có kiểu `(ip: string,
gioHienTai: number) => {...}` — gọi được BAO NHIÊU LẦN tuỳ ý, với BẤT
KỲ đối số hợp lệ nào, KHÔNG có giới hạn "số lần gọi" nào Ở TẦNG COMPILER.
Biên dịch sạch, kể cả gọi HÀNG TRIỆU lần.
::
:::
::::

::::code{#viet_createratelimiter}
Tự viết BA phần cốt lõi của `createRateLimiter`.

```typescript title=starter
type RateLimiter = { check: (ip: string, gioHienTai: number) => { allowed: boolean; retryAfterMs?: number } };

function createRateLimiter(maxRequests: number, windowMs: number): RateLimiter {
  const windows = new Map<string, number[]>();
  return {
    check: (ip, gioHienTai) => {
      const timestamps = (windows.get(ip) ?? []).filter((t) => ___);
      if (timestamps.length >= maxRequests) {
        const camNhat = timestamps[0] ?? gioHienTai;
        return { allowed: false, retryAfterMs: ___ };
      }
      timestamps.push(___);
      windows.set(ip, timestamps);
      return { allowed: true };
    },
  };
}

const limiter = createRateLimiter(3, 60000);
console.log(limiter.check("1.2.3.4", 0).allowed);
```

```typescript title=solution
type RateLimiter = { check: (ip: string, gioHienTai: number) => { allowed: boolean; retryAfterMs?: number } };

function createRateLimiter(maxRequests: number, windowMs: number): RateLimiter {
  const windows = new Map<string, number[]>();
  return {
    check: (ip, gioHienTai) => {
      const timestamps = (windows.get(ip) ?? []).filter((t) => gioHienTai - t < windowMs);
      if (timestamps.length >= maxRequests) {
        const camNhat = timestamps[0] ?? gioHienTai;
        return { allowed: false, retryAfterMs: windowMs - (gioHienTai - camNhat) };
      }
      timestamps.push(gioHienTai);
      windows.set(ip, timestamps);
      return { allowed: true };
    },
  };
}

const limiter = createRateLimiter(3, 60000);
console.log(limiter.check("1.2.3.4", 0).allowed);
```

```typescript title=test
const lTest = createRateLimiter(2, 1000);
if (lTest.check("a", 0).allowed !== true) throw new Error("request 1/2 phải được phép");
if (lTest.check("a", 100).allowed !== true) throw new Error("request 2/2 phải được phép");
if (lTest.check("a", 200).allowed !== false) throw new Error("request 3 (vượt ngưỡng) phải bị từ chối");

if (lTest.check("b", 200).allowed !== true) throw new Error("IP khác không bị ảnh hưởng, request đầu tiên phải được phép");

// Sau khi timestamp cũ trượt khỏi cửa sổ, request mới lại được phép
if (lTest.check("a", 1500).allowed !== true) throw new Error("sau khi timestamp cũ trượt khỏi cửa sổ, request mới phải được phép lại");

// Biên: đúng ngưỡng (hiệu số = windowMs) phải bị LOẠI khỏi cửa sổ (dùng <, không phải <=)
const lBienDung = createRateLimiter(1, 1000);
lBienDung.check("bd", 0);
if (lBienDung.check("bd", 1000).allowed !== true) throw new Error("hiệu số ĐÚNG BẰNG windowMs phải bị loại khỏi cửa sổ (dùng <, không phải <=) — request phải được phép");
const lBienLech = createRateLimiter(1, 1000);
lBienLech.check("bl", 0);
if (lBienLech.check("bl", 999).allowed !== false) throw new Error("hiệu số nhỏ hơn windowMs (dù chỉ 1ms) vẫn phải tính là còn trong cửa sổ — request phải bị từ chối");

// Phân biệt phép TRỪ đúng với phép CỘNG sai: timestamp lớn, hiệu số nhỏ
const lTimestampLon = createRateLimiter(1, 1000);
lTimestampLon.check("big", 500000);
if (lTimestampLon.check("big", 500050).allowed !== false) throw new Error("phải tính HIỆU SỐ (trừ) giữa gioHienTai và t, không phải tổng — timestamp lớn với hiệu số nhỏ vẫn phải còn trong cửa sổ");

// retryAfterMs phải TÍNH ĐÚNG (windowMs trừ khoảng thời gian đã trôi qua)
const lRetry = createRateLimiter(1, 1000);
lRetry.check("rt", 0);
const kqRetry = lRetry.check("rt", 300);
if (kqRetry.retryAfterMs !== 700) throw new Error("retryAfterMs phải đúng bằng windowMs - (gioHienTai - camNhat) = 1000 - 300 = 700");
```

:::hints
- kind: attention
  body: "Lọc timestamp CŨ: điều kiện GIỐNG bài trước (gioHienTai - t < windowMs). Tính retryAfterMs: thời gian CÒN LẠI tới khi timestamp CŨ NHẤT (camNhat) trượt khỏi cửa sổ = windowMs TRỪ khoảng thời gian ĐÃ TRÔI QUA (gioHienTai - camNhat). Ghi timestamp MỚI: chính LÀ gioHienTai của lần gọi NÀY."
- kind: strategy
  body: "gioHienTai - t < windowMs : windowMs - (gioHienTai - camNhat) : gioHienTai — ba mảnh ghép cốt lõi."
- kind: one-line
  body: "___ (lọc) = gioHienTai - t < windowMs\n___ (retryAfterMs) = windowMs - (gioHienTai - camNhat)\n___ (ghi timestamp) = gioHienTai"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Rate limiter: Map trong closure, mỗi IP độc lập, cửa sổ tự trượt theo
thời gian. Bước tiếp theo: header bảo mật — mỗi header chặn MỘT loại
tấn công riêng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Rate limiting bảo vệ khỏi brute-force. Còn nhiều mối đe doạ KHÁC
(clickjacking, MIME-sniffing...) có header HTTP riêng để chặn — chúng
là gì?
::::

::::checkpoint{mastery=0.8}
::::
