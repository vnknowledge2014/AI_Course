# Mạch Realm 2 — Toán & Toán rời rạc

> Tài liệu thiết kế. Nó quyết định **thứ tự** các bài của Realm 2 và, quan
> trọng hơn, **vì sao** thứ tự đó là thứ tự đúng. Sửa thứ tự thì sửa ở đây
> trước, sửa file bài học sau.

Realm này đi **song song** với Realm 1, không nối tiếp. Người học có thể vào
đây khi chưa biết vòng lặp hay hàm — nên không bài nào được giả định điều đó.
Python chỉ dùng để KIỂM một ý tưởng, không phải chủ đề.

## Phong cách: New Math kết hợp Common Core

Ba điều phân biệt nó với cách dạy toán quen thuộc:

1. **Cấu trúc trước quy trình.** Dạy phép nhân là gì trước khi dạy cách nhân.
   Người học thuộc quy trình mà không hiểu cấu trúc sẽ đứng hình ngay lần đầu
   bài toán đổi hình dạng.
2. **Nhiều biểu diễn cho cùng một ý.** Thanh số, mô hình vùng, sơ đồ dải —
   mỗi cái làm lộ ra một mặt khác nhau. Người chỉ có một biểu diễn thì chỉ
   giải được những bài trông giống biểu diễn đó.
3. **Luôn hỏi "vì sao đúng", không chỉ "làm thế nào".** Đây là chỗ nối thẳng
   sang T2.3 (Logic & chứng minh) và sang mọi thứ ở Realm 3.

## Luật của mạch

Ba ràng buộc giống hệt các realm khác:

1. **Mỗi bài đúng một khái niệm mới.**
2. **Mỗi bài kết bằng một câu hỏi bỏ ngỏ, và bài kế tiếp trả lời nó.**
3. **Công cụ mới chỉ xuất hiện sau khi bài trước đã tạo ra sự bất tiện mà nó
   giải quyết.**

## T2.1 — Cảm nhận số (Realm 2 · Toán & Toán rời rạc · Python dùng để KIỂM · 44 bài)

## T2.1 — Cảm nhận số (Realm 2 · Toán & Toán rời rạc · Python chỉ dùng để KIỂM · 44 bài)

> Hiện vật chạy suốt track: **vườn của Byte** — nắm hạt (đếm được, không bẻ nhỏ được) và luống đất đo bằng sải dây (đo được, bẻ nhỏ được). Mặt đất là mốc 0; rễ ở dưới, mầm ở trên. Mọi sự bất tiện tích luỹ trên đúng một cái vườn ấy.

| # | slug | Tiêu đề | Khái niệm mới (đúng một) | `reflect` cuối bài | Dựa trên |
|---|---|---|---|---|---|
| 1 | `mot-so-luon-la-may-cai-gi` | Mấy — nhưng mấy *cái gì* | Một con số không đứng một mình: nó luôn là câu trả lời cho "mấy **cái** gì". Bỏ cái đơn vị đi thì con số chưa nói được điều gì | Byte đếm đống hạt ra 12. An đếm lại, không nhìn Byte, cũng ra 12. Cái gì bảo đảm chuyện đó — sao không thể ra 11 hay 13? | R0.9 `so-khong-can-nhay` |
| 2 | `dem-la-ghep-doi-mot-mot` | Đếm là ghép đôi | Đếm = ghép **mỗi vật với đúng một** tên số theo dãy 1, 2, 3…; vì là ghép một–một nên đếm theo thứ tự nào cũng ra cùng một kết quả | Đếm hạt thì ai cũng ra 12. Nhưng đo luống đất, Byte bảo "dài 4", An bảo "dài 12" — và không ai đo sai cả. Chuyện gì đã khác? | 1 |
| 3 | `do-la-dem-cai-thuoc` | Đo là đếm cái thước | Số đo = đếm xem **cái thước đã chọn** đặt lặp lại mấy lần; đổi thước thì con số đổi, còn lượng đất thì không đổi tí nào | Vậy đếm hạt cũng là đo, với thước là "một hạt"? Nếu hai chuyện là một, thử bẻ đôi cái thước xem: nửa sải dây vẫn là dây, còn nửa hạt là gì? | 2, 1 |
| 4 | `thuoc-be-nho-duoc-hay-khong` | Thước bẻ được và thước không bẻ được | Chỗ khác nhau **thật** giữa số đếm và số đo nằm ở đơn vị: đơn vị rời bẻ nhỏ là mất nghĩa, đơn vị đo bẻ nhỏ vẫn còn nghĩa — nên giữa hai số đo luôn còn chỗ trống | "Giữa 3 và 4 vẫn còn chỗ" — nghe thì gật đầu, nhưng *chỗ* ấy ở đâu? Phải vẽ ra bức tranh nào mới chỉ tay vào nó được? | 3 |
| 5 | `thanh-so-moi-so-mot-cho` | Thanh số: mỗi số một chỗ | **Thanh số** — mọi số nằm trên một đường thẳng, hai vạch liền nhau cách nhau đúng một đơn vị; số là một **chỗ**, không phải một đống | Trên thanh số, muốn chỉ ra 137 hạt thì phải đếm qua 137 vạch. Có cách nào nói "137" mà không phải đếm 137 lần không? | 4, 2 |
| 6 | `gom-muoi-thanh-mot-bo` | Gom mười thành một bó | **Đóng gói** — gom 10 vật rời thành một bó rồi đi đếm bó; bó là một đơn vị mới, to gấp mười đơn vị cũ | Byte có 13 bó và 7 hạt lẻ. Viết ra giấy thế nào cho gọn — chẳng lẽ vẽ 13 cái bó? Mà viết "13" với "7" cạnh nhau thì ai biết đâu là bó, đâu là hạt? | 5, 2 |
| 7 | `cho-dung-noi-gia-tri` | Chỗ đứng nói giá trị | **Giá trị theo vị trí** — cùng chữ số 3, đứng cột khác thì mang lượng khác (3 hạt / 3 bó / 3 bó-của-bó); vị trí làm thay việc phải viết tên đơn vị ra | Byte có 3 bó-của-bó, không bó lẻ nào, và 7 hạt. Cột giữa chẳng có gì để viết. Bỏ trống rồi viết "37" à? | 6 |
| 8 | `so-khong-giu-cho` | Số 0 giữ chỗ | `0` không mang nghĩa "không có gì", nó mang nghĩa "**cột này rỗng**" — nhờ nó mà 307 khác hẳn 37 | Cột nào cũng phải đủ **mười** mới được lên bó. Vì sao lại là mười? Ai chọn con số đó, và chọn số khác có được không? | 7 |
| 9 | `vi-sao-lai-la-muoi` | Vì sao lại là mười | **Cơ số là một lựa chọn**, không phải bản chất của số: bó theo 5 thì vẫn đúng chừng ấy hạt nhưng dãy chữ số khác hẳn; 10 đến từ mười ngón tay | Bó theo 10 hay theo 5 thì đống hạt vẫn nguyên chừng ấy. Nhưng Byte vừa hái thêm một đống nữa và đổ chung vào. "Đổ chung" — trong toán gọi là gì, và nó làm gì với mấy cái bó? | 8, 6, R0.5 `bit-va-byte` |
| 10 | `cong-la-gop-hai-dong` | Cộng là gộp hai đống | **Cộng = gộp** hai lượng thành một lượng; và vì chỉ là đổ chung vào nhau nên gộp đống nào trước cũng ra một kết quả | 3 hạt gộp với 2 bó ra "5" — 5 **cái gì**? Câu hỏi này có trả lời được không? | 9, 1 |
| 11 | `chi-gop-duoc-thu-cung-don-vi` | Chỉ gộp được thứ cùng đơn vị | Phép cộng chỉ có nghĩa khi hai lượng **cùng một đơn vị** — đó mới là lý do người ta viết thẳng cột, chứ không phải cho đẹp mắt | 7 hạt cộng 5 hạt là 12 hạt. Nhưng cột hạt lẻ chỉ chứa nổi tới 9. Cái thứ mười đi đâu? | 10, 7 |
| 12 | `nho-mot-la-dong-bo-lai` | "Nhớ 1" chính là đóng bó | "Nhớ 1" không phải mẹo tính nhẩm: nó là **đóng gói lại** — đủ mười cái lẻ thì thành một bó, và cái bó ấy sang đứng ở cột bên trái | Tới giờ cộng vẫn là chuyện của hai *đống*. Trên thanh số ở bài 5 — nơi một số là một *chỗ* chứ không phải một đống — thì "cộng 3" trông ra sao? | 11, 6 |
| 13 | `cong-la-buoc-sang-phai` | Cộng là bước sang phải | Trên thanh số, cộng = **dịch sang phải** đúng bấy nhiêu đơn vị; một phép cộng, hai bức tranh (gộp đống / dời chỗ) và chúng luôn khớp nhau | Dịch sang phải là cộng. Vậy dịch sang **trái** là phép gì, và nó trả lời câu hỏi nào của Byte? | 12, 5 |
| 14 | `bot-di-la-lui-lai` | Bớt đi là lùi lại | **Trừ = bớt đi** = dịch sang trái trên thanh số | "12 bớt 5" và "từ 5 tới 12 còn cách bao xa" nghe như hai câu hỏi chẳng dính gì nhau, thế mà cùng ra 7. Vì sao? | 13 |
| 15 | `tru-la-khoang-cach` | Trừ là khoảng cách | `a − b` = **khoảng cách từ b tới a** trên thanh số — cũng đúng là "phải cộng thêm bao nhiêu vào b thì tới a" | Từ 5 tới 12 là 7 bước sang phải. Từ 12 tới 5 cũng 7 bước, nhưng ngược chiều. Viết `5 − 12` thì bạn phải lùi qua bên trái số 0. Bên trái số 0 có gì? | 14, 13 |
| 16 | `ben-trai-so-khong` | Bên trái số 0 | **Số âm** — thanh số chạy tiếp về bên trái 0; 0 là một **mốc do người chọn** (mặt đất của vườn), không phải điểm cuối của thế giới | −5 và 5 cách 0 bằng nhau, chỉ khác phía. Vậy giữa −5 và −2, cái nào lớn hơn? "5 lớn hơn 2" có kéo theo "−5 lớn hơn −2" không? | 15, 5 |
| 17 | `lon-hon-la-dung-ben-phai` | Lớn hơn nghĩa là đứng bên phải | **So sánh = vị trí trên thanh số**: `a > b` khi a đứng bên phải b — nên −2 > −5, dù rễ sâu 5 phân thì "sâu hơn" rễ 2 phân | `5 − 12` giờ đã có câu trả lời: −7. Nhưng nếu phép trừ nào cũng lùi được, thì trừ có còn là một phép riêng nữa không, hay nó là phép cộng đội lốt? | 16, 13, R0.24 `dung-hay-sai` |
| 18 | `tru-la-cong-so-doi` | Trừ là cộng với số đối | Mỗi số có một **số đối** (cùng khoảng cách tới 0, khác phía), và `a − b = a + (−b)` — phép trừ tan vào phép cộng, không cần luật riêng | Cộng thì gộp từng ít một. "Mỗi luống 8 cây, có 5 luống" — cộng 8 năm lần thì xong. Nhưng 50 luống thì cộng 50 lần? | 17, 16 |
| 19 | `may-lan-mot-lo` | Mấy lần một lô | **Nhân = mấy lần một lượng**; hai con số đứng hai vai khác nhau — một cái nói *lô to bao nhiêu*, cái kia nói *lấy mấy lô* | 5 luống mỗi luống 8 cây, và 8 luống mỗi luống 5 cây — hai cái vườn trông khác hẳn nhau. Số cây có khác nhau không? | 18, 10 |
| 20 | `mang-chu-nhat-xoay-mot-goc` | Mảng chữ nhật xoay một góc | Xếp cây thành **mảng chữ nhật** hàng × cột: xoay 90° là đổi vai hai con số mà không thêm bớt cây nào → `a × b = b × a` | Mảng 7 hàng × 13 cột. Đếm từng cây thì 91 lần. Cắt cái mảng ấy thành mấy miếng dễ đếm hơn được không? | 19, 5 |
| 21 | `cat-mang-thanh-mieng` | Cắt mảng thành miếng | **Mô hình vùng và tính chất phân phối**: cắt 13 cột thành 10 + 3 thì `7×13 = 7×10 + 7×3`; cắt kiểu nào cũng ra đúng chừng ấy cây | Bạn vừa nhân 7 với 13 mà không cộng 13 lần — và cái cột "10" trong phép cắt đúng là cái bó ở bài 6. Nhưng "nhân" còn một nghĩa nữa: "luống này dài **gấp 3** luống kia". Ở đó thì đâu là hàng, đâu là cột? | 20, 6, 7 |
| 22 | `nhan-la-keo-gian-thanh-so` | Nhân là kéo giãn thanh số | **Nhân như co giãn**: nhân 3 = kéo cả thanh số ra xa mốc 0 ba lần = **đo lại đúng lượng ấy bằng cái thước nhỏ đi ba lần** (bài 3 quay lại) | Kéo giãn 3 lần thì mọi chỗ ra xa hơn; nhân 1 thì không đổi gì. Vậy nhân với −1 làm gì với thanh số? | 21, 3, 5 |
| 23 | `nhan-am-la-lat-thanh-so` | Nhân số âm là lật thanh số | Nhân với −1 = **lật** thanh số quanh 0; lật hai lần thì mọi chỗ về đúng chỗ cũ — nên `(−1)×(−1) = 1` là hệ quả nhìn thấy được, không phải luật học thuộc | Kéo giãn ba lần là `×3`. Kéo giãn ba lần liên tiếp thì viết `3 × 3 × 3`. Kéo mười lần thì phải viết mười con 10 cạnh nhau — mỏi tay chưa? | 22, 16, 18 |
| 24 | `viet-gon-phep-nhan-lap-lai` | Viết gọn phép nhân lặp | **Luỹ thừa** — `10⁵` nghĩa là 10 nhân với chính nó, **năm thừa số**; số mũ đếm *số lần thừa số xuất hiện*, chứ không phải kết quả | 10¹ = 10, 10² = 100, 10³ = 1000. Nhìn quen chưa? Đó đúng là mấy cái cột ở bài 7. Thế cột hạt lẻ — cột "một" — là 10 mũ mấy? | 23, 19 |
| 25 | `bang-vi-tri-la-day-luy-thua` | Bảng vị trí là dãy luỹ thừa | Mỗi bước sang **trái** trong bảng vị trí là nhân 10, nên các cột chính là 10⁰, 10¹, 10², …; và `10⁰ = 1` không phải quy ước tuỳ tiện mà là hệ quả của việc đi ngược lại đúng một bước | Đi ngược thêm một bước nữa thì số mũ tụt xuống dưới 0, và mỗi bước ngược là **chia** cho 10. Nhưng khoan — suốt 24 bài Byte chưa chia lần nào. Chia là gì? | 24, 7, 9 |
| 26 | `chia-deu-cho-may-phan` | Chia đều cho mấy phần | **Chia đều** — biết *số phần*, đi tìm *cỡ mỗi phần*: 12 hạt cho 3 luống thì mỗi luống mấy hạt | "12 hạt chia cho 3 luống" và "12 mét dây cắt thành từng đoạn 3 mét" đều viết `12 : 3`. Nhưng con số 3 đứng ở hai vai khác nhau. Khác chỗ nào? | 25, 19 |
| 27 | `chia-la-do-xem-lot-may-lan` | Chia là đo xem lọt mấy lần | **Chia như đo** — biết *cỡ mỗi phần*, đi tìm *số phần*: "3 mét lọt vào 12 mét mấy lần"; đây đúng là câu hỏi *đo* của bài 3, chỉ viết bằng dấu chia | Hỏi "3 lọt vào 12 mấy lần" thì trả lời được. Hỏi "**0** lọt vào 12 mấy lần" thì trả lời sao? | 26, 3 |
| 28 | `vi-sao-khong-chia-cho-khong` | Vì sao không chia được cho 0 | `12 : 0` không có kết quả vì **câu hỏi hỏng**, không phải vì máy cấm: đặt đoạn 0 mét bao nhiêu lần cũng không lấp nổi 12 mét — và không có số nào nhân với 0 ra 12 | Còn 13 mét cắt thành đoạn 3 mét thì được 4 đoạn và **thừa 1 mét**. Chỗ thừa ấy — gọi nó bằng một con số được không? | 27 |
| 29 | `phan-con-thua` | Phần còn thừa | **Số dư** — phần còn lại vì *chưa đủ thêm một lần nữa*; nên dư luôn nhỏ hơn cái thước đang đo | 1 mét thừa ấy, tính theo "đoạn 3 mét" thì là bao nhiêu đoạn? Chưa được một đoạn, nhưng cũng không phải không có gì. Trên thanh số nó đứng ở đâu? | 28, 27 |
| 30 | `be-nho-cai-thuoc` | Bẻ nhỏ cái thước | Khi thước không vừa thì **bẻ đơn vị thành b phần bằng nhau**; một phần trong đó là một **đơn vị mới**, và tên của nó là `1/b` | Vậy `1/4` là một cái thước mới, nhỏ đi bốn lần. Thế `3/4` là gì: ba cái bánh mỗi cái chia tư, hay ba lần cái thước mới ấy? | 29, 4, 3 |
| 31 | `phan-so-la-may-lan-don-vi-moi` | Phân số là mấy lần đơn vị mới | `a/b` = **a bản sao của đơn vị `1/b`** — mẫu nói *thước cỡ nào*, tử nói *lấy mấy cái*; nhờ thế phân số là một **chỗ** trên thanh số, một con số đàng hoàng chứ không phải "một phần của cái bánh" | 3/4 nằm giữa 0 và 1. Thế 7/4 thì sao — lấy 7 cái thước cỡ 1/4, có chỗ nào cho nó không, hay phân số bị nhốt trong khoảng 0–1? | 30, 5, 19 |
| 32 | `phan-so-khong-bi-nhot-duoi-mot` | Phân số không bị nhốt dưới 1 | Không có gì cấm tử lớn hơn mẫu: `7/4 = 4/4 + 3/4 = 1 và 3/4` — mẫu **không phải cái bánh**, mẫu chỉ là cỡ thước | Đặt 1/2 và 2/4 lên thanh số thì chúng rơi trúng cùng một chỗ. Hai cách viết, một con số — chuyện gì đang xảy ra? | 31, 10 |
| 33 | `doi-thuoc-khong-doi-luong` | Đổi thước, không đổi lượng | **Phân số tương đương** — nhân cả tử lẫn mẫu với cùng một số là *đo lại đúng lượng ấy bằng thước nhỏ hơn bấy nhiêu lần*; chia cả hai xuống là đo bằng thước to hơn (rút gọn). Đây là bài 3 nói lại bằng phân số | Cùng một lượng viết được vô số cách. Vậy làm sao biết 2/3 với 3/4 bên nào lớn hơn, khi hai bên đang đo bằng hai cái thước khác nhau? | 32, 3, 21 |
| 34 | `muon-so-sanh-thi-cung-thuoc` | Muốn so sánh thì phải cùng thước | So sánh phân số = **quy về cùng một thước** (mẫu chung) rồi đếm xem bên nào nhiều đơn vị hơn — chứ không so tử với tử, mẫu với mẫu | Cùng thước thì so được. Vậy cộng thì sao — `1/2 + 1/3` bằng bao nhiêu, và vì sao **không** phải `2/5`? | 33, 17 |
| 35 | `cong-phan-so-van-la-luat-cu` | Cộng phân số vẫn là luật cũ | Cộng phân số chỉ làm được khi **cùng mẫu**, vì đó vẫn đúng là luật bài 11: chỉ gộp được thứ cùng đơn vị. Cộng tử, giữ nguyên mẫu — vì mẫu là *tên đơn vị*, không phải một lượng đem cộng | Mỗi lần cộng lại phải đi tìm mẫu chung thì mệt. Có loại phân số nào mà mẫu **luôn sẵn giống nhau**, khỏi quy đồng lần nào không? | 34, 11, 12 |
| 36 | `bang-vi-tri-keo-sang-phai` | Bảng vị trí kéo sang phải | **Số thập phân là phân số cơ số 10**: kéo bảng giá trị vị trí sang bên phải dấu phẩy thì các cột là 1/10, 1/100 = 10⁻¹, 10⁻² — nên `0,25` chính là `25/100`, mẫu do vị trí lo, khỏi phải viết ra | `1/4` viết được thành `0,25`. Còn `1/3` — bẻ thước ra ba phần — viết thành thập phân thì ra số nào? | 35, 25, 7 |
| 37 | `phan-so-nao-thanh-thap-phan-duoc` | Phân số nào viết được thành thập phân | Chỉ phân số **quy đồng được về mẫu 10ⁿ** mới có thập phân dừng lại được; `1/3 = 0,333…` không bao giờ dừng, vì bẻ 10 thành 3 phần bằng nhau thì lần nào cũng dư | Mẫu 10, 100, 1000 tiện vì lúc nào cũng thẳng cột. Nhưng khi cần so vườn 40 cây với vườn 250 cây, người ta lại chỉ dùng **đúng một** mẫu duy nhất. Mẫu nào, và vì sao lại là nó? | 36, 33, 29 |
| 38 | `mot-cai-thuoc-cho-tat-ca` | Một cái thước cho tất cả | **Phần trăm** — chốt cứng mẫu ở 100 (`1% = 1/100`) để mọi lượng, dù to nhỏ khác nhau, đều so được với nhau bằng một con số duy nhất | 30% vườn nhà Byte và 30% vườn nhà An — cùng 30%, mà số cây khác hẳn. Vậy con số 30 ấy còn thiếu điều gì mới đủ nghĩa? | 37, 34, 33 |
| 39 | `phan-tram-cua-cai-gi` | Phần trăm *của* cái gì | Mỗi phần trăm gắn chặt với một **cái toàn thể** = 100%; đổi cái toàn thể thì cùng một con số phần trăm mang lượng khác — nên "tăng 50% rồi giảm 50%" **không** quay về chỗ cũ | Phần trăm luôn so một phần với **cái toàn thể chứa nó**. Còn khi so số cây với số luống — hai thứ chẳng cái nào nằm trong cái nào — thì so kiểu gì? | 38, 22 |
| 40 | `ti-so-so-hai-thu-khac-loai` | Tỉ số: so hai thứ không cùng loại | **Tỉ số** `a : b` = "cứ a cái này thì có b cái kia" — không đòi cái nào phải chứa cái nào, và **không đổi khi cả hai cùng nhân lên** (đúng luật bài 33, chỉ khác chỗ dùng) | 90 hạt trên 6 luống, và 75 hạt trên 5 luống — hai tỉ số viết ra trông chẳng giống nhau. Làm sao biết vườn nào gieo dày hơn? | 39, 33, 27 |
| 41 | `quy-ve-mot-don-vi` | Quy về một đơn vị | **Tỉ lệ** — chia để tạo ra một **đơn vị ghép**: "mấy hạt **trên một** luống"; quy cả hai vườn về cùng một đơn vị ghép thì so sánh được ngay | Bạn vừa viết `90 : 6` và đọc là "15 hạt trên một luống". Cũng dấu ấy, bài 27 đọc là "6 lọt vào 90 mấy lần", bài 26 đọc là "chia 90 cho 6 phần". Một ký hiệu, ba cách đọc — vậy ký hiệu toán rốt cuộc ghi lại **cái gì**? | 40, 27, 3 |
| 42 | `vi-sao-nhan-truoc-cong` | Vì sao nhân trước cộng | **Thứ tự phép toán không phải quy ước tuỳ tiện**: `3 + 4 × 5` là cách viết lại câu "3 cây lẻ, và 4 luống mỗi luống 5 cây" — phép nhân **gói** một lượng thành một khối rồi mới đem gộp, nên nó phải xong trước; tính từ trái sang thì cái gói biến mất | Nhưng nếu ý bạn đúng là "gộp 3 với 4 trước, rồi mới nhân 5"? Cái ý ấy có thật và đếm được ngoài vườn — chỉ là ký hiệu chưa nói ra nổi. | 41, 19, 10, 24 |
| 43 | `ngoac-va-nhung-cai-ngoac-an` | Dấu ngoặc và những cái ngoặc ẩn | `( )` là chỗ bạn **nói thẳng cấu trúc** thay vì để thứ tự mặc định nói hộ — và bạn đã dùng ngoặc từ lâu mà không biết: gạch phân số và số mũ đều **là ngoặc sẵn** | Giờ Byte đọc được mọi biểu thức trong track. Vườn có 6 luống, mỗi luống 15 hạt, 20% chết, phần sống chia vào các ô rộng 3/4 mét — một câu hỏi duy nhất mà phải dùng lại gần như mọi bài đã học. Viết ra được không? | 42, 24, 31 |
| 44 | `boss-vuon-cua-byte` | BOSS: Vườn của Byte | *(không khái niệm mới — bài tổng hợp)* một bài toán vườn đi qua đủ mạch: đơn vị & thước → giá trị vị trí → cộng/trừ có số âm → mảng & co giãn → chia đều/chia đo & dư → phân số tương đương → thập phân → phần trăm → tỉ lệ → thứ tự phép toán; mỗi bước chốt bằng một dòng Python so hai vế | Mọi con số trong bài đều là số cụ thể. Nhưng sang năm Byte trồng **n** luống, mỗi luống **h** hạt — chưa biết n và h bằng bao nhiêu mà vẫn muốn viết sẵn công thức. Đặt tên cho một con số **chưa biết** thì viết thế nào? *(dẫn sang T2.2 — Đại số & hàm số)* | 1–43 |

**Vì sao thứ tự này đúng**

**Vì sao thứ tự này là thứ tự đúng**

**0. Một trục duy nhất: ĐƠN VỊ.** Cả 44 bài chỉ có một trục — *một con số là mấy lần một cái thước đã chọn*. Bài 1–5 dựng cái trục ấy; bài 6–9 cho thấy chữ số viết ra chỉ là cách ghi lại việc **đóng gói đơn vị**; bài 10–18 là các phép toán đọc theo trục ("chỉ gộp được thứ cùng đơn vị"); bài 19–25 là chỗ đơn vị **đổi cỡ** (nhân = đo bằng thước nhỏ hơn, luỹ thừa = đổi cỡ nhiều lần); bài 26–37 là chỗ đơn vị **bẻ nhỏ** (chia → dư → phân số → thập phân); bài 38–41 là chỗ đơn vị được **chốt cứng** (phần trăm) hoặc **ghép lại** (tỉ lệ); bài 42–43 là chỗ ký hiệu phải nói ra cái cấu trúc ấy. Nhờ một trục duy nhất, mọi khái niệm sau đều là câu cũ nói lại chỗ mới, chứ không phải chương mới trong sách. Và mọi bài chạy trên đúng **một hiện vật** — vườn của Byte — nên sự bất tiện *tích luỹ trên một vật*, không tan đi mỗi bài một ví dụ.

**1. Vì sao "số đếm vs số đo" phải là bốn bài đầu chứ không phải một câu định nghĩa.** Đây là nơi mọi khó khăn về sau được gieo mầm. Người học phải tự tay chạm vào chuyện "cùng một luống, hai người ra hai con số, không ai sai" (bài 2→3) thì mới chấp nhận được rằng **mẫu số là cỡ thước** (bài 31) chứ không phải cái bánh, và rằng `1/2 = 2/4` là đổi thước chứ không phải mẹo nhân chéo (bài 33). Đặt "phân số là gì" ở giữa track mà không có bốn bài này ở đầu thì lại rơi vào đúng cái bẫy sách giáo khoa hay mắc: dạy phân số như "phần của một cái bánh", rồi tới `7/4` là gãy.

**2. Vì sao thanh số ở bài 5, sớm hơn hầu hết chương trình.** Vì thanh số là công cụ trả lời câu hỏi bài 4 để hở ("giữa 3 và 4 còn chỗ — chỗ ở đâu?"), và vì sáu chỗ về sau đều sống nhờ nó: cộng là dịch phải (13), trừ là khoảng cách (15), số âm là bên trái 0 (16), so sánh là bên phải (17), nhân là kéo giãn (22), phân số là một **chỗ** chứ không phải một miếng (31). Dạy thanh số muộn thì sáu bài kia phải mỗi bài tự dựng lấy một bức tranh riêng — và người học không bao giờ thấy chúng là **cùng một** bức tranh.

**3. Vì sao "vì sao lại là mười" nằm ngay sau số 0 giữ chỗ (bài 8→9), không nằm cuối.** Bài 6–8 vừa bắt người học sống trong luật "đủ mười thì lên bó" ba bài liền; đó đúng là lúc câu hỏi "sao lại mười?" tự bật ra. Trả lời nó bằng cách **bó theo 5** cho thấy 10 là một lựa chọn của loài có mười ngón tay — và người học nhận ra bảng giá trị vị trí là một *bộ máy*, không phải một sự thật. Đặt bài này ở cuối track thì nó thành chuyện vui bên lề; đặt ở đây thì nó là bản lề cho bài 25 (cột = luỹ thừa của 10), bài 36 (thập phân = phân số mẫu 10ⁿ), bài 38 (phần trăm = chốt mẫu ở 100), và mở sẵn cửa sang R3.T1 (bit/byte).

**4. Vì sao số âm đứng ở bài 16–18, trước cả phép nhân — ngược với thông lệ.** Luật thứ ba của mạch quyết định chỗ này: sự bất tiện xuất hiện ở bài 15 chứ không ở đâu khác. Ngay khi trừ được đọc là **khoảng cách có hướng**, `5 − 12` đòi một chỗ đứng bên trái số 0 — không thể hoãn câu hỏi đó lại mười lăm bài để chờ "đến lượt số âm". Và đẩy số âm lên sớm trả lãi ngay ba lần: (a) bài 18 làm phép trừ **biến mất** như một phép riêng, `a − b = a + (−b)`, nên về sau không có luật trừ nào phải học thêm; (b) bài 22–23 giải thích được `(−1)×(−1) = 1` bằng *lật thanh số hai lần*, tức là bằng một bức tranh nhìn thấy được, thay vì bằng bảng dấu học thuộc — đây là bài trả lời "vì sao đúng" đắt giá nhất track; (c) bài 25 có sẵn số mũ âm để bài 36 kéo bảng vị trí sang phải.

**5. Vì sao luỹ thừa chen vào giữa nhân và chia (24–25), chứ không xếp cuối cùng.** Luỹ thừa sinh ra từ đúng một sự mỏi tay có thật: bài 23 vừa bắt viết `10 × 10 × 10 × 10 × 10`. Nhưng lý do quan trọng hơn là bài 25 **quay lại giải thích cái người học đã dùng suốt 18 bài**: các cột trong bảng giá trị vị trí chính là 10⁰, 10¹, 10² — một khoảnh khắc "à ra thế" chỉ xảy ra được khi bảng vị trí đã cũ và luỹ thừa còn mới. Và chính bài 25 **sinh ra nhu cầu chia**: đi ngược một bước trong bảng là chia cho 10, mà tới lúc đó Byte chưa chia lần nào. Nếu để luỹ thừa ở cuối track thì cả hai lợi ích này mất sạch, và phép chia phải tự mở màn không lý do.

**6. Vì sao chia có hai bài riêng (26, 27) và vì sao bài 27 mới là bài đắt.** "Chia đều" là nghĩa ai cũng có sẵn; "chia như đo" — *cái này lọt vào cái kia mấy lần* — là nghĩa gần như không ai được dạy, mà lại là nghĩa duy nhất giải thích nổi ba chuyện về sau: vì sao không chia được cho 0 (28), vì sao có số dư (29), và vì sao chia phân số về sau lại là "lọt mấy lần". Tách hai nghĩa thành hai bài là cách duy nhất để người học biết mình đang vấp nghĩa nào. Bài 28 (chia cho 0) là mắt xích ngắn nhất trong mạch nhưng vẫn là mắt xích: `reflect` của 27 hỏi thẳng nó, và `reflect` của nó giao lại con số dư cho bài 29.

**7. Vì sao phân số nhận sáu bài, và vì sao đúng thứ tự đó.** Sáu bài này là trọng tâm của track, và chúng xếp theo đúng chuỗi hiểu lầm mà người học thật sự mắc: dư → phải bẻ thước (30) → `a/b` là *a lần cái thước `1/b`* chứ không phải "một miếng bánh" (31) → nên `7/4` có chỗ đàng hoàng (32) → nên hai cách viết cùng một chỗ là chuyện đổi thước (33) → nên so sánh phải quy về cùng thước (34) → nên cộng cũng thế, và đó vẫn là luật bài 11 (35). Bài 35 là chỗ trục đơn vị đóng vòng: người học không học một luật cộng phân số mới, họ chỉ **áp lại** cái luật đã có từ bài 11. Bỏ bài 30 thì 31 mất chỗ dựa; bỏ 32 thì hiểu lầm "phân số luôn bé hơn 1" sống sót và giết bài 37 (`1/3 = 0,333…`); bỏ 33 thì 34–35 tụt xuống thành hai mẹo quy đồng.

**8. Vì sao thập phân → phần trăm → tỉ số, chứ không phải tỉ số trước.** Ba khối này là ba câu trả lời khác nhau cho cùng một sự bất tiện do bài 34–35 gây ra: *đi tìm mẫu chung mệt quá*. Thập phân chốt mẫu ở 10ⁿ để cộng luôn thẳng cột (36); phần trăm chốt mẫu ở 100 để **so** được giữa hai cái toàn thể khác nhau (38); tỉ số bỏ hẳn ý "cái toàn thể" đi khi hai đại lượng không cái nào chứa cái nào (40). Đặt tỉ số trước phần trăm — như phần lớn giáo trình — thì phần trăm chỉ còn là "tỉ số phần trăm", một cái tên; đặt sau, phần trăm được sinh ra từ nhu cầu *có một cái thước chung* rồi mới lộ ra giới hạn của chính nó ở bài 39 ("của cái gì?"), và cái giới hạn đó chính là cửa mở cho tỉ số.

