# Đối chiếu `byte-rust` ↔ `rustc` — báo cáo lỗ hổng nhận oan

**Sinh ngày:** 2026-08-19 · **rustc:** 1.97.1 · **Cổng:** `crates/byte-rust-conformance`
**Nguồn phạm vi:** [ADR-002 §4](../decisions/ADR-002-rust-scope.md)

Tài liệu này được sinh từ việc chạy cổng đối chiếu. **Không vá `byte-rust` theo tài
liệu này một cách máy móc** — xem [§7 Nguyên tắc vá](#7-nguyên-tắc-vá).

```bash
cd crates/byte-rust-conformance
cargo run              # cả hai nguồn (mặc định)
cargo run -- --chi-am  # chỉ corpus âm thủ công
cargo run -- --im      # bỏ chi tiết từng ca, chỉ in bảng
```

---

## 1. Số liệu

| Nguồn corpus âm | Chương trình | Đồng ý với `rustc` | ChuaHoTro | Từ chối oan | **NHẬN OAN** |
|---|---:|---:|---:|---:|---:|
| **A. Mutant cơ học** (`mutate.rs`) | 58 | 58 (100 %) | 0 | 0 | **0** |
| **B. Corpus âm thủ công** (`corpus_am.rs`) | 100 | 0 (0 %) | 0 | 0 | **100** |

> **CẢNH BÁO — tài liệu này ĐÃ CŨ.** Nó chụp trạng thái ngày 19/08, lúc còn
> 100/100 nhận oan. Chạy lại cổng để lấy số thật:
>
> ```bash
> cd crates/byte-rust-conformance && cargo run --release -q -- --am
> ```
>
> Trạng thái hiện tại: **0 nhận oan, 0 từ chối oan, cổng MỞ.** Ba ca cuối
> (E0499, E0502, E0597) đã bịt bằng `crates/byte-rust/src/borrow_check.rs`.
> Phần dưới giữ lại vì §2 và §7 vẫn còn giá trị — bài học "nguồn A xanh vì lý
> do sai" không cũ đi.

**Cổng: FAIL.** 100 chương trình mà `rustc` từ chối được `byte-rust` trả `Dat`.

Phân bố 100 ca nhận oan:

| Theo module chứa lỗ hổng | Ca | | Theo mã lỗi `rustc` bỏ lọt | Ca |
|---|---:|---|---|---:|
| `tyck.rs` | 62 | | `E0308` lệch kiểu | 32 |
| `interp.rs` | 20 | | `E0382` dùng sau move | 10 |
| `move_check.rs` | 18 | | `E0004` match không vét cạn | 7 |
| | | | `E0599` không có phương thức | 6 |
| | | | `E0061` sai số đối số | 4 |
| | | | `E0384` gán lại biến bất biến | 4 |
| | | | `E0425` không tìm thấy tên | 4 |
| | | | `E0507`/`E0594`/`E0596`/`E0609` | 12 |
| | | | 18 mã còn lại (mỗi mã 1–2 ca) | 21 |

Không nhóm nào trong tám nhóm có dù chỉ một ca đồng ý:

```
Số học & ép kiểu ngầm                    0/13
Mượn, khả biến của tham chiếu, vòng đời  0/12
Chuyển quyền sở hữu                      0/12
Khớp mẫu & tính vét cạn                  0/15
Kiểu của biểu thức & vùng mù mã chết     0/12
Struct & enum                            0/12
Phương thức dựng sẵn                     0/12
Khả biến của binding & phân giải tên     0/12
```

---

## 2. Điều quan trọng nhất: nguồn A xanh **vì lý do sai**

Trước báo cáo này, cổng merge xanh 58/58. Kết luận tự nhiên — "engine không nhận
oan" — là **sai**. Nguồn A xanh vì mười khuôn mẫu đột biến trong `mutate.rs` không
hỏi đúng câu hỏi, chứ không phải vì engine trả lời đúng.

Tệ hơn: có ít nhất một loại đột biến xanh **vì một lỗi không liên quan**. Đột biến
`gan_lai_khong_mut` (7/7 "đồng ý") chèn `<bien> = Default::default();` để thử luật
E0384. Chạy trực tiếp:

| Chương trình | Kết cục | Mã |
|---|---|---|
| `let a = 10; … a = Default::default();` | `KhongDat` | `BR0550` |
| `let a = 10; … a = 99;` | **`Dat`** | — |

`byte-rust` từ chối chương trình thứ nhất vì **không tìm thấy hàm
`Default::default`**, không phải vì phát hiện gán lại biến bất biến. Đổi vế phải
thành một số nguyên thường là lọt ngay. Bảy "đồng ý" của loại đột biến này không đo
được gì về E0384.

Cùng cơ chế với hai loại còn lại:

| Đột biến | Vì sao xanh | Phá bằng cách nào |
|---|---|---|
| `bien_chua_khai_bao` (10/10) | `BR0501` nổ **lúc chạy**, vì đột biến chèn vào đường chạy của `main` | Bọc trong `if false { … }` → `Dat` |
| `goi_ham_khong_co` (10/10) | `BR0550` nổ **lúc chạy**, cùng lý do | Bọc trong nhánh chết → `Dat` (ca `ham_khong_ton_tai_nhanh_chet`) |

**Hệ quả cho quy trình:** con số của nguồn A không được đọc một mình như bằng chứng
engine đúng. Nó chỉ là lưới hồi quy cho những lỗ hổng đã vá. Mọi lỗ hổng mới phải
vào nguồn B, và `mutate.rs` cần thêm một phép đột biến tổng quát: **bọc mã sai vào
nhánh không chạy** — đó là đường vòng chung cho mọi kiểm tra chỉ tồn tại ở `interp`.

---

## 3. Lỗ hổng xuyên suốt: `T::Mo` đang gánh hai việc khác nhau

Trước khi xếp hạng từng lỗ hổng, phải nói về nguyên tắc thiết kế sinh ra phần lớn
chúng. `tyck.rs` ghi rõ:

> `Mo` nghĩa là "không suy ra được" — không bao giờ báo lỗi dựa trên kiểu này.

Nguyên tắc này **đúng** và cần giữ: nó là thứ ngăn bộ kiểm nhẹ báo oan. Vấn đề là
`T::Mo` hiện gánh **hai khái niệm khác hẳn nhau**:

1. *"Kiểu này suy sau cũng được, chưa cần biết."* — im lặng là đúng.
2. *"Bộ kiểm gặp cấu trúc nó không mô hình hoá."* — im lặng là **nói dối**.

Nghĩa (2) xuất hiện ở khắp nơi: `BieuThuc::ChiSo` không có nhánh riêng trong
`kieu_cua` nên rơi vào `khac => { self.duyet_con(khac); T::Mo }`;
`kieu_tra_ve_phuong_thuc` trả `_ => T::Mo` cho mọi tên lạ; `tra()` trả `T::Mo` cho
tên **không tồn tại**; `KhoiTaoStruct` trả `T::Mo` cho struct chưa khai báo. Rồi
`chac_chan_lech` mở đầu bằng `(Mo, _) | (_, Mo) => false`, nên mọi giá trị đi qua
một trong các chỗ đó được **giặt sạch** khỏi mọi phép kiểm hạ nguồn.

Ca `first_tra_tham_chieu` cho thấy hậu quả rõ nhất: hàng phòng thủ **tốt nhất** của
`tyck` — luật `lech_tham_chieu` ở `tyck.rs:302-305`, luật duy nhất trong crate phân
biệt `&T` với `T` — bị vô hiệu vì nó loại trừ tường minh `T::Mo`:

```rust
(T::Tham(_), t) | (t, T::Tham(_)) if !matches!(t, T::Tham(_) | T::Mo)
```

`v.first().unwrap()` cho `T::Mo` (cả `first` lẫn `unwrap` đều không có trong bảng),
nên `nhan_doi(v.first().unwrap())` qua cửa.

**Đề xuất — thay đổi rẻ nhất, đòn bẩy lớn nhất trong toàn báo cáo:** tách `T::Mo`
làm hai biến thể.

```rust
pub enum T {
    …
    /// Chưa suy ra được, nhưng hợp lệ. Không bao giờ sinh lỗi. (giữ nguyên)
    Mo,
    /// Bộ kiểm KHÔNG mô hình hoá được chỗ này. Mọi phép kiểm lẽ ra chạy trên
    /// giá trị này phải phát `ChuaHoTro`, không được im lặng.
    ChuaBiet(&'static str),
}
```

Luật: `chac_chan_lech` vẫn trả `false` cho `ChuaBiet` (không báo oan), nhưng **mọi
điểm kiểm** (`BR0301` đối số, `BR0302` trả về, `let`, vét cạn `match`) khi gặp
`ChuaBiet` phải phát `ChuaHoTro(ly_do)` thay vì bỏ qua. Điều này chuyển hàng chục ca
`Dat` thành `ChuaHoTro` — **đúng kết cục ADR-002 §1 dành cho chúng** — mà không cần
viết một dòng suy luận kiểu mới nào. Đó là cách duy nhất làm cổng xanh một cách
trung thực trước khi các bản vá thật kịp về.

---

## 4. Xếp hạng lỗ hổng theo mức nguy hiểm với người học

Tiêu chí xếp hạng: **(a)** khái niệm bị dạy sai có cốt lõi không, **(b)** người học
gặp nó sớm và thường xuyên đến đâu, **(c)** chương trình có in ra **kết quả sai**
kèm tick xanh không (thiệt hại kép: người học rút ra kết luận sai về *ngữ nghĩa*
Rust, chứ không chỉ về *cái gì compile được*).

---

### 🔴 #1 — `CauLenh::Let` có chú thích kiểu KHÔNG BAO GIỜ được đối chiếu

**Module:** `tyck.rs:420-424` (`fn khoi`) · **8 ca** · `E0308`, `E0600`, lint tràn số

```rust
let t_gt = gia_tri.as_ref().map(|e| self.kieu_cua(e)).unwrap_or(T::Mo);
let t = kieu.as_ref().map(|k| self.chuan_hoa(tu_kieu_ast(k))).unwrap_or(t_gt);
```

Hai kiểu được tính ra, rồi chú thích **ghi đè im lặng** kiểu suy ra. `chac_chan_lech`
không bao giờ được gọi. Mọi `let x: T = <biểu thức sai kiểu>` đều lọt, bất kể `T`.

**Vì sao xếp #1.** Ba lý do cộng lại:

1. `let x: i64 = …` là cấu trúc người học viết **nhiều nhất**, từ bài đầu tiên.
2. Đây là **lỗ hổng gốc**: kiểu sai được ghi vào bảng biến (dòng 425) rồi lan ra mọi
   suy luận sau. `tyck` không chỉ bỏ sót — nó **tin một điều sai** rồi dùng điều sai
   đó để xác nhận tiếp.
3. Ca `chia_nguyen_gan_f64` là chế độ hỏng tệ nhất có thể có với học liệu:

   ```rust
   let a: i64 = 7;  let b: i64 = 2;
   let c: f64 = a / b;      // rustc: E0308
   println!("{}", c);       // byte-rust: Đạt, in ra "3"
   ```

   Người học nhận tick xanh **kèm một kết quả sai**, và rút ra rằng Rust chia như
   thế. Đây đúng cái bẫy chia-nguyên-vs-chia-thực mà `rustc` tồn tại để chặn.

Mỉa mai nhất: `chac_chan_lech` **đã có sẵn luật đúng** cho nhiều ca trong nhóm này
(`(Struct(a), Struct(b)) => a != b`), và `tyck` **đã dùng nó** cho đối số hàm
(`BR0301`) và kiểu trả về (`BR0302`). Chỉ riêng `let` là quên.

**Đề xuất vá** (~6 dòng, mã lỗi mới `BR0303`):

```rust
CauLenh::Let { mau, kieu, gia_tri, .. } => {
    let t_gt = gia_tri.as_ref().map(|e| self.kieu_cua(e)).unwrap_or(T::Mo);
    let t = match kieu {
        Some(k) => {
            let khai = self.chuan_hoa(tu_kieu_ast(k));
            if khai.chac_chan_lech(&t_gt) { /* phát BR0303 */ }
            khai
        }
        None => t_gt,
    };
    …
}
```

⚠️ Một điều kiện: trong ngữ cảnh `let`, phải **tắt** nhánh nới lỏng
`(a, Tham(b)) => a.chac_chan_lech(b)` của `chac_chan_lech` (`tyck.rs:76-79`). Nhánh
đó tồn tại để tránh báo oan chỗ auto-deref, nhưng auto-deref **không áp dụng** cho
chú thích kiểu của `let`; giữ nguyên thì ca
`let_khai_bao_kieu_gia_tri_nhung_gan_tham_chieu` vẫn lọt.

---

### 🔴 #2 — Ép kiểu số ngầm: `byte-rust` hành xử như JavaScript

**Module:** `tyck.rs` (`tu_kieu_ast:93-99`, `chac_chan_lech:84`, `HaiNgoi`) +
`interp.rs:1180-1181` + `value.rs:168,206-207` · **12 ca** · `E0277`, `E0308`, `E0600`

Ba quyết định độc lập cộng dồn thành một lỗ hổng:

| Nơi | Nội dung | Hậu quả |
|---|---|---|
| `tu_kieu_ast:93-99` | `i8…i128 \| u8…u128 \| isize \| usize` → **một** `T::SoNguyen` | Mất cả bề rộng lẫn tính dấu |
| `chac_chan_lech:84` | `(SoNguyen, SoThuc) \| (SoThuc, SoNguyen) => false` | Mọi lệch nguyên/thực im lặng |
| `interp.rs:1180` | `(SoNguyen(a), SoThuc(b)) => (*a as f64, *b)` | Tự nâng kiểu **lúc chạy** |

Tầng ba là nguy hiểm nhất: nó không chỉ bỏ sót lỗi, nó **thực thi ngữ nghĩa của một
ngôn ngữ khác**. `10 + 2.5` in ra `12.5`. Một hàm khai báo `-> usize` trả về `3.5`.
`v[i]` với `i: i64` chạy bình thường.

**Vì sao xếp #2.** "Rust không ép kiểu số ngầm" là một trong ba, bốn bài học trung
tâm nhất khi chuyển từ Python/JavaScript sang Rust — và cũng là thứ khiến người mới
khó chịu nhất, nên là thứ **cần được giải thích nhất**. `byte-rust` hiện xoá bài học
đó, rồi thay bằng đúng ngữ nghĩa mà người học vừa rời bỏ.

Ba bộ kiểm **đã tồn tại và tính đúng** nhưng bị `chac_chan_lech` nuốt lỗi: `BR0301`
(đối số) tính đúng `mong=SoThuc / thực=SoNguyen`; `BR0302` (kiểu trả về) tính đúng
`mong=SoThuc / t_than=SoNguyen`. Cả hai gọi `chac_chan_lech` rồi rơi vào nhánh trả
`false`. **Đây không phải chỗ thiếu bộ kiểm — là chỗ bộ kiểm bị bịt miệng.**

**Đề xuất vá — hai bước, làm đúng thứ tự:**

**Bước 1 — cho kiểu số một danh tính.**

```rust
pub enum KieuNguyen { I8, I16, I32, I64, I128, Isize,
                      U8, U16, U32, U64, U128, Usize,
                      /// literal chưa bị ghim — linh hoạt, hợp với mọi bề rộng
                      ChuaGhim }
pub enum T { SoNguyen(KieuNguyen), SoThuc(KieuThuc), … }
```

Luật `chac_chan_lech`: `ChuaGhim` hợp với mọi `SoNguyen` khác (giữ tính linh hoạt
của literal, tránh báo oan) — nhưng **hai bề rộng cụ thể khác nhau thì lệch**, và
`SoNguyen(cụ thể)` vs `SoThuc` thì **lệch**. Một thay đổi này đóng luôn
`u32_cong_i64`, `len_usize_dung_nhu_i64`, `chi_so_i64`, `tran_literal_u8`,
`am_gan_usize`, `abs_tren_u32`, `pow_doi_so_i64`.

**Bước 2 — `interp` phải NGỪNG cứu vãn.** Xoá các nhánh ép kiểu chéo:
`interp.rs:1180-1181` (số học), `value.rs:206-207` (so sánh), `value.rs:168`
(bằng nhau), `interp.rs:894` (`Ep`: `_ => v`). Thay bằng lỗi nội bộ *"engine gặp
giá trị sai kiểu"*. Nếu `tyck` đúng thì các nhánh đó không thể chạm tới; làm chúng
**ồn ào** biến câu trả lời sai âm thầm thành lỗi engine nhìn thấy được.

⚠️ `ADR-002 §5` đã chốt `usize`/`isize` = 64-bit cố định. `KieuNguyen` phải tôn
trọng điều đó, đừng mượn `usize` của host.

---

### 🔴 #3 — Chuyển quyền sở hữu: `move_check` không có khái niệm "nơi chốn"

**Module:** `move_check.rs:141-142` (gốc rễ) + `:280, :332, :336, :344, :384, :417`
· **12 ca** · `E0382`, `E0507`

Gốc rễ nằm ở dòng đầu của `ghi_move`:

```rust
let BieuThuc::DuongDan { doan, span } = bt else { return };
```

`ghi_move` chỉ biết **tên biến trần**. Mọi phép move qua đường dẫn — `p.ten`, `t.0`,
`v[0]` — thoát im lặng ngay dòng đầu. `TrangThai` (`:37-45`) cũng gắn với tên biến
chứ không gắn với nơi chốn, nên **khái niệm partial move không tồn tại**.

Bốn nhánh duyệt thì đọc toán hạng mà không bao giờ gọi `ghi_move`:

| Dòng | Nhánh | Bỏ lọt |
|---|---|---|
| `:344` | `Tuple \| Macro` | `let t = (s, 1);` · `vec![s]` |
| `:384` | `KhopMau` | `match x { Some(s) => … }` nuốt `x` |
| `:417` | `Cho` | `for x in v` nuốt `v` |
| `:332` | `HaiNgoi` | `let b = a + "!";` nuốt `a: String` |
| `:291` | `GoiPhuongThuc` | `fn an(self)` do người học tự viết |
| `:280` | `GiaiTham` gộp với `Muon` | `*s` trên `&String` (E0507) |

**Vì sao xếp #3.** Ownership là khái niệm **định nghĩa** Rust. Và ca `for_nuot_vec`
là lỗi kinh điển bậc nhất của người mới:

```rust
let v = vec![1, 2, 3];
for x in v { … }          // into_iter(self) nuốt v
println!("{}", v.len());  // rustc: E0382 — byte-rust: Đạt, in "3"
```

Tick xanh ở đúng chỗ này gây hại nhiều hơn bất cứ đâu: người học kết luận rằng vòng
lặp `for` **không** lấy quyền sở hữu, rồi mang niềm tin đó sang `cargo` thật và
không hiểu vì sao bị chặn.

Đáng chú ý về mặt kỹ thuật: `moc_vong_lap` (`:80-129`) được xây rất công phu để bắt
move biến ngoài vào **trong** thân vòng lặp — nhưng bỏ trống chính cái **bị vòng lặp
nuốt**. Và `co_the_khong_copy` (`:62-64`) **đã** lan truyền tính không-Copy qua
`HaiNgoi`; module đã biết phép hai ngôi có thể dính kiểu không-Copy, chỉ là lượt
duyệt không dùng thông tin đó.

**Đề xuất vá — nâng `ghi_move` lên đường dẫn nơi chốn:**

```rust
enum NoiChon {
    Bien(String),
    Truong(Box<NoiChon>, String),   // p.ten  và  t.0  (parser dùng chung TruyCapTruong)
    ChiSo(Box<NoiChon>),            // v[0]
}
```

`TrangThai` khoá theo `NoiChon`. Luật: move `p.ten` đánh dấu `p.ten` là đã move
**và** đánh dấu `p` là move-một-phần (đọc nguyên `p` sau đó → `E0382`).

Kèm theo:
- `Tuple` → `ghi_move` từng phần tử. `Macro` phải **tách theo tên**: `vec!` thì
  `ghi_move`; `println!`/`print!`/`eprintln!`/`format!`/`assert*!` thì không (chúng
  thật sự chỉ mượn).
- `KhopMau` → `ghi_move(gia_tri)` khi mẫu ràng buộc theo giá trị. Đối tượng `match`
  được đánh giá **thẳng hàng trước khi rẽ nhánh**, nên đây đúng là ca luật 1 của
  ADR-002 §2, **không** phải ca cần CFG.
- `Cho` → `ghi_move(day)` (trừ khi `day` là `&v` / `v.iter()` / một `Dai`).
- `GoiPhuongThuc` → tra chữ ký trong `Muc::Impl` (`move_check.rs:455-460` đã duyệt
  qua đó): `self` → move, `&mut self` → mượn khả biến, `&self` → mượn thường. Giả
  định hiện tại "mọi receiver đều là mượn" đúng cho phương thức dựng sẵn nhưng sai
  với hàm người học tự viết.
- `GiaiTham` **tách khỏi** `Muon`: `&x` là mượn (an toàn), `*x` là lấy hẳn. Gộp
  chúng đang dạy **ngược** đúng luật *"không lấy được đồ ra khỏi thứ mình chỉ mượn"*.

Ca `khoi_tao_struct_tu_truong` chứng minh vá riêng từng nhánh duyệt là **chưa đủ**:
nhánh `KhoiTaoStruct` (`:301-309`) **đã làm đúng** phần của nó (`ghi_move` mỗi
trường), nhưng vẫn lọt vì `ghi_move` thoát ở dòng đầu. Phải vá gốc trước.

---

### 🔴 #4 — Không tồn tại pha kiểm khả biến nào

**Module:** `tyck.rs:425, :572, :382` + `interp.rs:574, :1225, :1232` · **12 ca**
· `E0384`, `E0596`, `E0594`, `E0070`

Thông tin cần thiết **đã được thu thập rồi bị vứt đi**. `ast.rs:117` khai:

```rust
fn ten_rang_buoc(&self, ra: &mut Vec<(String, bool, Span)>)
//                                            ^^^^ chính là co_the_sua
```

Nhưng **mọi** điểm tiêu thụ đều viết `for (n, _, _) in ten` — `tyck.rs:425` (`let`),
`tyck.rs:572` (tham số hàm), `tyck.rs:382` (biến lặp `for`). Cờ khả biến bị `_` nuốt
ở cả ba chỗ. `interp` thì cấp `co_the_sua: true` cho `&mut` dựa trên **cú pháp**
(`interp.rs:574-580`) chứ không hỏi binding gốc, và `gan_vao` (`:1225`) ghi đè mà
không hỏi gì.

`grep -rn 'E0384\|E0596\|E0594' crates/byte-rust/src/` → **không có kết quả nào.**

**Vì sao xếp #4.** `let` vs `let mut` là bài thứ hai của mọi giáo trình Rust, và là
điểm khác biệt đầu tiên người học nhìn thấy so với Python/JS. Ca `shadow_mat_mut`
còn trúng đúng cái bẫy shadowing kinh điển — tưởng `mut` còn hiệu lực sau khi
`let n = n + 1` tạo binding mới:

```rust
let mut n = 1;  n = 2;
let n = n + 1;  // binding MỚI, không mut
n = 10;         // rustc: E0384 — byte-rust: Đạt
```

**Đề xuất vá** — đây là lỗ hổng **rẻ nhất so với giá trị** trong toàn báo cáo, vì dữ
liệu đã có sẵn:

1. Bảng biến của `tyck` lưu `(T, bool /*co_the_sua*/)` thay vì `T`. Sửa ba chỗ
   `for (n, _, _)` thành `for (n, mut_, _)`.
2. Thêm hàm `goc_binding(place) -> Option<&str>` truy ngược `d.x.y` → `d`, `v[0]` →
   `v`. Khả biến ở Rust là thuộc tính của **binding**, lan xuống mọi trường — đúng
   thứ ca `sua_truong_struct_khong_mut` đang bỏ lọt.
3. Ba luật mới trong `tyck`:
   - gán vào nơi chốn có gốc không `mut` → `E0384`/`E0594`;
   - `&mut <nơi chốn>` với gốc không `mut` → `E0596`;
   - receiver `&mut self` trên gốc không `mut` → `E0596` (cần bảng chữ ký từ #3/#8).
4. `Muc::Const` phải vào **bảng riêng, bất biến**. Hiện `interp.rs:218-222` gọi thẳng
   `dat_bien`, biến `const` thành biến thường toàn cục; `tyck::nap` (`:151-172`) bỏ
   qua `Muc::Const` hoàn toàn → `E0070`.

Lưu ý phải làm ở **tầng tĩnh**. Hai ca `ghi_qua_tham_chieu_chi_doc` và
`tham_chieu_thuong_cho_tham_so_mut` chứng minh: `interp` **có** luật đúng (`BR0536`)
nhưng là kiểm **động** — đặt phép ghi vào nhánh `if` không chạy là thoát sạch. Kiểm
chứng đối chứng: cùng chương trình với điều kiện `true` cho `KhongDat [BR0536]`.

---

### 🟠 #5 — Luật `E0499` mà ADR-002 hứa đích danh **không tồn tại trong mã**

**Module:** `move_check.rs` · **8 ca** · `E0499`, `E0502`, `E0507`, `E0597`, `E0106`

ADR-002 §2 viết:

> Chỉ hai luật, cả hai quyết định được **không cần CFG**:
> 1. Dùng lại một binding không-`Copy` sau khi đã move… → `E0382`
> 2. **Hai `&mut` tới cùng một chỗ trong cùng một câu lệnh → `E0499`**

```
$ grep -rn 'E0499\|E0502\|E0507\|E0597\|E0106' crates/byte-rust/src/
(không có kết quả)
```

Luật thứ hai — được quảng cáo là một trong hai cổng an toàn về ownership — **chưa
bao giờ được cài đặt**. `enum TrangThai` (`:37-45`) chỉ có `Song`/`DaMoveThang`/
`DaMoveTrongNhanh`; không có trạng thái "đang bị mượn khả biến". `BieuThuc::Muon`
(`:280`) chỉ đi thẳng vào con, và `co_the_khong_copy` (`:68`) trả `false` cho mọi
`Muon` nên các binding mượn thậm chí không được theo dõi.

**Vì sao xếp #5 chứ không cao hơn.** Đây là lỗi **quy trình** nghiêm trọng hơn là
lỗi kỹ thuật: ADR-002 §2 nói rõ mọi tình huống mượn phức tạp phải trả
`ChuaHoTro("borrow-check-cfg")` — nghĩa là *thiếu luật thì được, im lặng thì không*.
Hiện tại không có luật **và** không có `ChuaHoTro`: engine chọn phương án thứ ba mà
ADR cấm. Người học bị thiệt hại gián tiếp (mất một lời thú nhận đáng lẽ có), chứ
chưa bị dạy sai một ngữ nghĩa cụ thể như #1–#4.

**Đề xuất vá — hai việc, đừng lẫn:**

**(a) Cài luật 2 đúng như ADR hứa** — hẹp, không cần CFG: trong **một câu lệnh**,
gom mọi `BieuThuc::Muon { co_the_sua: true }` theo nơi chốn; ≥ 2 lần tới cùng nơi
chốn → `E0499`. Đóng `hai_muon_mut_trong_cung_mot_cau_lenh`.

**(b) Với mọi thứ còn lại, phát `ChuaHoTro`, đừng im lặng.** `E0502`, `E0597`,
`E0106`, và `hai_muon_mut_qua_hai_cau_lenh` đòi phân tích vùng sống thật — ADR-002
đã quyết định **không** làm, và quyết định đó đúng. Nhưng khi ấy chúng phải rơi vào
`ChuaHoTro("borrow-check-cfg")`, kèm câu ADR đã soạn sẵn về việc mở bản Desktop.

Riêng `E0106` (`tra_ve_tham_chieu_toi_bien_cuc_bo`) thì **rẻ và nên làm ngay**: chỉ
cần kiểm chữ ký, không cần vùng sống — kiểu trả về có `&` mà hàm không có tham số
tham chiếu nào → `E0106`. Không đụng gì tới NLL.

Ghi chú kiến trúc quan trọng: `E0597` (`tham_chieu_song_lau_hon_thu_no_tro_toi`)
`interp` **không thể** bắt được về nguyên tắc — giá trị nằm trong `Rc` nên khi khối
đóng, ô nhớ vẫn sống. Ngữ nghĩa runtime của `byte-rust` về cấu trúc không tái tạo
được lớp lỗi này. Chỉ pha tĩnh mới bắt được, và pha tĩnh không có luật nào.

---

### 🟠 #6 — Vùng mù mã chết: kiểm tra chỉ tồn tại ở `interp`

**Module:** `tyck.rs` (thiếu luật) + `interp.rs` (có luật nhưng động) · **12 ca**
· `E0061`, `E0425`, `E0599`, `E0609`, `E0614`

`rustc` kiểm kiểu **mọi** hàm và **mọi** nhánh, kể cả hàm không ai gọi và nhánh không
bao giờ chạy. `byte-rust` có nhiều luật đúng — nhưng đặt ở `interp`, nên chúng chỉ
thấy đường mà chương trình thực sự đi qua.

| Luật | Chỉ có ở | Đường vòng |
|---|---|---|
| `BR0506` sai số đối số | `interp` | đặt lời gọi trong `if n > 5` với `n = 2` |
| `BR0550` không tìm thấy hàm | `interp` | như trên |
| `BR0501` không tìm thấy tên | `interp` | như trên |
| `BR0511` không có trường | `interp` | đặt trong hàm không được gọi |
| `BR0552` không có phương thức | `interp` | đặt trong hàm không được gọi |
| `BR0520` giải tham chiếu sai | `interp` | đặt trong hàm không được gọi |

Đã kiểm chứng đối chứng cho từng ca: cùng lời gọi `cong(1)` đặt ở nhánh **sống** cho
`KhongDat [BR0506]`, đặt trong nhánh chết cho `Dat`.

**Vì sao xếp #6.** Lỗ hổng này không dạy sai một khái niệm cụ thể — nó **vô hiệu hoá
tầng phòng thủ** của mọi khái niệm khác. Chính chú thích ở `interp.rs:485-488` đã
cảnh báo *"kiểm-lúc-chạy chỉ thấy nhánh đã đi qua"* — nhưng cảnh báo đó chỉ được vá
cho `move`, không vá cho phân giải tên, arity, trường, hay phương thức.

**Đề xuất vá:**

1. **Pha phân giải tên tĩnh** — rẻ nhất trong cả báo cáo. Vấn đề là `tyck::tra`
   (`:209-211`) kết thúc bằng `.unwrap_or(T::Mo)`, gộp "tên chưa suy được kiểu" với
   "tên **không tồn tại**". Tách hai khái niệm đó (xem §3) là đóng luôn cả bốn ca
   `E0425`. `khoi()` **đã** duyệt câu lệnh tuần tự nên `dung_truoc_khi_khai_bao` rơi
   ra miễn phí. Không cần CFG, không đụng lập luận NLL của ADR-002 §2.
2. **Nhân bản mọi luật `interp` chỉ-động lên `tyck`** ở dạng tĩnh: đếm arity trong
   `GoiHam` (`tyck` hiện chỉ đối chiếu **kiểu** qua `mong.get(i)`, không bao giờ so
   `doi_so.len()` với `ck.tham_so.len()`, và đối số **thừa** bị `if let Some(m)`
   nuốt im lặng); tra trường; tra phương thức.
3. **Thêm đột biến "bọc vào nhánh chết" vào `mutate.rs`** để lớp lỗi này không bao
   giờ tái xuất mà cổng không biết.

---

### 🟠 #7 — Vét cạn `match` chỉ hoạt động cho enum ở tầng ngoài cùng

**Module:** `tyck.rs` (`fn khop_mau`) · **7 ca** · `E0004`

`tyck.rs` tự mô tả là *"kiểm kiểu NHẸ + kiểm tính vét cạn của match"*. Tính vét cạn
là **tính năng được tuyên bố** của module — nên khoảng cách giữa lời hứa và thực tế
ở đây đáng kể:

| Bỏ lọt | Vì sao |
|---|---|
| `bool` thiếu nhánh | Khối kiểm bọc trong `if let T::Enum(ten) = &t_gt` |
| số nguyên thiếu nhánh | như trên; `Mau::HangSo` rơi vào `_ => {}` |
| tuple thiếu nhánh | `kieu_cua` chỉ trả `T::Rong` cho tuple **rỗng**; tuple thường → `T::Mo` |
| khoảng `0..=5` thiếu | `Mau::Dai` không có nhánh xử lý nào |
| enum lồng nhau | `filter_map` lấy `duong_dan.last()` rồi coi biến thể là phủ **trọn**, không đệ quy vào mẫu con |
| `Some(1)` (payload hằng số) | như trên — và trúng ngay `Option`, ví dụ dạy học phổ biến nhất |
| nhánh bao quát **có guard** | `co_bao_quat = true` được đặt mà không xét `n.dieu_kien`; việc lọc guard chỉ làm ở `phu_khong_dieu_kien`, vốn chỉ dành cho `Mau::BienThe` |

**Đề xuất vá.** Thay cách đếm "tên biến thể ở tầng ngoài" bằng một thuật toán tính
hữu dụng có nhân chứng (Maranget) trên ma trận mẫu nhỏ. Phạm vi đề nghị: enum,
`bool`, tuple của hai thứ đó, và số nguyên/khoảng (coi là **chưa phủ** trừ khi có
mẫu bao quát không guard). Ba luật bắt buộc:

1. **Guard không bao giờ tính là phủ** — `rustc` cũng vậy.
2. **Đệ quy vào mẫu con** — `Hop::Chua(Mau::Do)` chỉ phủ khi mẫu con bất khả bác bỏ.
3. **Ngoài phạm vi thì `ChuaHoTro("vet-can")`, không im lặng** — slice, `@`, mẫu
   `Or` phức tạp. Đây là chỗ áp dụng §3 trực tiếp.

---

### 🟠 #8 — Phương thức dựng sẵn: không có chữ ký, và bảng hiện có thì **sai**

**Module:** `tyck.rs:319, :523-538` + `interp.rs:1599-1600, :1697-1714` · **14 ca**
· `E0308`, `E0599`, `E0369`, `E0061`

Toàn bộ thân nhánh `GoiPhuongThuc` trong `tyck` là:

```rust
let chu = self.kieu_cua(doi_tuong);
for a in doi_so { self.kieu_cua(a); }   // ← tính kiểu rồi VỨT ĐI
kieu_tra_ve_phuong_thuc(&chu, ten)
```

Kiểu đối số được tính rồi bỏ. Không có bảng chữ ký phương thức nào trong crate. Đối
lập hẳn với nhánh `GoiHam` vốn có `mong.get(i)` + `lech_tham_chieu`.

Và `kieu_tra_ve_phuong_thuc` **nhận** `chu: &T` nhưng gần như không dùng — nó dispatch
theo **tên** phương thức:

```rust
"to_string" | "to_uppercase" | "to_lowercase" | "trim" | "push_str" => T::Chuoi,
"abs" | "pow" | "min" | "max" | "sum" => …
```

Ba loại lỗi khác nhau ở đây, cần phân biệt khi vá:

**(a) Bảng SAI, không phải thiếu.** `push_str` được khai là trả `String` — thực tế
trả `()`. Đây nguy hiểm hơn lỗ `T::Mo` vì bộ kiểm **chủ động khẳng định sai** thay
vì im lặng. Tệ hơn, `interp.rs:1711` cũng trả `Chuoi` (và không đột biến `s`), nên
hai tầng sai **cùng một hướng** và phép kiểm kiểu trả về **tự xác nhận cái sai của
chính nó**.

**(b) Engine dạy một API không tồn tại.** `interp.rs:1631` cài `(Day(d), "sum")`
trực tiếp trên `Vec`, coi `Vec` là iterator. Và `interp.rs:1799` liệt kê chính
`"sum"` trong **danh sách gợi ý phương thức hợp lệ** khi báo `BR0552`. Engine không
chỉ bỏ sót lỗi — nó **chủ động dạy** người học viết `v.sum()` thay vì
`v.iter().sum()`, và người học sẽ mang đúng thói quen đó sang `cargo` thật.

**(c) Ép kiểu ngầm lúc chạy — mẫu lặp bốn lần.** `interp.rs:1712`:

```rust
(Chuoi(s), "push_str") => { let x = ds.first().map(|v| v.hien_thi()).unwrap_or_default(); … }
```

`s.push_str(7)` ép số thành chuỗi qua `hien_thi()` — **đúng thứ mà thông báo lỗi
`BR0531` của chính engine này tuyên bố Rust không làm**. Cùng chiêu
`ds.first().map(|v| v.hien_thi())` dùng lại nguyên xi ở `:1697` (`contains`),
`:1701` (`starts_with`), `:1706` (`split`). Đây là một **mẫu** sai lặp bốn lần, không
phải một ca lẻ. (Ca này còn in ra `"so: "` — thiếu hẳn số 7 — mà vẫn `Dat`.)

**(d) Hai nhánh bắt-tất-cả đặt sai vị trí.** `interp.rs:1599-1600`:

```rust
(v, "clone")     => Ok(v.clone()),
(v, "to_string") => Ok(Chuoi(Rc::new(v.hien_thi()))),
```

Đứng **trước** mọi nhánh theo kiểu nên khớp với **mọi** `GiaTri`. `.to_string()` gọi
được trên vector, struct, enum, `()` — bất kể có `Display` hay không.

**Đề xuất vá:**

```rust
struct ChuKyPhuongThuc {
    chu_the: T,          // kiểu chủ thể chấp nhận được
    nhan: NhanSelf,      // Self | ThamChieu | ThamChieuMut
    tham_so: Vec<T>,
    tra_ve: T,
}
// khoá theo (kiểu chủ thể, tên) — KHÔNG phải theo tên
```

- Cặp `(kiểu cụ thể, tên)` không có trong bảng, với chủ thể kiểu **đã biết** →
  `E0599`. Với chủ thể `T::Mo`/`ChuaBiet` → `ChuaHoTro`, không im lặng.
- Sửa hai mục sai đã biết: `push_str -> T::Rong`; `sum` **bỏ khỏi** `Vec` (và bỏ
  khỏi danh sách gợi ý ở `interp.rs:1799`); `to_string` gắn điều kiện `Display`.
- Chuyển hai nhánh bắt-tất-cả **xuống dưới** mọi nhánh theo kiểu.
- Xoá mọi `hien_thi()` dùng làm ép kiểu ngầm ở `:1697, :1701, :1706, :1712`.
- `NhanSelf::ThamChieuMut` nối thẳng vào luật khả biến của #4 (đóng
  `push_vec_khong_mut`) và luật mượn của #3 (đóng `phuong_thuc_nuot_self`).

⚠️ Ca `len_usize_dung_nhu_i64` là cảnh báo quan trọng: đây là ca **duy nhất** mà
`tyck` thực sự chạy kiểm đối số mà vẫn lọt, vì `"len" => T::SoNguyen` và `i64` cũng
là `T::SoNguyen`. **Thêm bảng chữ ký thôi chưa đủ — phải làm #2 (tách bề rộng)
trước, nếu không bảng mới sẽ vẫn mù.**

---

### 🟡 #9 — Struct & enum: bảng đã nạp sẵn nhưng chỉ tra ở một chiều

**Module:** `tyck.rs` (`nap`, `KhoiTaoStruct`, `khop_mau`) + `interp.rs:~858, ~1067,
~1232, ~1441` · **14 ca** · `E0560`, `E0308`, `E0609`, `E0422`, `E0026/27`, `E0599`,
`E0061`, `E0559`

`tyck::nap` (`:172-178`) **đã nạp** `truong_struct: HashMap<String, HashMap<String, T>>`
với đúng kiểu từng trường. Bảng đó chỉ được tra ở nhánh `TruyCapTruong` khi **đọc**
trường. Chiều khởi tạo, chiều mẫu, chiều ghi đều không tra.

| Chiều | Trạng thái | Ca |
|---|---|---|
| Đọc trường | ✅ có tra | — |
| Khởi tạo (kiểu trường) | ❌ không tra | `truong_sai_kieu` |
| Khởi tạo (trường thừa) | ❌ chỉ kiểm chiều **thiếu** (`BR0510`) | `truong_thua` |
| Mẫu `match` | ❌ không tra | `mau_truong_khong_ton_tai`, `thieu_truong_struct` |
| Ghi (`d.z = 9`) | ❌ `insert` vô điều kiện | `ghi_truong_khong_ton_tai` |
| Struct tuple | ❌ `nap` chỉ nạp `ThanStruct::TheoTen` | `tuple_struct_sai_chi_so` |

Ba lỗ hổng `interp` đặc biệt đáng chú ý vì chúng **bịa ra giá trị**:

- `KhoiTaoStruct` (`:~858`) dựng map trường **trước**, rồi
  `if let Some(sd) = self.struct_def.get(&ten)`. Struct không tồn tại thì bỏ qua luôn
  cả khối kiểm, và dòng cuối vẫn `Ok(GiaTri::Struct { ten, truong })` — **bịa struct
  từ hư không**. Mọi định danh viết hoa gõ nhầm đều thành chương trình "Đạt".
- `gan_vao/TruyCapTruong` (`:~1232`) `insert` trường mới vào map runtime: **struct mọc
  thêm trường lúc chạy, y như JavaScript**.
- `tinh_duong_dan` (`:~1067`) chỉ kiểm **vế trái**: hễ enum tồn tại thì **mọi** biến
  thể bịa ra đều hợp lệ (`Mau::Tim`). Và mọi `Enum::X` đều trả
  `BienThe { gia_tri: vec![] }` — dựng một `Tron` **rỗng**, giá trị mà kiểu `Hinh`
  không cho phép tồn tại.

**Đề xuất vá:** tra `truong_struct` ở **cả bốn chiều**; nạp thêm
`ThanStruct::TheoViTri` (struct tuple, khoá `"0"`, `"1"`…); ghi **arity** của biến
thể enum trong `nap` rồi kiểm ở `GoiHam` + `tinh_duong_dan` + `khop_mau`; `interp`
phải **báo lỗi** cho tên struct/biến thể chưa khai báo thay vì dựng giá trị.

Hai ca đáng ghi riêng vì chúng lộ một khuôn mẫu chung: `sai_so_truong_bien_the`
(`interp.rs:412`: `if ps.len() != gia_tri.len() { return Ok(false) }`) và
`truong_struct_khong_ton_tai` (`:434`: `let Some(x) = f.get(k) else { return Ok(false) }`)
đều **biến một lỗi kiểu thành "không khớp" lúc chạy**, rồi nhánh `_` nuốt nốt hậu
quả. Mọi chỗ `khop_mau` trả `Ok(false)` cần được rà: "mẫu này không khớp giá trị
này" và "mẫu này không thể khớp kiểu này" là hai chuyện khác nhau.

---

### 🟡 #10 — Kiểu các nhánh không bao giờ được hợp nhất

**Module:** `tyck.rs` (`BieuThuc::Neu`, `khop_mau`, `duyet_con/TraVe`) · **6 ca**
· `E0308`

```rust
// BieuThuc::Neu
let a = self.khoi(than);  let b = self.kieu_cua(nl);
if a == T::Mo { b } else { a }        // kiểu nhánh `else` bị VỨT
// khop_mau
if kieu_nhanh == T::Mo { kieu_nhanh = t; }   // chỉ giữ nhánh ĐẦU TIÊN
```

Không bao giờ gọi `chac_chan_lech` giữa các nhánh. Ca `de_quy_ho_tuong_lech_kieu`
cho thấy thông tin **đã có sẵn**: `tyck` tra đúng chữ ký `la_chan`, tính ra `T::Bool`
cho nhánh `else`, rồi vứt ngay giá trị đó ở dòng kế tiếp.

Một bất đối xứng đã kiểm chứng: dấu `;` ở **cuối thân hàm** thì `tyck` bắt được
(`fn f() -> i64 { 7; }` → `KhongDat [BR0302]`), nhưng dấu `;` ở cuối một **nhánh
`else`** thì lọt hoàn toàn.

`return` sớm cũng không được kiểm: `duyet_con` xử lý `BieuThuc::TraVe` bằng cách
tính kiểu rồi vứt, vì bộ kiểm **không mang theo ngữ cảnh "kiểu trả về của hàm hiện
tại"**. `BR0302` chỉ so kiểu của `gia_tri_cuoi`.

**Đề xuất vá:** một hàm `hop_nhat(a, b) -> T` phát lỗi khi `a.chac_chan_lech(&b)`,
dùng ở `Neu` và `khop_mau`. Thêm trường `tra_ve_hien_tai: Option<T>` vào
`BoKiemKieu`, đặt khi vào thân hàm, đối chiếu ở mọi `BieuThuc::TraVe`.

---

### 🟡 #11 — `String` và `&str` là cùng một kiểu

**Module:** `value.rs` (`GiaTri::Chuoi`) + `interp.rs:1121-1123` + `tyck.rs`
· **3 ca** · `E0308`, `E0369`, `E0382`

`interp.rs:1121-1123` có đúng **một** nhánh cộng chuỗi, với chú thích ngay trên nó
ghi *"Nối chuỗi: `String + &str`"*:

```rust
if let (Chuoi(a), Chuoi(b)) = (&t, &p) { … }
```

Không có gì phân biệt được vế phải là mượn hay sở hữu. Rust có **ba** trường hợp
(`String + &str` hợp lệ; `String + String` là `E0308`; `&str + &str` là `E0369`) —
`byte-rust` chỉ có **một** trạng thái để biểu diễn cả ba.

Đây cũng là gốc của `cong_chuoi_nuot_move`: `impl Add<&str> for String` nhận `self`
theo giá trị — **chính lý do Rust bắt viết `a + &b`**.

**Đề xuất vá:** tách `GiaTri::Chuoi` thành `Chuoi` (sở hữu) và `LatChuoi` (mượn), và
thêm `T::LatChuoi` bên cạnh `T::Chuoi`. Đây là thay đổi lan rộng nhất trong báo cáo
(chạm `value.rs`, `interp.rs`, `tyck.rs`), nên xếp sau — nhưng nó là điều kiện cần
để dạy đúng `&str` vs `String`, một trong những chỗ người học Rust vấp nhiều nhất.

---

## 5. Bảng tra ngược: module → lỗ hổng → ca

| Module | Lỗ hổng gốc (mục) | Ca |
|---|---|---:|
| `tyck.rs` | #1 `Let` không đối chiếu | 8 |
| `tyck.rs` + `interp.rs` + `value.rs` | #2 ép kiểu số ngầm | 12 |
| `move_check.rs` | #3 không có nơi chốn; 4 nhánh thiếu `ghi_move` | 12 |
| `tyck.rs` + `interp.rs` | #4 không có pha khả biến | 12 |
| `move_check.rs` | #5 luật mượn không tồn tại (ADR drift) | 8 |
| `tyck.rs` (thiếu) / `interp.rs` (động) | #6 vùng mù mã chết | 12 |
| `tyck.rs` | #7 vét cạn `match` | 7 |
| `tyck.rs` + `interp.rs` | #8 phương thức dựng sẵn | 14 |
| `tyck.rs` + `interp.rs` | #9 struct & enum | 14 |
| `tyck.rs` | #10 hợp nhất kiểu nhánh | 6 |
| `value.rs` + `interp.rs` | #11 `String` vs `&str` | 3 |

Tổng cộng > 100 vì nhiều ca chạm nhiều lỗ hổng cùng lúc — đó là điểm quan trọng:
**vá một lỗ hổng thường không đủ để một ca chuyển sang `KhongDat`.** Ví dụ
`am_gan_usize` cần cả #1 (`let` đối chiếu) lẫn #2 (giữ tính dấu).

---

## 6. Thứ tự vá đề xuất

Sắp theo **(giá trị dạy học) ÷ (chi phí)**, có tính phụ thuộc giữa các bước.

| # | Việc | Module | Chi phí | Mở khoá |
|---|---|---|---|---|
| 0 | Tách `T::Mo` / `T::ChuaBiet` (§3) | `tyck.rs` | Nhỏ | Biến ~15 ca `Dat` thành `ChuaHoTro` **trung thực** |
| 1 | `CauLenh::Let` gọi `chac_chan_lech` (#1) | `tyck.rs` | ~6 dòng | 8 ca |
| 2 | Pha khả biến — giữ `co_the_sua` (#4) | `tyck.rs` | Nhỏ | 12 ca |
| 3 | Phân giải tên tĩnh (#6.1) | `tyck.rs` | Nhỏ | 4 ca `E0425` |
| 4 | `KieuNguyen` + gỡ ép kiểu ở `interp` (#2) | `tyck.rs`, `interp.rs`, `value.rs` | Vừa | 12 ca; **điều kiện cần cho #8** |
| 5 | `NoiChon` + 4 nhánh `ghi_move` (#3) | `move_check.rs` | Vừa | 12 ca |
| 6 | Arity + tra bảng struct/enum mọi chiều (#9) | `tyck.rs`, `interp.rs` | Vừa | 14 ca |
| 7 | Bảng chữ ký phương thức (#8) | `tyck.rs`, `interp.rs` | Vừa | 14 ca (**sau** bước 4) |
| 8 | `hop_nhat` nhánh + ngữ cảnh `return` (#10) | `tyck.rs` | Nhỏ | 6 ca |
| 9 | Vét cạn `match` thật (#7) | `tyck.rs` | Lớn | 7 ca |
| 10 | Luật `E0499` + `ChuaHoTro` cho phần còn lại (#5) | `move_check.rs` | Vừa | 8 ca; **đóng khoảng cách với ADR-002** |
| 11 | Tách `String` / `&str` (#11) | `value.rs`, `interp.rs`, `tyck.rs` | Lớn | 3 ca |

**Bước 0 nên làm trước tiên** kể cả khi chưa vá gì khác: nó không sửa một suy luận
nào, nhưng chuyển đúng thứ ADR-002 §1 gọi là *"lời thú nhận của công cụ"* từ im lặng
sang nói thẳng. Một `ChuaHoTro` trung thực không gây hại cho người học; một `Dat`
oan thì có.

Song song, sửa `mutate.rs`: đổi `Default::default()` thành literal trong
`gan_lai_khong_mut`, và thêm phép đột biến **bọc mã sai vào nhánh chết**.

---

## 7. Nguyên tắc vá

**Không vá cho vừa khít 100 ca trong `corpus_am.rs`.** Chúng là *triệu chứng*, không
phải *đặc tả*. Vá đến mức đúng từng chương trình cụ thể — so tên biến, so hình dạng
AST, thêm ca đặc biệt cho `for x in v` — sẽ làm cổng xanh trong khi lỗ hổng còn
nguyên, và đó **đúng chế độ hỏng mà ADR-002 tồn tại để chặn**. Nguồn A của báo cáo
này là bằng chứng sống: nó đã xanh 100 % suốt thời gian qua trong khi engine nhận
oan 100 ca.

Ba câu hỏi cho mỗi bản vá:

1. **Vá ở gốc chưa?** `khoi_tao_struct_tu_truong` cho thấy vá nhánh duyệt trong
   `move_check` không đủ nếu `ghi_move` vẫn chỉ nhận tên biến trần.
2. **Có tạo báo oan không?** Mọi luật mới phải chạy qua **corpus dương 364 tệp** ở
   `crates/byte-rust-conformance/corpus/`. Từ chối oan xấu hơn thiếu luật — ADR-002
   đã bác NLL-rút-gọn chính vì lý do này.
3. **Nếu chưa vá được thì đã thú nhận chưa?** Thiếu luật là **chấp nhận được**;
   im lặng thì không. Mọi chỗ chưa mô hình hoá phải phát `ChuaHoTro`, không phải
   `Dat`.

Khi một ca chuyển sang `KhongDat`, **đừng xoá nó khỏi `corpus_am.rs`** — nó trở thành
lưới hồi quy. Nhưng lưới hồi quy chỉ có giá trị khi ta biết nó xanh **vì lý do đúng**;
xem lại §2 trước khi tin bất kỳ con số 100 % nào.
