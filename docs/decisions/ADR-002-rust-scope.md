# ADR-002 — Phạm vi đóng của `byte-rust`, và ba kết cục thay vì hai

**Trạng thái:** Chốt · 2026-08-19 · Thay thế [ADR-001](ADR-001-rust-runtime.md)
**Chi tiết đầy đủ:** `MASTERPLAN.md` §4.3

---

## Điều ADR-001 hứa sai

> *"Ownership/borrowing được mô phỏng và kiểm tra — interpreter bắt buộc phải báo
> lỗi mượn giống compiler thật."*

**NLL được định nghĩa trên MIR** — trên đồ thị luồng điều khiển cộng phân tích vùng
sống của region. Nó không tái tạo trung thực được trên AST. Một bản "NLL rút gọn
trên AST" sẽ từ chối đúng những chương trình mà `rustc` chấp nhận từ edition 2018,
tức dạy người học một luật **không tồn tại**.

Với học liệu, dạy sai tệ hơn không dạy.

## Quyết định

### 1. Ba kết cục, không phải hai

Đây là thay đổi quan trọng nhất. Engine cũ chỉ có `Pass` / `Fail`, nên mọi thứ nó
không hiểu đều rơi vào một trong hai — và cả hai đều là lời nói dối:

| Kết cục | Nghĩa | Người học thấy gì |
|---|---|---|
| `Pass` | Byte đã chạy code và nó đúng | ✅ + huy hiệu |
| `Fail` | Byte đã chạy code và nó sai | Chẩn đoán ba tầng |
| **`ChuaHoTro(tính_năng)`** | **Byte không đủ sức kiểm tra chỗ này** | Nói thẳng, kèm lối đi khác |

`ChuaHoTro` **không phải là lỗi của người học**, nên không được hiển thị như lỗi và
không được trừ điểm. Nó là lời thú nhận của công cụ.

### 2. Move-check hẹp — thứ duy nhất về ownership mà tầng WASM kiểm

Chỉ hai luật, cả hai quyết định được **không cần CFG**:

1. Dùng lại một binding không-`Copy` sau khi đã move, trong cùng một block, trên
   chuỗi câu lệnh thẳng hàng, không rẽ nhánh → `E0382`.
2. Hai `&mut` tới cùng một chỗ trong **cùng một câu lệnh** → `E0499`.

Mọi tình huống khác (qua nhánh, qua vòng lặp, reborrow, lifetime tường minh) →
`ChuaHoTro("borrow-check-cfg")`, kèm câu:

> *"Byte chưa đủ giỏi để kiểm tra luật mượn trong tình huống này. Mở bản Desktop và
> bấm 'Đối chiếu với cargo' để có câu trả lời chính xác."*

### 3. Ownership được dạy ở đâu, nếu không phải bằng borrow checker

- **World `memory-city`**: move/borrow là **dữ liệu của world**, không phải kết quả
  phân tích code tuỳ ý. Người học dự đoán "dòng nào compiler sẽ chặn và vì sao",
  world diễn lại.
- **Tầng R-D**: `cargo` thật qua Tauri sidecar trên Desktop — con đường **duy nhất**
  có phản hồi borrow-check thật, và tài liệu phải nói thẳng điều đó.

### 4. Cổng merge là corpus ÂM, không phải corpus dương

Chạy CI trên toàn bộ code đúng trích từ sách **không đo được** chế độ hỏng nguy
hiểm nhất: **false accept** — báo `Pass` cho code mà `rustc` từ chối. Đó đúng là
cách engine giả cũ hỏng.

`crates/byte-rust-conformance` sinh mutant từ corpus dương (xoá `&`, đổi `&`→`&mut`,
xoá `.clone()`, dùng sau move, xoá nhánh `match`…), chạy từng mutant qua `rustc`
thật, rồi đòi `byte-rust` **hoặc** phát đúng mã lỗi **hoặc** trả `ChuaHoTro`.

> **Một `Pass` cho mutant mà `rustc` từ chối = FAIL MERGE.**

### 5. Hai bẫy ngữ nghĩa chốt ngay trong value model

- **`usize`/`isize` = 64-bit cố định cho mọi build.** Không được mượn `usize` của
  host: nếu không, `usize::MAX`, `size_of`, và overflow sẽ cho ba kết quả khác nhau
  giữa WASM32, native 64-bit và `rustc` trên CI.
- **`HashMap` phải xáo thứ tự duyệt theo seed đổi mỗi lần chạy**, đúng như Rust
  thật. Làm nó tất định cho tiện chấm bài sẽ dạy người học một điều sai. Conformance
  chỉ so sánh sau khi sort.

## Hệ quả

- Ước lượng khối lượng: `byte-rust` v1 ≈ **9.000–13.000 dòng** (hiện có ~5.700),
  không phải ~1.200 như ADR-001 ngầm định. v1.1 (trait/generic/`dyn`) thêm ~5.000.
- `crates/byte-rust` chuyển từ `packages/` sang `crates/` theo bố cục monorepo.
- Mọi thông báo hiện đang nói "chưa hỗ trợ" ở dạng lỗi thường (ví dụ `BR0542`
  macro) phải đổi sang kết cục `ChuaHoTro`.

---

## Phụ lục — Trạng thái đo được, 2026-08-21

Cổng đối chiếu `rustc` (`crates/byte-rust-conformance`, chạy `--am`):

| | |
|---|---|
| Mutant cơ học | **0/58 nhận oan** |
| Corpus âm thủ công | **3/100 nhận oan** (khởi điểm 61/100) |
| Từ chối oan | **0** — không mã Rust hợp lệ nào bị chặn |
| Test workspace | 113 xanh |

Ba ca còn lọt, và vì sao chúng còn lọt:

| Ca | rustc | Vì sao AST không đủ |
|---|---|---|
| `hai_muon_mut_qua_hai_cau_lenh` | E0499 | Phải biết hai tham chiếu có CÙNG SỐNG ở một thời điểm không |
| `doc_bien_khi_dang_bi_muon_mut` | E0502 | Như trên — vùng sống của một mượn, không phải vị trí cú pháp của nó |
| `tham_chieu_song_lau_hon_thu_no_tro_toi` | E0597 | Cần so sánh vòng đời hai vùng nhớ |

Cả ba đều quy về **vùng sống**, và vùng sống được định nghĩa trên đồ thị luồng
điều khiển chứ không trên cây cú pháp. Đây đúng là kết luận §2 đã đưa ra trước
khi viết một dòng mã nào — nó không đổi.

Những gì §2 nói là "không làm được trên AST" hoá ra có ba lớp khác nhau, và
phân biệt được ba lớp ấy mới là phần đáng học:

1. **Không cần luồng, chỉ cần nhìn đúng chỗ.** `cong(&mut x, &mut x)` — hai
   mượn trong cùng một lời gọi thì chắc chắn sống cùng lúc. Trả `&<biến cục
   bộ>` — chỉ cần biết cái tên sinh ra ở đâu. Cả hai đã vá (BR0355, BR0356).
2. **Không cần luồng, chỉ cần bảng chữ ký.** `fn an(self)` nuốt bộ nhận, còn
   `fn an(&self)` thì không — chữ ký nằm ngay trong khối `impl` của chính
   chương trình đó. Đã vá.
3. **Thật sự cần luồng.** Ba ca ở bảng trên.

Bài học cho những đợt sau: khi một luật "cần phân tích luồng", hãy hỏi lại
xem nó có thật sự cần không, hay chỉ cần một thông tin đang nằm sẵn đâu đó mà
chưa ai nối vào.