**9. Vì sao thứ tự phép toán là bài áp chót, và vì sao nó nói được "vì sao".** Thứ tự phép toán là bài về **ký hiệu**, nên chỉ dạy được khi đã có đủ thứ để viết ra: `+ − × ÷`, luỹ thừa, gạch phân số. Quan trọng hơn, chỉ tới đây câu trả lời "vì sao" mới thật: nhân được ưu tiên **không phải vì có người quy định thế**, mà vì nhân là phép **đóng gói một lượng thành một khối** (bài 19 và 21 đã dựng sẵn hình ảnh đó) — và một khối thì phải gói xong mới đem gộp. Bài 43 đóng lại bằng một quan sát mà người học tự kiểm được: gạch phân số và số mũ hoá ra là ngoặc đã có sẵn từ bài 31 và bài 24. `reflect` của bài 41 ("một ký hiệu, ba cách đọc — ký hiệu ghi lại cái gì?") là bản lề mở đúng vào đây.

**10. Kiểm tính không thừa — bỏ một bài thì mạch đứt ở đâu.** Bỏ 4 thì 5 không có lý do tồn tại và toàn bộ khối phân số mất nền. Bỏ 8 thì 36 gãy (`0,25` và `0,205` lẫn nhau). Bỏ 11 thì 12, 35 và mọi chuyện "thẳng cột" thành mẹo. Bỏ 15 thì 16 không có ai gọi tới, và số âm quay về chỗ "đến lượt nó trong sách". Bỏ 21 thì 22 không có chỗ bám và bài 42 mất hình ảnh "gói". Bỏ 25 thì 26 mở màn không lý do và 36 mất số mũ âm. Bỏ 32 thì 37 gãy. Bỏ 39 thì 40 không có sự bất tiện nào để chữa. Ở chiều ngược lại, không bài nào dạy lại thứ bài trước đã dạy: thanh số (5), luật cùng đơn vị (11), đổi thước (3) được **dùng lại** bảy tám lần nhưng không được giới thiệu lại lần nào — mỗi lần dùng lại đều được gọi tên kèm số bài, để người học thấy mình đang xài đồ cũ.

**11. Ranh giới với track lân cận và với Realm 1.** Track này không dùng vòng lặp, không dùng hàm, không dùng `list` — Python xuất hiện đúng ở vai **trọng tài**: người học phát biểu một khẳng định về số rồi bắt máy trả `True`/`False` (chỗ này chỉ cần `print`, phép toán và `==`, tức đúng những gì R0 bài 9/14/24 đã có). Nhờ vậy người học đi song song với Realm 1 vẫn theo được, và người đã học R1 cũng không thấy thừa. Chữ thay số, biểu thức, phương trình để nguyên cho T2.2 — và đó đúng là câu hỏi bỏ ngỏ của bài 44. Ước/bội/số nguyên tố, làm tròn, căn bậc hai, ký hiệu khoa học để cho T2.2/T2.5. Cơ số 2 chỉ được **chạm** ở bài 9 rồi bàn giao cho R3.T1.

### Phản biện độc lập

## Verdict

Mạch này chắc hơn hẳn mức trung bình: trục "đơn vị" giữ được suốt 44 bài, khối 13–18 (trừ → khoảng cách → số âm → cộng số đối) và khối 26–35 (hai nghĩa của chia → dư → phân số) là hai đoạn hay nhất, gần như không có chỗ bắt bẻ. Tôi tìm được **3 vấn đề nặng, 4 vấn đề vừa**. Không có bài nào thừa theo nghĩa lặp nội dung.

---

## NẶNG

### 1. Thiếu hẳn một mắt xích: "lấy a/b của một lượng" — mà bài 38, 39, 41, 43, 44 đều xài (tiêu chí 3 + 4)

Rà lại 30 → 41: bài 31 dạy `a/b` = a bản sao của thước `1/b` — tức là a/b **của một đơn vị**. Không bài nào dạy `a/b` **của một lượng khác** (3/4 của luống 12 mét). Nhưng:

- Bài 38 muốn có nghĩa thì phải tính được "30% của 40 cây" — đó đúng là lấy 30/100 của một lượng.
- Bài 39 nói "tăng 50% rồi giảm 50% không về chỗ cũ" — phải tính 50% của một lượng **hai lần**, và bài 39 đứng *trước* bài 41 (nơi mới có kỹ thuật quy về một đơn vị).
- Reflect bài 43 yêu cầu "20% chết" và "chia vào các ô rộng **3/4 mét**" — cái sau là chia cho một phân số, chưa hề được dạy.

Đây không phải chuyện tiểu tiết: nó là phép tính duy nhất mà cả khối 38–44 đứng lên trên, và nó đang vô hình.

**Sửa (chính):** chèn một bài giữa 32 và 33 — *"Lấy mấy phần của một lượng"*: `3/4 của 12 mét = chia 12 cho 4 (bài 26), rồi lấy 3 lần (bài 19)`. Reflect bài 32 đổi thành câu tạo nhu cầu: *"7/4 có chỗ đàng hoàng rồi. Nhưng vườn dài 12 mét, 'lấy 3/4 vườn' là mấy mét — 3/4 lúc này đo cái gì?"*. Reflect bài mới giữ nguyên câu hiện có của 32 ("1/2 và 2/4 rơi trúng cùng một chỗ…") để nối sang 33. Bài 38 khi đó tính `p%` bằng đúng bài này: *1% là thước cỡ 1/100 **của vườn ấy**, 30% là 30 lần cái thước đó* — không phải mẹo mới.

**Nếu 44 là trần cứng:** gộp bài 28 vào bài 27 (xem mục 7 bên dưới) để lấy chỗ. Đừng cắt bài 32 hay 33.

### 2. Bài 25 hứa "số mũ âm" rồi 11 bài sau mới trả, và nó mượn phép chia trước khi phép chia tồn tại (tiêu chí 3)

Reflect bài 25 hỏi **hai** thứ: (a) "số mũ tụt xuống dưới 0", (b) "chia là gì". Bài 26 chỉ trả lời (b). Cái (a) bị bỏ lửng tới tận bài 36, nơi nó xuất hiện như một dòng phụ (`10⁻¹, 10⁻²`) trong bài mà khái niệm chính là thập phân — tức bài 36 đang lén mang hai khái niệm chỉ vì bài 25 nợ.

Thêm nữa, cột "khái niệm mới" của 25 nói `10⁰ = 1` là "hệ quả của việc đi ngược lại đúng một bước", mà đi ngược một bước = chia cho 10 — bài 25 dùng phép chia để chứng minh, rồi reflect mới thú nhận là chưa ai học chia.

**Sửa:** (i) Trong bài 25, biện minh `10⁰ = 1` bằng **mở bó**, không bằng chia: "sang trái là gom mười bó thành một bó to; sang phải là *mở* một bó ra thành mười cái nhỏ" — đó là luật bài 6 chạy ngược, người học đã có. (ii) Cắt vế "số mũ tụt xuống dưới 0" khỏi reflect 25, chỉ giữ: *"Mở bó mãi thì tới cột 'một' là hết. Mà mở một bó ra chia đều cho mười — 'chia' là gì?"* (iii) Để bài 36 được **giới thiệu** `10⁻¹` như đồ mới nó kiếm được, chứ không phải trả nợ cũ.

### 3. Bài 22 nhét hai bức tranh, và bức thứ hai chưa có ai cần (tiêu chí 1 + 4)

Cột khái niệm của 22 nói hai chuyện khác nhau về mặt hình ảnh: (a) **kéo giãn thanh số quanh mốc 0**, (b) **đo lại đúng lượng ấy bằng cái thước nhỏ đi ba lần**. Chúng tương đương về số, nhưng là hai mô hình tinh thần và người học phải nuốt cả hai trong một bài.

Kiểm nhu cầu: reflect bài 21 hỏi "gấp 3 thì đâu là hàng, đâu là cột?" — chỉ (a) trả lời câu đó. Còn (b) mãi tới **bài 33** mới được dùng ("nhân cả tử lẫn mẫu là đo lại bằng thước nhỏ hơn"). Tức là (b) được dạy sớm 11 bài trước chỗ nó có việc.

**Sửa:** bài 22 giữ đúng một khái niệm — *nhân là kéo giãn thanh số*. Chuyển vế "đo lại bằng thước nhỏ đi b lần" xuống bài 33, nơi nó chính là nội dung của bài; ở 33 gọi tên nó là "bài 3 nói lại", không phải "bài 22 nói lại". Bài 22 vẫn có thể nhắc bài 3 bằng một câu, miễn không tính là khái niệm mới.

---

## VỪA

### 4. Bài 9 → 10 là bản lề yếu nhất mạch (tiêu chí 4)

Reflect 9 không sinh ra sự bất tiện nào; nó dựng một sự kiện mới ("Byte vừa hái thêm một đống nữa"). Theo đúng luật thiết kế của chính bạn, phép cộng đang vào vì "tới lượt nó".

**Sửa:** cho reflect 9 khai thác thứ bài 6–9 vừa dựng: *"Byte có 4 bó 3 hạt, An mang tới 2 bó 5 hạt, đổ chung. Đếm lại từ 1 thì phí công đóng bó mấy bài liền — có cách nào dùng lại hai con số đã đếm mà không đếm lại không?"* Lúc đó bài 10 không phải là "gộp", nó là *cách tránh đếm lại từ đầu* — một nhu cầu thật, và nó dựng sẵn cả bài 11 (bó gộp bó, hạt gộp hạt).

### 5. Bài 41 → 42 nhắm trượt một bài, và bài 21 đã dùng lén quy tắc ưu tiên (tiêu chí 3 + 4)

Reflect 41 hỏi "ký hiệu toán rốt cuộc ghi lại **cái gì**" — đó là câu hỏi của bài **43** (ngoặc = nói thẳng cấu trúc), không phải của 42. Bài 42 cần một sự bất tiện cụ thể: *cùng một dòng chữ, hai người ra hai kết quả*. Sự bất tiện ấy thực ra đã có từ **bài 21**, chỗ bạn viết `7×13 = 7×10 + 7×3` — dòng đó chỉ đúng nếu nhân làm trước, và không bài nào nói vì sao.

**Sửa:** (i) Bài 21 viết `(7×10) + (7×3)` có ngoặc, kèm một câu "vì sao bỏ được ngoặc thì bài 42 trả lời". (ii) Reflect 41 đổi thành: *"Byte viết cả vườn thành một dòng: `3 + 4 × 5`. An đọc từ trái sang, ra 35. Byte ra 23. Cùng một dòng, hai kết quả — ai đúng?"* (iii) Câu "một ký hiệu, ba cách đọc — ký hiệu ghi lại cái gì" chuyển thành reflect của 42, thay cho câu hiện tại, hoặc đứng cạnh nó.

### 6. Bài 37 hỏi "vì sao lại là 100", bài 38 không trả lời (tiêu chí 3)

Cột khái niệm của 38 chỉ nói "chốt cứng mẫu ở 100", không nói **vì sao 100** mà không phải 10 hay 1000. Reflect 37 hỏi thẳng câu đó.

**Sửa:** bài 38 phải trả bằng đồ đã có: 10 thì quá thô (`1/10` không tách nổi 40 cây với 250 cây), 1000 thì mỗi lần đều ra số lẻ dài; 100 = `10²` (bài 24) là bó-của-bó (bài 7) — mức bẻ nhỏ đầu tiên đủ mịn mà vẫn viết được bằng số nguyên. Nếu không trả câu này, bài 38 tụt xuống thành một định nghĩa và bài 9 mất một chỗ trả lãi.

### 7. Bài 23 và bài 28 là hai cái lá — không bài nào sau đó dùng lại (tiêu chí 2)

Áp đúng phép thử của bạn: bỏ 23, viết lại reflect 22 thành "kéo giãn ba lần liên tiếp thì viết thế nào" là nối thẳng sang 24; trong toàn track không có chỗ nào cần `(−1)×(−1)=1`. Bỏ 28 cũng vậy — nội dung của nó không được bài nào gọi lại, nó chỉ chuyền số dư sang 29 bằng reflect.

Tôi **không** đề nghị cắt bài 23: nó bịt cái lỗ do 16 + 19 tạo ra (người học có số âm, có phép nhân, mà không có luật ghép hai thứ đó), và đó là câu "vì sao đúng" đắt nhất track. Nhưng phải cho nó một chỗ dùng lại, nếu không nó vẫn là lá.

**Sửa:** (a) Bài 44 (BOSS) thêm một bước dưới mặt đất: rễ ở độ sâu −2 phân, mỗi lần xới nông lên 3 phân, làm 4 lần → dùng cả 16, 18 và 23. (b) Bài 28: hoặc giữ nguyên và cho Python làm chỗ trả lãi (`12/0` báo lỗi — bài này giải thích máy **không** cấm, câu hỏi mới hỏng), hoặc — nếu cần một chỗ cho bài mới ở mục 1 — gộp 28 vào 27 làm nửa sau của chính câu hỏi bài 27 ("3 lọt vào 12 mấy lần" → "0 lọt vào 12 mấy lần"), vì `12 : 0` đúng là ca suy biến của bài 27 chứ không phải khái niệm mới.

---

## NHẸ (một dòng mỗi cái)

- **Bài 6 → 7:** không bài nào sở hữu bước *đóng bó lần hai*. Reflect 6 cho "13 bó", nhưng để viết ra "137" phải gom 10 bó thành bó-của-bó trước — bài 6 không dạy, bài 7 coi như đã có. Thêm một câu vào khái niệm bài 6: "đủ mười **bó** thì lại gom thành một bó to hơn — cùng một luật, lặp lại".
- **Bài 21:** cột khái niệm có chữ "và" thật ("mô hình vùng **và** tính chất phân phối"). Đây là một ý hai tên, không phải hai ý — nhưng nên đặt tên một cái: khái niệm là *cắt mảng thì phép nhân tách theo*, còn "tính chất phân phối" chỉ là từ vựng đặt cuối bài.
- **Bài 16:** vế "0 là một mốc do người chọn" không được bài nào sau đó dùng tới (track không có nhiệt độ, không có đổi gốc toạ độ). Hạ nó xuống thành câu kể trong truyện (Byte chọn mặt đất làm mốc), đừng để nó ngang hàng với "số âm".
- **Bài 35:** đây là bài duy nhất ngoài 44 không có khái niệm thật sự mới — và bạn tự nói vậy ("chỉ áp lại luật bài 11"). Nên ghi thẳng vào cột như bài 44: *"không khái niệm mới — bài đóng vòng"*. Giữ bài, chỉ đừng giả vờ nó mới.
- **Bài 8 vs bài 28:** bài 8 nói "`0` **không** mang nghĩa 'không có gì'", bài 28 lại bắt người học hình dung "đoạn dây **0 mét**" — tức đúng nghĩa "không có gì". Sửa bài 8 thành "trong một dãy chữ số, `0` còn thêm một việc nữa: giữ chỗ cho cột rỗng".
- **Reflect bài 23:** trượt số — "kéo giãn ba lần liên tiếp thì viết `3 × 3 × 3`. Kéo mười lần thì phải viết mười con **10**". Phải là mười con 3, hoặc đổi cả ví dụ sang 10 ngay từ đầu (nên chọn cách sau, vì bài 24 lấy ví dụ `10⁵` và bài 25 cần đúng cột 10).
- **Bài 42, cẩn thận nói quá:** "nhân trước cộng" *có* phần là quy ước (người ta chọn nó vì nó ghi gọn được cấu trúc hay gặp nhất), không phải một sự thật bị ép buộc. Track này bán "vì sao đúng" nên chỗ này nếu overclaim sẽ tự bắn vào chân khi có người hỏi "sao không quy ước ngược lại?". Viết an toàn hơn: *"quy ước này được chọn, và chọn thế là vì nó ghi lại đúng cái cấu trúc gói-rồi-gộp ở bài 19 và 21 mà không cần ngoặc"*.

---

## Nói thẳng chỗ tốt

Bốn quyết định sau tôi thử phá không được, và chúng là lý do mạch này hơn giáo trình chuẩn: **số âm ở 16–18 trước phép nhân** (sự bất tiện sinh ở 15, hoãn là mất); **tách chia đều / chia đo thành 26 và 27** (đây là bài 27 gánh cả 28, 29 và mọi chuyện phân số về sau); **luỹ thừa chen vào 24–25 để quay lại giải thích bảng vị trí đã dùng 18 bài** (đúng là chỉ chạy được ở vị trí này); và **thập phân → phần trăm → tỉ số** chứ không phải tỉ số trước (bài 39 sinh ra bài 40 bằng một giới hạn thật, không bằng mục lục). Chuỗi 30 → 31 → 32 → 33 cũng đúng chuỗi hiểu lầm ngoài đời, không cần động.

---

## T2.2 — Đại số & hàm số (Realm 2 · Toán & Toán rời rạc · Python chỉ để KIỂM · 36 bài)

## T2.2 — Đại số & hàm số (Realm 2 · 36 bài · hiện vật xuyên suốt: **xe bánh mì của Byte**)

| # | slug | Tiêu đề | Khái niệm mới (đúng một) | `reflect` cuối bài | Dựa trên |
|---|---|---|---|---|---|
| 1 | `cho-trong-nhan-nhieu-so` | Ô trống trên bảng giá | Một **ô trống** trong câu tính nhận được **nhiều** giá trị khác nhau — mỗi lần điền một số, ra một kết quả; nó không phải "một số bị giấu" | Byte viết "▢ ổ bánh mì và ▢ chai nước". Hai ô này có buộc phải điền cùng một số không — và làm sao viết cho người khác biết ô nào là ô nào? | T2.1 · nhân, thứ tự phép tính |
| 2 | `chu-cai-la-ten-cua-o-trong` | Chữ cái chỉ là cái tên dán lên ô trống | Một **chữ cái là tên của một ô trống**: mọi chỗ mang cùng một chữ là cùng một ô (phải điền cùng số); chữ khác nhau là ô khác nhau | Bạn từng viết `n = 5` trong Python — cái tên ấy đang giữ đúng một số. Còn `n` ở đây chưa giữ số nào. Vậy `15000 × n` đã là một con số chưa? | 1, R0.11 `dat-ten-cho-gia-tri` |
| 3 | `cau-tinh-chua-ra-so` | Câu tính chưa ra số | **Biểu thức** — một câu tính còn ô trống thì chưa phải một con số; nó là một *cách tính đang chờ* | Đang chờ thì chờ cái gì? Nếu Byte đưa cho nó số 20, chuyện gì xảy ra với `15000 × n`? | 2 |
| 4 | `dien-vao-thi-ra-so` | Điền vào thì mới ra số | **Thay giá trị** — điền một số vào **mọi** chỗ có chữ đó, biểu thức thu lại thành đúng một con số | Điền 1 ra một số, điền 2 ra số khác. Nếu điền lần lượt cả bảy ngày trong tuần rồi ghi lại — bạn được thứ gì? | 3, T2.1 · thứ tự phép tính |
| 5 | `bang-cua-moi-lan-dien` | Bảng của mọi lần điền | **Bảng giá trị** — một biểu thức không sinh ra một số mà sinh ra cả một bảng: mỗi lần điền là một dòng, hai cột (điền gì / ra gì) | Hai người viết hai câu tính trông chẳng giống nhau: `15000 × n + 30000` và `15000 × (n + 2)`. Hai bảng của chúng giống hay khác? | 4, R0.31 `lam-lai-nhieu-lan` |
| 6 | `hai-cau-tinh-mot-bang` | Hai câu tính, một bảng | **Biểu thức tương đương** — bằng nhau ở **mọi** giá trị điền vào, khác hẳn "tình cờ bằng nhau ở một vài số" | Hai câu ấy khác nhau đúng một dấu ngoặc. Bảng chỉ thử được vài chục số, mà "mọi số" thì thử không hết. Có luật nào cho phép tháo dấu ngoặc mà chắc chắn đúng với mọi số? | 5, T2.1 · giao hoán & kết hợp |
| 7 | `mo-dau-ngoac` | Mở dấu ngoặc | **Phân phối** — `a × (b + c) = a × b + a × c`, đúng với mọi số vì đó chỉ là hai cách đếm cùng một khay bánh | Mở ra được thì gấp lại được không? `15000n + 30000` có quay về dạng có ngoặc không, và lấy gì làm "cái chung"? | 6, T2.1 · diện tích hình chữ nhật |
| 8 | `rut-cai-chung-ra-ngoai` | Rút cái chung ra ngoài | **Nhân tử chung** — đọc ngược luật phân phối: phần chung của các cụm được rút ra trước ngoặc | `3n + 5n` — hai cụm này chung nhau cái gì? Rút nó ra thì trong ngoặc còn lại gì? | 7 |
| 9 | `gop-cai-cung-loai` | Chỉ gộp được thứ cùng loại | **Hạng tử đồng dạng** — `3n + 5n = 8n` chính là rút `n` ra ngoài; còn `8n + 6` thì dừng: nó vẫn là một biểu thức đúng, chỉ là **không gộp thêm được** | Rút gọn cách mấy thì câu tính vẫn còn ô trống, vẫn chưa ra số. Suốt chín bài chưa ai hỏi "n bằng bao nhiêu". Đến lúc nào câu hỏi đó mới có nghĩa? | 8, R0.16 `khi-hai-kieu-khong-hop` |
| 10 | `dau-bang-la-mot-cau-hoi` | Dấu `=` đặt ra một câu hỏi | **Phương trình** — hai biểu thức nối bằng `=` không phải một phép tính mà một **lời khẳng định**: mỗi lần điền vào, nó hoá ra đúng hoặc sai | Điền 20 vào thì câu ấy đúng. Còn số nào khác cũng làm nó đúng nữa không? Có bao nhiêu số như thế? | 9, 3, R0.24 `dung-hay-sai` |
| 11 | `nhung-so-lam-cau-do-dung` | Những số làm câu đó đúng | **Nghiệm & tập nghiệm** — đáp án của một phương trình là *tập hợp* mọi giá trị điền vào làm nó đúng | Quét từ 1 tới 100 thì tìm ra. Nhưng nếu nghiệm là 2,5 hoặc là số có bảy chữ số thì quét tới bao giờ? Có cách nào không phải thử từng số? | 10 |
| 12 | `cai-can-hai-dia` | Cái cân hai đĩa | Một phương trình đọc được như một **cái cân đang thăng bằng**: hai đĩa nặng bằng nhau, ô trống là gói hàng chưa biết nặng bao nhiêu | Cân đang thăng bằng và bạn muốn nó vẫn thăng bằng. Vậy được phép động vào hai cái đĩa theo kiểu nào? | 11 |
| 13 | `lam-gi-cung-lam-ca-hai-dia` | Làm gì cũng phải làm cả hai đĩa | **Phép biến đổi giữ nguyên nghiệm (cộng/trừ)** — bớt hoặc thêm cùng một lượng ở hai vế cho một phương trình **mới** nhưng **cùng tập nghiệm** | Cộng trừ thì rõ rồi. Còn chia đôi cả hai đĩa — cân có còn thăng bằng? Nhân cả hai đĩa lên gấp ba thì sao? | 12 |
| 14 | `nhan-chia-ca-hai-dia` | Nhân, chia cả hai đĩa | Nhân/chia hai vế cho cùng một số **khác 0** cũng giữ nguyên tập nghiệm — và nhân với 0 thì hỏng, vì mọi phương trình đều thành `0 = 0` | Bạn có hai loại phép giữ nghiệm. Dùng chúng theo **thứ tự nào** để cuối cùng ô trống còn đứng một mình trên một đĩa? | 13, T2.1 · chia và số 0 |
| 15 | `tach-o-trong-ra-mot-minh` | Tách ô trống ra đứng một mình | **Giải phương trình bậc nhất** — gỡ lần lượt những gì đứng cạnh chữ (gỡ cộng trừ trước, gỡ nhân chia sau) cho tới dạng `n = một số` | Chuỗi biến đổi dài bốn năm bước. Lỡ sai một bước, bạn ra một con số khác — mà nó trông vẫn y hệt một đáp án đúng. Làm sao biết? | 14, 13 |
| 16 | `thu-lai-o-cau-goc` | Thử lại ở câu gốc | **Kiểm nghiệm** — thay giá trị tìm được vào phương trình **ban đầu** (không phải vào dòng vừa viết); đúng thì hai vế ra cùng một số | Mọi phương trình tới giờ đều có chữ ở đúng một vế. Nếu tiền vốn cũng phụ thuộc số ổ — chữ xuất hiện ở **cả hai** đĩa — thì gỡ từ đâu? | 15, 4 |
| 17 | `chu-o-ca-hai-dia` | Chữ đứng ở cả hai đĩa | Bớt đi cùng **một cụm có chữ** ở hai vế (cụm ấy cũng là một lượng) để dồn hết chữ về một bên | Có lần dồn xong bạn sẽ thấy chữ biến mất sạch, chỉ còn `0 = 5`. Chẳng còn ô trống nào để điền nữa. Phương trình ấy nghiệm bằng bao nhiêu? | 16, 13, 9 |
| 18 | `khi-khong-so-nao-dung` | Khi không số nào làm nó đúng | **Vô nghiệm** — `0 = 5` nghĩa là mọi cách điền đều sai, tập nghiệm rỗng. (Chỗ này "chữ là số bí ẩn cần tìm" sụp hẳn: không có số nào để tìm) | Có câu không số nào làm đúng. Vậy có câu nào ngược lại — điền số nào vào cũng đúng — không? | 17, 11 |
| 19 | `khi-so-nao-cung-dung` | Khi số nào cũng đúng | **Đồng nhất thức** — biến đổi ra `0 = 0` nghĩa là mọi số đều là nghiệm, vì hai vế vốn là hai **biểu thức tương đương** đội lốt phương trình | Ba loại câu trả lời: một số, không số nào, mọi số. Nhưng câu hỏi thật của quán là "bán bao nhiêu ổ thì **đủ bù** tiền vốn" — đủ bù nghĩa là bằng **hoặc hơn**. Dấu `=` nói được chuyện "hơn" không? | 18, 6 |
| 20 | `hon-kem-thay-cho-bang` | Hơn kém thay cho bằng | **Bất phương trình** — thay `=` bằng `>` hoặc `≥`; tập nghiệm không còn là một điểm mà là cả một **khoảng**, tô được lên trục số (mút đặc / mút rỗng) | Giải nó có dùng lại được đúng các phép của cái cân không? Thử nhân hai vế với `−1`, rồi kiểm lại bằng một con số cụ thể xem. | 19, 11, T2.1 · trục số |
| 21 | `khi-dau-phai-quay-nguoc` | Khi dấu phải quay ngược | Nhân/chia hai vế của bất phương trình cho một số **âm** thì phải **đảo chiều** dấu — còn cộng/trừ thì không (đây cũng là chỗ hình ảnh cái cân hết dùng được) | Nghiệm giờ là cả một khoảng, và bạn đang tô nó lên một đường chỉ có **một hàng số**. Nhưng bảng ở bài 5 có tận **hai cột**. Một dòng của bảng ấy ghi lên đâu? | 20, 14, T2.1 · số âm |
| 22 | `mot-diem-can-hai-so` | Một điểm cần hai con số | **Hệ toạ độ** — hai trục vuông góc, một **cặp có thứ tự** `(x; y)` chỉ đúng một điểm; đổi thứ tự là điểm khác hẳn | Bảng ở bài 5 có bảy dòng, vậy là bảy điểm. Bảy điểm ấy nằm bừa bãi hay xếp thành hình gì? | 21, 5, T2.1 · trục số |
| 23 | `cham-ca-bang-len-mat-phang` | Chấm cả bảng lên mặt phẳng | **Đồ thị** — hình của một biểu thức: mỗi dòng bảng thành một điểm; bảy điểm của `15000n` nằm **thẳng hàng** | Nhìn hình là đọc ra được số tiền cho từng số ổ. Nhưng nếu có một số ổ mà hình cho tới **hai** mức tiền khác nhau thì cái bảng ấy còn dùng được không? | 22, 5 |
| 24 | `may-mot-vao-mot-ra` | Cái máy: một đầu vào, đúng một đầu ra | **Hàm số** — quy tắc mà mỗi đầu vào cho ra **đúng một** đầu ra (phát hiện bằng cách phân loại máy nào được, máy nào không, rồi mới đặt tên) | Quán có mấy cái máy: máy tính tiền thu, máy tính tiền lãi, máy đổi ra số ly nước đá. Gọi cái nào cũng là "cái máy" thì loạn. Đặt tên cho một cái máy kiểu gì? | 23, 4 |
| 25 | `dat-ten-cho-cai-may` | Đặt tên cho cái máy | **Ký hiệu hàm** `f(n)` — `f` là tên cái máy, `f(20)` là thứ nó nhả ra khi bỏ 20 vào; đọc là "ép của hai mươi" | `f(n) = 15000n` vẽ ra một đường **thẳng**. Vì sao lại thẳng — chứ không cong, không gãy khúc? | 24, R0.37–39 `def/return` |
| 26 | `moi-buoc-them-dung-mot-luong` | Mỗi bước thêm đúng một lượng | **Độ dốc là tốc độ đổi** — đầu vào thêm 1 thì đầu ra đổi một lượng **cố định**; chính sự cố định đó làm đồ thị thẳng, và lượng đổi âm thì đường đi xuống | Hai xe cùng bán 15 nghìn một ổ, nhưng một xe phải trả 100 nghìn tiền thuê chỗ. Hai đường có **cùng độ dốc** — vậy chúng khác nhau ở chỗ nào trên hình? | 25, 23, 5 |
| 27 | `cho-bat-dau-cua-duong-thang` | Chỗ bắt đầu của đường thẳng | `y = ax + b` — `b` là giá trị lúc đầu vào bằng 0, cũng là chỗ đường cắt trục dọc; **hai con số `a` và `b` tả trọn một đường thẳng** | `a` và `b` đủ cho **mọi** đường thẳng. Nhưng diện tích mảnh sân hình vuông cạnh `n` mét thì sao — bảng của nó có tăng đều một lượng cố định không? | 26, 22 |
| 28 | `khi-tang-khong-deu` | Khi mỗi bước tăng một kiểu | **Hàm bậc hai** — `n × n`: mỗi bước tăng một lượng **khác nhau** (phải lấy chênh lệch của chênh lệch mới thấy cái cố định), nên đồ thị **cong** | Tăng giá thì lãi mỗi ổ cao hơn nhưng bán được ít ổ hơn — đường cong này có một chỗ cao nhất. Chỗ ấy nằm đâu, và trên hình có gì giúp nhận ra nó? | 27, 7, T2.1 · luỹ thừa |
| 29 | `dinh-va-hai-ben-doi-xung` | Đỉnh, và hai bên đối xứng | **Đỉnh của parabol** — chỗ cao nhất (hoặc thấp nhất); hai bên đối xứng nhau, nên **hai đầu vào khác nhau cho cùng một đầu ra** — vẫn là hàm, vì luật chỉ cấm chiều ngược lại | Đường thẳng thì thêm đều, parabol thêm không đều nhưng vẫn là **thêm**. Còn nếu mỗi giờ men trong thúng bột không thêm mà **nhân đôi** thì bảng trông thế nào? | 28, 24 |
| 30 | `moi-buoc-nhan-voi-cung-mot-so` | Mỗi bước nhân với cùng một số | **Hàm mũ** — bước đi cố định là một phép **nhân** (`2ⁿ`) chứ không phải phép cộng: tăng trưởng nhân | Bảy giờ men mới có 128 phần, trong khi một đường thẳng dốc 1000 đơn vị mỗi giờ vẫn đang bỏ xa nó. Sau 20 giờ thì ai hơn ai? | 29, 26, T2.1 · luỹ thừa |
| 31 | `nhan-deu-vuot-moi-cong-deu` | Nhân đều rồi sẽ vượt mọi cộng đều | Một hàm mũ (cơ số > 1) **cuối cùng luôn vượt** mọi hàm bậc nhất, dù đường thẳng ấy dốc tới đâu — dốc chỉ làm chậm ngày bị vượt | Máy nào bạn cũng đang bỏ số vào để lấy kết quả ra. Nhưng câu hỏi ở quán thường ngược lại: "muốn thu 300 nghìn thì phải bán mấy ổ". Có bỏ **kết quả** vào để đòi lại **đầu vào** không? | 30, 26 |
| 32 | `chay-nguoc-cai-may` | Chạy ngược cái máy | **Hàm ngược** — cái máy đảo chiều: đưa đầu ra, trả lại đúng đầu vào đã tạo ra nó; và giải một phương trình bậc nhất chính là chạy ngược cái máy ấy tại **một** điểm | Máy tính tiền chạy ngược được. Nhưng máy tính diện tích sân hình vuông ở bài 28 — đưa 9 mét vuông vào máy ngược, nó phải trả về 3 hay −3? | 31, 15, 25 |
| 33 | `khi-khong-chay-nguoc-duoc` | Khi không chạy ngược được | Máy chỉ chạy ngược được khi **hai đầu vào khác nhau không bao giờ cho cùng một đầu ra**; chỗ đối xứng của parabol là chỗ hỏng, và cách chữa là **cắt bớt đầu vào cho phép** (chỉ nhận cạnh ≥ 0) | Giờ bạn có cả tủ máy: máy xuôi, máy ngược, máy thẳng, máy cong. Muốn tính tiền lãi thì phải tính tiền thu trước rồi mới trừ vốn — hai việc, hai máy. Nối chúng thành một máy được không? | 32, 29, 24 |
| 34 | `noi-hai-may-lien-nhau` | Nối hai máy nối đuôi nhau | **Hợp hai hàm** — đầu ra của máy này cắm thẳng vào đầu vào máy kia; `g(f(n))` là một cái máy **mới**, dùng được mà không cần mở ra xem bên trong | Nối `f` rồi `g` thì được. Nối `g` rồi `f` cũng được. Hai cách nối cho ra cùng một cái máy chứ? | 33, 25, 4 |
| 35 | `doi-thu-tu-doi-ket-qua` | Đổi thứ tự nối, đổi luôn kết quả | Hợp hàm **không đổi chỗ được**: `f(g(n))` nói chung khác `g(f(n))` — thứ tự nối là một phần của cái máy; riêng cặp xuôi–ngược nối lại thì trả về chính đầu vào | Cả track chỉ có một ý: chữ là một ô trống, và một cái máy là một quy tắc điền ô ấy. Ghép hết lại, có tả trọn một buổi bán hàng bằng vài cái máy không? | 34, 32, 6 |
| 36 | `boss-xe-banh-mi-cua-byte` | BOSS: Xe bánh mì của Byte | *(không khái niệm mới — bài tổng hợp)* một buổi bán hàng dựng bằng: biểu thức → phương trình → bất phương trình → đồ thị → bậc nhất/bậc hai/mũ → máy ngược → hợp máy | Bạn vừa nối hai máy thành một máy mới mà không cần biết bên trong chúng có gì — ở Realm 4, chính phép nối đó là cách người ta viết cả một chương trình. Nhưng trước đã: bài 19 bảo "mọi số đều là nghiệm". Bạn tin nó vì đã thử vài số, hay vì **chứng minh** được? *(dẫn sang T2.3 — Logic & chứng minh)* | 1–35 |

