# Nợ chấm điểm — ĐÃ PHÂN LOẠI XONG (cổng đã nối vào cong.sh)

`tools/kiem_dot_bien.mjs` lấy chính lời giải, sửa một Ý trong phần người học
phải điền (mọi nút cùng loại một lượt), rồi hỏi cách chấm có bắt được không.
475 đột biến trên 230 bước `code` có chỗ trống, trong 242 bài — 21 bước lọt.

**Chưa nối vào `cong.sh`,** và chưa cái nào dưới đây được đối chiếu tay. Mỗi
cái phải phân loại thành một trong ba, và ba cách xử lý khác hẳn nhau:

1. **Tương đương thật** — không cách chấm nào phân biệt nổi, vì không có gì để
   phân biệt. Ví dụ đã soi: `tim-ngay-cao-nhat` đổi `>` thành `>=` trong vòng
   tìm max cho ra ĐÚNG cùng một con số. Xử lý: khai miễn trừ kèm lý do.
2. **Tương đương theo DỮ LIỆU** — hai bên khác nhau về nghĩa, nhưng bộ dữ liệu
   bài dùng để chấm không có giá trị nào rơi vào chỗ khác nhau ấy. Ví dụ: đếm
   ngày vượt ngưỡng 200 000 mà trong sổ không ngày nào đúng 200 000. Đây là
   **lỗ chấm điểm thật**, và cách sửa là thêm một giá trị đúng mốc vào dữ
   liệu — không phải khai miễn trừ.
3. **Lỗ thật sự** — cách chấm quên hẳn một chỗ trống. Xử lý: siết khối test
   hoặc luật static. ĐỪNG nới lời giải.

Loại 2 là loại đáng giá nhất và cũng dễ nhầm thành loại 1 nhất: cả hai đều
"chạy ra cùng kết quả". Khác nhau ở chỗ loại 2 chỉ cùng kết quả vì bài chưa
bao giờ hỏi tới cái mốc — mà cái mốc chính là thứ bài dạy.

Hạn chế của cổng phải nói thẳng: nó chỉ đột biến được bước `code` có `___` và
có số dòng `starter` khớp `solution`. Xanh KHÔNG có nghĩa mọi cách chấm đều
chặt.

| bài | bước | đột biến vẫn lọt |
|---|---|---|
| `nen-tang.gia-tri-bien-kieu.chuoi-dai-bao-nhieu` | `do-ten-vao-cot` | đổi MỌI dấu <= thành < (2 chỗ) |
| `nen-tang.ham-vien-gach.chan-ngay-o-cua` | `cong-cho-don-giao-hang` | đổi MỌI hằng số 0 thành 1 (1 chỗ) |
| `nen-tang.list-dict-set-tuple.boss-so-chi-tieu-thang` | `ghep-ca-cuon-so-thang` | đổi MỌI hằng số 100000 thành 100001 (1 chỗ) |
| `nen-tang.list-dict-set-tuple.may-nhin-o-dau-truoc` | `hoi-thang-may-cap-nao-truoc` | đổi MỌI dấu < thành <= (2 chỗ) |
| `nen-tang.re-nhanh-va-lap.ai-duoc-tinh-truoc` | `mot-dong-bon-bac` | đổi MỌI dấu > thành >= (4 chỗ) |
| `nen-tang.re-nhanh-va-lap.boss-so-chi-tieu-cua-byte` | `giu-lai-ngay-ky-luc` | đổi MỌI dấu > thành >= (2 chỗ) |
| `nen-tang.re-nhanh-va-lap.buoc-nhay-cua-range` | `hot-bot-noi-nuoc-dung` | đổi MỌI hằng số 120 thành 121 (1 chỗ) |
| `nen-tang.re-nhanh-va-lap.dem-nhung-luot-dang-ke` | `dem-ngay-vuot-nguong` | đổi MỌI dấu > thành >= (1 chỗ) |
| `nen-tang.re-nhanh-va-lap.dieu-kien-trong-dieu-kien` | `them-mot-tang` | đổi MỌI dấu > thành >= (2 chỗ) |
| `nen-tang.re-nhanh-va-lap.duong-di-toi-mot-dong` | `viet-lai-cho-phang` | đổi MỌI dấu > thành >= (1 chỗ) |
| `nen-tang.re-nhanh-va-lap.giu-lai-ky-luc` | `tim-ngay-cao-nhat` | đổi MỌI dấu > thành >= (1 chỗ) |
| `nen-tang.re-nhanh-va-lap.nhanh-khong-bao-gio-toi` | `xep-lai-hang-ro` | đổi MỌI dấu > thành >= (4 chỗ) |
| `onboarding.ra-lenh-cho-byte.chi-can-mot-ve-dung` | `tam-bien-khuyen-mai` | đổi MỌI dấu > thành >= (1 chỗ) |
| `onboarding.ra-lenh-cho-byte.hai-dieu-kien-cung-dung` | `dan-to-giay-len-tuong` | đổi MỌI dấu < thành <= (1 chỗ) |
| `onboarding.ra-lenh-cho-byte.lam-lai-nhieu-lan` | `chan-nuoc-dung` | đổi MỌI hằng số 6 thành 7 (1 chỗ) |
| `onboarding.ra-lenh-cho-byte.neu-thi` | `dat-mien-phi` | đổi MỌI dấu > thành >= (2 chỗ) |
| `toan.cam-nhan-so.mang-chu-nhat-xoay-mot-goc` | `trong-tai-hai-cau` | đổi MỌI hằng số 12 thành 13 (2 chỗ) |
| `toan.cam-nhan-so.phan-so-khong-bi-nhot-duoi-mot` | `do-hai-luong` | đổi MỌI hằng số 7 thành 8 (1 chỗ) |
| `toan.cam-nhan-so.phan-so-la-may-lan-don-vi-moi` | `hoi-may-hai-cau` | đổi MỌI hằng số 5 thành 6 (1 chỗ) |
| `toan.dai-so-va-ham-so.dau-bang-la-mot-cau-hoi` | `may-phan-ba-loi-khang-dinh` | đổi MỌI hằng số 21 thành 22 (1 chỗ) |
| `toan.dai-so-va-ham-so.khi-dau-phai-quay-nguoc` | `hai-luat-tranh-nhau` | đổi MỌI hằng số 20000 thành 20001 (3 chỗ) |


