# Mạch Realm 0 — từ chưa từng chạm lập trình tới ra lệnh được cho máy

> Tài liệu thiết kế. Nó quyết định **thứ tự** 40 bài của Realm 0 và, quan
> trọng hơn, **vì sao** thứ tự đó là thứ tự đúng. Sửa thứ tự thì sửa ở đây
> trước, sửa file bài học sau.

## Luật của mạch

Ba ràng buộc dưới đây là thứ phân biệt một mạch học với một mục lục:

1. **Mỗi bài đúng một khái niệm mới.** Hai khái niệm trong một bài nghĩa là
   người học không biết mình đang vấp cái nào khi vấp.
2. **Mỗi bài kết bằng một câu hỏi bỏ ngỏ, và bài kế tiếp trả lời nó.** Cột
   `reflect` không phải phần tóm tắt — nó là bản lề. Nếu bỏ một bài đi mà
   mạch vẫn liền, bài đó thừa.
3. **Công cụ mới chỉ xuất hiện sau khi bài trước đã tạo ra sự bất tiện mà nó
   giải quyết.** Không giới thiệu `for` vì "đến lượt vòng lặp trong sách", mà
   vì người học vừa phải gõ mười dòng `print` giống hệt nhau.

## Bảng mạch

| # | slug | Tiêu đề | Khái niệm mới (đúng một) | `reflect` cuối bài | Dựa trên |
|---|---|---|---|---|---|
| 1 | `byte-noi-gi` | Byte nói gì | *(đã viết)* máy chờ lệnh; `print` nói ra một câu chữ | Nếu viết `print(Xin chào)` — không dấu nháy — máy sẽ làm gì? | — |
| **2** | `chuong-trinh-la-gi` | Chương trình là gì | Chương trình = danh sách mệnh lệnh **được ghi lại**, chạy lại được khi bạn không ngồi đó | Tờ giấy hướng dẫn pha cà phê nằm trên bàn — ai làm nó "chạy"? Còn máy tính, lúc không ai bảo gì, nó đang làm gì? | 1 |
| **3** | `khi-khong-ai-bao-gi` | Khi không ai bảo, máy làm gì | Máy **chờ** — nó không đoán, không tự bổ sung ý bạn quên nói | Bạn tắt máy rồi bật lại. Lệnh bạn viết hôm qua nằm ở đâu mà máy tìm lại được? | 2 |
| **4** | `file-va-thu-muc` | Cái hộp có tên: file và thư mục | File = nội dung + một cái tên + một chỗ đứng (thư mục chứa nó) | Bên trong một file thực ra là gì? Bài hát, tấm ảnh và một câu chữ — chúng giống nhau ở điểm nào khi nằm trong máy? | 3 |
| **5** | `bit-va-byte` | Một ô chỉ chứa có hoặc không | Bit = ô nhỏ nhất, chỉ mang một trong hai trạng thái; tám bit đứng cạnh nhau gọi là một byte | Chữ "Phở" không phải 0 cũng không phải 1. Vậy nó nằm được trong những ô chỉ chứa 0 và 1 bằng cách nào? | 4 |
| **6** | `vi-sao-may-chi-hieu-so` | Vì sao máy chỉ hiểu số | Có một **bảng quy ước**: mỗi ký tự được gán một con số, nên chữ cũng là số | Nếu tất cả đều là số, thì lúc bạn gõ một câu cho máy, bạn gõ **ở đâu**? | 5 |
| **7** | `terminal-la-gi` | Terminal: chỗ nói chuyện với máy | Terminal = nơi bạn gõ một dòng, máy trả lời một dòng, luân phiên | Quay lại câu hỏi bỏ ngỏ ở bài 1: bỏ hai dấu nháy trong `print("Xin chào")` thì máy **thấy** gì? | 6 |
| **8** | `chu-va-ten` | Chữ và tên là hai thứ khác nhau | Trong nháy = chữ (đọc nguyên văn); không nháy = một **tên** máy phải đi tìm — tìm không ra thì `NameError` | `print(2 + 3)` — `2` và `3` không có nháy, cũng chẳng phải tên. Máy sẽ xử lý thế nào? | 7, 1 |
| **9** | `so-khong-can-nhay` | Số thì máy tính được | Số viết không nháy, và máy **tính toán** được với số (khác hẳn chữ) | Nếu đặt dấu `+` giữa hai câu chữ — `"Phở" + " bò"` — máy "cộng" kiểu gì? | 8 |
| **10** | `noi-hai-cau-chu` | Nối hai câu chữ | Dấu `+` giữa hai chuỗi là **ghép nối**, không phải cộng | Mỗi lần nhắc "Phở bò tái nạm" lại gõ nguyên câu thì mỏi tay. Có cách nào đặt tên cho một giá trị để gọi lại? | 9 |
| **11** | `dat-ten-cho-gia-tri` | Đặt tên cho một giá trị | Dấu `=` **dán một cái tên** lên một giá trị (biến) | Nếu dán chính cái tên đó lên một giá trị khác, giá trị cũ đi đâu? | 10 |
| **12** | `doi-gia-tri-cua-ten` | Đổi giá trị của một cái tên | Gán lại làm cái tên trỏ sang giá trị mới; giá trị cũ không còn ai gọi được | `print(mon_an)` — `mon_an` không có nháy. Theo bài 8, máy đi tìm một cái tên; lần này nó tìm **thấy**. Nó in ra chữ "mon_an" hay thứ bên trong? | 11 |
| **13** | `in-gia-tri-cua-ten` | In ra thứ mà cái tên đang giữ | Tên biến đặt trong `print` (không nháy) sẽ in ra **giá trị** nó đang giữ | `ten_quan = "Phở Thìn"` và `gia_pho = 45000`. Hai giá trị này khác loại nhau. Máy có phân biệt được không? | 12, 8 |
| **14** | `moi-gia-tri-co-mot-kieu` | Mỗi giá trị có một kiểu | **Kiểu** — mỗi giá trị thuộc một loại (chữ hay số), và `type()` cho bạn biết loại đó | `45000` chia cho 2 ra `22500`. Còn chia cho 4? Kết quả có còn là số nguyên? | 13 |
| **15** | `so-le-va-so-nguyen` | Khi con số có phần lẻ | Số thập phân (`float`) là một kiểu riêng — và phép chia luôn cho ra kiểu này | Nếu viết `"45000"` (có nháy) rồi `+ 5000`, máy sẽ ghép chuỗi hay cộng số? | 14 |
| **16** | `khi-hai-kieu-khong-hop` | Khi hai kiểu không đi cùng nhau | `TypeError` — máy không có quy ước nào để cộng một câu chữ với một con số, nên nó dừng và nói ra | Lỗi hiện ra năm sáu dòng chữ. Dòng nào là dòng đáng đọc nhất? | 15, 14 |
| **17** | `doc-thong-bao-loi` | Đọc thông báo lỗi | Traceback đọc **từ dòng cuối lên**: dòng cuối nói loại lỗi, dòng trên nói lỗi ở dòng số mấy | Mọi lỗi vừa rồi đều xảy ra khi chương trình **đang chạy**. Có loại lỗi nào máy phát hiện trước cả khi chạy dòng đầu tiên không? | 16 |
| **18** | `loi-truoc-khi-chay` | Lỗi máy thấy trước khi chạy | `SyntaxError` — thiếu ngoặc hoặc thiếu nháy khiến câu lệnh không đọc nổi, máy không chạy **dòng nào cả** | Đến giờ chương trình của bạn chỉ nói một mình. Làm sao để nó hỏi bạn một câu rồi **chờ** bạn trả lời? | 17 |
| **19** | `may-hoi-lai-ban` | Máy hỏi lại bạn | `input()` dừng chương trình, chờ người gõ, rồi đưa lại thứ vừa gõ | Bạn gõ `25` vào ô hỏi tuổi. Máy nhận được **con số** 25, hay hai ký tự `2` và `5`? | 18, 13 |
| **20** | `input-luon-tra-ve-chu` | Thứ bạn gõ luôn là chữ | `input()` luôn trả về chuỗi, kể cả khi bạn gõ toàn chữ số | Vậy muốn cộng thêm một tuổi cho người ta thì làm sao? | 19, 14 |
| **21** | `doi-chu-thanh-so` | Đổi chữ thành số | `int()` chuyển một chuỗi chữ số thành số thật, dùng tính toán được | Nếu người ta gõ "hai mươi lăm" thay vì "25", `int()` sẽ làm gì? | 20 |
| **22** | `khi-doi-kieu-that-bai` | Khi đổi kiểu không thành | `ValueError` — đúng loại việc, nhưng **nội dung** không hợp lệ (khác hẳn `TypeError`) | Bạn đã có tên và tuổi trong hai cái tên. Ghép chúng vào một câu chào tự nhiên mà không phải cộng chuỗi lằng nhằng — được không? | 21, 16, 17 |
| **23** | `chen-gia-tri-vao-cau` | Chèn giá trị vào giữa câu | f-string: đặt `{ten}` ngay trong câu chữ, máy thay bằng giá trị | Máy chào ai cũng đúng một câu. Làm sao để nó nói **khác đi** với người trên 18 tuổi? | 23←22, 10 |
| **24** | `dung-hay-sai` | Đúng hay sai | So sánh (`>`, `==`) cho ra một giá trị chỉ có hai trạng thái: `True` / `False` | Bạn có `True` trong tay rồi. Nhưng làm sao biến nó thành "chỉ chạy dòng này khi True"? | 23, 14 |
| **25** | `neu-thi` | Nếu... thì | `if` chạy phần thân **chỉ khi** điều kiện là `True` | Vì sao dòng bên trong `if` phải lùi vào? Không lùi thì máy hiểu sao? | 24 |
| **26** | `thut-dau-dong` | Thụt đầu dòng: máy biết đâu là "bên trong" | Thụt lề là cách máy xác định khối lệnh nào thuộc về `if` | `if` lo phần điều kiện đúng. Còn khi điều kiện sai, máy không làm gì cả sao? | 25 |
| **27** | `neu-khong-thi` | Còn không thì | `else` chạy khi và chỉ khi điều kiện của `if` là `False` | Quán phở có ba cỡ tô, ba mức giá. Hai nhánh có đủ không? | 26 |
| **28** | `nhieu-loi-re` | Nhiều hơn hai lối rẽ | `elif` kiểm tra lần lượt và **dừng ở nhánh đúng đầu tiên** | Muốn giảm giá cho người vừa là học sinh **vừa** đến trước 8 giờ — hai điều kiện phải cùng đúng. Viết thế nào? | 27 |
| **29** | `hai-dieu-kien-cung-dung` | Hai điều kiện cùng đúng | `and` chỉ cho `True` khi **cả hai** vế đều đúng | Còn khuyến mãi cho học sinh **hoặc** người trên 65 tuổi — chỉ cần một vế đúng. `and` có làm được không? | 28, 24 |
| **30** | `chi-can-mot-ve-dung` | Chỉ cần một vế đúng | `or` cho `True` khi **ít nhất một** vế đúng | In bảng giá 10 cỡ tô mà gõ 10 dòng `print` thì sao? Có cách nào bảo máy làm lại một việc nhiều lần? | 29 |
| **31** | `lam-lai-nhieu-lan` | Bảo máy làm lại | `for ... in range(n)` chạy cùng một khối lệnh `n` lần | Mỗi vòng in ra y hệt nhau. Có thứ gì trong vòng lặp thay đổi theo từng lượt không? | 30, 26 |
| **32** | `bien-doi-moi-vong` | Cái tên đổi giá trị mỗi vòng | Biến lặp mang một giá trị **khác nhau** ở mỗi lượt chạy | `range` cho ra 0, 1, 2... Nhưng thực đơn quán phở là "tái", "chín", "nạm" — không phải số. Chứa chúng ở đâu? | 31, 11 |
| **33** | `mot-cho-chua-nhieu-gia-tri` | Một chỗ chứa nhiều giá trị | `list` — một dãy giá trị **có thứ tự**, viết trong `[ ]` | Muốn lấy riêng món đầu tiên trong thực đơn thì gọi nó là món số mấy? | 32 |
| **34** | `dem-tu-khong` | Đếm từ 0 | Chỉ số bắt đầu từ `0`, nên món đầu tiên là `[0]` | Thực đơn 20 món mà viết 20 dòng lấy từng chỉ số thì mệt. Bài 31 dạy lặp — nối hai thứ đó lại được không? | 33 |
| **35** | `duyet-tung-mon` | Đi qua từng món | `for` lấy lần lượt **từng phần tử** của list, không cần chỉ số | Quán thêm món mới giữa buổi. Danh sách đã viết sẵn có thêm vào được không? | 34, 31 |
| **36** | `them-mon-vao-danh-sach` | Thêm vào danh sách | List **sửa được**: `.append()` gắn thêm một phần tử vào cuối | Đoạn tính tiền tô phở bạn đã chép lại ba lần ở ba chỗ. Có cách nào viết một lần rồi gọi lại? | 35, 33 |
| **37** | `dat-ten-cho-mot-viec` | Đặt tên cho một việc | `def` gói một nhóm lệnh dưới một cái tên; gọi tên thì cả nhóm chạy | Hàm chào của bạn lúc nào cũng chào "Lan". Làm sao chào đúng tên người đang đứng trước máy? | 36, 11, 26 |
| **38** | `gui-thong-tin-vao-ham` | Gửi thông tin vào hàm | Tham số — chỗ trống trong hàm, được điền vào lúc gọi | Hàm tính tiền **in ra** kết quả. Nhưng muốn lấy con số đó đem cộng tiếp thì sao? | 37 |
| **39** | `ham-tra-ket-qua-ve` | Hàm đưa kết quả ra ngoài | `return` trả một giá trị **về chỗ gọi** — khác `print` chỉ hiện lên màn hình | Bạn đã có đủ: hỏi, rẽ nhánh, lặp, danh sách, hàm. Ghép lại thành một chương trình biết trả lời được không? | 38, 13 |
| **40** | `boss-may-tra-loi-tu-dong` | BOSS: Máy trả lời tự động | *(không khái niệm mới — bài tổng hợp)* ghép `input` → `int` → `if/elif/else` → `list` → `def/return` thành một chương trình hỏi–đáp chạy được | Chương trình của bạn quên sạch mọi thứ khi tắt. Làm sao để nó nhớ được sổ chi tiêu của hôm qua? *(dẫn sang Realm 1)* | 19–39 |