**Vì sao thứ tự này đúng**

**Vì sao thứ tự này là thứ tự đúng**

**0. Một trục duy nhất, một hiện vật duy nhất.** Cả 36 bài chỉ có một trục: *ô trống*. Bài 1–9 dạy điền ô và biến đổi hình dạng câu tính khi ô vẫn còn đó; bài 10–21 hỏi ngược lại "điền gì thì câu này đúng"; bài 22–27 vẽ **mọi** cách điền lên một mặt phẳng; bài 28–35 phân loại các kiểu quy tắc điền. Và mọi bài chạy trên cùng một hiện vật — xe bánh mì của Byte (giá một ổ, tiền thuê chỗ, men nở trong thúng bột) — nên sự bất tiện **tích luỹ trên một vật**, không tan đi mỗi bài một ví dụ khác. Python xuất hiện đúng 8 lần và luôn ở vai **kiểm**, không bao giờ ở vai dạy: bài 5 (in bảng), 6 (so hai bảng trên cả trăm giá trị), 11 (quét tìm nghiệm), 16 (`assert` thay số vào câu gốc), 25 (`def f(n)` là chính cái máy vừa đặt tên), 31 (tìm `n` đầu tiên mà `2**n > 1000*n`), 34 (`lai(thu(n))`), 36.

**1. Vì sao suốt chín bài đầu không có một dấu `=` nào.** Đây là chỗ sửa nỗi sợ đại số, và nó phải sửa bằng *cấu trúc mạch*, không bằng một câu dặn dò. Nếu bài đầu tiên có chữ đã kèm dấu `=` thì người học lập tức học được rằng "chữ là con số bị giấu, việc của tôi là tìm ra nó" — sau đó mọi lời đính chính đều vô hiệu. Nên bài 1 cho ô trống **nhiều** giá trị trước khi có bất kỳ chữ cái nào (concrete trước, ký hiệu sau — quy tắc CRA), bài 5 biến "nhiều giá trị" thành một **bảng** nhìn thấy được, và mãi bài 10 dấu `=` mới xuất hiện — lúc đó nó không thể bị đọc thành "hãy tìm số bí ẩn", vì người học đã có sẵn hình ảnh cả một bảng để đối chiếu. Bài 9 kết bằng đúng câu chốt hạ: *suốt chín bài chưa ai hỏi "n bằng bao nhiêu"* — câu hỏi đó được để dành, và vì được để dành nên khi tới nó có nghĩa.

**2. Vì sao phân phối và nhân tử nằm ở 7–9, trước phương trình chứ không sau.** Bài 6 tạo ra một sự bất lực rất cụ thể: bảng thử được một trăm số, mà "mọi số" thì không thử hết. Phân phối đến đúng chỗ đó — nó là **luật đầu tiên đổi được hình dạng một biểu thức mà không cần thử**. Đặt nó sau phương trình (như phần lớn sách) thì nó chỉ còn là một mẹo biến đổi trong lúc giải, và người học học thuộc chứ không hiểu. Ba bài 7 → 8 → 9 là **một luật đọc theo ba chiều**: mở ra, gấp lại, và gấp lại trên hai cụm cùng chữ (`3n + 5n = 8n` chính là rút `n` ra ngoài) — nhờ vậy "gộp hạng tử đồng dạng" không phải quy tắc thứ tư phải nhớ mà là hệ quả của luật thứ nhất. Bài 9 cũng là chỗ duy nhất trong track mượn lại `TypeError` của Realm 0, và mượn có rào: máy Python **dừng lại**, còn `8n + 6` thì hoàn toàn hợp lệ, chỉ là hết gộp được.

**3. Vì sao cái cân xuất hiện ở bài 12, và vì sao nó phải chết ở bài 21.** Cái cân là mô hình concrete, nên theo CRA nó phải đứng **trước** thao tác biến đổi (13–14) chứ không đi kèm. Nhưng nó chỉ được đứng ở đó vì bài 11 vừa chứng minh cách "quét từng số" là ngõ cụt — không có bài 11 thì cân chỉ là một hình vẽ trang trí. Quan trọng hơn: cái cân là mô hình **có hạn**, và mạch này dùng chính chỗ hạn của nó làm bài học. Đĩa cân không hình dung nổi việc nhân hai vế với `−1`; đó đúng là bài 21, nơi người học buộc phải bỏ cân, quay về trục số và kiểm bằng một con số cụ thể. Một mô hình bị bỏ đúng lúc dạy được nhiều hơn một mô hình được giả vờ là đúng mãi.

**4. Vì sao 18–19 là đỉnh của track, và vì sao chúng phải nằm ở đúng đó.** "Vô nghiệm" và "mọi số đều là nghiệm" là **bằng chứng thực nghiệm** cho luận điểm mở đầu: nếu chữ thật sự là một con số bị giấu thì hai chuyện này không thể xảy ra — không thể có "số bị giấu mà không có số nào", càng không thể có "số bị giấu mà là mọi số". Chúng chỉ đứng được ở vị trí 18–19 vì cần đủ ba thứ có trước: tập nghiệm (11) để nói ra "rỗng" và "tất cả", chuỗi biến đổi giữ nghiệm (13–15) để chữ có đường mà biến mất, và chữ ở hai vế (17) để nó biến mất *một cách tự nhiên* chứ không phải do người soạn bài dựng sẵn. Bài 19 còn khép lại một vòng dài: `0 = 0` xảy ra vì hai vế vốn là hai biểu thức tương đương của bài 6 — mười ba bài sau, khái niệm cũ quay lại đội một cái lốt khác.

**5. Vì sao hệ toạ độ không phải một chương mới mà là con của bài 5.** Bài 21 kết thúc với một tập nghiệm tô trên trục số — một hàng số, một chiều. Bảng ở bài 5 thì có hai cột. Toạ độ vào bằng đúng chỗ hở đó: *một dòng của bảng ghi lên đâu?* Nhờ vậy bài 22 không phải "mở bài về mặt phẳng toạ độ" mà là câu trả lời cho một sự bất tiện có thật, và bài 23 chỉ việc chấm lại **bảng cũ của xe bánh mì**, không phải một dữ liệu mới. Người học nhìn thấy bảng, biểu thức và đồ thị là **ba khuôn mặt của một thứ** — đúng quy tắc "tối thiểu hai biểu diễn liên kết động" — thay vì ba chương rời.

**6. Vì sao "hàm số" mãi bài 24 mới có tên, sau đồ thị — ngược với sách giáo khoa.** Sách thường định nghĩa hàm trước rồi vẽ đồ thị minh hoạ. Ở đây làm ngược, vì quy tắc "định nghĩa được **phát hiện** chứ không được phát": bài 23 kết bằng một phản ví dụ (một đầu vào cho hai mức tiền), người học phân loại máy nào dùng được / máy nào không, **rồi** hệ thống mới đặt tên "hàm số". Ký hiệu `f(n)` lại lùi thêm một bài nữa (25), vì ký hiệu luôn đi cuối và vì nó phải sinh ra từ một sự bất tiện thật: quán có ba cái máy, gọi cái nào cũng là "cái máy" thì loạn. Bài 25 cũng là chỗ Python trả ơn: `def f(n): return 15000*n` không phải kiến thức mới với người vừa xong Realm 0 — nó là bằng chứng rằng ký hiệu toán học mới học chính là thứ họ đã gõ từ lâu.

**7. Vì sao bậc hai đứng trước hàm mũ, và cả hai đứng trước hàm ngược.** Ba họ hàm được xếp theo **cách một bước đi thay đổi**: thêm một lượng cố định (26) → thêm một lượng đổi dần (28) → nhân với một số cố định (30). Đó là một trục duy nhất, tăng dần, nên bài 30 không phải chủ đề mới mà là bước thứ ba của cùng câu hỏi. Còn hàm ngược **bắt buộc** phải nằm sau bậc hai: chỗ hỏng của máy ngược (hai đầu vào cho cùng một đầu ra) chính là tính đối xứng của parabol đã dựng ở bài 29 — nếu dạy hàm ngược ngay sau hàm bậc nhất thì "một–một" là một điều kiện không ai vi phạm, tức là một điều kiện vô nghĩa. Bài 32 còn thu hồi một món nợ cũ: **giải phương trình bậc nhất ở bài 15 chính là chạy máy ngược tại một điểm** — người học nhận ra mình đã làm việc đó suốt mà chưa có tên gọi.

**8. Vì sao hợp hàm đứng cuối, và kiểm tính không thừa.** Hợp hàm (34–35) là cây cầu sang Realm 4, nên nó phải đứng ở chỗ người học đã có **nhiều máy đủ khác nhau** để nối — nối hai đường thẳng thì chẳng thấy gì, nối máy-tiền-thu với máy-trừ-vốn rồi đổi thứ tự thì thấy ngay. Bài 35 (`f∘g ≠ g∘f`) gieo đúng hạt giống mà T4.2 cần và đồng thời khép cặp xuôi–ngược của 32. Kiểm tính không thừa: bỏ 5 thì 22–23 không có bảng nào để chấm; bỏ 6 thì 19 tụt xuống thành một mẹo; bỏ 11 thì cái cân ở 12 không giải quyết nỗi bất tiện nào; bỏ 17 thì 18–19 phải dựng cảnh giả; bỏ 26 thì 27–28 không có "đều/không đều" để so; bỏ 29 thì 33 mất chỗ hỏng để chỉ vào; bỏ 24 thì 32–35 nói về "cái máy" mà chưa ai định nghĩa máy. Ở chiều ngược lại, không bài nào dạy lại thứ T2.1 đã dạy: trục số, số âm, thứ tự phép tính, giao hoán/kết hợp, luỹ thừa được **dùng lại liên tục** nhưng không được giới thiệu lại lần nào; và chứng minh (vì sao một luật đúng với mọi số) được cố ý để hở — chính chỗ hở đó là câu bỏ ngỏ của bài 36, bàn giao sang T2.3.

### Phản biện độc lập

## Kết luận trước

Mạch này chắc hơn phần lớn mạch tôi từng đọc: 34/35 mối nối `reflect(N) → khái niệm(N+1)` khớp thật, không có bài nào chỉ nằm đó "vì đến lượt nó". Bốn lập luận trụ (không có dấu `=` trong 9 bài đầu; phân phối trước phương trình; cân chết ở 21; toạ độ là con của bài 5) đều đứng vững khi thử phá. Vấn đề thật có, nhưng ít và cục bộ. Dưới đây là những chỗ tôi giữ lại được sau khi thử bác bỏ.

---

## 1. Bài lén nhét hai khái niệm

**Bài 29 — nặng nhất, và chữ "và" nằm ngay trên tiêu đề.**
Cột khái niệm chứa ba mệnh đề: (a) đỉnh là chỗ cao/thấp nhất, (b) hai bên đối xứng, (c) hai đầu vào cho cùng một đầu ra vẫn là hàm. (c) là gọi lại luật bài 24 — không tính. Nhưng (a) và (b) đang được đặt ngang hàng, thành hai sự thật phải nhớ.
**Sửa:** đảo quan hệ, đừng tách bài. Khái niệm duy nhất là **đối xứng**: parabol có một cái gương. Đỉnh không phải phát hiện thứ hai, nó là **tên của điểm duy nhất nằm trên trục gương** — thứ rơi ra từ (b) chứ không đứng cạnh (b). Viết lại cột khái niệm thành: *"Parabol có một trục gương: hai đầu vào khác nhau, cách đều trục ấy, cho cùng một đầu ra. Điểm duy nhất không có bạn đối xứng chính là đỉnh."* Lúc đó (a) là hệ quả, (c) là kiểm lại luật cũ, bài còn đúng một ý.

**Bài 35 — mệnh đề thứ hai là kiến thức bài 32 bị dạy lại.**
"Hợp hàm không đổi chỗ được" + "riêng cặp xuôi–ngược nối lại thì trả về chính đầu vào". Vế sau **đã nằm trong định nghĩa bài 32** ("đưa đầu ra, trả lại đúng đầu vào đã tạo ra nó") — nó không phải khái niệm mới ở đây.
**Sửa:** đánh dấu nó là **gọi lại**, không phải dạy: *"thử `f` rồi `g⁻¹`… và thử đúng cặp xuôi–ngược của bài 32 — đây là cặp duy nhất bạn đã biết trước là đổi thứ tự cũng như nhau."* Một câu, dán nhãn callback, hết.

**Bài 33 — "cắt bớt đầu vào cho phép" là một khái niệm chưa ai giới thiệu.**
Đó là *miền xác định*. Cả track không có bài nào nói "một cái máy còn có tập đầu vào nó chịu nhận". Nó đang được nhét vào mệnh đề phụ của bài 33 dưới dạng cách chữa.
**Sửa:** đừng thêm bài. Biến nó thành **sự thật của hiện vật, không phải luật toán**: cái sân vốn không có cạnh âm — cái *máy* `n × n` nhận mọi số, cái *sân* thì không. Cách chữa không phải "ta cắt miền" mà "ta nhớ ra máy này đang dùng cho sân, mà sân thì chưa bao giờ nhận số âm". (Điều kiện để làm được: xem mục 4.)

**Bài 20 — ô nặng nhất track, nhưng chưa vi phạm.**
Ba thứ trong một ô: đổi `=` thành `>`/`≥`; tập nghiệm là một khoảng; tô trục số với mút đặc/rỗng. Tôi cho là **một** khái niệm vì mút đặc/rỗng chính là `>` với `≥` vẽ ra — đúng luật "hai biểu diễn liên kết". Nhưng nếu thân bài phải dạy thêm ký hiệu khoảng `(3; +∞)` thì nó thành hai bài đội một mũ.
**Sửa:** cấm ký hiệu khoảng trong bài 20 — không chỗ nào từ 21 đến 36 cần tới nó. Giữ đúng hình tô.

Các ô còn lại dùng "và" mà tôi **không** tính là vi phạm: bài 14 (nhân 0 là *ranh giới* của cùng một luật), bài 26 (dốc âm là *một ca* của cùng một luật), bài 32 (giải phương trình = chạy ngược tại một điểm là *thu nợ*, không phải dạy mới).

---

## 2. Bài thừa

Tôi thử bỏ từng bài. **Không bài nào thừa hẳn.** Hai bài đáng lo, vì lý do khác nhau:

**Bài 3 — thừa theo đúng bài kiểm của chính bạn, và chỉ thoát nhờ một việc mà cột khái niệm chưa nói ra.**
Thử: reflect(2) hỏi *"vậy `15000 × n` đã là một con số chưa?"* → khái niệm(4) *"điền vào thì mới ra số"* trả lời trọn vẹn: chưa, và đây là lúc nó thành số. Mạch nối thẳng, bài 3 rơi ra không để lại lỗ. Lý do bài 3 vẫn phải sống nằm ở bài 6–9: ở đó người học **đổi hình dạng một biểu thức mà không hề điền số nào** — muốn làm thế thì biểu thức phải là một *vật* cầm được, so được, chứ không phải một phép tính dở dang. Nhưng cột khái niệm bài 3 hiện chỉ phát biểu phần **phủ định** ("chưa phải một con số"), mà phần phủ định thì bài 4 nuốt trọn.
**Sửa:** viết lại cột khái niệm bài 3 thành mệnh đề khẳng định: *"Biểu thức là một **vật**: nó có hình dạng, đọc được, chép lại được, so với vật khác được — dù chưa ra số."* Lúc đó nợ của bài 3 được trả ở 6–9 chứ không ở 4, và bài kiểm bỏ-thử không còn nuốt nó.

**Bài 31 — bài duy nhất trong track không trả gì về sau.**
Không có bài nào từ 32 đến 36 dùng "mũ vượt tuyến tính". Đáng chú ý: đoạn "kiểm tính không thừa" của bạn biện hộ cho 5, 6, 11, 17, 24, 26, 29 — nhưng **không** nhắc 31 lần nào. Nó không rơi ra chỉ vì reflect(30) được viết riêng để cần tới nó; đổi reflect(30) một câu là 31 bốc hơi mà không bài nào phía sau kêu.
Nó vẫn đáng giữ (đỉnh trí tuệ của cụm ba họ hàm, và là hạt giống độ phức tạp cho Realm 4). Nhưng phải **trả nợ**, hai chỗ:
- **Sửa reflect(31):** hiện nó bẻ lái sang máy tính tiền — câu hỏi ngược không mọc ra từ nội dung bài 31 chút nào, nó gá vào. Cho câu hỏi ngược mọc từ chính men: *"Muốn men đủ 1000 phần thì phải chờ mấy giờ?"* Đó **đúng là** chạy ngược cái máy mũ, nó sinh ra từ bài 31, và nó rơi thẳng vào bài 32 (bài 32 rồi trả lời trên máy bậc nhất vì máy ấy gỡ được).
- **Sửa reflect(36) hoặc câu bắc cầu Realm 4:** cho bài 31 một chỗ được gọi tên lại (chương trình chạy `2ⁿ` bước). Không thì nó mãi là một bài đẹp không ai cần.

**Bài 12 — sống, nhưng bằng lý do yếu.** Bỏ 12 thì bài 13 mất từ vựng ("hai đĩa"), nên bài kiểm giữ nó lại. Đó là *sự cần thiết về từ vựng*, không phải về nhận thức — loại yếu nhất. Điều kiện để nó không tụt xuống thành hình minh hoạ: thân bài 12 phải bắt người học **phân loại thao tác nào giữ được thăng bằng, thao tác nào làm lệch**, và dự đoán trước khi bài 13 phát luật. Nếu bài 12 chỉ vẽ cái cân rồi nói "phương trình giống thế này", nó nên bị gộp vào 13.

**Cặp 8–9 là cặp sát nhau nhất còn lại.** Chúng tách được vì 8 rút *một con số* ra, còn 9 rút *chính chữ* ra và thêm ranh giới "hết gộp được". Muốn giữ khoảng cách đó: bài 8 chỉ được dùng nhân tử chung là số (`15000n + 30000`), tuyệt đối không đụng `3n + 5n` — nếu bài 8 lỡ làm ví dụ ấy thì bài 9 mất hết phần mới.

---

## 3. Đứt mạch

Ba mươi bốn mối nối khớp. Hai chỗ hở:

**11 → 12 (mối nối yếu nhất track).** reflect(11) hỏi *"có cách nào không phải thử từng số?"* Bài 12 **không** trả lời câu đó — nó đổi cách nhìn. Câu trả lời thật mãi 13–15 mới tới. Người học rời bài 12 vẫn chưa có cách nào ngoài quét.
**Sửa:** đổi reflect(11) để nó hỏi đúng thứ bài 12 đưa được, chứ đừng hứa thứ bài 12 không có. Ví dụ: *"Không thử từng số thì phải động thẳng vào chính câu ấy. Nhưng câu ấy là một lời khẳng định đang đúng — động vào mà nó hoá sai thì hỏng. Có vật gì ngoài đời cũng 'đang đúng' theo kiểu đó, và động vào nó người ta phải giữ gìn cái gì?"* Bài 12 trả lời trọn (cái cân), reflect(12) rồi mới hỏi "được phép động kiểu nào" → 13. Mạch kín, không phải hứa nợ.

**31 → 32 (bẻ lái, không phải nối tiếp).** Đã nói ở mục 2: reflect(31) hiện không mọc từ bài 31. Sửa bằng câu hỏi "mấy giờ thì men đủ 1000 phần".

Không còn chỗ nào khác. Đặc biệt các mối tôi ngờ nhất đều kín thật: 9→10 (câu hỏi *"n bằng bao nhiêu"* được để dành 9 bài rồi mới có nghĩa — đây là mối nối tốt nhất trong track), 19→20 ("đủ bù nghĩa là bằng hoặc hơn"), 21→22 ("một dòng của bảng hai cột ghi lên đâu"), 23→24 (phản ví dụ đẻ ra định nghĩa).

---

## 4. Khái niệm đến trước nhu cầu

**Bài 27 — `y = ax + b` là ký hiệu duy nhất trong track đến theo lịch sách giáo khoa.**
Bất tiện thật ở bài 27 là "hai xe cùng dốc, khác nhau ở đâu trên hình" — và nó được gỡ trọn vẹn bằng `f(n) = 15000n + 100000`, tức là ký hiệu track đã sở hữu từ bài 25. Chẳng có sự bất tiện nào đòi phải đổi `f`/`n` thành `y`/`x`. Nguy hiểm cụ thể: người học đang cầm ba cách viết cho cùng một thứ (`15000n`, `f(n) = 15000n`, `y = 15000x`) mà không bài nào hoà giải chúng.
**Sửa:** phát biểu khái niệm bài 27 nguyên trong `f(n) = an + b` — *"`b` là thứ máy nhả ra khi bỏ 0 vào, cũng là chỗ đường cắt trục dọc; hai con số `a`, `b` tả trọn một đường"*. Đẩy `y = ax + b` xuống một dòng phụ ("sách hay viết thế này, `y` chính là cột kết quả bạn đã chấm lên trục dọc từ bài 23"), hoặc dời hẳn sang bài 36. Đừng để nó chiếm chỗ của khái niệm thật.

**Bài 29 — "đối xứng" đến trước khi hiện vật cho phép nhìn thấy nó, và bài 32–33 lãnh hậu quả.**
Bài 28 dựng bậc hai trên **mảnh sân cạnh `n` mét**. Sân thì không có cạnh âm — nên trên chính hiện vật ấy, parabol chỉ có nửa bên phải: **không có đối xứng nào để thấy**, không có "hai đầu vào cho cùng một đầu ra" nào để phát hiện. Bài 29 buộc phải phát đối xứng như một sự thật về một bức hình chưa ai cần. Hậu quả dây chuyền: reflect(32) hỏi *"đưa 9 m² vào máy ngược, trả 3 hay −3?"* — trong khung mảnh sân, −3 chưa bao giờ là ứng viên, câu hỏi rỗng; và cách chữa của bài 33 ("chỉ nhận cạnh ≥ 0") không chữa gì cả, vì chưa có gì hỏng.

**Sửa — một gói ba bài, không thêm bài nào:**
- **28:** tách rõ **cái máy** khỏi **cái sân**. Máy `f(n) = n × n` nhận *mọi* số (T2.1 đã có số âm); mảnh sân chỉ là một lần dùng máy ấy. Chỉ cần một câu, nhưng nó làm −3 thành ứng viên sống ở bài 32 và làm "cạnh ≥ 0" thành một cái cắt thật ở bài 33.
- **29:** dạy đối xứng trên **đường lãi theo giá**, không trên mảnh sân. Đặt ô trống là *"tăng giá bao nhiêu nghìn so với giá đang bán"* — số âm nghĩa là hạ giá, hoàn toàn có nghĩa trong xe bánh mì. Lúc đó "hai mức giá khác nhau cho **cùng một** khoản lãi" là một quan sát có thật, gây sốc, và đỉnh là chỗ lãi cao nhất — đúng thứ reflect(28) đã hứa. (Hiện reflect(28) hứa một đường cong **có chỗ cao nhất**, tức parabol quay xuống, mà bài 28 chỉ dựng `n × n` quay lên — cái gói này vá luôn chỗ hụt đó.)
- **33:** mảnh sân quay lại đúng lúc, làm cách chữa: máy nhận mọi số thì hỏng, cái sân vốn không nhận số âm thì chạy ngược được.

Ngoài hai chỗ này, tôi không tìm thấy khái niệm nào đến sớm. Cân (12) đợi 11 mới vào, toạ độ (22) đợi 21, hàm (24) đợi phản ví dụ 23, ký hiệu `f` (25) đợi "ba cái máy gọi tên loạn", kiểm nghiệm (16) đợi chuỗi năm bước — tất cả đều đúng thứ tự bất tiện-trước-tên-sau.

---

## Ngoài bốn tiêu chí — bốn cái sẽ cắn khi viết thân bài

- **`0 = 0` ở bài 14 nói ngược với `0 = 0` ở bài 19.** Bài 14 dạy: nhân hai vế với 0 thì *hỏng*, mọi phương trình thành `0 = 0`. Bài 19 dạy: ra `0 = 0` nghĩa là *mọi số đều là nghiệm*. Cả hai đều đúng, nhưng người học sẽ va nhau. Bài 19 phải gọi lại bài 14 một câu: *"bạn đọc `0 = 0` là 'mọi số' chỉ vì mọi bước bạn vừa đi đều là phép giữ nghiệm — lỡ nhân hai vế với 0 thì `0 = 0` chẳng nói gì hết."*
- **reflect(11) dùng nghiệm `2,5`** trong ngữ cảnh số ổ bánh mì — 2,5 ổ là vô nghĩa, và người học tinh ý sẽ mất tin vào hiện vật. Đổi sang đại lượng chia lẻ được (kg bột, lít nước) hoặc bỏ hẳn, giữ mỗi lý do "nghiệm có bảy chữ số".
- **Bài 24 cần một phản ví dụ sống trong xe bánh mì**, không phải giả định. reflect(23) đang hỏi "nếu có" — nếu bài 24 không chỉ ra được một cái máy thật của quán vi phạm luật (ví dụ "số ổ đã bán → lúc mấy giờ" — cùng một con số ứng với hai thời điểm trong ngày), thì phần "phân loại máy nào được, máy nào không" tụt xuống thành bài tập phiếu, và định nghĩa lại thành *được phát* chứ không *được phát hiện*.
- **Không bài nào dạy việc dịch câu hỏi của quán thành câu tính.** Track dùng kỹ năng này ở mọi bài và BOSS 36 đòi nó ở mức cao nhất ("dựng cả một buổi bán hàng"), nhưng nó chưa từng được gọi tên hay luyện riêng. Không cần thêm bài — nhưng nên có ít nhất hai reflect (gợi ý: bài 10 và bài 19) hỏi thẳng *"câu hỏi này của quán viết thành câu tính nào?"* thay vì đưa sẵn phương trình.

---

## T2.3 — Logic & chứng minh (Realm 2 · Toán & Toán rời rạc · Python CHỈ để KIỂM · 32 bài)

## Mạch T2.3 — Logic & chứng minh

Người học vào track đã xong R0 + R1 (biết `True/False`, `and/or/not`, `if/elif/else`, `while`, biến cộng dồn, biến cờ, `break`, `list`, `def/return`, đọc traceback) và xong T2.1 (chẵn/lẻ, chia hết, số nguyên tố, phân số tối giản) + T2.2 (biến, biểu thức, công thức theo `n`, hàm số).

**Hiện vật xuyên suốt:** CLB cờ vua lớp 6A — 6 thành viên (Nam, Lan, Minh, Hoa, Tú, Khanh), một bảng nội quy, một sổ quỹ. Bài 21 là chỗ hiện vật **cố ý gãy**: 6 người thì kiểm hết được, số tự nhiên thì không — và chính cú gãy đó là cửa vào nửa chứng minh. Bài 30–32 quay về đúng vòng `while` cộng dồn của T1.2.

**Python trong track này không bao giờ là lời giải.** Nó dựng bảng chân lý (`itertools.product`), kiểm một câu trên 6 thành viên (`all` / `any`), đi săn phản ví dụ, và cuối cùng viết bất biến thành `assert` trong thân vòng. Bài 21 dạy thẳng: máy nói "chưa thấy sai", không bao giờ nói "đúng".

