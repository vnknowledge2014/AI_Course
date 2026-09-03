---
id: co-so-du-lieu.mot-hop-nhieu-hinh.gioi-han-mini-engine
title: "Giới hạn của mini-engine"
summary: "Tổng kết CÓ hệ thống mọi thứ SurrealDB thật làm được mà mini-engine (q07-q09) KHÔNG hề chạm tới: mutation (CREATE/UPDATE/DELETE), transaction (atomicity thật), aggregation (GROUP BY/count), chỉ mục tự bảo trì (DEFINE INDEX), kiểu dữ liệu phong phú (record link/mảng/object), và multi-model (document + graph + relational trong MỘT engine). Vậy TẠI SAO còn xây mini-engine LÀM gì — câu trả lời: hiểu CƠ CHẾ bên dưới trước khi dùng công cụ thật, không phải để THAY THẾ nó."
locale: vi
track: co-so-du-lieu
module: mot-hop-nhieu-hinh
order: 11
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [db.gioi-han-mini-engine]
requires: [db.aggregation-group-by]
concepts: [db.gioi-han-mini-engine]
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
Chín bài SO sánh — mỗi bài ĐỀU tìm ra một điều SurrealDB thật LÀM
được mà mini-engine KHÔNG. Gộp TẤT cả lại, danh sách trông ra SAO?
::::

::::explain{#danh-sach-gioi-han}
Sáu khoảng cách LỚN, mỗi CÁI đã thấy Ở một bài RIÊNG:

1. **Mutation** (bài 4) — mini-engine (q07-q09) CHỈ đọc. SurrealDB
   thật CÓ `CREATE`/`UPDATE`/`DELETE`.
2. **Đồ thị nguyên sinh** (bài 5-6) — mini-engine tự XÂY `Canh{tu,
   den}` cộng hai chỉ mục TAY. SurrealDB thật CÓ `RELATE`/`->` sẵn.
3. **Chỉ mục tự bảo trì** (bài 7) — mini-engine PHẢI gọi lại `xay_
   chi_muc` thủ CÔNG mỗi khi dữ liệu đổi. `DEFINE INDEX` tự ĐỘNG
   đồng bộ.
4. **Transaction thật** (bài 8) — mini-engine KHÔNG có khái niệm
   "nhiều câu ghi CÙNG thành công hay cùng KHÔNG". `BEGIN`/`COMMIT`/
   `CANCEL` LÀ atomicity thật.
5. **Kiểu dữ liệu phong phú** (bài 3, 9) — mini-engine CHỈ có `i64`.
   SurrealDB thật CÓ document schemaless, record link, mảng, object
   lồng.
6. **Aggregation** (bài 10) — executor q08 CHỈ có Quét/Lọc/ChọnCột.
   `GROUP BY`/`count()` LÀ một lớp thao TÁC hoàn toàn khác.

VÀ một điểm CHUNG xuyên suốt CẢ sáu: SurrealDB LÀ "một hộp, NHIỀU
hình" — MỘT engine DUY nhất làm ĐỦ document, relational, graph,
key-value. Mini-engine chỉ chạm ĐÚNG một lát MỎNG của relational.
::::

::::example{#dieu-mini-engine-lam-dung}
NHƯNG chín bài SO sánh CŨNG cho THẤY: trên đúng phần VIỆC mini-engine
LÀM (SELECT...WHERE, bài 1-2), hành VI khớp CHÍNH xác VỚI SurrealDB
thật. `Quét → Lọc → ChọnCột` (q08) LÀ đúng CƠ chế BÊN dưới một câu
`SELECT` thật — không phải một PHIÊN bản "gần đúng" hay "đơn giản
HOÁ sai". `MucChiMuc`/`xay_chi_muc` (q09) LÀ đúng Ý tưởng ĐẰNG sau
`DEFINE INDEX` — chỉ khác Ở chỗ SurrealDB LÀM nó tự động VÀ hiệu
quả HƠN nhiều.
::::

::::predict{#doan-tai-sao-hoc commitOnce}
Nếu SurrealDB thật LÀM được mọi thứ mini-engine LÀM (và nhiều HƠN),
tại sao q07-q09 KHÔNG bắt đầu bằng việc dạy SurrealDB thật NGAY từ
đầu?

:::opt{correct}
Vì hiểu ĐƯỢC "tra_cuu quét tuyến tính", "so sánh byte trên khoá
đã mã hoá", "chỉ mục LÀ hai mục MucChiMuc" — TỰ tay viết — LÀ cách
DUY nhất để một dòng `DEFINE INDEX`/`RELATE` thôi LÀ hộp đen
:::

:::opt
Vì SurrealDB thật QUÁ phức tạp để người MỚI học ngay TỪ đầu — mini-
engine LÀ một phiên bản "TẬP dượt" đơn giản hoá, sau KHI thành thạo
mới CHUYỂN sang bản thật
::why
Gần đúng ở việc bạn NHẬN ra mini-engine ĐƠN giản hơn SurrealDB
thật RẤT nhiều — MỘT quan sát đúng VỀ độ phức tạp.

Chỗ lệch: "ĐƠN giản hơn để dễ HỌC hơn" không PHẢI lý do chính —
q10 (quest NÀY) đã CHỨNG minh SurrealDB thật hoàn toàn dùng ĐƯỢC
ngay Ở trình ĐỘ này (bài 1 chạy nó chỉ SAU chín quest). Lý do THẬT
là: một dòng `DEFINE INDEX` HAY `RELATE` chỉ ngừng LÀ "ma thuật" khi
bạn ĐÃ tự tay xây được PHIÊN bản đơn giản của chính CƠ chế đó —
hiểu CƠ chế TRƯỚC khiến công cụ thật trở nên minh BẠCH, không phải
ngược lại.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Danh sách khoảng cách ĐÃ đủ. Còn một điểm chung TINH tế hơn: cách
hai bộ máy PHÂN tích một điều kiện WHERE phức TẠP (AND/OR) — có
khớp nhau KHÔNG?
::::

::::reflect{#nghi-lai}
Chín bài trước LÀ một bài TẬP đối chiếu CÓ hệ thống: mỗi bài LẤY
đúng MỘT khái niệm mini-engine đã DẠY (hoặc CHƯA dạy), RỒI chạy
SurrealDB thật để xem NÓ làm điều đó thế nÀO. Kết quả không PHẢI
"mini-engine SAI" hay "mini-engine THỪA" — mà LÀ mini-engine dạy
đúng LÕI của một PHẦN việc (thực thi SELECT...WHERE), còn SurrealDB
thật LÀ một hệ thống hoàn CHỈNH xây trên rất nhiều lõi TƯƠNG tự,
ghép LẠI. Còn MỘT câu hỏi cuối: khi câu WHERE có AND/OR (q07's
`CayAst` VỚI `VaNut`/`HoacNut`), hành vi HAI bộ máy có khớp KHÔNG?
::::

::::checkpoint{mastery=0.85}
::::