---

**Vì sao thứ tự này là thứ tự đúng**

1. Sáu bài không code (2–7) đi từ *cái người học đã thấy* (một tờ giấy hướng dẫn, một cái hộp có tên) xuống dần tới *cái người học chưa từng thấy* (bit, bảng mã, terminal) — mỗi bậc trả lời đúng câu hỏi mà bậc trước vừa để hở, nên tới bài 8 người học đã có sẵn hai ý niệm bắt buộc để hiểu dấu nháy: "máy chỉ chứa số" và "chữ chỉ là số theo một bảng quy ước".

2. Toàn bộ Module 1 xoay quanh **một trục duy nhất — máy phân biệt CHỮ với TÊN như thế nào** (bài 8 nêu, bài 9–10 làm rõ bằng số và phép nối, bài 11–13 biến "tên" thành biến, bài 14–15 đặt tên cho chính sự phân biệt đó là "kiểu", bài 16–18 cho thấy điều gì xảy ra khi trục này bị vi phạm); nhờ vậy ba bài lỗi cuối module không phải chủ đề mới mà là *hệ quả* của thứ vừa học — lỗi đến đúng lúc người học đủ hiểu để đọc nó.

3. Module 2 chỉ mở ra khi người học đã có đủ ba thứ để tiếp nhận nó: `input` cần hiểu kiểu (bài 20–22 tái dùng nguyên `TypeError`/`ValueError` của bài 16–17), `if` cần một giá trị đúng/sai *đã có sẵn từ phép so sánh* (24 trước 25), `for` cần biến (32 dựa bài 11) và `list` cần chỉ số trước khi được duyệt (34 trước 35) — mỗi công cụ mới xuất hiện đúng lúc bài trước vừa tạo ra sự bất tiện mà nó giải quyết, chứ không xuất hiện vì "đến lượt nó trong sách".

**Một lưu ý cần quyết định trước khi viết:** bài 1 hiện có code (`print`), nên "6 bài đầu không code" đang rơi vào bài 2–7. Hai lựa chọn: (a) coi bài 1 là bài *nếm thử* — người học chỉ điền một chỗ trống, chưa gọi là học code, và ràng buộc tính từ bài 2; hoặc (b) chuyển bài 1 xuống vị trí 8, khi đó `reflect` của nó nối thẳng vào bài `chu-va-ten`. Mạch trên viết theo (a), và bài 7 được thiết kế để *trả lại* lời hứa mà `reflect` bài 1 đã đưa ra ("bài sau bạn sẽ thử") — nếu chọn (a) thì cần sửa một câu trong `reflect` bài 1 thành "cuối module này bạn sẽ thử".