| # | slug | Tiêu đề | Khái niệm mới (đúng một) | `reflect` cuối bài | Dựa trên |
|---|---|---|---|---|---|
| 1 | `cau-nao-phan-xu-duoc` | Câu nào phân xử được | **Mệnh đề** — câu mà chuyện nó đúng hay sai là chuyện phân xử được (khác câu hỏi, câu sai khiến, câu nêu ý thích) | "Sân trường rộng" — Nam bảo đúng, Lan bảo sai, và không ai nói dối cả. Còn "CLB có 6 thành viên" thì đếm một cái là xong. Hai câu ấy khác nhau ở chỗ nào? | R0.24 `dung-hay-sai` |
| 2 | `dung-sai-do-su-viec` | Đúng hay sai do sự việc, không do người nói | **Giá trị chân lý** — mỗi mệnh đề mang đúng một trong hai giá trị; "chưa biết" là chuyện của người đang xét, không phải giá trị thứ ba | "Nam đã nộp quỹ" là đúng. Vậy câu "Nam chưa nộp quỹ" mang giá trị gì? Có bao giờ hai câu ấy cùng đúng không? Cùng sai? | 1, R0.24 |
| 3 | `noi-nguoc-lai-mot-cau` | Nói ngược lại một câu | **Phủ định** — dựng từ một mệnh đề một mệnh đề mới luôn mang giá trị ngược lại; trong hai câu ấy luôn có đúng một câu đúng | Ghép hai câu bằng chữ "và": "Nam ngã **và** Nam khóc" nghe khác "Nam khóc **và** Nam ngã". Logic có nghe ra chỗ khác nhau đó không? | 2, T1.2.5 `noi-nguoc-lai` |
| 4 | `va-chi-nhin-hai-gia-tri` | Phép "và" chỉ nhìn hai giá trị | **Hội (và)** — giá trị của câu ghép chỉ do giá trị Đ/S của hai vế quyết định, không do nội dung, thứ tự hay quan hệ nhân quả giữa chúng | Ở quán, "gọi trà **hoặc** cà phê" nghĩa là chọn một. Nếu bạn bê về cả hai cốc, câu ấy còn đúng không? | 3, R0.29 `hai-dieu-kien-cung-dung` |
| 5 | `hoac-gom-ca-hai` | "Hoặc" gồm cả hai | **Tuyển (hoặc)** — đúng khi **ít nhất một** vế đúng, kể cả khi cả hai cùng đúng | Bạn vừa xét bốn kiểu điền Đ/S cho hai vế. Lấy gì bảo đảm bốn kiểu ấy là đủ, không sót? Ba vế thì bao nhiêu kiểu? | 4, R0.30 `chi-can-mot-ve-dung` |
| 6 | `xet-du-moi-truong-hop` | Xét đủ mọi trường hợp | **Bảng chân lý** — liệt kê có hệ thống mọi tổ hợp giá trị của các vế (n vế → 2ⁿ dòng), mỗi tổ hợp đúng một lần, không sót không lặp | Dựng bảng cho câu "P hoặc không P": cột kết quả toàn Đ, dù P là câu gì đi nữa. Bạn dựng nhầm, hay câu ấy có gì lạ? | 5, T1.2.6 `ai-duoc-tinh-truoc` |
| 7 | `cot-ket-qua-khong-doi` | Khi cột kết quả không đổi | **Hằng đúng và hằng sai** — có những câu ghép đúng ở mọi dòng, và những câu sai ở mọi dòng ("P và không P"); giá trị của chúng không phụ thuộc sự việc nào cả | "Không phải là không P" — dựng bảng thì cột của nó trùng khít cột của "P". Hai câu viết khác chữ mà cùng một bảng thì có thay nhau được ở mọi chỗ không? | 6, 3 |
| 8 | `cung-bang-thi-thay-duoc-nhau` | Cùng một bảng thì thay được cho nhau | **Tương đương logic** — hai câu có bảng chân lý trùng khít thì thay thế nhau ở bất kỳ chỗ nào mà không đổi giá trị của câu bao ngoài | "Không phải cả Nam và Lan đều nộp quỹ" — nhiều người đọc thành "cả hai đều chưa nộp". Dựng hai bảng xem chúng có trùng nhau không. Nếu không, thì câu nào mới đúng là phủ định? | 7, 6 |
| 9 | `phu-dinh-di-vao-trong` | Phủ định đi vào trong thì "và" hoá "hoặc" | **Luật De Morgan** — đẩy phủ định vào trong ngoặc thì "và" đổi thành "hoặc" và ngược lại | Bảng nội quy CLB có dòng: "Nếu là thành viên thì phải đeo thẻ." Dòng này cũng ghép hai vế — nhưng không phải "và", cũng không phải "hoặc". **Khi nào** thì dòng nội quy ấy bị coi là sai? | 8, 4, 5 |
| 10 | `neu-thi-la-mot-luat` | "Nếu... thì" là một lời hứa | **Kéo theo (P → Q)** — câu ghép chỉ **sai ở đúng một dòng**: vế trước đúng mà vế sau sai; luật chỉ bị phá bởi một thành viên không đeo thẻ | Bảng còn hai dòng chưa ai bàn tới: hai dòng "vế trước sai". Một người **không** phải thành viên, không đeo thẻ — nội quy có bị phá không? | 9, 2 |
| 11 | `khi-ve-truoc-khong-xay-ra` | Khi vế trước không xảy ra | **Chân lý rỗng — sai kéo theo bất kỳ** — vế trước sai thì cả câu đúng, bất kể vế sau, vì không ai vi phạm được; "P → Q đúng" **không** có nghĩa "Q đúng" | Nội quy đúng, và bạn thấy một người đang đeo thẻ. Kết luận "người đó là thành viên" — chắc chưa? | 10 |
| 12 | `doi-cho-hai-ve` | Đổi chỗ hai vế | **Mệnh đề đảo (Q → P)** — đổi chỗ hai vế cho ra một câu **khác**, giá trị có thể khác hẳn câu gốc | Thử cách khác: giữ nguyên thứ tự nhưng phủ định cả hai vế — "không phải thành viên thì không phải đeo thẻ". Câu này có phải câu gốc không? Bảng của nó giống bảng của ai? | 11, 10 |
| 13 | `phu-dinh-ca-hai-ve` | Phủ định cả hai vế | **Mệnh đề phản (¬P → ¬Q)** — cũng là một câu khác câu gốc, và bảng của nó **trùng khít bảng mệnh đề đảo** | Đổi chỗ thì hỏng. Phủ định thì hỏng. Làm **cả hai** cùng lúc thì sao — hai cái hỏng có bù nhau không? | 12, 8, 3 |
| 14 | `vua-doi-cho-vua-phu-dinh` | Vừa đổi chỗ vừa phủ định | **Phản đảo (¬Q → ¬P) tương đương câu gốc** — bảng trùng khít câu gốc, nên ở mọi chỗ hai câu ấy thay được cho nhau | Câu gốc và phản đảo luôn đi cùng nhau; câu gốc và đảo thì không. Nhưng nếu **cả gốc lẫn đảo** cùng đúng thì sao? Thư viện trường viết: "được mượn sách **khi và chỉ khi** có thẻ." Ba chữ "và chỉ khi" thêm vào điều gì? | 13, 12, 8 |
| 15 | `khi-va-chi-khi` | Khi và chỉ khi | **Tương đương hai chiều (P ↔ Q)** — khẳng định cả P → Q lẫn Q → P cùng lúc; đây là dạng của mọi **định nghĩa**, nên định nghĩa dùng được theo cả hai hướng | Mọi câu từ đầu track tới giờ đều nói về **một** người có tên. Nhưng nội quy viết "mọi thành viên đều đeo thẻ" — không nêu tên ai. Riêng câu "bạn ấy đeo thẻ" thì đúng hay sai? | 14, 12, 10 |
| 16 | `cau-con-cho-trong` | Câu còn một chỗ trống | **Câu mở (vị từ)** — câu có chỗ trống thì **chưa** mang giá trị chân lý; điền một thành viên vào mới thành mệnh đề (trong Python: một hàm nhận tên, trả `True`/`False`) | Điền lần lượt 6 cái tên thì được 6 mệnh đề, 6 giá trị Đ/S. Câu "mọi thành viên đều đeo thẻ" thu 6 giá trị ấy về thành **một** giá trị bằng cách nào? | 15, 1, T2.2 (hàm số) |
| 17 | `voi-moi-la-va-keo-dai` | "Với mọi" là chữ "và" kéo dài | **Lượng từ với mọi (∀)** — biến câu mở thành mệnh đề; đúng khi **tất cả** mệnh đề con đều đúng, tức là chữ "và" nối 6 lần | Nếu "với mọi" chỉ là "và" kéo dài, thì "hoặc" kéo dài đọc thành câu tiếng Việt gì? | 16, 4 |
| 18 | `ton-tai-la-hoac-keo-dai` | "Tồn tại" là chữ "hoặc" kéo dài | **Lượng từ tồn tại (∃)** — đúng khi **ít nhất một** mệnh đề con đúng | Muốn khẳng định "tồn tại", chỉ cần chỉ ra một người là xong. Muốn khẳng định "với mọi", phải xét cả 6. Vậy muốn **bác bỏ** "với mọi" thì cần mấy người? | 17, 5 |
| 19 | `mot-nguoi-la-du-de-bac-bo` | Một người là đủ để bác bỏ | **Phản ví dụ** — một trường hợp làm câu mở sai là đủ, và là **cách duy nhất**, để bác bỏ một câu "với mọi" (hệ quả ngã ra ngay: tổ nào không có ai thì không có phản ví dụ nào — `all([])` là `True`) | "Không phải mọi thành viên đều đeo thẻ" và "tồn tại một thành viên không đeo thẻ" — hai câu ấy là một hay là hai? Còn phủ định của "tồn tại" thì đọc ra sao? | 18, 17, 3 |
| 20 | `phu-dinh-cau-co-luong-tu` | Phủ định một câu có lượng từ | **Phủ định lượng từ** — ¬∀ thành ∃¬, ¬∃ thành ∀¬: chính là De Morgan kéo dài | 6 thành viên thì máy kiểm hết trong một nháy. Nhưng "với mọi số tự nhiên n, n² + n + 41 là số nguyên tố" — máy thử 39 số đầu, không sai lần nào. Đã kết luận được chưa? | 19, 9, 17, 18 |
| 21 | `kiem-nghin-lan-van-chua-du` | Kiểm nghìn lần vẫn chưa đủ | **Kiểm hữu hạn không phải chứng minh** — máy chỉ nói được về những trường hợp nó đã xét; câu trên đúng 40 lần rồi **sai ở n = 40** | Máy chỉ nói được về những số nó đã thử. Muốn nói một câu đúng cho **mọi** số chẵn mà không thử số nào, bạn phải làm việc với chữ "chẵn" chứ không với từng con số. Vậy "chẵn" viết ra thành cái gì mà tính toán được? | 20, 19, T2.1 (số nguyên tố) |
| 22 | `mo-dinh-nghia-ra` | Mở định nghĩa ra | **Dùng định nghĩa** — thay chữ bằng dạng tính được: "n chẵn" nghĩa là "có số nguyên k để n = 2k"; vì định nghĩa là câu hai chiều nên dùng được cả chiều mở ra lẫn chiều gói lại | Có dạng n = 2k rồi. Lấy **hai số chẵn bất kỳ** cộng lại — nhưng "bất kỳ" thì viết ra là số nào? | 21, 15, T2.1, T2.2 |
| 23 | `xet-mot-so-bat-ky` | Xét một số bất kỳ | **Chứng minh trực tiếp** — đặt tên cho một đối tượng bất kỳ thoả giả thiết, chỉ dùng giả thiết + định nghĩa + điều đã chứng minh, đi tới kết luận; vì không dùng gì riêng của nó nên lập luận đúng cho mọi trường hợp | Thử câu "nếu n² chẵn thì n chẵn". Mở định nghĩa ra được n² = 2k, rồi tắc — từ đó không nặn ra được n = 2m. Tắc thì bỏ à? | 22, 10 |
| 24 | `chung-minh-cau-phan-dao-thay` | Chứng minh câu phản đảo thay cho câu gốc | **Chứng minh bằng phản đảo** — vì phản đảo tương đương câu gốc (bài 14), chứng minh "n lẻ thì n² lẻ" là đã chứng minh xong câu gốc | Phản đảo chỉ dùng được cho câu có dạng "nếu... thì". Còn "√2 không viết được thành phân số" — làm gì có vế trước để mà đảo. Bắt đầu từ đâu? | 23, 14, 3 |
| 25 | `gia-su-dieu-nguoc-lai` | Giả sử điều ngược lại | **Chứng minh phản chứng** — giả sử phủ định của điều cần chứng minh, suy ra một câu **hằng sai** ("P và không P"), nên điều giả sử ấy sai | Phản chứng đưa bạn tới **một** câu, xong là hết. Nhưng "1 + 2 + ... + n = n(n+1)/2 với mọi n" không phải một câu — nó là vô hạn câu, mỗi n một câu. Mà câu thứ k+1 chỉ hơn câu thứ k đúng **một số hạng**. Tận dụng được chỗ "chỉ hơn một chút" đó không? | 24, 7, 3, T2.1 (phân số tối giản) |
| 26 | `hai-viec-du-de-do-ca-hang` | Hai việc là đủ để đổ cả hàng | **Nguyên lý quy nạp** — chứng minh vô hạn mệnh đề bằng đúng hai việc: câu đầu tiên đúng, và **từ** câu thứ k đúng **suy ra** câu thứ k+1 đúng (hàng domino, bậc thang) | Việc thứ hai bảo "giả sử bậc k đúng" — tức là giả sử chính cái mình đang muốn chứng minh? Nghe như đi vòng tròn. | 25, 10, T2.2 (công thức theo n) |
| 27 | `duoc-phep-muon-bac-truoc` | Được phép mượn bậc ngay trước | **Giả thiết quy nạp** — trong bước quy nạp, "P(k) đúng" là **giả thiết của một câu kéo theo**, không phải điều đã có sẵn; không vòng tròn vì bậc đầu chẳng mượn ai, mỗi bậc sau chỉ mượn bậc đã đổ | Bước quy nạp của câu "n = n + 1" cũng đúng: nếu k = k+1 thì cộng 1 hai vế được k+1 = k+2. Vậy mọi số đều bằng số liền sau nó? | 26, 10, 11 |
| 28 | `khong-co-bac-dau-thi-khong-do` | Không có bậc đầu thì không đổ | **Cơ sở quy nạp là bắt buộc** — bước quy nạp chỉ **chuyển tiếp**, nó không khởi động được gì; thiếu cơ sở thì cả dây chuyền không bao giờ bắt đầu | Có cơ sở, có bước. Nhưng câu "mọi số ≥ 2 đều viết được thành tích các số nguyên tố": để tách 12 = 3 × 4 bạn cần biết **4** tách được, chứ chẳng cần biết gì về 11. Bước quy nạp chỉ cho mượn bậc liền trước — thế thì tắc? | 27, 26 |
| 29 | `muon-tat-ca-cac-bac-duoi` | Mượn tất cả các bậc dưới | **Quy nạp mạnh** — bước quy nạp được phép giả sử **mọi** mệnh đề từ cơ sở tới k, không chỉ riêng bậc k | Quy nạp chạy theo "bậc thứ n". Vòng `while` cũng chạy theo lượt: lượt 1, lượt 2, lượt 3... Trong vòng cộng dồn sổ quỹ CLB, sau **mỗi** lượt có câu nào cứ đúng đi đúng lại không? | 28, 27, T2.1 (số nguyên tố) |
| 30 | `dieu-dung-lai-sau-moi-luot` | Điều đúng lại sau mỗi lượt | **Bất biến vòng lặp** — một câu đúng trước lượt đầu, và hễ đúng trước một lượt thì đúng sau lượt đó; chứng minh nó **chính là quy nạp theo số lượt** (cơ sở = trước vòng, bước = một lượt thân vòng) — viết thành `assert` trong thân vòng để máy KIỂM | Bất biến đúng suốt vòng. Nhưng nếu vòng cứ chạy mãi thì nó cứ đúng mãi mà bạn chẳng bao giờ có kết quả trong tay. Ở T1.2 bạn đã gặp vòng không chịu dừng. Lấy gì bảo đảm vòng này dừng? | 29, 26, T1.2.9 `xem-lai-dieu-kien-luc-nao`, T1.2.12 `cong-don-qua-tung-luot` |
| 31 | `vi-sao-vong-chac-chan-dung` | Vì sao vòng chắc chắn dừng | **Thước đo dừng** — chỉ ra một đại lượng nguyên không âm giảm ít nhất 1 mỗi lượt; số tự nhiên không giảm mãi được (chính là quy nạp nhìn ngược), nên vòng phải dừng — đây là bằng chứng cho "bước tiến" mà T1.2 mới chỉ dặn | Vòng dừng rồi, và bất biến vẫn đúng. Ghép hai điều đó lại đã đủ để nói `tong` bằng đúng tổng quỹ 6 tháng chưa — hay còn thiếu một mẩu tin nữa, mẩu mà **chỉ lúc thoát** mới có? | 30, 26, T1.2.10 `vong-lap-khong-chiu-dung` |
| 32 | `boss-bang-chung-cho-mot-vong-lap` | BOSS: Bằng chứng cho một vòng lặp | *(không khái niệm mới — bài tổng hợp)* mẩu tin còn thiếu là: **lúc thoát, điều kiện lặp sai**; ghép bất biến + điều kiện lặp sai + thước đo dừng thành một bằng chứng trọn vẹn cho vòng cộng quỹ, rồi dùng Python `assert` chỉ để kiểm lại | Bằng chứng bạn vừa viết bằng tiếng Việt, máy không đọc được — nên vẫn phải chạy test mới yên tâm. Có cách nào nói cho máy nghe **một phần** bằng chứng, để nó bắt lỗi trước cả khi chạy? *(dẫn sang R3 — bit, byte & kiểu dữ liệu, và xa hơn là kiểm chứng chương trình)* | 22–31, T1.2.12, T1.2.23 `la-co-nho-mot-su-that` |

**Vì sao thứ tự này đúng**

## Vì sao thứ tự này là thứ tự đúng

**0. Một trục duy nhất: "câu này đúng hay sai, và làm sao bạn biết".** Nửa đầu (1–20) mài một câu cho tới khi nó nói được mọi thứ và bị phủ định đúng cách. Nửa sau (21–32) hỏi lấy gì bảo đảm nó đúng. Bài 21 là bản lề của cả track: nó không dạy công cụ nào, nó **phá vỡ** công cụ mà 20 bài trước vừa dựng lên (máy kiểm hết 6 thành viên trong một nháy — rồi gặp miền vô hạn thì chịu). Mọi phương pháp chứng minh sau đó xuất hiện như thứ đi gỡ đúng chỗ tắc vừa lộ ra, không phải như mục lục của một cuốn sách logic.

**1. Vì sao `and`/`or`/`not` được dựng lại chứ không dạy lại.** Người học đã gõ `and`, `or`, `not` suốt T1.2 — dạy lại là xúc phạm. Cái mới ở bài 4–5 không phải toán tử mà là một sự thật họ chưa từng bị hỏi: **giá trị câu ghép chỉ do giá trị hai vế quyết định** — nên "Nam ngã và Nam khóc" bằng "Nam khóc và Nam ngã", điều mà tiếng Việt không chịu. Đó là lần đầu logic tách khỏi ngôn ngữ đời thường, và nó phải xảy ra trước bảng chân lý, vì bảng chân lý chỉ có nghĩa khi bạn đã chấp nhận rằng nội dung hai vế không còn quan trọng nữa. Bài 5 (hoặc bao gồm cả hai) đặt ngay sau vì đó là chỗ tiếng Việt cãi lại lần thứ hai, và cãi mạnh hơn.

**2. Vì sao bảng chân lý ở bài 6 chứ không phải bài 1.** Bảng chân lý là công cụ **trả lời một câu hỏi** — "lấy gì bảo đảm tôi xét đủ?" — và câu hỏi ấy chỉ có sau khi bài 5 bắt người học tự liệt kê bốn trường hợp bằng tay và thấy mình không chắc. Mở track bằng bảng chân lý thì nó là một cái bảng phải chép. Mở nó ở bài 6 thì nó là sự nhẹ nhõm. Ngay sau đó, bài 7 (hằng đúng / hằng sai) không phải phần lý thuyết thêm: nó là thứ người học **tự vấp phải** khi dựng bảng cho "P hoặc không P" và thấy cột kết quả toàn Đ — đúng luật New Math số 3, định nghĩa được phát hiện chứ không được phát. Và nó là món nợ trả ở bài 25: phản chứng không nói nổi nếu chưa có chữ "hằng sai".

**3. Vì sao kéo theo phải đứng sau De Morgan, và chân lý rỗng phải đứng riêng một bài.** "Sai suy ra bất kỳ" là chỗ người học rụng nhiều nhất trong mọi giáo trình logic, vì nó bị dạy như một quy ước phải nuốt. Ở đây nó không phải quy ước: bài 10 định nghĩa kéo theo qua **nội quy CLB bị phá lúc nào**, và câu trả lời "chỉ bị phá bởi một thành viên không đeo thẻ" là điều bất kỳ đứa trẻ nào cũng gật đầu. Hai dòng "vế trước sai" khi đó không còn là quy ước — chúng là hệ quả không thể chối: người ngoài CLB thì không phá được nội quy của CLB. Tách bài 11 ra riêng vì nó mang **hai** hệ quả trái ngược trực giác cần thời gian riêng: câu đúng mà chẳng nói gì về vế sau, và câu đúng mà chẳng cần ai kiểm. Gộp nó vào bài 10 là bảo đảm cả hai đều trượt.

**4. Vì sao đảo — phản — phản đảo là ba bài chứ không phải một.** Sách thường gói ba câu họ hàng vào một trang bảng đối chiếu, và người học thuộc bảng mà vẫn nhầm suốt đời. Ba bài ở đây có ba nội dung khác nhau hẳn: bài 12 phát hiện **đổi chỗ thì hỏng**, bài 13 phát hiện **phủ định cũng hỏng, và hỏng y hệt kiểu vừa rồi** (bảng của phản trùng bảng của đảo — một cú bất ngờ thật sự, không phải một dòng trong bảng), bài 14 phát hiện **hai cái hỏng bù nhau**. Mỗi bài là một lần dựng bảng và một lần bị bất ngờ. Và bài 14 không phải điểm dừng: nó là công cụ được rút ra dùng ở bài 24, cách đó mười bài — đúng kiểu công cụ được cất đi rồi lấy ra đúng lúc tắc.

**5. Vì sao lượng từ được dựng thành "và kéo dài" / "hoặc kéo dài".** Trên một CLB 6 người (luật New Math số 4: 5–7 phần tử, đồ vật thật), ∀ và ∃ **thật sự là** chuỗi và/hoặc dài — nên bài 17–18 không giới thiệu gì mới về bản chất, chúng chỉ đặt tên cho thứ người học đã có. Cái giá phải trả là bài 16 (câu mở) bắt buộc phải đứng trước: chưa có chỗ trống thì chưa có gì để lượng hoá. Cái được trả lại là bài 20: phủ định lượng từ không phải luật mới phải học thuộc, nó là De Morgan bài 9 kéo dài — và người học **đoán ra được** trước khi được nói. Đây cũng là lý do bài 19 (phản ví dụ) chen vào giữa 18 và 20: nó tạo ra sự bất đối xứng sắc nhất của track (bác bỏ "mọi" tốn một người, bác bỏ "tồn tại" tốn cả sáu), và chính sự bất đối xứng ấy là thứ bài 20 tổng quát hoá thành luật.

**6. Vì sao chứng minh trực tiếp phải chờ tới bài 23.** Không ai chứng minh được điều gì về "số chẵn" chừng nào "chẵn" còn là một chữ. Bài 22 (mở định nghĩa) là bậc thang bị thiếu trong hầu hết giáo trình: nó biến chữ thành n = 2k, và nó chỉ hợp lệ vì bài 15 đã cho biết định nghĩa là câu **hai chiều** nên đi được cả hai lượt. Thứ tự ba phương pháp cũng có lý do: trực tiếp (23) → phản đảo (24) → phản chứng (25), theo đúng thứ tự **độ tắc** mà người học vừa gặp. Phản đảo sinh ra từ một bế tắc cụ thể ở cuối bài 23 ("n² = 2k rồi tắc"), không sinh ra vì tới lượt. Phản chứng đứng cuối vì nó là thứ duy nhất dùng được cho câu **không có** dạng "nếu... thì" — và bài 24 phải để hở đúng chỗ đó trước.

**7. Vì sao quy nạp tách làm bốn bài, và tách theo đúng bốn kiểu hiểu sai.** 26 dựng nguyên lý; 27 gỡ nghi ngờ "vòng tròn" — nghi ngờ này **luôn** xuất hiện và nếu không gỡ thì người học làm bài đúng mà không tin bài mình làm; 28 cho thấy bước đúng mà thiếu cơ sở thì kết luận sai bét (n = n+1), tức là cơ sở không phải thủ tục hành chính; 29 mở ra quy nạp mạnh **chỉ khi** bài 28 vừa dựng một bài toán mà bậc liền trước vô dụng (12 cần 4, không cần 11). Nếu dạy quy nạp mạnh ngay sau quy nạp thường, nó chỉ là "phiên bản mạnh hơn, cứ dùng cái này cho chắc" — và người học mất hẳn cảm giác vì sao có hai thứ.

**8. Vì sao track kết bằng vòng lặp, và kết bằng đúng vòng lặp của T1.2.** Bất biến vòng lặp (30) **không phải khái niệm mới** theo nghĩa toán học — nó là quy nạp với "bậc n" đổi tên thành "sau lượt n". Đặt nó ngay sau quy nạp mạnh là để người học nhận ra điều đó chứ không phải học thêm một thứ. Ba bài cuối trả ba món nợ của T1.2, đúng theo thứ tự T1.2 đã vay: bài 30 trả cho `tong = tong + tien` (T1.2.12 chỉ dặn "biến tích luỹ phải sinh trước vòng" mà không nói tại sao đúng), bài 31 trả cho "mỗi `while` cần một bước tiến" (T1.2.10 dặn suông — giờ mới có bằng chứng: thước đo giảm trên số tự nhiên), bài 32 trả cho toàn bộ BOSS sổ chi tiêu (chạy 30 test xanh, nhưng xanh không phải đúng — chính là bài 21 nói lại lần cuối, ở nơi nó cắn đau nhất).

**9. Python xuất hiện ở đâu và không được xuất hiện ở đâu.** Python dựng bảng chân lý (6–9), kiểm câu trên 6 thành viên bằng `all`/`any` (17–19), săn phản ví dụ (20–21), và viết bất biến thành `assert` (30–32). Nó **không bao giờ** được phép là câu trả lời: bài 21 tồn tại chính để đóng cánh cửa đó lại, và mọi bài sau 21 đều dùng Python theo đúng một vai — kiểm lại thứ đã chứng minh, hoặc bác bỏ thứ chưa chứng minh. Đây cũng là chỗ hai luật CRA được tôn trọng: chữ tiếng Việt trước (Task 1–3), rồi `and`/`or`/`not` của Python làm bậc **biểu diễn** trung gian mà người học đã quen, ký hiệu ¬ ∧ ∨ → ↔ ∀ ∃ mới xuất hiện cuối cùng, kèm cách đọc.

**10. Kiểm tính không thừa — bỏ một bài thì mạch đứt ở đâu.** Bỏ 7 thì 25 không có chữ "hằng sai" để dẫn tới. Bỏ 8 thì 13, 14, 20, 24 mất tiêu chuẩn "trùng bảng thì thay được" và biến thành bốn mẹo rời. Bỏ 11 thì 19 (miền rỗng) và 32 (vòng chạy 0 lượt vẫn đúng) mất gốc. Bỏ 15 thì 22 không có quyền mở định nghĩa theo chiều ngược. Bỏ 16 thì 17 lượng hoá trên hư không. Bỏ 21 thì cả nửa sau của track không có lý do tồn tại. Bỏ 27 thì người học làm quy nạp mà không tin. Bỏ 28 thì 29 không có bế tắc để gỡ. Bỏ 31 thì bài 32 nói về kết quả của một vòng chưa chắc đã có kết quả. Ở chiều ngược lại, không bài nào dạy lại thứ R0/R1 đã dạy: `True/False`, `and/or/not`, `while`, biến cộng dồn, biến cờ được **dùng lại liên tục** mà không được giới thiệu lại lần nào.

**11. Ranh giới với các track lân cận.** Tập hợp, quan hệ, ánh xạ, lực lượng để nguyên cho T2.4 — nên mọi "với mọi" ở đây chạy trên một danh sách 6 tên hoặc trên số tự nhiên, không bài nào cần ký hiệu ∈ hay {x | ...}. Đếm số dòng bảng chân lý dừng ở "gấp đôi mỗi lần thêm một vế", phần 2ⁿ và tổ hợp để cho T2.5. Bảng chân lý cho mạch điện, biểu thức chuẩn tắc, thoả được (SAT) không đụng tới — chúng thuộc R3. Curry–Howard, chứng minh là chương trình, kiểu phụ thuộc đã được MASTERPLAN đẩy lên đỉnh T2.6, nên bài 32 chỉ **trỏ** sang chứ không mở ra. Cả track đúng một bài không mang khái niệm mới (bài 32), giữ đúng tỉ lệ của R0 và T1.2.

### Phản biện độc lập

Đã đối chiếu với `content/onboarding/MACH.md` (R0), `content/nen-tang/MACH.md` (T1.2), và `MASTERPLAN.md` §8.4. Mọi tham chiếu R0/T1.2 trong bảng đều **đúng số, đúng slug** (R0.24/29/30, T1.2.5/6/9/10/12/23 — kiểm từng dòng). Fact toán ở bài 21 cũng đúng: n²+n+41 nguyên tố với n = 0…39, hỏng đúng ở n = 40 (= 41²).

Chuỗi bản lề 1→32 **liền, không đứt**: tôi kiểm từng cặp N→N+1, cả 31 cặp đều có bài sau trả lời đúng câu hỏi bài trước để hở. Đó là chỗ mạch này mạnh nhất và tôi không bịa ra vấn đề ở đó.

Nhưng có 8 lỗi thật. Xếp theo mức nghiêm trọng.

---

## A. LỖI NẶNG

### A1. Không có bài nào dạy MODUS PONENS — mà bài 23–32 chạy bằng nó
Track có **15 bài** về ngữ nghĩa của `→` (10, 11, 12, 13, 14, 15) nhưng **không một bài nào** dạy cách *dùng* một câu kéo theo: có `P → Q` đúng **và** có `P` đúng thì rút ra `Q`. Bài 11 chỉ dạy mặt phủ định của nó ("`P → Q` đúng **không** có nghĩa `Q` đúng") — tức là dạy cái anti-rule mà chưa bao giờ dạy cái rule.

Chỗ nó cắn:
- Bài 23 "chỉ dùng giả thiết + định nghĩa + điều đã chứng minh, **đi tới kết luận**" — động cơ của "đi tới" chính là tách rời.
- Bài 27 lập luận "không vòng tròn vì bậc đầu chẳng mượn ai, mỗi bậc sau chỉ mượn bậc đã đổ". Lập luận này **chỉ đọc được** nếu người học đã biết: có `P(1)` đúng, có `P(1) → P(2)` đúng, nên `P(2)` đúng. Nếu chưa biết, bài 27 — bài quan trọng nhất khối quy nạp — trở thành lời trấn an chứ không phải lời giải thích.
- Bài 30 (bất biến) là modus ponens lặp lại theo lượt.

**Sửa (không tốn thêm bài):** đổi trục bài 11 từ *"khi vế trước không xảy ra"* thành *"một luật cho bạn cái gì, và không cho bạn cái gì"*. Nội quy "thành viên thì đeo thẻ" + "Nam là thành viên" ⟹ **Nam đeo thẻ** (đây là cách *duy nhất* một luật sinh ra kết luận). Cũng nội quy đó + "Nam không phải thành viên" ⟹ **luật im lặng, và im lặng thì không ai phá được** — chân lý rỗng rơi ra như hệ quả, không còn là quy ước phải nuốt. Cũng nội quy đó + "Nam đeo thẻ" ⟹ **luật vẫn im lặng** → dẫn thẳng sang bài 12.

Cách này còn chữa luôn chỗ chính người thiết kế tự thú trong lập luận §3: *"Tách bài 11 ra riêng vì nó mang **hai** hệ quả trái ngược trực giác"* — hai hệ quả trong một bài đúng là dấu hiệu tiêu chí 1. Gộp lại dưới một trục "luật chỉ nói khi vế trước khớp" thì thành **một** khái niệm, và khái niệm đó là thứ đang thiếu.

*(Phương án dự phòng nếu muốn giữ nguyên bài 11: gộp 12+13 thành một bài "hai cách làm hỏng, cùng một kiểu hỏng" để lấy một slot cho modus ponens. Tôi không khuyến nghị — lập luận §4 bảo vệ việc tách 12/13/14 khá vững.)*

### A2. Bài 5 dạy lại nguyên văn R0.30 — và bài 3 dạy lại T1.2.5
Lập luận §10 khẳng định *"không bài nào dạy lại thứ R0/R1 đã dạy"*. Sai ở hai chỗ, kiểm được bằng chữ:

- **R0.30 `chi-can-mot-ve-dung`**: "`or` cho `True` khi **ít nhất một** vế đúng."
- **Bài 5**, cột khái niệm: "Tuyển (hoặc) — đúng khi **ít nhất một** vế đúng, kể cả khi cả hai cùng đúng."

Nửa đầu là R0.30 chép lại. Cái mới **chỉ là** nửa sau — "hoặc" tiếng Việt loại trừ, `or` của logic bao gồm. **Sửa:** viết lại cột khái niệm bài 5 thành đúng cái đó: *"'Hoặc' của logic bao gồm cả hai — khác 'hoặc' tiếng Việt vốn ngầm hiểu là chọn một"*. Bỏ mệnh đề "ít nhất một vế đúng" khỏi cột khái niệm, đẩy nó xuống phần ôn.

Tương tự:
- **T1.2.5 `noi-nguoc-lai`**: "`not` lật `True` thành `False` và ngược lại."
- **Bài 3 `noi-nguoc-lai-mot-cau`**: "Phủ định — dựng từ một mệnh đề một mệnh đề mới luôn **mang giá trị ngược lại**."

Slug gần trùng, câu khái niệm gần trùng. Cái mới ở bài 3 là vế sau: **trong hai câu luôn có đúng một câu đúng** (và phủ định áp lên *câu tiếng Việt*, không chỉ lên biến boolean — chỗ người học thật sự trượt là phủ định câu có "và", có "mọi", chứ không phải `not True`). **Sửa:** đổi slug + tiêu đề bài 3 để nêu cái mới (ví dụ `luon-co-dung-mot-cau-dung`), và cột khái niệm dẫn bằng luật bài trung, không dẫn bằng "lật giá trị".

### A3. Bài 16 (câu mở) đến MUỘN 7 bài — bài 9–15 đã lượng hoá lén
Bài 15 kết bằng: *"Mọi câu từ đầu track tới giờ đều nói về **một** người có tên."* Câu này **sai với chính track**. Bài 9 dựng nội quy "Nếu **là thành viên** thì phải đeo thẻ" — không tên ai. Bài 10–14 xử lý nó như một mệnh đề `P → Q` suốt năm bài, trong khi `P` = "là thành viên" và `Q` = "đeo thẻ" đều là **câu mở**. Bài 11 còn nói thẳng "một người **không** phải thành viên" — tức là đang thế trị vào một chỗ trống chưa được thừa nhận là có.

Đây không phải lỗi thứ tự (bài 16 đứng đúng chỗ nó cần cho bài 17), mà là lỗi **nợ ngầm**: năm bài liên tiếp mượn trước một khái niệm chưa tới lượt.

**Sửa:** buộc bài 10–14 dùng **từng thành viên có tên**, mỗi lần một người: "Nếu **Nam** là thành viên thì **Nam** phải đeo thẻ", "Nếu **Khanh** là thành viên thì…". Dòng trên bảng nội quy được giới thiệu như **cách viết tắt sáu câu**, và nói thẳng: "chỗ trống trên tấm bảng ấy, ta để tới bài 16". Làm vậy thì câu mở ở bài 16 không phải một khái niệm rơi từ trên xuống — nó là **món nợ được gọi tên**, và bài 15 nói thật.

### A4. Bài 2 — tiêu đề nêu một thứ, cột khái niệm nêu thứ khác
Đúng dấu hiệu tiêu chí 1:
- Tiêu đề: "Đúng hay sai **do sự việc, không do người nói**" → tính khách quan.
- Khái niệm: "Giá trị chân lý — mỗi mệnh đề mang **đúng một trong hai** giá trị; 'chưa biết' không phải giá trị thứ ba" → tính lưỡng trị.

Hai thứ khác nhau. Tính khách quan là thứ trả lời reflect của bài 1 ("Sân trường rộng"). Tính lưỡng trị là thứ bài 3 (đúng một câu đúng) và bài 25 (phản chứng: không có cửa thứ ba) cần.

Thêm nữa, bài 1 đã tự trả lời reflect của mình: cột khái niệm bài 1 loại sẵn "câu nêu ý thích", mà "Sân trường rộng" chính là loại đó.

**Sửa:** dời tính khách quan lên bài 1 — khái niệm bài 1 thành *"Mệnh đề — câu mà việc nó đúng hay sai là chuyện của **sự việc**, không phải chuyện của người nói"*, và cột khái niệm bài 1 chỉ loại **câu hỏi + câu sai khiến** (loại được bằng ngữ pháp), để "Sân trường rộng" là bài tập thật bên trong bài 1. Reflect bài 1 đổi thành cửa vào lưỡng trị: *"Mệnh đề nào cũng đúng hoặc sai. Vậy 'Ngày mai Khanh đến CLB' — chưa ai biết. 'Chưa biết' có phải giá trị thứ ba không?"* Bài 2 khi đó mang **đúng một** khái niệm.

---

## B. LỖI VỪA

### B1. Bài 7 — cột khái niệm có chữ "và", và reflect không luyện mặt nào trong hai mặt
"**Hằng đúng và hằng sai**". Tôi chấp nhận đây vẫn là *một* khái niệm nếu đọc theo tiêu đề (`cot-ket-qua-khong-doi` — cột hằng, hai vị). Nhưng reflect của bài 7 lại là **phủ định kép** ("Không phải là không P"), mà cột của phủ định kép **không hằng** — nó biến thiên theo P. Tức là bài 7 kết bằng một ví dụ phản lại chính tiêu đề của nó.

Hậu quả thật: **"hằng sai" không được luyện lần nào**, mà bài 25 (phản chứng) sống bằng đúng chữ đó — lập luận §2 cũng nói thế ("phản chứng không nói nổi nếu chưa có chữ hằng sai").

**Sửa reflect bài 7:** *"'P và không P' — cột toàn S, dù P là câu gì. Còn 'không phải là không P' thì cột lại **trùng khít cột của P** — không toàn Đ, cũng không toàn S. Hai câu viết khác chữ mà cùng một cột thì có thay nhau được không?"* Giữ nguyên cầu nối sang bài 8, mà hằng sai được sờ tận tay.

### B2. Bài 19 là bài THỪA theo đúng phép thử của chính tài liệu
Bỏ bài 19 ra: reflect bài 18 hỏi *"muốn **bác bỏ** 'với mọi' thì cần mấy người?"* — bài 20 (`¬∀` thành `∃¬`) trả lời câu đó **trực tiếp và tổng quát hơn**. Mạch nối liền. Đó chính là định nghĩa "thừa" ở luật 2.

