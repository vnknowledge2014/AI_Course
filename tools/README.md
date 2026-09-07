# `tools/` — pipeline kiểm chất lượng nội dung

`cong.sh` là điểm vào chính — chạy toàn bộ cổng theo đúng thứ tự:

```bash
bash tools/cong.sh          # đầy đủ (~15 cổng, gồm chạy lời giải thật qua Pyodide/WASM + mutation-testing)
bash tools/cong.sh --nhanh  # bỏ 4 cổng chậm nhất (lời giải thật, mutation, Rust test, đối chiếu rustc)
```

## Script được `cong.sh` gọi trực tiếp (9 Python + 2 Node)

| Script | Cổng đo gì |
|---|---|
| `kiem_khong_mat.py` | Khối mã trong `.lesson.md` không bị mất khi biên dịch sang JSON |
| `kiem_bai_hoc.py` | "Hiến chương sư phạm" — cấm 8 cụm từ hạ thấp người học, cấm khung `TODO —` chưa điền |
| `kiem_do_thi.py` | Đồ thị tiền đề `teaches`/`requires` không có nút treo |
| `kiem_so_hoc.py` | Mọi đẳng thức số học viết trong văn xuôi đều đúng |
| `kiem_su_that.py` | Hằng số "thế giới hư cấu" không tự đổi giữa các bài |
| `kiem_bi_mat.py` | Không có credential thật lọt vào git |
| `trich_schema.py --check` | Schema `content-schema` khớp `MASTERPLAN.md` §5 |
| `kiem_app_dong_bo.py` | App ship đúng số bài repo có |
| `kiem_doan_truoc.py` | Khối `predict` đo thật, không lộ đáp án ngầm |
| `kiem_ma_bai_hoc.mjs` | Chạy THẬT mọi lời giải qua đúng engine app dùng (Pyodide/TS/WASM) |
| `kiem_dot_bien.mjs` | Mutation-testing — sửa nhỏ lời giải đúng thành sai, đòi cách chấm phải bắt được |

`nap_goi_pyodide.mjs` không phải cổng — là helper dùng chung, được `import` bởi hai script `.mjs` ở trên.

## Script KHÔNG nằm trong `cong.sh` — chạy tay khi cần

| Script | Khi nào chạy |
|---|---|
| `inventory.py --check` | Kiểm kê nội dung thật trên đĩa — có wiring riêng qua `package.json` script `inventory`, tách biệt khỏi `cong.sh` vì đây là báo cáo tiến độ, không phải cổng pass/fail nội dung. |
| `tien_do.py` | `cong.sh` CÓ gọi (in cuối, không tính vào 15 cổng) — đo % tiến độ v1.0 thật. |
| `di_tru_sach.py` | Di trú 159 chương từ `fp/*_Books` sang `content/legacy/*.chapter.md`. Chạy TAY sau khi sửa sách nguồn trong `fp/` — xem `fp/README.md`. Không tự động vì di trú là một quyết định biên tập (khi nào đẩy bản mới sang content), không phải kiểm tra. |
| `trich_rust.py` | Trích snippet Rust từ `fp/Rust_Books` vào `crates/byte-rust-conformance/corpus/`. Chạy TAY khi cần mở rộng corpus đối chiếu `rustc` — output đã có sẵn trong git (364 file), không cần chạy lại trừ khi `fp/Rust_Books` đổi. |
| `spec_watch.mjs` | Đối chiếu bề mặt MCP/A2A mà khoá học ghim (`packages/mcp-kit/src/spec-2026-06.ts`) với spec upstream thật — cổng "conformance hàng tuần" của MASTERPLAN §9, chạy bởi `.github/workflows/spec-watch.yml` (cron tuần), KHÔNG phải cổng merge. Drift không sửa cơ học được (đổi từ vựng = quyết định nội dung), nên quản bằng baseline `spec-watch-bo-qua.json` kèm lý-do bắt buộc. Chạy tay: `node tools/spec_watch.mjs`. |

## Quy ước

- Không phụ thuộc ngoài chuẩn Python (đọc YAML/markdown bằng tay thay vì kéo `pyyaml`) — `tools/` cố tình nhẹ.
- Hai script `.mjs` (`kiem_ma_bai_hoc.mjs`, `kiem_dot_bien.mjs`) PHẢI chạy qua Node, không viết lại bằng Python — chúng chấm bài qua chính engine JS mà trình duyệt học viên dùng (`packages/exec-python`/`exec-typescript`/`exec-rust`), viết lại bằng Python sẽ chấm qua một runtime KHÁC với runtime thật.
- Ngoại lệ có chủ ý (đổi số cố ý, mutant vô hại, v.v.) khai trong `content/curriculum/*.yaml` (`so-hoc-sai-co-y.yaml`, `su-that-the-gioi.yaml`, `doan-truoc-bo-qua.yaml`, `dot-bien-bo-qua.yaml`), không hardcode trong script.