---

# Kết quả phân loại

Đã soi từng cái trong 21, và kết luận không giống lúc mới nhìn.

**Chỉ MỘT là lỗ chấm điểm thật, và đã sửa:** `lam-lai-nhieu-lan · chan-nuoc-dung`.
Bài dạy `range(6)`, chú thích trong khối test viết "hiện ra **đủ sáu lượt**",
nhưng luật chấm là `contains` — nên `range(7)`, thậm chí `range(600)`, cũng
xanh. Bài tự nhận một điều cách chấm của nó không kiểm, mà đếm đủ số lượt
chính là thứ bài dạy. Đã đổi sang `match: regex` đếm đủ sáu dòng.

**20 cái còn lại không phải lỗi của bài nào cả.** Chúng lộ ra một tính chất
CẤU TRÚC của Realm 0 và Realm 1: bài chấm mã viết thẳng trên ĐÚNG MỘT bộ dữ
liệu cố định, nên bất kỳ ngưỡng nào bộ dữ liệu ấy không bắc qua đều không kiểm
được. Muốn kiểm thì hoặc đổi dữ liệu — kéo theo mọi bài dùng chung sổ, đúng
thứ cổng sổ sự thật dựng ra để chặn — hoặc gói việc vào một hàm rồi gọi nhiều
lần, mà hàm là khái niệm của T1.3, tới sau.

Thêm nữa, phần lớn những ngưỡng ấy KHÔNG phải khái niệm bài dạy. `neu-thi` dạy
`if`; thêm một ca đúng 65 tuổi để phân biệt `>` với `>=` là lén đưa vào khái
niệm thứ hai, phạm luật "mỗi bài đúng một khái niệm mới". Chỗ khác nhau giữa
hai dấu ấy có bài riêng — `dung-hay-sai` — và bài ấy CÓ chấm đúng cái mốc, bằng
một tình huống dựng lên chỉ để làm việc đó.

Cả 20 đã khai vào `content/curriculum/dot-bien-bo-qua.yaml`, **từng cái một,
kèm lý do riêng**. Khai bừa để cổng xanh là tự tay dựng lại đúng thứ cả dự án
đi bắt.

**Hai chỗ đáng làm sau, cổng chỉ ra được mà người đọc thì không:**

- ~~`chuoi-dai-bao-nhieu`~~ — **ĐÃ LÀM.** Thêm `ten_3 = "bạc xỉu nóng"`, dài
  đúng 12 chỗ tức đúng bề rộng cột. Giờ `<=` mới có chỗ chứng tỏ mình khác `<`:
  vừa khít cột thì tính là vừa. Miễn trừ của bài đã BỎ — bài tự phân biệt được,
  không cần tha nữa.
- ~~`phan-so-khong-bi-nhot-duoi-mot`~~ — **rút lại.** Soi kỹ thì khối test của
  bài đã có sẵn `assert 4/4 == 1`, nên chỗ hoà KHÔNG thiếu. Và thêm một câu
  `print(4/4 > 1)` cũng chẳng đóng được đột biến, vì `5/5 > 1` cũng `False` y
  như `4/4 > 1`. Không sửa — một thay đổi không đóng được gì mà chỉ làm bài dài
  thêm thì không đáng.

**Từ T1.3 trở đi không còn cái cớ "bài chưa có hàm".** Bài mới phải chấm được
chỗ hoà nếu chỗ hoà là thứ nó dạy — lời nhắc cho người viết đã có trong script
workflow T1.5.