Lập luận §5 bào chữa rằng bài 19 "tạo ra sự bất đối xứng sắc nhất". Không đúng — sự bất đối xứng **đã được phát biểu trọn vẹn trong reflect bài 18** ("khẳng định 'tồn tại' chỉ cần một người, khẳng định 'với mọi' phải xét cả 6"). Bài 19 không tạo ra nó, nó chỉ nhắc lại.

Tôi **không** đề nghị xoá bài 19 — tên gọi "phản ví dụ" là tài sản mà bài 21 và cả nghề toán sống nhờ. Đề nghị **làm cho nó cần thiết**:

1. **Sửa reflect bài 18** thành câu hỏi về *giá* của cả bốn việc, không chỉ hai: *"Khẳng định 'tồn tại' tốn một người, khẳng định 'với mọi' tốn cả sáu. Vậy còn **bác bỏ**: bác bỏ 'với mọi' tốn mấy người, bác bỏ 'tồn tại' tốn mấy?"* Bài 19 khi đó trả lời một câu mà bài 20 không trả lời (bài 20 nói *vì sao* có quy luật đó, bài 19 nói *giá* là bao nhiêu).
2. **Lôi `all([]) == True` ra khỏi ngoặc đơn.** Hiện nó đang nằm trong dấu ngoặc ở cột khái niệm bài 19, mà lập luận §10 lại tuyên bố bài 32 dựa vào nó ("vòng chạy 0 lượt vẫn đúng"). Một sự thật chịu lực mà bị nhét trong ngoặc thì chắc chắn rơi. Cho nó một `predict` riêng trong bài 19: *"Tổ trực nhật tuần này chưa xếp ai. 'Mọi người trong tổ đều đeo thẻ' — đúng hay sai?"* Đây cũng là lần thứ hai chân lý rỗng của bài 11 hiện ra, và người học **tự nhận ra** là cùng một chuyện.

### B3. Bài 29 (quy nạp mạnh) là ngõ cụt — 30, 31, 32 không dùng nó lần nào
Reflect bài 29 tự bỏ rơi nội dung bài 29: nó bắc cầu sang bất biến vòng lặp bằng *quy nạp thường* ("quy nạp chạy theo bậc thứ n, vòng `while` cũng chạy theo lượt"). Bất biến chỉ cần bậc liền trước. Nên sau khi trả xong món nợ 12 = 3×4 của bài 28, quy nạp mạnh **không được dùng lại lần nào trong track**.

Có chỗ nó dùng được, và đang bị bỏ lỡ: **bài 31**. Cột khái niệm bài 31 viết "số tự nhiên không giảm mãi được (chính là quy nạp nhìn ngược)" — nhưng không nói *quy nạp nào*. Lập luận "không có dãy số tự nhiên giảm vô hạn" chính là **nguyên lý sắp thứ tự tốt**, và cách trình bày tự nhiên của nó là **phản ví dụ nhỏ nhất**: giả sử vòng không dừng, lấy lượt **đầu tiên** hỏng — muốn nói được "đầu tiên" thì phải nói về **mọi** lượt trước nó, tức đúng thứ bài 29 vừa cấp phép.

**Sửa:** dựng bài 31 theo lối phản ví dụ nhỏ nhất và ghi thẳng vào cột "Dựa trên": `30, 29, 26, 25`. Bài 29 hết ngõ cụt, và bài 25 (phản chứng) được dùng lại lần thứ hai ở chỗ nó thật sự đắt giá.

### B4. Bài 8 và bài 15 là hai loại "tương đương" mà track không bao giờ phân biệt
Bài 8: *tương đương logic* — hai câu **trùng bảng** thì thay được cho nhau. Bài 15: *tương đương hai chiều* `P ↔ Q` — một **phép nối**, cho ra một cột.

Đây là cặp bị nhầm nhiều nhất trong mọi giáo trình logic nhập môn, và ở đây chúng bị đặt cách nhau bảy bài, cùng gọi là "tương đương", **không một dòng nào nối**. Người học sẽ đọc bài 15 như bài 8 dạy lại — đúng thứ mà lập luận §10 tự hào là đã tránh.

Cái tệ hơn: chỗ nối lại đang có sẵn và miễn phí. Bài 7 vừa cho "hằng đúng", bài 8 cho "trùng bảng". Ghép hai cái đó ra đúng một câu: **hai câu tương đương ⟺ cột của `P ↔ Q` toàn Đ**.

**Sửa:** thêm một `predict` bắt buộc vào bài 15 — dựng bảng cho `P ↔ Q` với `P` = câu gốc, `Q` = phản đảo (bài 14), thấy cột toàn Đ; rồi với `Q` = mệnh đề đảo (bài 12), thấy cột không toàn Đ. Một cú là ba bài 7, 8, 14 cùng được trả tiền, và bài 24 ("phản đảo tương đương nên thay được") có chân đứng thay vì một lời hứa.

---

## C. VẶT — sửa một dòng là xong

| # | Vấn đề | Sửa |
|---|---|---|
| Bài 6 | `itertools.product` xuất hiện **trước** sự bất tiện mà nó gỡ — vi phạm luật 3 của chính tài liệu. Ba vế = 8 dòng chép tay mới là chỗ đau. | Bài 6 dựng bảng bằng `for` lồng `for` (dùng lại T1.2.21 — đang vắng mặt trong cả cột "Dựa trên"). Để dành `product` cho bài 9, khi công thức có ngoặc và 3 biến. |
| Bài 6 | Cột khái niệm viết "n vế → **2ⁿ** dòng", trong khi §11 tự tuyên bố "đếm số dòng dừng ở *gấp đôi mỗi lần thêm một vế*, phần 2ⁿ để cho T2.5". Tự mâu thuẫn. | Bỏ ký hiệu `2ⁿ` khỏi cột khái niệm bài 6. |
| Bài 9 | Cột "Dựa trên" thiếu **T1.2.7 `dau-ngoac-noi-ro-y`** — De Morgan là bài đầu tiên có `¬(P ∧ Q)`, tức là bài đầu tiên ngoặc mang nghĩa. | Thêm vào cột. |
| Bài 10 | Slug `neu-thi-la-mot-**luat**`, tiêu đề "…là một **lời hứa**". Hai ẩn dụ khác nhau, mà bài 11 sống bằng đúng ẩn dụ đó (lời hứa chỉ *bội* khi đã có nghĩa vụ). | Chọn "lời hứa", sửa slug. |
| Bài 20 | Reflect nói "máy thử **39 số đầu**", bài 21 nói "**đúng 40 lần** rồi sai ở n = 40". Số tự nhiên gồm 0, nên n = 0…39 là **40** số. | Bài 20 sửa thành "thử tới n = 39" hoặc "40 số đầu". |
| Bài 21 | Chỉ nói máy **không** làm được gì. Đọc trần thì thành "máy vô dụng" — mà 30–32 lại dựng cả bằng chứng quanh `assert`. | Thêm nửa còn lại vào cột khái niệm: máy vẫn **kết luận dứt điểm** được hai việc — tìm ra phản ví dụ (bác bỏ ∀) và tìm ra nhân chứng (chứng minh ∃). Nó chỉ không nói được chữ "đúng" cho ∀ trên miền vô hạn. |
| Bài 23→24 | Cầu nối yếu hơn §6 tuyên bố: "n² = 2k rồi tắc" cũng gỡ được bằng **phản chứng** (bài 25), nên bài 24 chưa phải câu trả lời duy nhất. | Reflect bài 23 chỉ thẳng vào *hình dạng* của chỗ tắc: *"Từ n² = 2k không nặn ra n = 2m. Thử lật câu hỏi: nếu n **không** chẵn thì n² ra sao? Câu lật ấy có phải câu gốc không — bảng để kiểm bạn đã dựng ở bài 14."* |
| Bài 27 | Cột khái niệm đã chứa sẵn đáp án bài 28: "không vòng tròn **vì bậc đầu chẳng mượn ai**". Bài 28 khi đó chỉ minh hoạ lại điều bài 27 đã kết luận. | Bỏ vế đó khỏi bài 27. Bài 27 chỉ trả lời nghi ngờ vòng tròn bằng **tư cách logic của giả thiết**: bạn không khẳng định `P(k)`, bạn khẳng định `P(k) → P(k+1)` — một câu kéo theo, và bài 10–11 đã dạy câu kéo theo đúng được mà vế trước chẳng cần đúng. (Sau khi sửa A1, chỗ này khớp hoàn hảo.) Chuyện "còn phải có bậc đầu" để nguyên cho bài 28. |

---

## D. Những chỗ tôi đã thử phá mà không phá được

- **Bỏ từng bài một:** ngoài bài 19 (mục B2), 30 bài còn lại đều đứt mạch khi bỏ. Tôi kiểm từng bài chứ không kiểm mẫu. Riêng bài 13 (mệnh đề phản) là bài ít được dùng lại nhất về sau — nhưng nó **không** thừa theo phép thử: reflect bài 12 hỏi thẳng về mệnh đề phản, bài 14 không trả lời câu đó.
- **Bài 24 nằm gọn trong bài 25:** chứng minh √2 vô tỉ dùng lại đúng kết quả "n² chẵn ⟹ n chẵn" của bài 24. Công cụ được cất đi rồi lấy ra đúng lúc cần — đây là chỗ đẹp nhất của cả track, đừng động vào.
- **Bài 21 làm bản lề:** đúng. Nó là bài duy nhất trong 32 bài **phá** công cụ thay vì dựng công cụ, và mọi phương pháp chứng minh sau đó có lý do tồn tại nhờ nó.
- **Ranh giới track:** không bài nào cần `∈`, `{x | …}`, tổ hợp, hay SAT. Giữ đúng biên với T2.4/T2.5/R3.
- **Tỉ lệ bài không khái niệm mới:** đúng 1/32 (bài BOSS), khớp R0 và T1.2.

---

## Tóm tắt việc phải làm

**Bắt buộc (mạch hỏng nếu không sửa):** A1 (dựng lại bài 11 quanh modus ponens) · A2 (viết lại cột khái niệm bài 3 và bài 5 để hết dạy lại R0.30 / T1.2.5) · A3 (bài 10–14 dùng thành viên có tên; nêu rõ món nợ câu mở) · A4 (dời tính khách quan lên bài 1, bài 2 chỉ còn lưỡng trị).

**Nên sửa:** B1 (reflect bài 7 phải luyện hằng sai) · B2 (reflect bài 18 hỏi giá cả bốn chiều; đưa `all([])` ra khỏi ngoặc) · B3 (bài 31 dựng theo phản ví dụ nhỏ nhất, dùng lại bài 29 và 25) · B4 (bài 15 phải nối `↔` với "trùng bảng" của bài 8).

**Vặt:** 8 mục ở bảng C.

Sau các sửa trên, số bài vẫn là **32** — không thêm, không bớt.

---

## T2.4 — Tập hợp, quan hệ, ánh xạ (Realm 2 · Toán & Toán rời rạc · Python CHỈ để KIỂM · 28 bài)

## Mạch T2.4 — Tập hợp, quan hệ, ánh xạ

Người học vào track đã xong R0 + R1 + T2.1 (số, phép tính, thanh số) + T2.2
(biến, biểu thức, phương trình) + T2.3 (mệnh đề, `and/or/not`, kéo theo,
tương đương, `all`/`any`, chứng minh). T2.3 CỐ Ý chưa dùng `∈`, `{x | …}`,
hay tổ hợp — biên đó dành đúng cho track này (xem "Ranh giới track" ở mục D
phía trên).

**QUAN TRỌNG — kiểm KỸ trước khi viết bài 1: R1.T1.4 ĐÃ dạy phần MÁY của
tập hợp rồi, track này KHÔNG được dạy lại.** Rà `content/nen-tang/
04-list-dict-set-tuple/`:

| Đã dạy ở R1 | Bài | Nội dung |
|---|---|---|
| `core.set` | 26 `ro-khong-chua-hai-lan` | `{...}` là một rổ; bỏ vào thứ đã có thì không đổi gì; `set(danh_sach)` lọc trùng |
| `core.set-unordered`, `core.set-membership` | 27 `ro-khong-xep-hang` | rổ không có chỗ đứng (`ro[0]` là `TypeError`); `in` trên rổ |
| `core.set-intersection` | 28 `phan-chung-cua-hai-ro` | `&` |
| `core.set-difference` | 29 `phan-rieng-cua-mot-ben` | `-` (và: đổi chỗ hai rổ là đổi hẳn câu trả lời) |
| `core.set-union`, `core.choose-container` | 33 `chon-dung-cho-chua` | `\|` |

