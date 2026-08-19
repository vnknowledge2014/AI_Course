# ADR-001 — Cách thực thi Rust của người học, offline, trên cả 7 nền tảng

**Trạng thái:** ⚠️ **BỊ THAY THẾ bởi [ADR-002](ADR-002-rust-scope.md)** · 2026-08-19

> Giữ lại làm lịch sử. Quyết định nền (không dùng `rustc`, tự viết interpreter
> biên dịch WASM) vẫn đứng vững và đã được đo. Nhưng **phạm vi** mà ADR này hứa
> thì sai ở một điểm quan trọng — xem §"Sai sót của ADR này" ở cuối.
**Bối cảnh:** Học liệu Byte Academy dạy Rust cho người mới, phải chạy offline trên
macOS / Linux / Windows / Chrome / Firefox / Android / iOS.

---

## Vấn đề

`fp/game/src/engine/runner.ts` hiện có hàm `runRust()` **giả**:

```ts
// Note: Offline Rust compilation in browser is too heavy (requires >50MB WASM).
async function runRust(code: string): Promise<RunResult> {
  if (code.includes('fn ') && !code.includes('}')) { /* báo thiếu ngoặc */ }
  return { success: true, output: "Compilation successful.\nTests passed.", ... };
}
```

Nó trả `success: true` cho **mọi** input không thiếu dấu `}`. Người học viết Rust sai
vẫn nhận dấu tick xanh. Với một học liệu, đây không phải thiếu sót — đây là dạy sai.

## Các phương án đã cân nhắc

| Phương án | Offline? | Mobile? | Kết luận |
|---|---|---|---|
| `rustc` tự host biên dịch sang WASM | ✅ | ❌ | Hàng trăm MB, vượt xa giới hạn app store. Loại. |
| Gọi Rust Playground từ xa | ❌ | ✅ | Phá yêu cầu offline-first. Chỉ giữ làm tuỳ chọn nâng cao. |
| Biên dịch trước từng bài tập ra WASM | ✅ | ✅ | Người học **không viết được code tuỳ ý**, chỉ chọn đáp án. Không đủ. |
| Giữ nguyên bản giả | ✅ | ✅ | Dạy sai. Loại tuyệt đối. |
| **Interpreter Rust-subset viết bằng Rust, biên dịch WASM** | ✅ | ✅ | **CHỌN** |

## Quyết định

Viết một **interpreter cho tập con Rust dùng trong giảng dạy**; bản thân interpreter
viết bằng Rust rồi biên dịch sang `wasm32-unknown-unknown` và đóng gói vào app.

Tập con phủ đúng những gì giáo trình dạy, mở rộng dần theo từng Realm:
`let`/`mut`, kiểu vô hướng, `fn`, `if`/`match`, `loop`/`while`/`for`, closure,
`struct`/`enum`, `Option`/`Result`, `Vec`/`HashMap`, trait + generic cơ bản.
~~Ownership/borrowing **được mô phỏng và kiểm tra** — interpreter bắt buộc phải báo
lỗi mượn giống compiler thật.~~ ← **CÂU NÀY SAI, xem ADR-002.**

## Bằng chứng thực nghiệm

Đã dựng prototype thật (`adr-001-probe/`): lexer + parser đệ quy xuống + evaluator
cho biểu thức số học, viết bằng Rust thuần, không dependency.

```
cargo build --release --target wasm32-unknown-unknown
  → Finished in 3.76s
  → wasm_probe.wasm = 16 KB
  → linear memory khởi tạo = 1.1 MB
```

Chạy trong host JS (Node 26.7 — cùng API `WebAssembly` mà Chrome, Firefox và WKWebView dùng):

```
✅ 1 + 2 * 3        => 7        ✅ 1 / 0  => LỖI
✅ (1 + 2) * 3      => 9        ✅ 1 + @  => LỖI
✅ 100 / 7          => 14
✅ -5 + 10          => 5
✅ 2 * (3 + 4) - 5  => 9
7/7 pass
```

**Ngoại suy lúc ra quyết định:** phần lõi ngôn ngữ (lex/parse/eval) chỉ tốn 16 KB.
Một interpreter phủ tập con nêu trên, kèm bảng lỗi tiếng Việt, ước tính
**300 KB – 1.5 MB**.

### Kết quả thực tế sau khi cài đặt xong

Crate `packages/byte-rust` đã hoàn thành lexer + parser + interpreter + ABI WASM
(~5.700 dòng, 96 test). Số đo thật:

| Chỉ số | Dự đoán | Thực tế |
|---|---|---|
| Kích thước `.wasm` | 300 KB – 1.5 MB | **306 KB** |
| Thời gian build | — | 6,65 s |
| Bộ nhớ lúc chạy | — | 1,1 MB |

Chạy từ host JS (Node 26.7, cùng API `WebAssembly` mà Chrome/Firefox/WKWebView dùng):

```
✅ chạy đúng                  6,9 ms   "Xin chào từ WASM!\n"
✅ tính tổng 1..=100          2,1 ms   "5050\n"
⚠️  lỗi ownership              1,7 ms   BR0530 @ 1:66
⚠️  vòng lặp vô hạn           91,4 ms   BR0500 — bị chặn, KHÔNG treo
✅ tiếng Việt trong chuỗi     0,1 ms   "Nguyễn Văn A — 25 tuổi\n"

sau 500 lần chạy liên tiếp: bộ nhớ vẫn 1,1 MB — không rò rỉ
```

306 KB nhỏ hơn Pyodide (13 MB) **42 lần**, nên nạp được cả trên mobile mà không
cần tải thêm gì.

## Hệ quả

**Được:**
- Rust chạy thật, chấm thật, offline, trên cả 7 nền tảng — cùng một binary WASM.
- Thông báo lỗi **bằng tiếng Việt và có tính sư phạm**, thay vì đổ nguyên lỗi `rustc`
  vốn quá tải với người mới. Đây là ưu điểm, không phải thoả hiệp.
- Kiểm soát được độ khó: interpreter chỉ chấp nhận phần cú pháp đã dạy, nên người học
  không lạc vào vùng chưa học mà không hiểu vì sao lỗi.

**Mất:**
- Không phải Rust đầy đủ. Code chạy được trong app **có thể** không biên dịch được bằng
  `rustc` thật nếu vượt tập con.
  → Giảm thiểu: mỗi Realm công bố rõ tập con hiện hành; bài Capstone hướng dẫn chạy
    `cargo` thật trên máy; thêm chế độ "đối chiếu với rustc" (tuỳ chọn, cần mạng).
- Phải tự bảo trì interpreter.
  → Giảm thiểu: bộ test đối chiếu — mọi snippet trong giáo trình phải cho **cùng kết quả**
    ở interpreter và ở `rustc` thật; chạy trong CI trên runner có toolchain đầy đủ.

## Áp dụng cho Python và TypeScript

Không đổi: Python dùng **Pyodide** (CPython thật, biên dịch WASM), TypeScript dùng
**Sucrase** transform rồi chạy. Cả hai đã hoạt động. Hai điểm cần thống nhất:
1. Cùng nằm sau một interface `ExecutionEngine` chung.
2. Cùng chạy trong **Web Worker** — hiện TypeScript đang chạy trên main thread bằng
   `new Function()`, nên một vòng lặp vô hạn của người học sẽ treo cả giao diện.


---

## Sai sót của ADR này

ADR-001 hứa interpreter sẽ "báo lỗi mượn giống compiler thật". **Không làm được,
và cố làm thì có hại.**

Lý do kỹ thuật: **NLL (Non-Lexical Lifetimes) được định nghĩa trên MIR** — trên đồ
thị luồng điều khiển cộng với phân tích vùng sống của region. Nó **không tái tạo
trung thực được trên AST**. Một bản "NLL rút gọn chạy trên AST" sẽ từ chối đúng
những chương trình mà `rustc` đã chấp nhận từ edition 2018 — tức là dạy người học
một luật ownership **không tồn tại**. Với học liệu, dạy sai còn tệ hơn không dạy.

Những gì thực sự giữ lại được, và đã cài trong `crates/byte-rust`:
- **Move-check hẹp**: dùng lại một binding không-`Copy` sau khi đã move, trên chuỗi
  câu lệnh thẳng hàng không rẽ nhánh. Quyết định được mà không cần CFG.
- Mọi tình huống ownership khác (qua nhánh, qua vòng lặp, reborrow, lifetime tường
  minh) phải trả kết cục thứ ba: **`ChuaHoTro`**, kèm tên tính năng.

Ước lượng khối lượng cũng sai: ADR-001 ngầm định interpreter cỡ 1.200 dòng.
Thực tế `byte-rust` v1 cần **9.000–13.000 dòng** (hiện có ~5.700).

Chi tiết phạm vi đóng: [ADR-002](ADR-002-rust-scope.md).