Nghĩa là: **thứ tự, trùng lặp, `in`, `\|`/`&`/`-` ĐỀU đã là kiến thức CŨ**
khi người học bước vào T2.4. Việc của track này KHÔNG PHẢI dạy lại cách
GÕ các phép đó — đó là việc track này BUỘC PHẢI TRÁNH, giống hệt lỗi A2 mà
T2.3 tự bắt được ở chính nó (mục D phía trên: "viết lại cột khái niệm bài
3 và bài 5 để hết dạy lại R0.30 / T1.2.5"). Việc CỦA track này là dạy lớp
**Ý NGHĨA TOÁN HỌC** đặt trên nền máy đã có: ký hiệu (`∈`, `⊆`, `∪`, `∩`,
`−`, `∅`, `×`), CÁCH CHỨNG MINH (hai tập bằng nhau qua `⊆` hai chiều), và
NHỮNG THỨ R1 CHƯA HỀ CÓ (tập con `⊆`, phần bù, đếm bù trừ, tích Descartes,
quan hệ, phản xạ/đối xứng/bắc cầu, tương đương/phân hoạch, ánh xạ/đơn ánh/
toàn ánh/song ánh/hợp thành — không dòng Python nào ở R1 chạm tới bất kỳ
cái nào trong nhóm sau).

Vì phần "máy" đã xong, 12 bài đầu (tập hợp cơ bản) đi NHANH hơn nhịp
thường của R2 — mỗi bài vẫn đúng MỘT khái niệm mới, nhưng khái niệm đó
thường chỉ là MỘT KÝ HIỆU gắn vào một thao tác Python đã quen tay, không
phải một thao tác Python mới toanh. 16 bài sau (cặp/quan hệ/ánh xạ) mới là
lãnh thổ hoàn toàn mới, và đi với nhịp đầy đủ như T2.1-T2.3.

**Hiện vật xuyên suốt:** Vườn của Byte (T2.1) giờ **lớn hơn** — không còn
một luống mà nhiều luống, không còn một mình Byte làm mà có thêm mấy người
làm vườn phụ (Lan, Minh, Tú), và mỗi luống trồng nhiều LOẠI rau khác nhau
(không phải một loại). Ba câu hỏi tự nhiên nảy ra từ khu vườn lớn hơn này,
và chúng CHÍNH LÀ ba phần của track:

1. "Luống này trồng NHỮNG loại rau nào?" — một CÂU TRẢ LỜI không quan tâm
   thứ tự kể, không quan tâm trồng mấy CÂY mỗi loại → **tập hợp**.
2. "Luống nào tưới ngày nào?" — một cặp (luống, ngày) đi cùng nhau →
   **quan hệ**.
3. "Luống nào ai phụ trách?" — mỗi luống gán cho ĐÚNG MỘT người → **ánh
   xạ**.

Ba câu hỏi ấy dùng lại đúng MỘT hiện vật, không hiện vật mới nào chen vào
giữa track — giữ luật đã áp dụng suốt R2.

**Python trong track này không bao giờ là lời giải, CHỈ để KIỂM** — và ở
12 bài đầu, nó là công cụ NGƯỜI HỌC ĐÃ BIẾT DÙNG (`set`, `in`, `\|`/`&`/
`-`, R1.T1.4), giờ gắn thêm ký hiệu toán. `all()`/`any()` (T2.3) kiểm
phản xạ/đối xứng/bắc cầu trên MỌI cặp; một `dict` kiểm một ánh xạ có thật
là "mỗi luống đúng một người" hay không.

| # | slug | Tiêu đề | Khái niệm mới (đúng một) | `reflect` cuối bài | Dựa trên |
|---|---|---|---|---|---|
| 1 | `tu-ro-sang-tap-hop` | Từ cái rổ sang tập hợp | **Tập hợp là TÊN TOÁN của cái rổ đã quen** (T1.4): "luống 1 trồng cà chua, xà lách, cà rốt" viết bằng Python đã biết, giờ đọc lại bằng tiếng toán — một nhóm chỉ cần biết CÓ gì, không cần thứ tự, không đếm trùng, ĐÚNG những gì bài 26-27 T1.4 đã chứng minh trên máy | Byte viết `cà chua ∈ luống_1` bằng CHỮ. Máy đọc được câu ấy không, hay phải dịch nó sang một dòng Python trước? | T1.4 `ro-khong-chua-hai-lan`, `ro-khong-xep-hang` |
| 2 | `ky-hieu-thuoc-ve` | Ký hiệu "thuộc về" | **`∈`** — ký hiệu toán của phép `in` đã biết; `cà chua ∈ luống_1` VÀ `"cà chua" in luong_1` nói CÙNG một câu, một bằng chữ Hy Lạp, một bằng Python | "Khoai tây `∈` luống_1" — Byte tra thấy sai. Có ký hiệu nào nói thẳng "không có mặt", khỏi viết `not (... ∈ ...)` dài dòng? | 1, T2.3 `cau-nao-phan-xu-duoc` |
| 3 | `khong-thuoc-ve` | Không thuộc về | **`∉`** — viết tắt của `not (x ∈ A)`, ứng đúng `x not in A` của Python; hai ký hiệu `∈`/`∉` phủ định nhau như `P`/`không P` (T2.3 bài 3) | Byte có bảy luống, và muốn nói "tập hợp những luống có trồng cà chua" mà KHÔNG liệt kê tay từng luống. Có cách viết nào khác `{luống_1, luống_3, ...}` không? | 2, T2.3 `noi-nguoc-lai-mot-cau` |
| 4 | `liet-ke-hay-mo-ta` | Liệt kê hay mô tả | **Hai cách dựng một tập hợp**: liệt kê từng phần tử (`{cà chua, xà lách}`) HOẶC mô tả một điều kiện phần tử phải thoả (`{luống \| luống có trồng cà chua}`) — cùng một tập, hai cách nói, ký hiệu `{x \| P(x)}` là MỚI (khác cú pháp comprehension T1.4 bài 30, dù cùng ý) | Byte có bảy luống. Nếu vườn có bảy TRĂM luống, liệt kê tay còn làm nổi không? Mô tả bằng điều kiện có cần biết CON SỐ bảy trăm đó không? | 3 |
| 5 | `tap-hop-rong` | Tập hợp rỗng | **`∅`** (ứng `set()` — KHÔNG PHẢI `{}`, vì `{}` là `dict` rỗng, một bẫy Python thật) — tập hợp không phần tử VẪN là một tập hợp hợp lệ, không phải "lỗi"; luống chưa gieo có tập loại-rau đúng là `∅` | Luống 5 chưa gieo gì — tập của nó là `∅`. Luống 6 trồng đúng "bí đỏ". Tập nào "nhỏ hơn" — và "nhỏ hơn" nghĩa là gì cho tập hợp, khi nó không phải một CON SỐ? | 4, T2.1 `so-khong-can-nhay` |
| 6 | `tap-con` | Tập con | **`⊆`** — A là tập con của B khi MỌI phần tử của A đều có mặt trong B; `∅ ⊆` MỌI tập hợp (không phần tử nào để phản chứng — chân lý rỗng, T2.3 bài 11); MỚI hoàn toàn, R1 không có phép này | "Luống rau lá xanh" = {xà lách, cải bó xôi} ⊆ "luống 2" = {xà lách, cải bó xôi, cà rốt}. Đảo lại — luống 2 có phải tập con của "rau lá xanh" không? Nếu KHÔNG cả hai chiều, hai tập ấy quan hệ gì? | 5, T2.3 `khi-ve-truoc-khong-xay-ra` |
| 7 | `hai-tap-bang-nhau` | Hai tập hợp bằng nhau | **A = B ⟺ (A ⊆ B) và (B ⊆ A)** — chứng minh hai tập bằng nhau bằng cách chứng minh HAI chiều tập con, đúng kỹ thuật "khi và chỉ khi" (T2.3 bài 15) áp dụng vào tập hợp; Python `==` đã trả lời được câu này (T1.4), bài này dạy CÁCH CHỨNG MINH nó, không phải cách gõ | `luong_1 = {"cà chua", "xà lách"}` và `luong_1_ban_2 = {"xà lách", "cà chua", "cà chua"}`. `luong_1 == luong_1_ban_2` cho `True` ngay — nhưng CHỨNG MINH bằng tay (không gõ `==`) thì phải chỉ ra ĐIỀU GÌ? | 6, T2.3 `khi-va-chi-khi` |
| 8 | `hop-hai-tap` | Hợp của hai tập hợp | **`∪`** — ký hiệu toán của `\|` đã biết (T1.4 bài 33); A ∪ B gồm phần tử thuộc A HOẶC B (hoặc cả hai — đúng nghĩa "hoặc" T2.3 bài 5) | `luong_1 ∪ luong_3` gồm cả cà chua, xà lách, cà rốt — xà lách chung CẢ HAI luống. Nó có bị "đếm hai lần" trong TẬP hợp kết quả không, hay tập hợp không có khái niệm "đếm mấy lần"? | 7, T2.3 `hoac-gom-ca-hai` |
| 9 | `giao-hai-tap` | Giao của hai tập hợp | **`∩`** — ký hiệu toán của `&` đã biết (T1.4 bài 28); A ∩ B gồm phần tử thuộc CẢ HAI, đúng nghĩa "và" T2.3 bài 4 | Có cặp luống nào mà `∩` ra `∅` không? Nếu có, hai luống ấy quan hệ gì với nhau — và tên riêng cho quan hệ "không chung gì" đó là gì? | 8 |
| 10 | `hai-tap-roi-nhau` | Hai tập hợp rời nhau | **A ∩ B = ∅** — hai tập KHÔNG chung phần tử gọi là rời nhau; luống 4 trồng {bí đỏ}, luống 6 trồng {khoai lang} là một cặp rời nhau; MỚI (một cái TÊN cho một tình huống, R1 chỉ có phép `&`, không có tên riêng cho kết quả rỗng của nó) | Byte muốn biết luống nào trồng cà chua mà KHÔNG trồng xà lách. `∪` gộp, `∩` giữ chung — phép NÀO lấy đúng "có cái này, không cái kia"? | 9 |
| 11 | `hieu-hai-tap` | Hiệu của hai tập hợp | **`−`** — ký hiệu toán của `-` đã biết (T1.4 bài 29); A − B gồm phần tử thuộc A mà không thuộc B; T1.4 đã dạy `A - B ≠ B - A` bằng máy, bài này đặt TÊN toán cho điều đó (không đối xứng) | `luong_1 − luong_3` = {cà chua}, `luong_3 − luong_1` = {cà rốt} — hai tập khác nhau. `∪` và `∩` có tính chất "đổi chỗ vẫn ra y hệt" (giao hoán, T2.1 bài 20) không, hay chỉ `−` mới thiếu nó? | 10 |
| 12 | `tap-vu-tru-va-phan-bu` | Tập vũ trụ và phần bù | **Tập vũ trụ `U`** (mọi loại rau CÓ trong toàn vườn) **và phần bù `Aᶜ = U − A`** — MỚI hoàn toàn: phần bù chỉ có nghĩa KHI đã chốt vũ trụ, khác `−` (không cần vũ trụ, R1 không có khái niệm này) | Cả vườn Byte trồng đúng bảy loại rau. Đổi "vũ trụ" từ "vườn Byte" sang "vườn của Lan" (nhiều loại rau hơn) — phần bù của luống 1 có đổi theo không, dù luống 1 không đổi gì? | 11, T2.1 `so-khong-can-nhay` |
| 13 | `dem-bang-bu-tru` | Đếm bằng bù trừ | **`\|A ∪ B\| = \|A\| + \|B\| − \|A ∩ B\|`** — cộng thẳng `\|A\|+\|B\|` đếm phần chung HAI LẦN, phải TRỪ lại một lần; bài đếm đầu tiên của track — đếm PHẦN TỬ của tập hợp (một con số), không đếm chính tập hợp | Luống 1 có 2 loại, luống 3 có 2 loại, giao nhau 1 loại → hợp `2+2−1=3`, khớp {cà chua, xà lách, cà rốt}. Vậy BA luống cùng lúc — công thức này còn đúng, hay phải cộng trừ thêm? *(để ngỏ — dành T2.5 tổ hợp)* | 12, 8, 9 |
| 14 | `cap-co-thu-tu` | Cặp có thứ tự | **`(a, b)` — thứ tự CÓ nghĩa**, tương phản THẲNG với bài 1 (tập hợp: thứ tự KHÔNG nghĩa): `(luống_1, "Hai") ≠ ("Hai", luống_1)`, và `(a,b)=(c,d)` chỉ khi `a=c` VÀ `b=d`; Python `tuple` (R1) đã có, đây là lớp Ý NGHĨA toán trên nó | "Luống 1 tưới thứ Hai" viết `(luống_1, "Hai")`. Byte có bảy luống, tuần bảy ngày. Liệt kê TẤT CẢ cặp (luống, ngày) CÓ THỂ có — kể cả cặp không tưới thật — được bao nhiêu cặp? | 13, T2.1 `mang-chu-nhat-xoay-mot-goc` |
| 15 | `tich-descartes` | Tích Descartes | **`A × B`** — tập hợp MỌI cặp có thứ tự `(a,b)` với `a ∈ A`, `b ∈ B`; bảy luống × bảy ngày cho `7×7=49` cặp (đúng phép nhân mảng chữ nhật, T2.1 bài 20) — TOÀN BỘ khả năng, chưa nói cái nào THẬT xảy ra | 49 cặp là mọi khả năng. Nhưng luống 1 CHỈ tưới thứ Hai và thứ Năm thật sự — hai cặp trong 49. Tập con "chỉ những cặp thật" ấy gọi là gì? | 14, T2.1 `mang-chu-nhat-xoay-mot-goc` |
| 16 | `quan-he-la-mot-tap-con` | Quan hệ là một tập con | **Quan hệ = một tập con của `A × B`** — không phải khái niệm tách rời, mà CHÍNH LÀ một tập hợp (bài 1-13) gồm những cặp (bài 14-15) THẬT xảy ra; "luống nào tưới ngày nào" là một quan hệ giữa Luống và Ngày | Quan hệ tưới nước viết ra là một ĐỐNG cặp rời rạc. Nhìn đống đó có DỄ thấy "luống nào tưới NHIỀU ngày nhất" không, hay cần cách trình bày khác? | 15, 1 |
| 17 | `bang-cua-mot-quan-he` | Bảng của một quan hệ | **Biểu diễn quan hệ bằng bảng 0/1** — hàng là luống, cột là ngày, ô đánh dấu nếu cặp thuộc quan hệ; MỘT quan hệ, HAI cách nhìn (đống cặp / bảng), đúng khớp bài 4 (mô tả tập hợp hai cách) | Đọc theo HÀNG luống 1: hai ô đánh dấu. Đọc theo CỘT thứ Hai: mấy ô — cột đó nói gì mà hàng không nói được? | 16, 4 |
| 18 | `mien-xac-dinh-mien-gia-tri` | Miền xác định và miền giá trị | **Miền xác định** (mọi luống XUẤT HIỆN Ở VỊ TRÍ ĐẦU ít nhất một cặp) và **miền giá trị** (mọi ngày Ở VỊ TRÍ SAU); có thể KHÁC `A`/`B` gốc nếu vài luống chưa tưới lần nào | Luống 7 chưa từng tưới — có mặt trong `A` (tập bảy luống) nhưng KHÔNG trong miền xác định. Điều đó nói gì về luống 7, so với luống 1 (đã tưới ít nhất một lần)? | 17, 6 |
| 19 | `phan-xa` | Phản xạ | **Quan hệ phản xạ** — MỌI phần tử quan hệ với CHÍNH NÓ; quan hệ MỚI "cùng khu vườn với" trên bảy luống: luống nào cũng cùng vườn với chính nó → phản xạ | "Luống A trồng nhiều loại HƠN luống B" — luống nào có nhiều loại hơn CHÍNH NÓ không? Quan hệ này có phản xạ không, và thiếu đúng cái gì so với phản xạ? | 18, T2.3 `dung-sai-do-su-viec` |
| 20 | `doi-xung` | Đối xứng | **Quan hệ đối xứng** — nếu A quan hệ B thì B CŨNG quan hệ A; "cùng khu vườn với" đối xứng, còn "tưới TRƯỚC" (luống 1 trước luống 3) thì KHÔNG | "Luống A trồng chung ít nhất một loại với luống B" (dùng lại `∩` bài 9, khác `∅`) — đối xứng không? Thử luống 1 và luống 3. | 19, 9 |
| 21 | `bac-cau` | Bắc cầu | **Quan hệ bắc cầu** — nếu A quan hệ B, B quan hệ C, thì A PHẢI quan hệ C; "cùng khu vườn với" bắc cầu, còn "trồng chung ít nhất một loại" (bài 20) thì KHÔNG (1 chung xà lách với 3, 3 chung cà rốt với 6, nhưng 1 và 6 có thể chẳng chung gì) | Ba tính chất vừa học — phản xạ, đối xứng, bắc cầu — "cùng khu vườn với" có ĐỦ CẢ BA. Một quan hệ đủ cả ba có tên riêng là gì? | 20, 19 |
| 22 | `quan-he-tuong-duong` | Quan hệ tương đương | **Quan hệ tương đương = phản xạ + đối xứng + bắc cầu, đủ cả ba** — "cùng lịch tưới nước" (hai luống tưới ĐÚNG cùng ngày) là một quan hệ tương đương THẬT trên bảy luống, kiểm bằng `all()`/`any()` (T2.3) trên mọi cặp | Luống 1 và 5 cùng lịch Hai-Năm. Luống 2 tưới riêng một mình ngày Ba. Gom MỌI luống theo "ai cùng lịch với ai" — mỗi luống rơi ĐÚNG MẤY nhóm? | 21, 19, 20, T2.3 `xet-du-moi-truong-hop` |
| 23 | `lop-tuong-duong-va-phan-hoach` | Lớp tương đương và phân hoạch | **Lớp tương đương** — nhóm mọi luống tương đương nhau thành MỘT lớp; các lớp KHÔNG chồng lấn (đối xứng+bắc cầu) và GỘP LẠI vừa khít cả bảy luống (phản xạ) — gọi là một **phân hoạch** | Bảy luống chia đúng ba lớp lịch-tưới. Đếm luống mỗi lớp rồi CỘNG — có đúng bằng bảy? Trùng hợp, hay quan hệ tương đương LUÔN bảo đảm điều đó? | 22, 8 |
| 24 | `anh-xa-la-quan-he-dac-biet` | Ánh xạ là quan hệ đặc biệt | **Ánh xạ** — một quan hệ từ A sang B mà MỖI phần tử A xuất hiện Ở VỊ TRÍ ĐẦU của ĐÚNG MỘT cặp (không thiếu, không thừa); "luống nào ai phụ trách" là ánh xạ NẾU mỗi luống có đúng một người | Phân công: luống 1→Lan, luống 2→Minh, luống 3→Lan, luống 4→(chưa ai nhận). Đây có phải ánh xạ từ tập bảy luống sang tập người làm vườn? Nếu không, THIẾU đúng chỗ nào? | 23, 16, T2.2 `chu-cai-la-ten-cua-o-trong` |
| 25 | `don-anh` | Đơn ánh | **Đơn ánh (một-một)** — hai luống KHÁC nhau không bao giờ chung một người phụ trách; kiểm bằng: số người xuất hiện trong bảng phân công phải bằng ĐÚNG số luống | Phân công đủ bảy luống, không luống nào trống, NHƯNG Lan phụ trách cả luống 1 và 3. Đây có đơn ánh không? Còn là ánh xạ hợp lệ (bài 24) không? | 24 |
| 26 | `toan-anh` | Toàn ánh | **Toàn ánh (phủ hết miền giá trị)** — MỌI người làm vườn có ÍT NHẤT một luống, không ai đứng ngoài; một ánh xạ có thể đơn ánh mà KHÔNG toàn ánh, toàn ánh mà KHÔNG đơn ánh, hoặc CẢ HAI | Byte có đúng bốn người làm vườn, bốn luống, phân công vừa đơn ánh vừa toàn ánh. Một ánh xạ có CẢ HAI tính chất ấy có tên riêng không — và nó "đảo ngược" được không? | 25 |
| 27 | `song-anh-va-hop-thanh` | Song ánh và hợp thành | **Song ánh** = đơn ánh + toàn ánh, và CHỈ khi đó ánh xạ mới ĐẢO NGƯỢC được (từ người làm vườn suy ra ĐÚNG MỘT luống); **hợp thành** — nối hai ánh xạ (luống→người, người→làng) thành một ánh xạ luống→làng | Mọi luống có: một TẬP loại rau (bài 1-13), một chỗ trong quan hệ tưới (bài 14-23), một người phụ trách qua song ánh (bài 24-27). Ghép cả ba lại thành MỘT bài toán vườn trọn vẹn trông như thế nào? | 26, 25 |
| 28 | `boss-khu-vuon-day-du` | BOSS — Khu vườn đầy đủ | *(không khái niệm mới — bài tổng hợp)* ghép tập hợp (bài 1-13) + quan hệ (bài 14-23) + ánh xạ song ánh/hợp thành (bài 24-27) trên MỘT khu vườn bốn luống: kiểm `⊆`/`∪`/`∩` bằng `assert`, kiểm quan hệ tương đương bằng `all()`, VÀ kiểm một ánh xạ có song ánh hay không | Mọi luống giờ có tập rau, quan hệ tưới, người phụ trách — NHƯNG "mấy CÂY mỗi loại", "mấy KILÔGAM thu hoạch" thì tập hợp không trả lời được (bài 1 đã bỏ hẳn số lượng). Track sau đếm được không, và đếm bằng cách nào? *(dẫn sang T2.5 — Tổ hợp, xác suất, thống kê)* | 1–27 |

**Vì sao thứ tự này đúng**

**1. Vì sao track KHÔNG mở bằng "đây là kiểu `set` trong Python" (như một
người mới sẽ mong đợi), mà mở bằng "đây là TÊN TOÁN của cái bạn đã biết".**
R1.T1.4 (bài 26-29, 33) đã dạy TOÀN BỘ phần máy: `set`, `in`, `\|`, `&`,
`-`, không thứ tự, không trùng. Dạy lại bất kỳ phần nào trong đó là vi
phạm trực tiếp luật #1 của R2 ("mỗi bài đúng một khái niệm MỚI") — và
đúng loại lỗi mà T2.3 tự bắt được ở A2 (dạy lại R0.30/T1.2.5). Bài 1 do
đó không dạy "tập hợp là gì" từ đầu; nó dạy phép DỊCH: đọc lại một thứ
ĐÃ CÓ bằng một NGÔN NGỮ khác (ký hiệu toán).

**2. Vì sao `∈`/`∉` (bài 2-3) tách hai bài dù cả hai đều chỉ là "ký hiệu
của thứ đã biết".** Cùng lý do T2.3 tách "mệnh đề" (bài 1) khỏi "phủ định"
(bài 3): một ký hiệu vững trước, ký hiệu thứ hai xuất hiện như PHÉP VIẾT
TẮT của "không" ghép ký hiệu cũ — không phải hai thứ độc lập học riêng.

**3. Vì sao tập con (bài 6) đứng trước hợp/giao/hiệu (bài 8-11), ngược
nhiều sách xếp phép toán trước quan hệ bao hàm.** Chân lý rỗng của
`∅ ⊆ A` cần bài 11 T2.3 làm nền — đặt sớm để dùng ngay công cụ vừa học;
và bài 7 (hai tập bằng nhau qua `⊆` hai chiều) cần bài 6 đứng ngay trước
nó, giống T2.3 đặt kéo theo (bài 10) ngay trước tương đương hai chiều
(bài 15).

**4. Vì sao "tập vũ trụ và phần bù" (bài 12) — khái niệm MỚI đầu tiên
không có sẵn phép Python tương ứng trong 12 bài đầu — đứng SAU CẢ BỐN
phép cơ bản (∪/∩/rời nhau/−), không đứng xen giữa.** Phần bù LÀ MỘT
TRƯỜNG HỢP của hiệu (`Aᶜ = U − A`), nên nó cần bài 11 (hiệu) đứng trước
làm nền — đặt sớm hơn thì phải giải thích `−` hai lần.

**5. Vì sao cặp có thứ tự (bài 14) đứng NGAY sau đếm bù trừ (bài 13), tạo
khúc ngoặt rõ trong track.** Track CỐ Ý nhắc lại bài 1 bằng tương phản:
tập hợp thì thứ tự KHÔNG quan trọng, cặp thì thứ tự QUAN TRỌNG — đặt cạnh
nhau (13 bài tập hợp xong mới qua cặp) để tương phản còn tươi.

**6. Vì sao quan hệ (bài 16) được định nghĩa THẲNG là "một tập con của
tích Descartes", không phải khái niệm hoàn toàn mới.** Bản lề sư phạm
quan trọng nhất track: định nghĩa lại thành "vẫn là tập hợp, chỉ khác
phần tử của nó là CẶP" thì MỌI kỹ thuật tập hợp (bài 1-13) áp dụng
NGUYÊN VẸN lên quan hệ, không học lại.

**7. Vì sao ba tính chất (phản xạ/đối xứng/bắc cầu, bài 19-21) đi TRƯỚC
"quan hệ tương đương" (bài 22) đúng ba bài liền, không gộp một định
nghĩa.** Mỗi tính chất cần MỘT phản ví dụ riêng để thấy nó THẬT là một
ràng buộc có thể vi phạm — gộp cả ba thì người học nhớ TÊN mà không cầm
được lý do vì sao BA điều kiện, không phải hai hay bốn.

**8. Vì sao ánh xạ (bài 24) được định nghĩa QUA quan hệ, không phải "một
cái máy nhận đầu vào trả đầu ra".** Cùng lý do mục 6: TOÀN BỘ 23 bài
trước áp dụng thẳng lên ánh xạ. Đơn ánh/toàn ánh khi đó chỉ là hai câu
hỏi ĐẾM khác nhau trên đúng một bảng phân công.

**9. Vì sao song ánh và hợp thành gộp một bài (27), rồi BOSS (28) KHÔNG
dạy khái niệm mới mà vẫn tách riêng khỏi bài 27.** Song ánh + hợp thành
là hệ QUẢ trực tiếp của bài 24-26, hợp lý đứng chung một bài dạy — nhưng
BOSS cần đứng RIÊNG vì nó ghép LẠI cả ba mảng lớn (tập hợp + quan hệ +
ánh xạ) trên một khu vườn DUY NHẤT, đúng vai trò BOSS trong R2 (T2.1 bài
44, T2.3 bài 32): không dạy, chỉ ghép.

**Ranh giới track:** không bài nào cần đếm số phần tử kiểu tổ hợp (chọn k
trong n, hoán vị) — bài 13 cố ý để ngỏ câu hỏi "ba tập cùng lúc thì sao",
dành T2.5. Không bài nào cần ma trận hay đồ thị có hướng thật — bài 17
chỉ dùng bảng 0/1 như cách TRÌNH BÀY, không đi sâu lý thuyết ma trận
(dành T2.6/T2.7). Và — biên quan trọng nhất — **không bài nào dạy LẠI
cách gõ `set`/`in`/`\|`/`&`/`-` trong Python; toàn bộ phần đó đã đóng ở
R1.T1.4.**
## T2.5 — Tổ hợp, xác suất, thống kê (Realm 2 · Toán & Toán rời rạc · Python CHỈ để KIỂM · 34 bài)

## Mạch T2.5 — Tổ hợp, xác suất, thống kê

Người học vào track đã xong T2.1-T2.4: số/phép tính (T2.1), biến/biểu thức
(T2.2), mệnh đề/`and`/`or`/`not`/chứng minh (T2.3), và **tập hợp/quan hệ/ánh
xạ** (T2.4) — nền tảng track này DỰA THẲNG vào, không phải chỉ tiếp nối.
T2.4 bài 13 (đếm bù trừ hai tập) và bài 28 (reflect đóng track: "mấy CÂY mỗi
loại, mấy KILÔGAM thu hoạch — tập hợp không trả lời được — track sau đếm
được không?") ĐỀU cố ý để ngỏ đúng cho track này trả lời.

**QUAN TRỌNG — kiểm KỸ trước khi viết bài 1: R1 (T1.2 bài 27, T1.4) đã dạy
phần MÁY của "đếm" và "trung bình" rồi, track này KHÔNG được dạy lại.** Rà:

| Đã dạy ở R1 | Bài | Nội dung |
|---|---|---|
| `core.counter-if` | T1.2 bài 14 `dem-nhung-luot-dang-ke` | đếm bằng vòng lặp + biến cộng dồn khi thoả điều kiện |
| `core.max-tracker` | T1.2 bài 15 `giu-lai-ky-luc` | giữ giá trị lớn nhất thấy được qua một lượt duyệt |
| `core.multi-accumulator` | T1.2 bài 27 `mot-luot-nhieu-cau-tra-loi` | tính **trung bình** bằng `tong / dem` trong CÙNG một lượt duyệt |

Nghĩa là: **vòng lặp đếm, cộng dồn, và phép chia `tong/dem` ra "trung bình"
ĐỀU đã là kiến thức CŨ.** Việc của track này KHÔNG PHẢI dạy lại cách GÕ các
phép đó (đúng luật đã lặp lại từ T2.3 tới T2.4). Việc CỦA track này là dạy
lớp **Ý NGHĨA TOÁN HỌC**: đếm CÓ HỆ THỐNG không cần liệt kê tay (quy tắc
cộng/nhân, hoán vị, tổ hợp), xác suất trên nền tập hợp đã có (T2.4), biến
ngẫu nhiên như một ÁNH XẠ (T2.4 bài 24), và VÌ SAO `tong/dem` đo được cái nó
đo (kỳ vọng, độ bền trước ngoại lệ) — không dòng Python nào ở R1 chạm tới bất
kỳ cái nào trong nhóm này.

**Hiện vật xuyên suốt:** vẫn khu vườn của Byte (T2.1-T2.4), giờ thêm ba lớp
dữ liệu mới mà ba phần của track lần lượt mở ra:

1. "Bốn luống, xếp thứ tự thu hoạch — có bao nhiêu cách?" — không liệt kê
   tay nổi khi số luống lớn → **đếm có hệ thống**.
2. "Túi hạt giống trộn lẫn, rút một hạt ngẫu nhiên — hạt nào ra?" — một tập
   con của MỌI khả năng (T2.4 bài 6, tập con) → **xác suất**.
3. "Sáu mùa vụ, mỗi mùa một con số kilôgam thu hoạch — con số nào ĐẠI DIỆN
   cho cả sáu?" — trung bình đã biết cách TÍNH (R1), giờ học Ý NGHĨA và giới
   hạn của nó → **thống kê**.

Ba câu hỏi ấy dùng lại đúng MỘT hiện vật, không hiện vật mới nào chen vào
giữa track — giữ luật đã áp dụng suốt R2.

**Python trong track này không bao giờ là lời giải, CHỈ để KIỂM** — `for`,
`if`, cộng dồn (R1) tính đếm/tần suất thật; `set`/`in`/`⊆` (T2.4) dựng không
gian mẫu và biến cố; `tong/dem` (R1 bài 27) tính trung bình — track này KIỂM
LẠI những con số ấy có khớp công thức tổ hợp/xác suất/thống kê vừa học hay
không, không dạy lại cách gõ.

| # | slug | Tiêu đề | Khái niệm mới (đúng một) | `reflect` cuối bài | Dựa trên |
|---|---|---|---|---|---|
| 1 | `quy-tac-cong` | Quy tắc cộng | **`\|A ∪ B\| = \|A\| + \|B\|`** khi `A ∩ B = ∅` (T2.4 bài 10) — trường hợp RIÊNG, không-chồng-lấn, của đếm bù trừ (T2.4 bài 13); ba túi hạt giống KHÔNG trộn (cà chua/xà lách/cà rốt) — tổng hạt là cộng thẳng ba số | Byte có MỘT túi hạt giống trộn LẪN — một hạt vừa mang nhãn "giống mới" vừa mang nhãn "chịu hạn" (hai nhãn CHỒNG trên CÙNG một hạt, không phải ba túi tách biệt). Cộng thẳng số hạt "giống mới" với số hạt "chịu hạn" có ra đúng tổng hạt mang ít nhất một nhãn không? | T2.4 `hai-tap-roi-nhau`, `dem-bang-bu-tru` |
| 2 | `quy-tac-cong-ba-tap` | Quy tắc cộng tổng quát — ba tập chồng lấn | **`\|A∪B∪C\| = \|A\|+\|B\|+\|C\|−\|A∩B\|−\|A∩C\|−\|B∩C\|+\|A∩B∩C\|`** — mở rộng bù trừ hai tập (T2.4 bài 13) lên BA, trả lời đúng câu hỏi T2.4 bài 13 để ngỏ ("ba tập cùng lúc thì sao") | Đếm nhãn xong. Giờ Byte muốn biết có bao nhiêu cách XẾP THỨ TỰ thu hoạch bốn luống trong một buổi sáng — luống nào trước, luống nào sau. Quy tắc cộng có trả lời được câu "bao nhiêu cách xếp" không, hay đây là một câu hỏi khác hẳn? | 1, T2.4 `dem-bang-bu-tru` |
| 3 | `quy-tac-nhan` | Quy tắc nhân | Khi một lựa chọn có `m` cách, lựa chọn TIẾP THEO (không phụ thuộc lựa chọn trước) có `n` cách, tổng số cách LÀ `m × n` — chính là `\|A × B\| = \|A\| · \|B\|` (T2.4 bài 15, tích Descartes) áp dụng lên NHIỀU bước liên tiếp, không chỉ hai tập | Bốn luống, MỖI luống chọn 1 trong 3 loại hạt giống, ĐỘC LẬP nhau (một luống dùng hạt nào không ảnh hưởng luống khác) → `3×3×3×3 = 81` cách. Nếu túi hạt CHỈ CÓ ĐÚNG MỘT hạt mỗi loại (dùng rồi thì hết, không dùng lại được) thì phép nhân trên còn đúng không? | 2, T2.4 `tich-descartes` |
| 4 | `dem-co-lap-lai-hay-khong` | Đếm: lặp lại được phép hay không | Hai kiểu đếm KHÁC nhau tuỳ ĐIỀU KIỆN của bài toán: **có lặp** (mỗi bước vẫn đủ `n` lựa chọn, dùng rồi vẫn còn) cho `n × n × ... × n = nᵏ`; **không lặp** (mỗi bước bớt một lựa chọn đã dùng) cho `n × (n−1) × (n−2) × ...` — CÙNG quy tắc nhân (bài 3), khác ở SỐ LỰA CHỌN mỗi bước có giữ nguyên hay giảm | Túi CHỈ có đúng bốn hạt giống, KHÔNG lặp — Byte muốn gieo CẢ BỐN hạt vào bốn luống, mỗi luống một hạt, MỌI thứ tự gieo đều tính là một cách khác nhau. Nhân `4×3×2×1` được bao nhiêu — và con số CUỐI (nhân tới 1) này có phải lúc nào cũng vậy không? | 3 |
| 5 | `giai-thua` | Giai thừa | **`n! = n × (n−1) × ... × 2 × 1`** — số cách sắp xếp TOÀN BỘ `n` phần tử không lặp (bài 4, trường hợp đếm HẾT); `0! = 1` (một cách sắp xếp "không có gì" — ĐÚNG một cách, không phải không cách nào, giống chân lý rỗng T2.3 bài 11) | Bốn luống, `4! = 24` cách xếp thứ tự thu hoạch TOÀN BỘ. Nhưng Byte chỉ có thời gian thu hoạch ĐÚNG hai luống sáng nay (không phải cả bốn) — xếp thứ tự cho HAI trong bốn luống được bao nhiêu cách, và `4!` còn dùng thẳng được không? | 4, T2.3 `khi-ve-truoc-khong-xay-ra` |
| 6 | `hoan-vi-chon-k` | Hoán vị chọn k | **`P(n,k) = n! / (n−k)!`** — số cách sắp xếp CÓ THỨ TỰ `k` phần tử được chọn TỪ `n` (bài 5 là trường hợp riêng `k=n`, vì `(n−n)! = 0! = 1`); chọn 2 trong 4 luống ĐỂ THU HOẠCH TRƯỚC-SAU: `P(4,2) = 4!/2! = 12` | Byte lại chọn hai luống, NHƯNG lần này KHÔNG quan tâm thu hoạch luống nào trước — chỉ cần biết ĐÚNG hai luống nào được chọn hôm nay. `P(4,2) = 12` có còn là câu trả lời đúng không, hay đang ĐẾM DƯ một thứ mà câu hỏi mới không cần? | 5 |
| 7 | `to-hop-chon-k` | Tổ hợp chọn k | **`C(n,k) = n! / (k! (n−k)!)`** — số cách chọn `k` phần tử từ `n`, KHÔNG quan tâm thứ tự; mỗi nhóm `k` phần tử bị `P(n,k)` (bài 6) đếm LẶP LẠI đúng `k!` lần (mọi cách sắp xếp lại CÙNG nhóm ấy), nên chia cho `k!` để đếm mỗi nhóm ĐÚNG một lần | Byte tính `C(4,2) = 6`, rồi tính tiếp `C(4,2)` theo cách chọn "hai luống KHÔNG được chọn" thay vì "hai luống ĐƯỢC chọn" — hai câu hỏi nghe khác hẳn nhau. Kết quả có trùng nhau không, và nếu có thì VÌ SAO? | 6, T2.4 `tu-ro-sang-tap-hop` |
| 8 | `doi-xung-to-hop` | Đối xứng của tổ hợp | **`C(n,k) = C(n,n−k)`** — chọn `k` phần tử GIỮ LẠI cũng chính là chọn `n−k` phần tử BỎ RA, hai câu hỏi đếm CÙNG một tập kết quả nhìn từ hai phía; giải đáp trực tiếp câu hỏi bài 7 để ngỏ | `C(4,2)=6` đối xứng quanh `k=2`. Xếp `C(4,0), C(4,1), C(4,2), C(4,3), C(4,4)` thành một hàng — `1, 4, 6, 4, 1`. Có cách nào TÍNH hàng tiếp theo (`n=5`) từ hàng NÀY, không cần tính lại giai thừa từ đầu? | 7 |
| 9 | `tam-giac-pascal` | Tam giác Pascal | **`C(n,k) = C(n−1,k−1) + C(n−1,k)`** — công thức truy hồi: một nhóm `k` phần tử từ `n` HOẶC chứa một phần tử cố định (chọn thêm `k−1` từ `n−1` còn lại) HOẶC không chứa nó (chọn đủ `k` từ `n−1` còn lại) — hai trường hợp RỜI NHAU (bài 1) cộng lại vừa khít | Cộng CẢ HÀNG `n=4`: `1+4+6+4+1 = 16 = 2⁴`. Trùng hợp, hay CỘNG một hàng Pascal LUÔN ra luỹ thừa của 2 — và nếu luôn đúng, nó đang ĐẾM cái gì? | 8 |
| 10 | `tong-to-hop-la-luy-thua-hai` | Tổng mọi tổ hợp là luỹ thừa của 2 | **`C(n,0) + C(n,1) + ... + C(n,n) = 2ⁿ`** — cộng số cách chọn 0, 1, 2, ..., `n` phần tử chính LÀ đếm MỌI tập con có thể của một tập `n` phần tử (mỗi phần tử: có mặt hay không, hai lựa chọn — quy tắc nhân bài 3 áp `n` lần) — đóng vòng THẲNG về T2.4 (mọi tập hợp có `2ⁿ` tập con, không phát biểu ở đó) | Đếm CÁCH CHỌN xong — mười bài liền không có một "hạt" hay "luống" ngẫu nhiên nào cả, chỉ đếm KHẢ NĂNG. Byte rút MỘT hạt thật từ túi trộn — hạt nào ra là chuyện MAY RỦI, không còn đếm hết mọi khả năng nữa. Có cách nào NÓI VỀ sự may rủi ấy bằng con số không? | 9, T2.4 `tu-ro-sang-tap-hop`, `hai-tap-bang-nhau` |
| 11 | `khong-gian-mau-va-bien-co` | Không gian mẫu và biến cố | **Không gian mẫu `Ω`** — tập hợp MỌI kết quả có thể (T2.4 bài 1, tập hợp); **biến cố `A`** — MỘT tập con của `Ω` (T2.4 bài 6, `⊆`), tức một NHÓM kết quả gộp lại vì cùng chung một điều kiện | Túi có 10 hạt: 5 cà chua, 3 xà lách, 2 cà rốt. `Ω` có 10 phần tử. Biến cố "rút được cà chua" có 5 phần tử. Hai con số ấy (5 và 10) nói được gì về ĐỘ MAY RỦI của việc rút trúng cà chua, so với rút trúng cà rốt? | T2.4 `tu-ro-sang-tap-hop`, `tap-con` |
| 12 | `xac-suat-co-dien` | Xác suất cổ điển | **`P(A) = \|A\| / \|Ω\|`** khi MỌI kết quả trong `Ω` đều ĐỀU khả năng (mỗi hạt như nhau, không hạt nào "dễ rút hơn") — biến ĐỘ MAY RỦI thành một con số giữa 0 và 1; `P(cà chua) = 5/10 = 0.5` | `P(cà chua) = 0.5`, `P(xà lách) = 0.3`, `P(cà rốt) = 0.2`. Cộng ba con số lại — `1.0`. Trùng hợp, hay `P(A)` của MỌI kết quả CÓ THỂ trong `Ω` LUÔN cộng lại đúng 1, và VÌ SAO? | 11 |
| 13 | `bien-co-phan-bu` | Biến cố phần bù | **`P(A') = 1 − P(A)`** — phần bù của biến cố (T2.4 bài 12, `Aᶜ = Ω − A`) có xác suất bằng phần CÒN LẠI của 1; "không rút được cà chua" và "rút được cà chua" CHIA HẾT xác suất 1.0, không thừa không thiếu | `P(\text{không cà chua}) = 1 − 0.5 = 0.5`. Kiểm lại bằng cách CỘNG THẲNG `P(xà lách) + P(cà rốt) = 0.3+0.2 = 0.5` — khớp. Hai cách tính RA CÙNG một số — cách nào dùng được khi biến cố "không cà chua" GỘP nhiều loại khác, cách nào NHANH hơn? | 12, T2.4 `tap-vu-tru-va-phan-bu` |
| 14 | `hai-bien-co-roi-nhau` | Hai biến cố rời nhau | **`P(A∪B) = P(A) + P(B)`** khi `A ∩ B = ∅` (T2.4 bài 10) — "rút cà chua" và "rút xà lách" rời nhau (một hạt không thể vừa là cà chua vừa là xà lách), cộng thẳng xác suất được; ứng trực tiếp quy tắc cộng (bài 1) lên xác suất | `P(\text{cà chua hoặc xà lách}) = 0.5+0.3 = 0.8`. Giờ Byte hỏi khác: "rút được hạt GIỐNG MỚI" và "rút được hạt cà chua" — một hạt có thể VỪA là cà chua VỪA là giống mới cùng lúc. Cộng thẳng hai xác suất ấy còn đúng không? | 13, T2.4 `hai-tap-roi-nhau` |
| 15 | `hop-hai-bien-co-tong-quat` | Hợp hai biến cố tổng quát | **`P(A∪B) = P(A) + P(B) − P(A∩B)`** — bù trừ (T2.4 bài 13, bài 2) áp lên xác suất khi `A`, `B` CHỒNG LẤN; trả lời đúng câu hỏi bài 14 để ngỏ, không đổi công thức nền, chỉ đổi thứ đang đếm (xác suất thay vì số phần tử) | Byte kiểm: nếu `A` và `B` rời nhau, `P(A∩B) = P(∅) = 0`, công thức bài này TỰ RÚT GỌN về công thức bài 14. Vậy bài 14 có còn cần dạy riêng không, hay nó luôn LÀ một trường hợp của bài này? *(để ngỏ có chủ đích — cả hai cùng hữu ích, mỗi cái nhanh hơn ở đúng tình huống của nó)* | 14, T2.4 `dem-bang-bu-tru` |
| 16 | `bien-co-doc-lap` | Biến cố độc lập | **`P(A∩B) = P(A) · P(B)`** khi `A`, `B` ĐỘC LẬP (biết `A` xảy ra không đổi gì cơ hội `B` xảy ra) — ứng quy tắc nhân (bài 3) lên xác suất; rút MỘT hạt từ luống 1 VÀ một hạt khác từ luống 3 (hai túi TÁCH RIÊNG) là độc lập | Rút rồi KHÔNG bỏ lại (rút hạt 1 xong, rút tiếp hạt 2 từ CÙNG túi đã vơi một hạt) — "rút được cà chua lần 1" có làm đổi cơ hội "rút được cà chua lần 2" không? Công thức nhân bài này còn dùng thẳng được không? | 15, 3 |
| 17 | `quy-tac-nhan-cho-day-phep-thu` | Quy tắc nhân cho một dãy phép thử | Mở rộng bài 16 lên `n` phép thử ĐỘC LẬP liên tiếp: **`P(A₁∩A₂∩...∩Aₙ) = P(A₁)·P(A₂)·...·P(Aₙ)`** — gieo bốn hạt, MỖI hạt độc lập nảy mầm với xác suất riêng, xác suất CẢ BỐN cùng nảy = tích bốn xác suất | Quy tắc nhân (bài này) cần các biến cố ĐỘC LẬP. Byte ĐÃ rút được một hạt và biết TRƯỚC nó là "giống mới" — xác suất hạt ĐÓ cũng là cà chua có còn tính bằng `P(cà chua)` như cũ không, hay phải tính LẠI vì đã có THÊM một thông tin? | 16, 3 |
| 18 | `xac-suat-co-dieu-kien` | Xác suất có điều kiện | **`P(A\|B) = P(A∩B) / P(B)`** — xác suất của `A`, BIẾT TRƯỚC `B` đã xảy ra (thu hẹp `Ω` xuống còn đúng `B`); "xác suất rút được cà chua, BIẾT TRƯỚC hạt vừa rút là giống mới" — thu hẹp không gian mẫu xuống túi con "giống mới" | `P(\text{cà chua}\|\text{giống mới}) = 0.5`, TRÙNG KHỚP `P(\text{cà chua})=0.5` không điều kiện (bài 12) — biết trước giống mới KHÔNG đổi cơ hội cà chua. Trùng hợp, hay hai biến cố này có một quan hệ đặc biệt — quan hệ đã có TÊN riêng ở bài 16? | 17, T2.4 `mien-xac-dinh-mien-gia-tri` |
| 19 | `kiem-tra-doc-lap-bang-dieu-kien` | Kiểm tra độc lập bằng điều kiện | **`A`, `B` độc lập ⟺ `P(A\|B) = P(A)`** — biết `B` xảy ra rồi mà cơ hội `A` KHÔNG đổi, đúng nghĩa "độc lập" (bài 16) phát biểu lại qua điều kiện (bài 18); hai định nghĩa, MỘT khái niệm, đúng kỹ thuật "khi và chỉ khi" (T2.3 bài 15) | Byte tính `P(\text{cà chua}\|\text{giống mới})` VÀ `P(\text{giống mới}\|\text{cà chua})` — hai con số này bằng công thức na ná nhau (cùng chia cho `P(A∩B)` Ở tử) nhưng mẫu số khác, và KHÁC nhau. Có công thức nào nối HAI chiều điều kiện ấy lại, suy chiều này TỪ chiều kia, không cần đếm lại từ đầu? | 18, 16, T2.3 `khi-va-chi-khi` |
| 20 | `cong-thuc-bayes` | Công thức Bayes | **`P(A\|B) = P(B\|A)·P(A) / P(B)`** — đảo chiều điều kiện, suy `P(A\|B)` từ `P(B\|A)` đã biết (thay vì đếm lại từ đầu); rút RA từ chính công thức bài 18 viết theo hai chiều (`P(A∩B)=P(A\|B)P(B)=P(B\|A)P(A)`), không phải một công thức mới độc lập | Byte biết `P(\text{nảy mầm}\|\text{giống mới}) = 0.9` và `P(\text{nảy mầm}\|\text{giống cũ}) = 0.6`, cùng tỉ lệ giống mới/cũ trong túi. MỘT hạt đã nảy mầm — xác suất nó LÀ giống mới bao nhiêu? Bayes trả lời được, nhưng cần THÊM dữ kiện gì trước khi tính? | 18 |
| 21 | `cay-xac-suat` | Cây xác suất | **Sơ đồ cây** — mỗi tầng một phép thử, mỗi nhánh một kết quả kèm xác suất RIÊNG (có điều kiện, bài 18); xác suất một ĐƯỜNG ĐI (từ gốc tới lá) là TÍCH các nhánh trên đường (quy tắc nhân, bài 17); cộng các đường cùng đích LÀ hợp (bài 14/15) — công cụ TRỰC QUAN ghép lại năm bài 14-20, không khái niệm số học mới | Chín bài liền (11-21) đều xoay quanh MỘT hạt hay MỘT lần thử. Byte giờ gieo BỐN MƯƠI hạt, mỗi hạt độc lập nảy mầm hay không — muốn hỏi "TRUNG BÌNH bao nhiêu hạt nảy mầm", không phải "hạt NÀY nảy mầm hay không". Câu hỏi mới cần công cụ gì? | 17, 15, 18, 20 |
| 22 | `bien-ngau-nhien-la-anh-xa` | Biến ngẫu nhiên là một ánh xạ | **Biến ngẫu nhiên `X`** — một ÁNH XẠ (T2.4 bài 24) từ không gian mẫu `Ω` sang một tập SỐ; "số hạt nảy mầm trong 4 hạt gieo" là ánh xạ từ MỖI kết quả có thể (tổ hợp nảy/không nảy của 4 hạt) sang một số nguyên từ 0 đến 4 | `X` gán MỖI kết quả trong `Ω` một số. NHIỀU kết quả khác nhau trong `Ω` có thể cùng gán ra MỘT giá trị của `X` (ví dụ "hạt 1,2 nảy, 3,4 không" VÀ "hạt 3,4 nảy, 1,2 không" cùng cho `X=2`) — ánh xạ này có đơn ánh không (T2.4 bài 25)? Điều đó nói lên gì? | T2.4 `anh-xa-la-quan-he-dac-biet`, `don-anh` |
| 23 | `phan-phoi-xac-suat` | Phân phối xác suất | **Bảng phân phối** của biến ngẫu nhiên rời rạc — liệt kê MỌI giá trị `X` có thể nhận, kèm `P(X=x)` cho MỖI giá trị (gộp MỌI kết quả trong `Ω` cho ra cùng giá trị đó, bài 22 để ngỏ); cột `P(X=x)` cộng lại LUÔN đúng 1 (bài 12 áp lên `X` thay vì lên `Ω` trực tiếp) | Bảng phân phối của "số hạt nảy trong 4" có năm dòng (`X=0` tới `X=4`). Byte muốn MỘT con số DUY NHẤT tóm tắt cả bảng — "trung bình bao nhiêu hạt nảy mầm MỖI LẦN gieo". Cộng thẳng năm giá trị `X` rồi chia 5 có đúng không, hay phải tính khác vì các dòng KHÔNG đều xác suất? | 22, 12 |
| 24 | `ky-vong` | Kỳ vọng | **`E[X] = Σ x · P(X=x)`** — trung bình CÓ TRỌNG SỐ theo xác suất, không phải trung bình thường của các giá trị `x`; giá trị `x` càng có xác suất cao càng "kéo" `E[X]` về phía nó — trả lời đúng câu hỏi bài 23 để ngỏ | `E[X] = 2.0` hạt nảy mầm — một con số nguyên Ở ví dụ này (bảng đối xứng), nhưng KHÔNG PHẢI luôn vậy. Byte gieo đúng MỘT lần — có lần nào `X` thật sự "bằng kỳ vọng" theo nghĩa MỘT kết quả cụ thể không? Vậy `E[X]` đang nói về CÁI GÌ, nếu không phải kết quả một lần gieo cụ thể? | 23 |
| 25 | `tuyen-tinh-cua-ky-vong` | Tuyến tính của kỳ vọng | **`E[X+Y] = E[X] + E[Y]`** — LUÔN đúng, kể cả khi `X`, `Y` KHÔNG độc lập (khác hẳn bài 16, nơi độc lập là ĐIỀU KIỆN bắt buộc); tổng kỳ vọng hai luống = kỳ vọng của tổng, cộng thẳng không cần biết `X`, `Y` có liên quan nhau hay không | Byte có bốn luống, MỖI luống một `Xᵢ` riêng (số hạt nảy mầm luống đó). `E[X₁+X₂+X₃+X₄]` cộng thẳng bốn kỳ vọng — nhanh hơn hẳn tính phân phối của TỔNG rồi mới lấy kỳ vọng. Kỳ vọng đo được TRUNG TÂM của `X`. Có con số nào đo được `X` "TẢN RA" xa trung tâm bao nhiêu không? | 24 |
| 26 | `phuong-sai-va-do-lech-chuan` | Phương sai và độ lệch chuẩn | **`Var(X) = E[(X−E[X])²]`** — trung bình BÌNH PHƯƠNG khoảng cách tới kỳ vọng (bình phương để khoảng cách ÂM không triệt tiêu khoảng cách DƯƠNG); **`σ = √Var(X)`** — độ lệch chuẩn, cùng ĐƠN VỊ với `X` (phương sai thì đơn vị bị bình phương theo) | Track đếm (1-10), xác suất (11-21), biến ngẫu nhiên (22-26) đều là LÝ THUYẾT — tính TRƯỚC khi gieo hạt thật. Byte GIEO THẬT sáu mùa liền, GHI LẠI sáu con số kilôgam thu hoạch. `tong/dem` (R1 bài 27) đã biết tính trung bình SÁU con số ấy — nó có phải `E[X]` của bài 24 không, hay là một thứ khác? | 25 |
| 27 | `trung-binh-du-lieu-that` | Trung bình của dữ liệu thật | **Trung bình mẫu `x̄ = (Σxᵢ)/n`** — TÍNH giống hệt `tong/dem` (R1 bài 27, đã biết CÁCH GÕ); nhưng khác `E[X]` (bài 24) Ở CHỖ: `x̄` tính từ dữ liệu ĐÃ XẢY RA (sáu mùa CÓ THẬT), `E[X]` tính từ xác suất DỰ ĐOÁN trước khi xảy ra — hai con số CÙNG công thức trọng số đều, khác NGUỒN gốc | Sáu mùa: `40, 42, 41, 39, 43, 5` (kg). Mùa cuối mất mùa nặng (sâu bệnh). `x̄ = 35`. Con số `35` có ĐẠI DIỆN đúng cho "một mùa BÌNH THƯỜNG" của Byte không, hay đang bị MỘT con số bất thường kéo lệch? | 26, 24 |
| 28 | `trung-vi` | Trung vị | **Trung vị** — sắp dữ liệu THEO THỨ TỰ, lấy giá trị Ở GIỮA (hoặc trung bình hai giá trị giữa nếu `n` chẵn); KHÔNG cộng dồn như `x̄`, chỉ quan tâm VỊ TRÍ sau khi sắp — một mùa bất thường không kéo nó đi xa | Sáu mùa sắp lại: `5, 39, 40, 41, 42, 43`. Trung vị (trung bình hai số giữa `40`, `41`) là `40.5` — GẦN năm mùa bình thường hơn hẳn `x̄=35`. Đổi mùa mất mùa từ `5` xuống `0` (mất trắng) — trung vị có đổi theo không? So với `x̄` thì sao? | 27 |
| 29 | `khi-nao-dung-trung-vi` | Khi nào dùng trung vị thay trung bình | **Trung vị BỀN trước ngoại lệ** (đổi một giá trị cực đoan gần như không đổi trung vị), **trung bình NHẠY** (mọi giá trị, kể cả cực đoan, đều góp phần trực tiếp vào tổng); chọn công cụ nào tuỳ CÂU HỎI — muốn biết "tổng cả năm" thì cần `x̄` (nhân lại ra tổng), muốn biết "mùa ĐIỂN HÌNH" thì trung vị đáng tin hơn | Sáu mùa của Byte không có mùa nào TRÙNG kilôgam với mùa khác. Lan (người làm vườn khác, T2.4) có SÁU mùa mà HAI mùa CÙNG cho đúng `40` kg — số nào xuất hiện NHIỀU LẦN nhất trong một tập dữ liệu, và nó khác trung vị Ở CHỖ nào? | 28, 27 |
| 30 | `yeu-vi-mode` | Yếu vị (mode) | **Mode** — giá trị (hoặc CÁC giá trị, có thể có nhiều mode) xuất hiện NHIỀU LẦN nhất; KHÁC trung vị (vị trí giữa) VÀ trung bình (tổng chia đều) — đo "phổ biến nhất", không đo "trung tâm" hay "đại diện tổng" | Sáu mùa của Lan: `40, 40, 38, 41, 39, 42`. Mode là `40` (2 lần). Đổi một mùa `38` thành `40` — giờ `40` xuất hiện BA lần, còn các số khác vẫn mỗi số MỘT lần. `x̄` và trung vị đổi Ở MỨC nào so với mode? | 29 |
| 31 | `phuong-sai-mau` | Phương sai mẫu và độ lệch chuẩn mẫu | **Phương sai mẫu `s² = (Σ(xᵢ−x̄)²)/n`** — CÙNG công thức Var(X) (bài 26) áp lên dữ liệu THẬT thay vì phân phối lý thuyết, dùng `x̄` (bài 27) thay `E[X]`; **độ lệch chuẩn mẫu `s = √s²`** — đo sáu mùa "TẢN RA" xa `x̄` bao nhiêu, cùng đơn vị kilôgam | Sáu mùa của Byte (có mùa mất trắng) và sáu mùa của Lan (đều đặn quanh 40) có THỂ cùng `x̄` gần bằng nhau (tuỳ số liệu), nhưng CHẮC CHẮN khác `s`. `s` lớn hơn nói lên điều gì về ĐỘ ĐÁNG TIN của việc dùng `x̄` để dự đoán mùa TỚI? | 26, 27 |
| 32 | `tu-phan-vi` | Tứ phân vị | **Tứ phân vị** `Q1`, `Q2`, `Q3` — chia dữ liệu ĐÃ SẮP thành BỐN phần đều nhau về SỐ LƯỢNG điểm (`Q2` chính LÀ trung vị, bài 28); `Q1` là trung vị của NỬA dưới, `Q3` là trung vị của NỬA trên — mô tả HÌNH DÁNG phân bố chi tiết hơn một con số trung tâm | Với tám mùa (đủ chia bốn phần rõ): `Q1=38`, `Q3=44`. Khoảng cách `Q3−Q1 = 6` gọi là gì, và một mùa TỤT xuống `10` kg (rất xa so với khoảng `38-44` bình thường) có nằm trong khoảng đó không? | 28 |
| 33 | `phat-hien-ngoai-le-iqr` | Phát hiện ngoại lệ bằng IQR | **`IQR = Q3 − Q1`**; một điểm dữ liệu LÀ ngoại lệ (outlier) nếu nó nằm NGOÀI khoảng `[Q1 − 1.5·IQR, Q3 + 1.5·IQR]` — một QUY TẮC cụ thể để trả lời câu hỏi bài 27 để ngỏ ("con số nào là bất thường"), không còn phải NHÌN bằng mắt | Mùa `5` kg của Byte (bài 27) — kiểm bằng IQR có thật sự là ngoại lệ, hay chỉ "trông có vẻ thấp"? Giờ track đã có: đếm (1-10), xác suất (11-21), biến ngẫu nhiên (22-26), thống kê mô tả (27-33). Ghép cả bốn phần lên MỘT vụ mùa trông như thế nào? | 32, 27 |
| 34 | `boss-mot-vu-mua-hoan-chinh` | BOSS — Một vụ mùa hoàn chỉnh | *(không khái niệm mới — bài tổng hợp)* ghép đếm (chọn `k` luống thu hoạch bằng `C(n,k)`) + xác suất (rút hạt từ túi trộn, `P(A)`) + biến ngẫu nhiên/kỳ vọng (`E[X]` số hạt nảy mầm) + thống kê (`x̄`, trung vị, IQR trên sáu mùa thật) TRÊN MỘT vụ mùa của khu vườn | Bốn mươi bốn bài từ T2.1: số, biến, mệnh đề, tập hợp, quan hệ, ánh xạ, đếm, xác suất, thống kê — mỗi phần một CÁCH nhìn khu vườn của Byte. Phần nào còn thiếu để trả lời "LUỐNG NÀO nên trồng gì, KHI NÀO" — một câu hỏi cần nhìn xa hơn MỘT vụ, sang cấu trúc LẶP LẠI qua thời gian? *(dẫn sang T2.6 — Đồ thị, modular, đại số trừu tượng)* | 1–33 |

**Vì sao thứ tự này đúng**

**1. Vì sao track KHÔNG mở bằng "đây là công thức tổ hợp `C(n,k)`" (như một
người mới sẽ mong đợi), mà mở bằng quy tắc cộng — trường hợp RIÊNG của một
công thức T2.4 đã dạy (đếm bù trừ, bài 13).** Đúng luật lặp lại từ T2.4: bài
1 không dạy khái niệm hoàn toàn mới, nó dạy phép ĐỌC LẠI một công thức ĐÃ CÓ
dưới một cái TÊN mới ("quy tắc cộng"), rồi NGAY bài 2 mở rộng công thức đó
lên ba tập — trả lời đúng câu hỏi T2.4 bài 13 cố ý để ngỏ.

**2. Vì sao quy tắc nhân (bài 3) tách hẳn khỏi quy tắc cộng (bài 1-2), không
gộp "hai quy tắc đếm cơ bản" thành một bài.** Cộng đếm khi các trường hợp
RỜI NHAU (hoặc-loại-trừ); nhân đếm khi các LỰA CHỌN nối tiếp nhau (và-đồng-
thời). Gộp hai thứ khác trục hoàn toàn (T2.3 bài 4-5: "và"/"hoặc" cũng tách
hai bài đúng vì lý do này) sẽ khiến người học lẫn khi nào cộng, khi nào nhân
— lỗi phổ biến nhất của người mới học tổ hợp.

**3. Vì sao "đếm có lặp hay không" (bài 4) đứng GIỮA quy tắc nhân (bài 3) và
giai thừa (bài 5), không gộp vào bài nào trong hai bài đó.** Giai thừa CHỈ
có nghĩa trong trường hợp KHÔNG lặp (mỗi bước bớt một lựa chọn) — nếu gộp
thẳng bài 3 sang bài 5, người học sẽ ngỡ MỌI phép đếm liên tiếp đều trừ dần,
trong khi quy tắc nhân bài 3 (ví dụ hạt giống có thể dùng lại mỗi luống) lại
KHÔNG trừ. Bài 4 dựng rõ ranh giới trước khi giai thừa xuất hiện như một
TRƯỜNG HỢP của "không lặp".

**4. Vì sao hoán vị (bài 6) đứng TRƯỚC tổ hợp (bài 7), dù nhiều người học sẽ
gặp tổ hợp trước trong đời thường ("chọn nhóm" nghe tự nhiên hơn "xếp thứ
tự").** Công thức tổ hợp (bài 7) ĐỊNH NGHĨA qua hoán vị — chia `P(n,k)` cho
`k!` — nên hoán vị phải LÀ nền đứng trước, đúng lối track T2.4 xây quan hệ
trên tập hợp, ánh xạ trên quan hệ (mỗi tầng định nghĩa QUA tầng trước).

**5. Vì sao xác suất (bài 11 trở đi) mở bằng "không gian mẫu là một tập hợp,
biến cố là một tập con", không mở bằng "xác suất là khả năng xảy ra".** Đây
là bản lề sư phạm quan trọng nhất track, giống hệt cách T2.4 định nghĩa quan
hệ QUA tập hợp (mục 6, T2.4): định nghĩa lại xác suất thành "một con số gắn
lên tập con của một tập hợp" thì MỌI kỹ thuật tập hợp (T2.4, bài 1-13 của
track này) áp dụng NGUYÊN VẸN — phần bù, hợp, giao, bù trừ đều đã có sẵn,
không học lại dưới cái tên mới.

**6. Vì sao "biến cố rời nhau" (bài 14) và "hợp tổng quát" (bài 15) TÁCH
hai bài dù bài 15 LÀM MẤT bài 14 (đặt `P(A∩B)=0` là ra ngay bài 14).** Cùng
lý do T2.4 bài 19-21 (ba tính chất tách ba bài): bài 14 cho một phản ví dụ
GỌN để thấy công thức đơn giản hoạt động trước khi bài 15 làm nó PHỨC TẠP
hơn — gộp thẳng sẽ khiến trường hợp rời-nhau (thường gặp nhất trong thực
hành) không được khắc riêng vào trí nhớ.

**7. Vì sao biến ngẫu nhiên (bài 22) được ĐỊNH NGHĨA là một ánh xạ, không
phải "một đại lượng có thể đổi giá trị theo may rủi".** Cùng bản lề mục 5,
áp lần hai: ánh xạ Ω→số ĐÃ có đầy đủ máy móc Ở T2.4 (bài 24-27: đơn ánh,
toàn ánh, ảnh của một phần tử) — bài 22 chỉ CHỈ RA biến ngẫu nhiên LÀ trường
hợp riêng của khái niệm đã học, không dạy lại "ánh xạ là gì" lần hai.

**8. Vì sao thống kê mô tả (bài 27 trở đi) KHÔNG mở lại "đây là cách tính
trung bình" bằng Python (R1 bài 27 đã dạy `tong/dem`), mà mở bằng "trung
bình mẫu khác kỳ vọng Ở CHỖ NÀO".** Đúng luật lặp từ T2.4 (không dạy lại
phần MÁY của R1) — bài 27 giả định người học ĐÃ gõ được `tong/dem` từ R1, và
dạy lớp Ý NGHĨA: phân biệt dữ liệu THẬT (thống kê) với xác suất DỰ ĐOÁN
(bài 22-26), một ranh giới khái niệm KHÔNG có trong bất kỳ dòng code R1 nào.

**9. Vì sao trung vị (bài 28) đứng NGAY sau trung bình (bài 27), rồi một bài
RIÊNG (bài 29) mới so sánh khi nào dùng cái nào — không gộp so sánh vào
ngay bài 28.** Mỗi công cụ cần đứng MỘT MÌNH trước để người học cầm được nó
mà không bị nhiễu bởi công cụ kia — đúng lối T2.4 tách ba tính chất (mục 7,
T2.4) trước khi ghép thành tương đương.

**10. Vì sao BOSS (bài 34) không dạy khái niệm mới nhưng vẫn đứng RIÊNG,
đúng vai trò BOSS đã lặp lại xuyên suốt R2 (T2.1 bài 44, T2.3 bài 32, T2.4
bài 28): không dạy, chỉ ghép BỐN mảng (đếm, xác suất, biến ngẫu nhiên,
thống kê) trên một vụ mùa DUY NHẤT của khu vườn.**

**Ranh giới track:** không bài nào chạm phân phối LIÊN TỤC (chuẩn, mũ —
cần tích phân, dành T2.7 ĐSTT & giải tích cho AI). Không bài nào chạm suy
diễn thống kê (kiểm định giả thuyết, khoảng tin cậy — cần phân phối lấy
mẫu, ngoài phạm vi track nền tảng này). Không bài nào chạm ma trận hiệp
phương sai hay hồi quy (dành T2.7/R8). Và — biên quan trọng nhất — **không
bài nào dạy lại cách gõ vòng lặp đếm, cộng dồn, hay `tong/dem` trong Python;
toàn bộ phần đó đã đóng ở R1 (T1.2 bài 14, 15, 27).**
## T2.6 — Đồ thị, modular, đại số trừu tượng (Realm 2 · Toán & Toán rời rạc · Python CHỈ để KIỂM · 28 bài)

## Mạch T2.6 — Đồ thị, modular, đại số trừu tượng

Người học vào track đã xong T2.1-T2.5 (số, biến, mệnh đề, tập hợp/quan
hệ/ánh xạ, tổ hợp/xác suất/thống kê).

**QUAN TRỌNG — kiểm KỸ trước khi viết bài 1: R3 "Khoa học máy tính"
(module `cau-truc-du-lieu`, DẠY đồ thị/cây bằng CODE — bảng kề, duyệt
DFS/BFS) đứng SAU toàn bộ realm `toan` trong `content/curriculum/thu-tu.
yaml` (`tools/kiem_do_thi.py` xếp HẠNG tuyến tính theo ĐÚNG thứ tự các
realm xuất hiện trong file đó — `toan` đứng TRƯỚC `khoa-hoc-may-tinh`).
Nghĩa là track NÀY **KHÔNG ĐƯỢC** `requires:` bất kỳ skill nào của R3
(`ds.graph`, `ds.graph-representation`, `ds.tree`,...) — cổng "Đồ thị
tiền đề" sẽ báo LỖI vì R3 CHƯA "được dạy" theo thứ tự tuyến tính đó, dù
đời thường R3 có thể học TRƯỚC hay SAU track này tuỳ người học.** Track
NÀY do đó dựng đồ thị HOÀN TOÀN từ MÁY MÓC đã có Ở T2.4 (quan hệ = tập
cặp, T2.4 bài 16) thay vì mượn CODE của R3 — **đồ thị vô hướng CHÍNH LÀ
một quan hệ ĐỐI XỨNG (T2.4 bài 20) trên tập đỉnh**, không cần khái niệm
"đồ thị" nào MỚI Ở tầng MÁY cả, chỉ cần ĐẶT TÊN toán học lên một quan hệ
đã biết cách dựng VÀ kiểm.

Việc CỦA track này LÀ dạy lớp **LÝ THUYẾT ĐỒ THỊ**: bậc của đỉnh, đường
đi/chu trình như đối tượng TOÁN học, liên thông, đường đi Euler, tô màu
— RỒI CHUYỂN sang một nhánh KHÁC hẳn của toán rời rạc, số học modular
(đồng dư, GCD, nghịch đảo modular) — RỒI KẾT bằng đại số trừu tượng
(nhóm), tái định vị "Ch1 Math Foundations" (lambda calculus nhẹ) LÀM
ĐỈNH track thay vì mở đầu (MASTERPLAN §9.1).

**Vì sao BA chủ đề (đồ thị, modular, đại số trừu tượng) đứng CHUNG một
track, dù nghe không liên quan.** Chúng dùng CHUNG một Ý TƯỞNG cốt lõi:
**cấu trúc LẶP LẠI/ĐÓNG dưới một phép toán** — đồ thị: bậc/liên thông là
tính chất BẤT BIẾN dưới các phép biến đổi; modular: phép cộng/nhân "quay
vòng" rồi ĐÓNG lại trong một tập hữu hạn; nhóm: bốn tiên đề mô tả CHÍNH
XÁC khi nào một phép toán "đóng và quay vòng tốt". Bài 25 (nhóm) chỉ RA
modular (bài 13-20) VÀ đồ thị đối xứng (bài 1-12) ĐỀU LÀ ví dụ CỤ THỂ của
MỘT cấu trúc trừu tượng — track ĐÓNG bằng cách NHÌN LẠI hai phần đầu qua
lăng kính THỨ BA.

**Hiện vật xuyên suốt:** khu vườn của Byte (T2.1-T2.5) — giờ NHÌN như một
**sơ đồ tưới nước**: các luống LÀ đỉnh, MỘT đường ống nối hai luống LÀ một
cạnh. Lịch tưới LẶP theo chu kỳ (thứ Hai, Tư, Sáu, rồi QUAY lại thứ Hai)
LÀ số học modular. Và "phép GHÉP hai lịch tưới" (hợp thành, T2.4 bài 27)
LÀ một PHÉP TOÁN có thể xét dưới góc nhìn nhóm.

**Python trong track này không bao giờ là lời giải, CHỈ để KIỂM** —
`set` (T2.4) dựng đồ thị (tập cạnh) VÀ kiểm bậc/liên thông; `%` (R1, `core.
modulo`) đã biết CÁCH GÕ, giờ gắn lớp Ý NGHĨA đồng dư/GCD/nghịch đảo.

| # | slug | Tiêu đề | Khái niệm mới (đúng một) | `reflect` cuối bài | Dựa trên |
|---|---|---|---|---|---|
| 1 | `dinh-va-canh-la-mot-tap-hop` | Đỉnh và cạnh là một tập hợp | **`G = (V, E)`** — đồ thị VÔ HƯỚNG LÀ một tập đỉnh `V` (T2.4 bài 1) CÙNG một quan hệ ĐỐI XỨNG `E` trên `V` (T2.4 bài 16 quan hệ, bài 20 đối xứng) — MỖI "cạnh" LÀ một cặp `(a,b)` MÀ `(b,a)` CŨNG thuộc `E`, đúng nghĩa ống tưới nối HAI CHIỀU; KHÔNG khái niệm máy MỚI, CHỈ đặt TÊN "đồ thị" lên một quan hệ đối xứng đã biết dựng | `(luong_1, luong_2) ∈ E` — theo ĐỊNH nghĩa đối xứng (T2.4 bài 20), `(luong_2, luong_1)` CŨNG phải Ở TRONG `E`. Nếu Byte CHỈ ghi MỘT chiều (quên ghi chiều kia) — quan hệ ĐÓ còn LÀ đồ thị vô hướng HỢP LỆ không? | T2.4 `quan-he-la-mot-tap-con`, `doi-xung` |
| 2 | `bac-cua-mot-dinh` | Bậc của một đỉnh | **Bậc `deg(v)`** — số cạnh KỀ một đỉnh (số ống tưới NỐI tới một luống); đếm bằng `sum(1 for e in E if v in e)` — quy tắc đếm (T2.5 bài 1) áp lên đồ thị | Luống 1 nối với luống 2 VÀ luống 3 — `deg(luong_1)=2`. Cộng THẲNG bậc của MỌI luống lại — con số ĐÓ có liên quan gì tới SỐ cạnh (số ống tưới) không? | 1, T2.5 `quy-tac-cong` |
| 3 | `tong-bac-la-hai-lan-so-canh` | Tổng bậc là hai lần số cạnh | **Bổ đề bắt tay: `Σ deg(v) = 2\|E\|`** — MỖI cạnh có ĐÚNG hai đầu mút, nên nó ĐÓNG GÓP đúng `1` vào bậc của MỖI đầu — cộng dồn qua MỌI cạnh ra `2` lần số cạnh; định lý ĐẦU tiên của track (chứng minh bằng ĐẾM hai cách, T2.3 kỹ thuật CHỨNG MINH) | Tổng bậc CẢ vườn LUÔN LÀ số CHẴN (`2×` một số nguyên). Có đồ thị nào mà TỔNG bậc LẺ không — hay đây LÀ một quy luật KHÔNG THỂ vi phạm? | 2 |
| 4 | `duong-di-va-chu-trinh` | Đường đi và chu trình | **Đường đi** — dãy đỉnh LIÊN TIẾP nối bằng cạnh, KHÔNG lặp đỉnh; **chu trình** — đường đi mà đỉnh ĐẦU trùng đỉnh CUỐI; hai khái niệm HÌNH THỨC hoá "đi từ luống này sang luống khác theo ống tưới" | Từ luống 1, có đường đi TỚI luống 4 không (qua luống 2 hoặc 3)? Nếu CÓ, và từ luống 4 CŨNG có đường quay VỀ luống 1 — cả VƯỜN có "nối liền" với nhau không? | 1, T2.4 `cap-co-thu-tu` |
| 5 | `do-thi-lien-thong` | Đồ thị liên thông | **Liên thông** — CÓ đường đi giữa MỌI cặp đỉnh; một sơ đồ tưới liên thông nghĩa LÀ nước CÓ thể chảy (qua nhiều ống nối tiếp) TỚI bất kỳ luống nào TỪ bất kỳ luống nào khác | Byte có TÁM luống, NHƯNG hệ ống tưới chia thành HAI cụm TÁCH biệt (không ống nào nối hai cụm). Đồ thị NÀY có liên thông không — và nếu KHÔNG, "phần" nào của nó LÀ liên thông? | 4 |
| 6 | `thanh-phan-lien-thong` | Thành phần liên thông | **Thành phần liên thông** — một NHÓM đỉnh TỐI ĐA mà MỌI cặp trong nhóm CÓ đường đi nối nhau; một đồ thị KHÔNG liên thông chia thành NHIỀU thành phần, các thành phần RỜI NHAU (T2.4 bài 10) VÀ gộp lại vừa khít TOÀN đồ thị — đúng CẤU TRÚC phân hoạch (T2.4 bài 23) | Hai cụm ống tưới TÁCH biệt LÀ hai thành phần liên thông. Nếu Byte nối THÊM đúng MỘT ống GIỮA hai cụm — số thành phần liên thông đổi THẾ NÀO? | 5, T2.4 `lop-tuong-duong-va-phan-hoach` |
| 7 | `cay-la-do-thi-khong-chu-trinh` | Cây là đồ thị liên thông không chu trình | **Cây (lý thuyết đồ thị)** — đồ thị liên thông (bài 5) VÀ KHÔNG chứa chu trình nào (bài 4); một cây có `n` đỉnh LUÔN có ĐÚNG `n−1` cạnh (hệ quả TRỰC TIẾP bổ đề bắt tay, bài 3) — MỘT khái niệm HÌNH THỨC hoàn toàn mới, không liên quan cấu trúc dữ liệu "cây" (nhánh cha-con, PHÂN cấp) mà một track LẬP TRÌNH khác có thể dùng | Hệ ống tưới của Byte CÓ chu trình — luống 1→2→3→1, nước CÓ thể chảy VÒNG. Bớt đúng MỘT ống trong vòng đó — đồ thị còn liên thông không, và còn chu trình nào không? | 6, 3 |
| 8 | `bac-vao-bac-ra` | Bậc vào, bậc ra | **Đồ thị CÓ HƯỚNG** — cạnh LÀ cặp CÓ thứ tự `(a,b)` (T2.4 bài 14, ống MỘT chiều: nước chảy TỪ `a` TỚI `b`); **bậc VÀO** (số cạnh TRỎ vào) VÀ **bậc RA** (số cạnh TRỎ ra) — hai con số TÁCH biệt, khác bậc thường (bài 2, đồ thị VÔ hướng) | Trạm bơm CHỈ đẩy nước MỘT chiều tới từng luống — bậc RA của trạm bơm LÀ 4 (bốn luống), bậc VÀO LÀ 0. Cộng bậc VÀO của TẤT CẢ đỉnh, rồi cộng bậc RA của TẤT CẢ đỉnh — hai tổng ĐÓ có bằng nhau không, và có bằng SỐ cạnh không? | 2, T2.4 `cap-co-thu-tu` |
| 9 | `bai-toan-bay-cay-cau` | Bài toán bảy cây cầu | **Bài toán Königsberg** — đi qua ĐỦ bảy cây cầu, MỖI cầu ĐÚNG một lần, rồi VỀ điểm xuất phát — Euler (1736) chứng minh KHÔNG THỂ, khai sinh lý thuyết đồ thị; bài NÀY thuật lại CÂU CHUYỆN, chưa đưa CÔNG thức (bài 10 mới đưa) | Euler KHÔNG thử hết mọi đường đi (T2.3 bài 21: kiểm hữu hạn không phải chứng minh) — ông tìm ra một tính chất CHUNG khiến bài toán VÔ VỌNG. Tính chất ĐÓ liên quan gì tới BẬC của các đỉnh (bài 2)? | 2, T2.3 `kiem-nghin-lan-van-chua-du` |
| 10 | `duong-di-euler` | Đường đi Euler | **Đường đi Euler** — đi qua MỌI cạnh của đồ thị ĐÚNG một lần (khác đường đi bài 4, vốn không lặp ĐỈNH; đường Euler không lặp CẠNH, đỉnh có thể ghé lại); **chu trình Euler** — đường Euler mà điểm đầu trùng điểm cuối | Sơ đồ tưới của Byte — Byte muốn đi bộ dọc THEO MỌI ống tưới ĐÚNG một lần để kiểm tra rò rỉ. Có luôn LÀM được không, hay tuỳ sơ đồ? | 4, 9 |
| 11 | `dieu-kien-ton-tai-duong-di-euler` | Điều kiện tồn tại đường đi Euler | **Định lý Euler** — một đồ thị liên thông có chu trình Euler ⟺ MỌI đỉnh bậc CHẴN (bài 2); có đường đi Euler (không cần đóng vòng) ⟺ ĐÚNG 0 hoặc 2 đỉnh bậc LẺ — giải quyết TRỌN VẸN bài toán Königsberg (bốn đỉnh bậc lẻ, bài 9) | Sơ đồ Byte có ĐÚNG hai luống bậc lẻ (luống 1 và luống 4). Đường đi Euler CÓ tồn tại — nhưng PHẢI bắt đầu VÀ kết thúc Ở ĐÂU? | 10, 3 |
| 12 | `to-mau-do-thi` | Tô màu đồ thị | **Tô màu đồ thị** — gán MỘT màu cho MỖI đỉnh sao cho HAI đỉnh KỀ nhau (có cạnh nối) KHÔNG cùng màu; **số màu tô** `χ(G)` — số màu ÍT NHẤT cần dùng; tưới hai luống KỀ nhau CÙNG lúc thì ÁP lực nước yếu, cần xếp LỊCH khác giờ = tô màu KHÁC | Bốn luống, mọi cặp ĐỀU kề nhau (đồ thị ĐẦY ĐỦ). Cần Ít NHẤT bao nhiêu "giờ tưới" (màu) khác nhau để KHÔNG hai luống kề nào tưới CÙNG giờ? | 2, T2.4 `to-hop-chon-k` |
| 13 | `phep-chia-co-du` | Phép chia có dư | **`a = q·n + r`, `0 ≤ r < n`** — phép chia có dư (Ý NGHĨA toán của `%`, R1 `core.modulo` đã dạy CÁCH GÕ); MỌI số nguyên `a` VÀ MỘT số `n>0` xác định DUY NHẤT cặp `(q, r)` — nền cho ĐỒNG DƯ (bài 14) | Lịch tưới lặp mỗi BA ngày: `Hai, Tư, Bảy, Hai, Tư, Bảy,...`. Ngày thứ `10` (đếm từ `1`) LÀ ngày nào trong chu kỳ? Có cách nào tính KHÔNG cần đếm tay từng ngày? | R1 `phan-con-thua` |
| 14 | `dong-du-modulo` | Đồng dư modulo n | **`a ≡ b (mod n)`** — `a` VÀ `b` CÙNG số dư khi chia cho `n` (bài 13); ĐÚNG mối quan hệ TƯƠNG ĐƯƠNG (T2.4 bài 22: phản xạ, đối xứng, bắc cầu — kiểm lại BẰNG chính công thức T2.4 đã viết) trên tập số nguyên | Ngày thứ `10` VÀ ngày thứ `13` (chu kỳ ba ngày, bài 13) — có ĐỒNG DƯ modulo `3` không? Nếu có, chúng có LUÔN rơi vào CÙNG một NGÀY trong chu kỳ không? | 13, T2.4 `quan-he-tuong-duong` |
| 15 | `cong-nhan-modular` | Cộng, nhân modular | **`(a+b) mod n = ((a mod n)+(b mod n)) mod n`**, TƯƠNG TỰ cho nhân — CỘNG/NHÂN rồi LẤY dư CHO kết quả GIỐNG hệt lấy dư TRƯỚC rồi cộng/nhân; hữu ích khi SỐ quá LỚN (không cần tính số ĐẦY ĐỦ trước khi lấy dư) | Byte tính ngày thứ `1000` rơi vào NGÀY nào (chu kỳ ba ngày) — nhân TRỰC TIẾP một số LỚN có tiện bằng lấy dư TRƯỚC không? Kiểm lại bằng CẢ hai cách, có RA cùng kết quả không? | 14 |
| 16 | `uoc-chung-lon-nhat` | Ước chung lớn nhất | **`gcd(a,b)`** — số LỚN NHẤT chia HẾT cả `a` LẪN `b`; **thuật toán Euclid**: `gcd(a,b) = gcd(b, a mod n)`, LẶP tới khi số DƯ LÀ `0` — MỖI bước THU NHỎ bài toán (T2.3 kỹ thuật quy nạp/thước đo dừng) | `gcd(48, 18)` — LẶP bằng tay VÀI bước theo công thức. Số LẦN lặp NHỎ hơn hẳn số LẦN kiểm TỪNG ước số một — VÌ SAO thuật toán NÀY nhanh hơn cách "thử từng số"? | 13, T2.3 `dieu-dung-lai-sau-moi-luot` |
| 17 | `nghich-dao-modular` | Nghịch đảo modular | **`a⁻¹ mod n`** — số `x` sao cho `(a×x) mod n = 1`; TỒN TẠI ⟺ `gcd(a,n)=1` (`a` VÀ `n` NGUYÊN TỐ CÙNG NHAU, bài 16) — thay THẾ được PHÉP CHIA modular (phép chia THƯỜNG không có Ý nghĩa TRONG modular) | `3` có nghịch đảo modulo `7` không? Thử NHÂN `3` VỚI từng số TỪ `1` tới `6`, lấy dư CHO `7` — số NÀO cho kết quả `1`? | 16, 15 |
| 18 | `dinh-ly-fermat-nho` | Định lý Fermat nhỏ | **`aᵖ⁻¹ ≡ 1 (mod p)`** khi `p` NGUYÊN TỐ VÀ `a` KHÔNG chia hết cho `p` — một CÔNG THỨC RÚT GỌN cho luỹ thừa modular LỚN, KHÔNG cần nhân LẶP hàng nghìn lần (T2.5 bài 17, quy tắc nhân CHO một dãy) | `2¹⁰⁰ mod 101` — nhân LẶP một trăm lần THÌ chậm. Fermat nói `2¹⁰⁰ ≡ 1 (mod 101)` NGAY LẬP TỨC (VÌ `101` nguyên tố). Kiểm bằng CÁCH tính vài LUỸ THỪA nhỏ trước, có THẤY chu kỳ LẶP LẠI không? | 17, T2.5 `quy-tac-nhan-cho-day-phep-thu` |
| 19 | `ma-hoa-caesar` | Mã hoá Caesar | **Mã Caesar** — DỊCH mỗi chữ cái ĐI `k` VỊ TRÍ trong bảng chữ cái, LẶP VÒNG khi qua hết (đúng modular, bài 14): `(chu + k) mod 26`; GIẢI mã LÀ dịch NGƯỢC `−k` — ứng dụng THẬT đầu tiên của cả cụm modular | Mã hoá "A" dịch `3` vị trí RA "D". Mã hoá "Z" (chữ CUỐI bảng) dịch `3` vị trí RA chữ NÀO — có "chạy quá" bảng chữ cái không, hay MODULAR tự ĐƯA nó QUAY lại đầu? | 15, 14 |
| 20 | `dong-ho-modular-tong-quat` | Đồng hồ modular tổng quát | **Modular LÀ một "đồng hồ" `n` giờ** — CỘNG/nhân rồi "QUAY VÒNG" khi vượt `n`; đồng hồ 12 giờ, lịch bảy NGÀY, mã Caesar (bài 19) ĐỀU LÀ cùng MỘT cấu trúc với `n` KHÁC nhau — tổng kết cụm, KHÔNG khái niệm TÍNH toán mới, chỉ NHÌN LẠI qua MỘT lăng kính chung | Đồng hồ 12 giờ, lịch tưới ba NGÀY, mã Caesar 26 chữ — CẢ BA đều "cộng RỒI quay vòng". Phép CỘNG modular NÀY có tính chất NÀO GIỐNG phép cộng số nguyên THƯỜNG (kết hợp? có đơn vị? có nghịch đảo?), và tính chất NÀO thì KHÔNG? | 19, 15, 17 |
| 21 | `phep-toan-hai-ngoi` | Phép toán hai ngôi | **Phép toán hai ngôi `∗`** trên tập `S` — nhận HAI phần tử của `S`, TRẢ VỀ một phần tử CỦA `S` (TÍNH ĐÓNG — kết quả KHÔNG "thoát ra ngoài" `S`); `+` trên số nguyên ĐÓNG, NHƯNG `÷` trên số nguyên KHÔNG đóng (T2.1 CHIA không LUÔN ra số nguyên) | Modular cộng (bài 15) trên tập `{0,1,...,n−1}` — kết quả CÓ LUÔN nằm TRONG tập đó không (ĐÓNG), hay có thể "chạy RA ngoài"? | 20, T2.1 `so-khong-can-nhay` |
| 22 | `tinh-ket-hop` | Tính kết hợp | **`(a∗b)∗c = a∗(b∗c)`** — nhóm NGOẶC KIỂU nào cũng ra CÙNG kết quả; `+` VÀ `×` số nguyên ĐỀU kết hợp, NHƯNG `−` THÌ KHÔNG (`(5−3)−1 ≠ 5−(3−1)`) — tính chất THỨ HAI của một phép toán "tốt" | Hợp thành ánh xạ (T2.4 bài 27, `f∘g`) — ghép BA ánh xạ liên tiếp theo hai CÁCH nhóm ngoặc khác nhau, có RA cùng kết quả không? | 21, T2.4 `song-anh-va-hop-thanh` |
| 23 | `phan-tu-don-vi` | Phần tử đơn vị | **Phần tử đơn vị `e`** — `e∗a = a∗e = a` VỚI MỌI `a` (KHÔNG đổi gì khi ghép); `0` LÀ đơn vị của `+`, `1` LÀ đơn vị của `×` (T2.1 đã dùng, giờ đặt TÊN); modular cộng CÓ đơn vị LÀ `0` | Modular NHÂN (bài 15, trên `{0,...,n−1}`) — đơn vị của NÓ LÀ số NÀO? Kiểm LẠI: nhân số ĐÓ với BẤT KỲ phần tử nào, kết quả có GIỮ nguyên không? | 22, T2.1 `so-khong-can-nhay` |
| 24 | `phan-tu-nghich-dao` | Phần tử nghịch đảo | **Nghịch đảo của `a`** — phần tử `a⁻¹` sao cho `a∗a⁻¹ = a⁻¹∗a = e` (bài 23); MỌI số nguyên có nghịch đảo CỘNG (`−a`), NHƯNG KHÔNG PHẢI mọi số có nghịch đảo NHÂN (chỉ `1`, `−1`) — modular nghịch đảo (bài 17) LÀ trường hợp RIÊNG của khái niệm NÀY | Modular cộng CÓ nghịch đảo cho MỌI phần tử không (thử `a` bất kỳ trong `{0,...,n−1}`, tìm `x` sao `(a+x) mod n = 0`)? So với modular NHÂN (bài 17, CHỈ có nghịch đảo khi `gcd=1`) thì SAO? | 23, 17 |
| 25 | `nhom-la-gi` | Nhóm là gì | **Nhóm `(S, ∗)`** — MỘT tập `S` VỚI một phép toán `∗` thoả CẢ BỐN: đóng (bài 21), kết hợp (bài 22), có đơn vị (bài 23), MỌI phần tử có nghịch đảo (bài 24); GỘP bốn bài liền thành MỘT định nghĩa, đúng LỐI T2.4 gộp ba tính chất thành tương đương (bài 22, T2.4) | `(số nguyên, +)` LÀ một nhóm (đủ cả bốn). `(số nguyên, ×)` có LÀ nhóm không — thiếu ĐÚNG tiên đề NÀO? | 21, 22, 23, 24, T2.4 `quan-he-tuong-duong` |
| 26 | `vi-du-nhom-modular` | Ví dụ nhóm: modular cộng | **`(ℤₙ, +mod n)` LUÔN LÀ một nhóm**, VỚI MỌI `n` — kiểm ĐỦ bốn tiên đề (bài 21-24) TRÊN modular cộng CỤ THỂ, bằng CHÍNH những hàm ĐÃ viết (bài 13-20); "đồng hồ modular" (bài 20) BÂY GIỜ có TÊN toán học CHÍNH XÁC: MỘT nhóm | Lịch tưới BA ngày (bài 13) VÀ đồng hồ mười HAI giờ ĐỀU LÀ nhóm modular, CHỈ khác `n`. Vườn của Byte (đồ thị, bài 1-12) có PHÉP toán nào ĐÓNG-kết hợp-đơn vị-nghịch đảo GIỐNG vậy KHÔNG, hay đồ thị KHÔNG có cấu trúc nhóm TỰ NHIÊN? | 25, 20 |
| 27 | `ham-nhu-mot-gia-tri` | Hàm như một giá trị | **Hàm LÀ một GIÁ TRỊ** — có thể GÁN cho biến, TRUYỀN làm đối số, TRẢ VỀ từ hàm KHÁC (đã dùng nhiều Ở R4 FP, `def`/lambda Python — bài NÀY đặt TÊN toán học: LAMBDA CALCULUS, nền TOÁN của lập trình HÀM); hợp thành ánh xạ (T2.4 bài 27) LÀ MỘT phép toán HAI ngôi TRÊN tập các HÀM | Hợp thành ánh xạ (`f∘g`, T2.4 bài 27) — tập TẤT CẢ song ánh TỪ một tập HỮU hạn VÀO chính nó, VỚI phép hợp thành — có phải MỘT nhóm không (đóng, kết hợp, đơn vị, nghịch đảo)? Đơn vị LÀ ánh xạ NÀO? | 26, 22, T2.4 `song-anh-va-hop-thanh` |
| 28 | `boss-tam-ban-do-vuon` | BOSS — Tấm bản đồ vườn | *(không khái niệm mới — bài tổng hợp)* ghép đồ thị (bậc, liên thông, bài 1-12) + modular (đồng dư, GCD, nghịch đảo, bài 13-20) + đại số trừu tượng (kiểm bốn tiên đề nhóm, bài 21-27) TRÊN MỘT sơ đồ tưới nước hoàn chỉnh của khu vườn | Track NÀY đóng bằng việc NHÌN lại đồ thị VÀ modular QUA lăng kính nhóm. NHƯNG "mấy CÂY", "mấy KILÔGAM" (T2.5) LÀ số RỜI RẠC — máy tính CÒN cần XỬ LÝ số LIÊN TỤC (đo đạc, dự đoán TRƠN) để LÀM được AI thật. Số liên tục VÀ phép TÍNH trên nó viết RA thành CÁI GÌ? *(dẫn sang T2.7 — ĐSTT & giải tích cho AI)* | 1–27 |

**Vì sao thứ tự này đúng**

**1. Vì sao track KHÔNG mở bằng "đồ thị là gì" (R3 ĐÃ dạy), mà mở bằng
"đồ thị LÀ một cặp tập hợp".** Đúng luật lặp lại từ T2.4/T2.5: bài 1 KHÔNG
dạy khái niệm hoàn toàn mới — nó dạy phép ĐỌC LẠI một cấu trúc R3 ĐÃ DỰNG
bằng CODE (bảng kề) dưới NGÔN NGỮ tập hợp (T2.4), mở đường để MỌI kỹ
thuật T2.4 (tập con, quan hệ, ánh xạ) áp DỤNG thẳng lên đồ thị.

**2. Vì sao bổ đề bắt tay (bài 3) — định lý ĐẦU tiên của track — đứng
NGAY sau khái niệm bậc (bài 2), TRƯỚC khi có đường đi/liên thông (bài
4-6).** Đây LÀ định lý ĐƠN GIẢN nhất có thể chứng minh BẰNG kỹ thuật ĐÃ
học (đếm hai cách, T2.5), dựng NIỀM TIN "đồ thị có ĐỊNH LÝ thật" sớm,
trước khi track đi vào những khái niệm PHỨC tạp hơn.

**3. Vì sao bài toán Königsberg (bài 9, kể CHUYỆN, KHÔNG công thức) tách
RIÊNG khỏi định lý Euler (bài 11, CÓ công thức).** Cùng LỐI T2.3 (bài 1
mở bằng câu hỏi, chưa công cụ): kể chuyện TRƯỚC để người học TỰ THẤY sự
BẤT LỰC (thử mãi không ra), rồi CÔNG THỨC (bài 11) xuất hiện đúng LÚC nó
GIẢI QUYẾT sự bất lực ĐÓ — không phải một định lý "từ TRÊN trời rơi
xuống".

**4. Vì sao modular (bài 13) mở bằng "phép chia có dư" — MỘT khái niệm R1
ĐÃ DẠY (`core.modulo`) — thay vì mở thẳng bằng "đồng dư".** Cùng bản lề
T2.4/T2.5: bài 13 KHÔNG dạy `%` (đã biết CÁCH gõ), nó dạy Ý NGHĨA `(q,r)`
làm NỀN cho đồng dư (bài 14) đứng NGAY sau.

**5. Vì sao GCD/Euclid (bài 16) đứng TRƯỚC nghịch đảo modular (bài 17),
dù nghe như HAI chủ đề tách BIỆT.** Nghịch đảo modular CHỈ tồn tại khi
`gcd(a,n)=1` — bài 16 PHẢI đứng trước để bài 17 có ĐIỀU KIỆN tồn tại RÕ
ràng, không phải một QUY TẮC "từ trên trời".

**6. Vì sao BỐN tiên đề nhóm (đóng, kết hợp, đơn vị, nghịch đảo — bài
21-24) TÁCH bốn bài RIÊNG trước khi gộp (bài 25), thay vì định nghĩa nhóm
NGAY từ đầu.** CÙNG lý do T2.4 tách BA tính chất tương đương (mục 7, T2.4)
— mỗi tiên đề cần MỘT phản ví dụ RIÊNG (`−` không kết hợp, `×` không CÓ
nghịch đảo cho MỌI số) để THẤY nó THẬT là một ràng buộc CÓ THỂ vi phạm.

**7. Vì sao "hàm như một giá trị" (bài 27, lambda calculus) đứng CUỐI,
NGAY trước BOSS, KHÔNG mở đầu track (khác MASTERPLAN cũ, ĐÃ sửa theo §9.1:
"HẠ KHỎI vị trí mở đầu → tái định vị làm ĐỈNH T2.6").** Lambda calculus
CHỈ có Ý NGHĨA sau khi người học ĐÃ thấy nhóm (bài 25) — hợp thành ánh xạ
(T2.4 bài 27) LÀ một phép toán hai NGÔI trên tập HÀM, và CÂU HỎI "tập
song ánh với hợp thành có LÀ nhóm không" chỉ ĐẶT RA được SAU khi đã có cả
hai khái niệm (nhóm VÀ hợp thành) sẵn sàng.

**Ranh giới track:** không bài nào dùng bảng kề kiểu `dict`-of-list hay
duyệt DFS/BFS bằng CODE thuật toán (đó LÀ việc của R3, một track LẬP
TRÌNH — track NÀY chỉ dùng `set` các cặp, đúng máy móc T2.4 sẵn có);
không bài nào đi SÂU thuật toán đồ thị (đường đi ngắn nhất, cây khung
nhỏ nhất — ngoài phạm vi TOÁN của track này); không bài nào chứng minh
CHẶT chẽ định lý Euler (chỉ PHÁT BIỂU VÀ áp dụng, chứng minh ĐẦY ĐỦ CẦN
quy nạp SÂU hơn phạm vi track); không bài nào ĐI SÂU đại số trừu tượng
NGOÀI nhóm (vành, trường — dành CHO một track SAU nếu MASTERPLAN mở
rộng); VÀ — biên quan trọng NHẤT — **track này KHÔNG `requires:` bất kỳ
skill nào của R3 "Khoa học máy tính" (xem "QUAN TRỌNG" đầu mục T2.6):
R3 đứng SAU `toan` trong `thu-tu.yaml`, nên mọi máy móc đồ thị Ở ĐÂY
PHẢI tự đứng vững trên riêng T2.4, không mượn R3.**
## T2.7 — ĐSTT & giải tích cho AI (Realm 2 · Toán & Toán rời rạc · Python CHỈ để KIỂM · 28 bài)

## Mạch T2.7 — ĐSTT & giải tích cho AI

Người học vào track đã xong T2.1-T2.6 (số, biến, mệnh đề, tập hợp/
quan hệ/ánh xạ, tổ hợp/xác suất/thống kê, đồ thị/modular/đại số trừu
tượng). Đây LÀ track CUỐI của R2, đóng TRỌN roadmap Toán.

**Hiện vật xuyên suốt:** khu vườn của Byte, GIỜ đo bằng số LIÊN TỤC —
chiều dài luống (mét), lượng nước (lít), giờ nắng (giờ) — thay VÌ chỉ
đếm "mấy cây" (T2.5, số rời rạc). BOSS T2.6 ĐÃ hỏi thẳng: "làm sao đo
một luống dài BAO NHIÊU MÉT, TRƠN VÀ liên tục?" — câu hỏi ĐÓ MỞ track
này.

**Vì sao ĐSTT (đại số tuyến tính) VÀ giải tích đứng CHUNG một track.**
Cả hai LÀ ngôn ngữ TOÁN học của học máy: **vector** biểu diễn một luống
bằng NHIỀU con số CÙNG lúc (dài, nước, nắng); **ma trận** biểu diễn CẢ
vườn cùng lúc; **đạo hàm** đo TỐC ĐỘ thay đổi (cây LỚN nhanh cỡ nào);
**gradient** LÀ "đạo hàm nhiều chiều", VÀ **gradient descent** — thuật
toán ĐỨNG sau HẦU hết việc "học" của AI hiện đại — LÀ đích đến CUỐI
của cả track: dùng đạo hàm để TÌM khẩu phần tưới TỐI ƯU.

**Python trong track này không bao giờ là lời giải, CHỈ để KIỂM** —
`list`/`tuple` (T1.4, R1) dựng vector VÀ ma trận; các PHÉP toán (`+`,
`*`, `**`, `sum`) ĐÃ biết CÁCH gõ, GIỜ gắn lớp Ý NGHĨA hình học/giải
tích. KHÔNG dùng thư viện số học (`numpy` KHÔNG xuất hiện) — MỌI vector/
ma trận LÀ `list` số THUẦN, mọi phép toán VIẾT tay bằng comprehension,
đúng LỐI "thấy được cỗ máy BÊN trong" xuyên suốt R2.

**Đạo hàm dạy KIỂU nào.** Track KHÔNG chứng minh giới hạn (ε-δ) — quá
sâu SO với phạm vi R2. Đạo hàm được giới thiệu QUA xấp xỉ SỐ (Δy/Δx
với Δx CỰC nhỏ, tính bằng CODE) VÀ quy tắc đại số (lũy thừa, tổng,
dây chuyền) cho các HÀM đa thức đơn giản — ĐỦ để hiểu gradient descent,
KHÔNG đủ để làm giải tích HÌNH thức. Đây LÀ ranh giới rõ ràng, đúng
tinh THẦN "giải tích cho AI", không phải "giải tích đại học".

| # | slug | Tiêu đề | Khái niệm mới (đúng một) | `reflect` cuối bài | Dựa trên |
|---|---|---|---|---|---|
| 1 | `vector-la-gi` | Vector là gì | **Vector** — một `tuple` số CÓ THỨ TỰ, MỖI luống giờ LÀ một điểm nhiều chiều: `(dai, nuoc, nang)`; tổng quát hoá cặp có thứ tự (T2.4 bài 14) lên NHIỀU hơn hai thành phần | Luống 1: `(2.0, 5.0, 6.0)` (dài 2m, tưới 5L, nắng 6h). Luống 2: `(3.0, 4.0, 5.0)`. Gộp lượng nước CỦA cả hai luống LẠI — cộng TỪNG cặp con số tương ứng, được không? | T2.4 `cap-co-thu-tu` |
| 2 | `cong-vector` | Cộng vector | **`u + v`** — cộng TỪNG thành phần TƯƠNG ỨNG: `(a1,a2)+(b1,b2)=(a1+b1,a2+b2)`; gộp hai đợt tưới LÀ cộng vector | Nhân MỖI thành phần của vector lên GẤP ĐÔI (tăng khẩu phần tưới gấp đôi) — có phải phép TOÁN khác cộng vector không? | 1 |
| 3 | `nhan-vo-huong-vector` | Nhân vô hướng | **`k·v`** — nhân MỖI thành phần VỚI cùng một số `k`; tăng khẩu phần GẤP `k` lần LÀ nhân vô hướng, KHÔNG phải cộng | So sánh vector `(4.0, 6.0)` VÀ vector `(2.0, 3.0)` — CÓ "cùng HƯỚNG" không (một LÀ bản phóng to của cái kia)? Đo "độ LỚN" một vector — dùng con số nào? | 2 |
| 4 | `do-dai-vector` | Độ dài vector | **`‖v‖ = √(v1²+v2²+...)`** — mở RỘNG định lý Pythagoras (T2.1) lên nhiều chiều; độ dài LÀ "khoảng cách TỪ gốc" | Hai luống Ở vị trí `(3.0, 4.0)` VÀ `(0.0, 0.0)` (gốc) — khoảng CÁCH giữa CHÚNG chính LÀ độ dài vector NÀO? | 3, T2.1 (Pythagoras) |
| 5 | `khoang-cach-hai-vector` | Khoảng cách giữa hai vector | **`d(u,v) = ‖u−v‖`** — khoảng cách Euclid LÀ độ dài của vector HIỆU; "hai luống GIỐNG nhau bao nhiêu" đo bằng khoảng CÁCH hồ sơ của chúng | Hai luống hồ sơ GẦN NHAU (khoảng cách NHỎ) — điều ĐÓ nói lên gì VỀ chúng? Có cách nào đo "giống nhau" mà KHÔNG quan tâm ĐỘ LỚN tuyệt đối, chỉ quan tâm HƯỚNG? | 4 |
| 6 | `tich-vo-huong` | Tích vô hướng | **`u·v = u1v1+u2v2+...`** — tổng CÁC tích từng cặp thành phần TƯƠNG ứng; MỘT con số DUY NHẤT tóm tắt "hai vector khớp NHAU bao nhiêu" | `u·v` VÀ `v·u` — có bằng NHAU không (giống hệt kiểm tra Ở T2.6, phép TOÁN nào giao hoán)? Con số `u·v` LỚN nghĩa LÀ gì VỀ mặt HÌNH học? | 5 |
| 7 | `goc-giua-hai-vector` | Góc giữa hai vector | **`cos θ = (u·v)/(‖u‖‖v‖)`** — tích vô hướng CHIA cho tích hai độ dài RA cosin của GÓC giữa chúng; `cos θ = 0` nghĩa LÀ VUÔNG góc (KHÔNG liên quan gì nhau) | Hai luống hồ sơ VUÔNG góc (`cos θ = 0`) — chúng "khác nhau HOÀN TOÀN" theo nghĩa NÀO? Có cách NÀO dùng `cos θ` làm thước đo "GIỐNG nhau" chuẩn HOÁ, không phụ thuộc ĐỘ LỚN? | 6, 4 |
| 8 | `do-tuong-dong-cosine` | Độ tương đồng cosine | **Độ tương đồng cosine** — CHÍNH LÀ `cos θ` (bài 7), đặt TÊN ứng dụng: so sánh "hồ SƠ" hai luống MÀ không quan tâm ĐỘ LỚN, chỉ quan tâm TỈ LỆ các thành phần — công cụ CHUẨN so sánh embedding trong AI hiện đại | Một VƯỜN CÓ nhiều luống — LÀM sao VIẾT gọn TOÀN bộ dữ liệu (nhiều vector) thành MỘT cấu trúc DUY nhất? | 7 |
| 9 | `ma-tran-la-gi` | Ma trận là gì | **Ma trận** — một BẢNG số, `list` CÁC vector hàng; TOÀN bộ vườn (MỖI luống MỘT hàng, MỖI cột MỘT chỉ số đo) LÀ một ma trận DUY nhất | Cộng HAI ma trận (hai lần ĐO của cùng khu vườn) — cộng NHƯ thế nào, đúng lối cộng vector (bài 2) không? | 8, 1 |
| 10 | `cong-ma-tran` | Cộng ma trận | **`A+B`** — cộng TỪNG phần tử Ở CÙNG vị trí hàng-cột; đúng lối cộng vector (bài 2), CHỈ thêm một CHIỀU | Nhân CẢ ma trận VỚI một số `k` (tăng đồng LOẠT mọi chỉ số đo LÊN `k` lần) — phép toán NÀY LÀ gì? | 9, 2 |
| 11 | `nhan-vo-huong-ma-tran` | Nhân vô hướng ma trận | **`k·A`** — nhân MỖI phần tử VỚI `k`; đúng lối nhân vô hướng vector (bài 3) | Ma trận NHÂN VỚI một VECTOR (không phải MỘT số) — kết quả LÀ GÌ, VÀ tính THẾ nào? | 10, 3 |
| 12 | `nhan-ma-tran-vector` | Nhân ma trận với vector | **`Av`** — MỖI thành phần của kết quả LÀ tích vô hướng (bài 6) của MỘT hàng ma trận VỚI `v`; đây LÀ **biến đổi TUYẾN TÍNH** — cách AI "biến" một vector đầu VÀO thành một vector ĐẦU ra | Nhân MA TRẬN vườn (nhiều luống) VỚI vector "trọng SỐ" `(1,1,1)` (cộng dồn cả BA chỉ số ĐO) — kết quả nói LÊN điều gì VỀ từng luống? | 11, 6 |
| 13 | `nhan-hai-ma-tran` | Nhân hai ma trận | **`AB`** — MỖI phần tử của kết quả LÀ tích vô hướng của MỘT hàng `A` VỚI MỘT cột `B`; nhân ma trận-vector (bài 12) LÀ trường hợp RIÊNG khi `B` chỉ CÓ một cột | `AB` VÀ `BA` — CÓ luôn bằng nhau không (giống câu hỏi Ở T2.6 VỀ hợp thành ánh xạ, bài 27)? | 12 |
| 14 | `ma-tran-don-vi` | Ma trận đơn vị | **Ma trận đơn vị `I`** — đường CHÉO toàn `1`, còn lại `0`; `AI = IA = A` VỚI MỌI `A` — đúng LỐI phần tử đơn vị (T2.6 bài 23), GIỜ trên ma trận | Ma trận `A` CÓ "nghịch đảo" `A⁻¹` sao `AA⁻¹=I` không — LUÔN có, hay CHỈ vài ma trận MỚI có (đúng LỐI T2.6 bài 24)? | 13, T2.6 `phan-tu-don-vi` |
| 15 | `ma-tran-chuyen-vi` | Ma trận chuyển vị | **`Aᵀ`** — đổi HÀNG thành cột (`Aᵀ[i][j] = A[j][i]`); "xoay" bảng số MÀ không đổi giá trị — hữu ích khi cần đọc dữ liệu THEO chiều KHÁC | Vườn CỦA Byte đo được BAO NHIÊU (dài, nước, nắng) — nhưng CÁC con số ĐÓ đo theo THỜI GIAN thì SAO? Luống LỚN lên bao NHANH — đo TỐC ĐỘ thay đổi thế NÀO? | 9 |
| 16 | `toc-do-thay-doi` | Tốc độ thay đổi | **Tốc độ thay đổi trung bình** — `Δy/Δx`, độ DỐC giữa hai thời điểm ĐO; luống dài `2m` NGÀY 1, `2.6m` ngày 4 — tốc độ LỚN trung bình LÀ `0.2m/ngày` | Đo tốc độ lớn GIỮA ngày 1 VÀ ngày 1.001 (khoảng CÁCH cực NHỎ) — con số CÓ ổn định LẠI một giá trị, hay cứ đổi MÃI khi khoảng cách càng NHỎ? | T2.1 (tỉ số) |
| 17 | `gioi-han-truc-quan` | Giới hạn trực quan | **Giới hạn** — khi `Δx` CÀNG nhỏ, `Δy/Δx` CÀNG tiến GẦN một con số CỐ ĐỊNH (kiểm bằng CODE: thử `Δx=0.1, 0.01, 0.001`, thấy dãy SỐ hội tụ); KHÔNG chứng minh ε-δ, chỉ QUAN sát số | Con số "hội tụ" ĐÓ — CHÍNH LÀ tốc độ lớn TẠI đúng MỘT thời điểm (KHÔNG phải trung bình GIỮA hai mốc nữa). Nó CÓ tên riêng không? | 16 |
| 18 | `dao-ham-la-gi` | Đạo hàm là gì | **Đạo hàm `f'(x)`** — độ dốc TỨC THỜI tại `x` (giới hạn của `Δy/Δx` khi `Δx→0`, bài 17); đo TỐC ĐỘ thay đổi CHÍNH XÁC tại một điểm, KHÔNG phải trung bình | `f(x)=x²` — tính `f'(x)` TẠI vài điểm bằng xấp xỉ SỐ (`Δx` nhỏ). Kết quả CÓ khớp công thức `2x` không? Có QUY tắc TỔNG quát nào cho `xⁿ` không? | 17 |
| 19 | `dao-ham-ham-da-thuc` | Đạo hàm hàm đa thức | **Quy tắc luỹ thừa: `d/dx[xⁿ] = n·xⁿ⁻¹`** — công thức ĐẠI SỐ thay cho tính xấp xỉ SỐ (bài 18); kiểm LẠI bằng xấp xỉ SỐ để THẤY công thức khớp | Hàm `f(x) = x² + x³` (TỔNG hai luỹ thừa) — đạo hàm của TỔNG có phải TỔNG của hai đạo hàm không? | 18 |
| 20 | `dao-ham-tong-hieu` | Đạo hàm tổng, hiệu | **`(f+g)' = f'+g'`, `(f−g)' = f'−g'`** — đạo hàm PHÂN PHỐI qua tổng/hiệu; TÍNH đạo hàm một hàm PHỨC bằng cách TÁCH thành TỪNG số hạng | Hàm HỢP `f(g(x))` (một hàm LỒNG trong hàm khác, T2.4 khái niệm hợp THÀNH) — đạo hàm của NÓ có ĐƠN giản là `f'(g'(x))` không, hay cần THÊM gì? | 19, T2.4 `song-anh-va-hop-thanh` |
| 21 | `quy-tac-day-chuyen` | Quy tắc dây chuyền | **`(f∘g)'(x) = f'(g(x))·g'(x)`** — đạo hàm hàm HỢP LÀ tích của "đạo hàm NGOÀI tại điểm trong" VÀ "đạo hàm TRONG"; kiểm bằng xấp xỉ số CHO một ca cụ thể | Hàm `f(x) = (x²)²` CÓ đúng MỘT điểm mà đạo hàm BẰNG `0` — điểm ĐÓ có Ý nghĩa GÌ (lớn nhất? nhỏ nhất? không đổi?) | 20 |
| 22 | `cuc-tri-dao-ham-bang-0` | Cực trị: đạo hàm bằng 0 | **Tại cực ĐẠI/cực tiểu, `f'(x)=0`** — độ dốc BẰNG không TẠI đỉnh/đáy (KHÔNG tăng cũng KHÔNG giảm); tìm GIÁ trị `x` tối ưu bằng cách GIẢI `f'(x)=0` | Hàm CHI PHÍ tưới nước CÓ NHIỀU biến (không chỉ MỘT `x`) — "độ dốc" theo TỪNG biến RIÊNG tính thế NÀO? | 21 |
| 23 | `dao-ham-rieng` | Đạo hàm riêng | **Đạo hàm RIÊNG `∂f/∂x`** — đạo hàm THEO một biến, GIỮ các biến KHÁC cố định; hàm `f(x,y)=x²+y²` CÓ hai đạo hàm riêng, MỘT cho `x`, MỘT cho `y` | Gộp CẢ hai đạo hàm riêng (`∂f/∂x`, `∂f/∂y`) lại thành MỘT cấu trúc DUY nhất — cấu trúc ĐÓ giống thứ GÌ đã học Ở đầu track? | 22, 1 |
| 24 | `vector-gradient` | Vector gradient | **`∇f = (∂f/∂x, ∂f/∂y)`** — VECTOR (bài 1) gồm TẤT CẢ đạo hàm riêng; `∇f` chỉ HƯỚNG mà `f` TĂNG nhanh NHẤT tại một điểm | Muốn hàm CHI PHÍ (bài 22) GIẢM nhanh NHẤT — nên đi THEO hướng gradient, hay hướng NGƯỢC LẠI? | 23 |
| 25 | `gradient-nguoc-huong-giam` | Gradient ngược hướng giảm | **`−∇f`** chỉ hướng `f` GIẢM nhanh NHẤT — đi ngược GRADIENT LÀ cách nhanh nhất để TỚI cực tiểu; nền tảng của thuật TOÁN học máy quan trọng nhất | Đi một BƯỚC ngược gradient — bước ĐÓ nên DÀI bao nhiêu? Đi CẢ quãng MỘT lần có ổn không, hay CẦN đi từng bước NHỎ? | 24 |
| 26 | `mot-buoc-gradient-descent` | Một bước gradient descent | **`x_mới = x − α·∇f(x)`** — `α` (tốc độ HỌC) LÀ một bước NHỎ theo hướng NGƯỢC gradient (bài 25); MỘT bước ĐƯA `x` TỚI gần cực tiểu hơn, chưa TỚI hẳn | LẶP LẠI một bước NHIỀU lần (bài 26) — kết quả CÓ hội tụ VỀ đúng điểm CỰC tiểu không? Thử `α` quá LỚN thì SAO? | 25, 16 |
| 27 | `gradient-descent-lap-lai` | Gradient descent lặp lại | **Lặp LẠI `x ← x − α∇f(x)`** cho tới khi `∇f(x)` GẦN `0` (bài 22) — CHÍNH LÀ cách phần LỚN mô hình AI "học": LẶP hàng NGHÌN bước NHỎ để GIẢM một hàm chi phí | Track NÀY đóng bằng gradient descent — thuật toán CỐT LÕI của AI hiện đại. VECTOR, tích vô hướng, đạo hàm, gradient — TẤT CẢ GẶP nhau Ở đâu trên MỘT bài toán DUY nhất? | 26, T2.3 `dieu-dung-lai-sau-moi-luot` |
| 28 | `boss-toi-uu-khau-phan-tuoi` | BOSS — Tối ưu khẩu phần tưới | *(không khái niệm mới — bài tổng hợp)* ghép vector (hồ sơ luống, bài 1-8) + ma trận (cả vườn, bài 9-15) + đạo hàm/gradient (bài 16-27): dùng gradient descent TÌM khẩu phần tưới TỐI ƯU giảm một hàm chi PHÍ đơn giản, VÀ dùng tích vô hướng SO sánh độ tương đồng hai luống | Track NÀY khép LẠI toàn bộ ROADMAP Toán (R2). Số rời rạc (đếm), logic (chứng minh), tập hợp (nhóm lại), tổ hợp/xác suất (đo bất ĐỊNH), đồ thị/modular/nhóm (cấu TRÚC lặp), VÀ giờ VECTOR/đạo hàm (đo LIÊN tục VÀ tối ưu) — sáu track, MỘT bộ CÔNG cụ. Byte sẵn SÀNG bước sang lập trình HÀM (R4) — CODE thật, KHÔNG chỉ Ý nghĩa toán. | 1-27 |

**Vì sao thứ tự này đúng**

**1. Vì sao vector (bài 1) mở bằng "tuple số có thứ tự", NGAY sau khi
nhắc lại cặp có thứ tự (T2.4).** Cùng bản lề T2.4/T2.5/T2.6: KHÔNG dạy
khái niệm hoàn toàn MỚI ngay bài 1 — vector LÀ sự TỔNG QUÁT hoá tự
nhiên của cặp có thứ tự (hai thành phần) lên `n` thành PHẦN, người học
ĐÃ có sẵn trực giác.

**2. Vì sao cộng/nhân vô hướng vector (bài 2-3) đứng TRƯỚC độ dài/tích
vô hướng (bài 4-6).** Phép toán ĐƠN giản (cộng từng thành PHẦN) trước,
phép toán CẦN diễn giải HÌNH học (độ dài, góc) sau — đúng nhịp "thao
tác trước, Ý nghĩa sau" đã DÙNG suốt R2.

**3. Vì sao ma trận (bài 9-15) đứng SAU trọn cụm vector, KHÔNG xen kẽ.**
Ma trận LÀ "nhiều vector gộp lại" — cần vector VỮNG trước khi GHÉP
thành bảng; VÀ nhân ma trận-vector (bài 12) tái DÙNG trực tiếp tích vô
hướng (bài 6), một phép TOÁN đã THÀNH thạo.

**4. Vì sao giải tích (bài 16-22) mở bằng "tốc độ thay đổi TRUNG BÌNH"
(bài 16) — một tỉ số ĐÃ quen (T2.1) — thay VÌ mở bằng "đạo hàm".** Đạo
hàm (bài 18) LÀ TRƯỜNG hợp GIỚI HẠN của tỉ số Δy/Δx khi Δx→0 (bài 17)
— dựng TỪ cái ĐÃ biết, không rơi từ TRÊN trời.

**5. Vì sao đạo hàm được KIỂM bằng xấp xỉ SỐ (CODE, thử Δx nhỏ dần)
TRƯỚC khi đưa công thức đại SỐ (bài 19, quy tắc luỹ THỪA).** Đúng
tinh THẦN "đo trước khi viết" xuyên suốt project — công thức `n·xⁿ⁻¹`
KHÔNG phải một QUY tắc phải NHỚ, mà LÀ điều CODE tự XÁC nhận.

**6. Vì sao gradient (bài 23-24) đứng NGAY sau cực trị (bài 22, `f'(x)=
0`), TRƯỚC gradient descent (bài 25-27).** Gradient LÀ "đạo hàm nhiều
CHIỀU" — người học cần THẤY rõ đạo hàm MỘT biến (VÀ Ý nghĩa `f'=0` LÀ
cực trị) TRƯỚC khi tổng QUÁT hoá lên nhiều biến; gradient descent (bài
25-27) MỚI LÀ đích ĐẾN, ba bài CUỐI dựng dần: hướng GIẢM → một BƯỚC →
lặp LẠI.

**7. Vì sao track KHÔNG dạy ma trận nghịch đảo, định thức, hay trị
riêng/vector riêng (eigenvalue/eigenvector).** NGOÀI phạm vi "giải
tích CHO AI" — gradient descent (đích ĐẾN track) KHÔNG cần chúng; bài
14 CHỈ nêu câu HỎI (nghịch đảo ma trận có LUÔN tồn tại không) làm
MÓC nối khái niệm, KHÔNG giải quyết.

**Ranh giới track:** không bài nào chứng MINH giới hạn bằng ε-δ (CHỈ
quan sát SỐ, bài 17); không bài nào tính đạo hàm hàm LƯỢNG giác, mũ,
LOG (chỉ đa thức, đủ CHO gradient descent CƠ bản); không bài nào DÙNG
`numpy` hay thư VIỆN đại số tuyến tính (mọi vector/ma TRẬN LÀ `list`
THUẦN, phép toán viết TAY); không bài nào GIẢI hệ phương trình tuyến
tính, tính ĐỊNH thức, ma trận nghịch đảo, TRỊ riêng/vector riêng (NGOÀI
phạm vi, dành CHO một track sâu HƠN nếu MASTERPLAN mở RỘNG); VÀ — biên
quan trọng NHẤT — track chỉ dừng Ở gradient descent CHO hàm MỘT/hai
biến ĐƠN giản, KHÔNG đụng tới backpropagation hay MẠNG nơ-ron (đó LÀ
việc của R8 AI/GenAI, xây TRÊN nền track này).
