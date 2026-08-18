# Chapter 4 — The Python Ecosystem & Tooling

> **Bạn sẽ học được**:
> - Hệ sinh thái công cụ hiện đại của Python (2024+): Tại sao chúng ta không dùng `pip` hay `venv` truyền thống nữa?
> - `uv` — Trình quản lý package và môi trường ảo siêu tốc (nhanh hơn gấp 10-100 lần pip).
> - **Linh hồn của Functional Python**: Static Type Checking với `mypy` hoặc `pyright`.
> - `ruff` — Linter và Formatter thần tốc viết bằng Rust.
> - Cài đặt IDE (VSCode + Pylance) đúng chuẩn để "ép" Python báo lỗi như một ngôn ngữ biên dịch.
>
> **Yêu cầu trước**: Không cần! Nếu bạn bỏ qua Part 0 vì quá nhiều lý thuyết, bạn đang ở đúng chỗ để bắt đầu thực hành.
> **Thời gian đọc**: ~30 phút | **Level**: Beginner
> **Kết quả cuối cùng**: Bạn có một môi trường lập trình Python cực kỳ nghiêm ngặt, chuyên nghiệp, tự động format code và báo lỗi kiểu dữ liệu ngay lập tức.

---

## 4.1 — Python: Ngôn ngữ động và Vấn đề của sự dễ dãi

Python mặc định là một ngôn ngữ động (dynamically typed). Bạn có thể truyền bất cứ thứ gì vào một hàm. Điều này rất tuyệt khi viết script nhỏ, nhưng là **cơn ác mộng** đối với Domain-Driven Design (DDD) và Functional Programming (FP).

Nếu không có Type Hints (Gợi ý kiểu dữ liệu), bạn sẽ không thể biết hàm `process_order(order)` nhận vào cái gì và trả ra cái gì nếu không đọc ruột code.

Để học khóa học này, chúng ta sẽ biến Python thành một ngôn ngữ "gần giống ngôn ngữ biên dịch" nhất có thể. Và để làm được điều đó, bộ ba công cụ `uv` - `pyright` - `ruff` là bắt buộc.

---

## 4.2 — Khởi tạo dự án với uv (Tạm biệt pip và venv)

Ngày xưa, để tạo một project Python, bạn phải chạy `python -m venv venv`, rồi `source venv/bin/activate`, rồi `pip install ...`. Quá rườm rà.

Hôm nay, chúng ta dùng **`uv`** (một công cụ quản lý Python viết bằng Rust, siêu tốc độ).
- Hướng dẫn cài đặt `uv`: [astral.sh/uv](https://docs.astral.sh/uv/)

Hãy tạo dự án đầu tiên:

```bash
# Tạo dự án mới
uv init py_cafe

# Di chuyển vào thư mục dự án
cd py_cafe

# Cài đặt thư viện (ví dụ: cài pydantic để validate dữ liệu)
uv add pydantic
```

Lệnh `uv init` sẽ tạo ra một file `pyproject.toml`. Đây là "Sổ hộ khẩu" của dự án, thay thế hoàn toàn cho file `requirements.txt` cũ kỹ. `uv` cũng tự động tạo môi trường ảo `.venv` cho bạn trong chớp mắt.

---

## 4.3 — Bật "Chế Độ Khó" cho Python: mypy / pyright

Bạn viết code thế này:
```python
def add(a: int, b: int) -> int:
    return a + b

add(1, "2") # Lỗi rành rành
```
Nếu bạn chạy bằng lệnh `python main.py`, nó sẽ Crash! Python mặc định **KHÔNG HỀ KIỂM TRA Type Hints** lúc chạy. Type Hints chỉ là "ghi chú" cho vui nếu bạn không có công cụ kiểm tra (Static Type Checker).

Có 2 công cụ phổ biến nhất:
1. **mypy**: Công cụ lâu đời nhất, chuẩn mực nhất.
2. **pyright**: Của Microsoft, nhanh hơn và khắt khe hơn. (Tích hợp sẵn trong VSCode qua extension Pylance).

Để đảm bảo code của bạn an toàn, hãy cài đặt `mypy` và cấu hình khắt khe nhất.
```bash
uv add --dev mypy
```

Thêm vào cuối file `pyproject.toml` của bạn:
```toml
[tool.mypy]
strict = true
disallow_untyped_defs = true
```

Bây giờ, mỗi khi bạn viết xong code, hãy chạy:
```bash
uv run mypy .
```
Nếu có bất kỳ lỗi sai kiểu dữ liệu nào, `mypy` sẽ chỉ mặt đặt tên ngay lập tức. Đây là cách chúng ta áp dụng tư duy "Make illegal states unrepresentable" trong Python.

---

## 4.4 — IDE & Trải Nghiệm Viết Code (VSCode)

Để trải nghiệm mượt mà nhất, bạn **BẮT BUỘC** phải cài VSCode và extension **Python** (của Microsoft).

Sau khi cài đặt, hãy cấu hình VSCode để tự động báo lỗi (như một compiler thực thụ):
1. Mở Cài đặt (Settings) trong VSCode (`Ctrl + ,`).
2. Tìm kiếm `Type Checking Mode`.
3. Đổi tùy chọn `Python > Analysis: Type Checking Mode` từ `off` sang `strict`.

**Tính năng tuyệt đỉnh: Inlay Hints (Gợi ý ngầm)**
Cũng trong Settings, tìm `Inlay Hints` và bật:
`Python > Analysis > Inlay Hints: Variable Types` và `Function Return Types`.
VSCode sẽ tự động hiển thị chữ mờ màu xám gợi ý kiểu dữ liệu cho bạn, giúp bạn học cách Python hiểu dữ liệu cực nhanh.

---

## 4.5 — Linter và Formatter: Ruff

Cộng đồng Python từng cãi nhau rất nhiều về việc dùng `flake8`, `black`, hay `isort`. Giờ đây, tất cả đã bị thay thế bởi **Ruff** — một linter và formatter viết bằng Rust, chạy nhanh gấp 10-100 lần.

Cài đặt Ruff:
```bash
uv add --dev ruff
```

Cấu hình trong `pyproject.toml`:
```toml
[tool.ruff]
line-length = 88
```

Để tự động sửa lỗi và căn chỉnh code:
```bash
# Kiểm tra lỗi phong cách
uv run ruff check .

# Tự động format code cho đẹp
uv run ruff format .
```

*Mẹo*: Cài extension `Ruff` trên VSCode và bật tính năng **Format On Save**. Cứ bấm `Ctrl + S` là code tự động đẹp!

---

## 🎉 Tóm tắt Workflow Hàng Ngày
- Tạo dự án và cài thư viện bằng **`uv`**.
- Luôn bật **Type Checking Mode: strict** trong VSCode.
- Code xong, chạy `uv run ruff format .` để format.
- Trước khi đẩy code lên git, chạy `uv run mypy .` để đảm bảo an toàn tuyệt đối.

Môi trường của bạn đã hoàn hảo, giống hệt một ngôn ngữ tĩnh mạnh mẽ. Hãy chuyển sang **Chapter 5** để học những dòng code an toàn đầu tiên nhé!

---

## ✅ Checkpoint 4

1. `uv` nhanh hơn `pip` nhờ đâu (nêu hai lý do)?
2. `ruff` thay được những công cụ nào cùng lúc?
3. `mypy` và `pyright` — nên dùng cái nào ở đâu?

<details>
<summary>Đáp án</summary>

1. Viết bằng Rust (không tốn thời gian khởi động interpreter cho mỗi thao tác), và dùng global cache + hard link thay vì tải/giải nén lại cho từng môi trường.
2. `flake8` (lint), `black` (format), `isort` (sắp import), `pyupgrade`, và một phần `pylint` — một binary thay cho cả bộ.
3. `pyright`/Pylance trong IDE cho phản hồi tức thì; `mypy` trong CI làm phán quyết cuối. Xem Phụ lục A.
</details>

---

## 🏋️ Bài tập

**Bài 1 (5 phút).** Tạo project mới bằng `uv init`, thêm `pydantic`, và xem `pyproject.toml` + `uv.lock` sinh ra.

**Bài 2 (10 phút).** Cấu hình `ruff` với `select = ["E","F","I","UP","B"]` rồi chạy trên một file code cũ. Từng nhóm rule bắt loại lỗi gì?

**Bài 3 (10 phút).** Bật Inlay Hints trong VS Code + Pylance. Viết một hàm không annotate và quan sát kiểu mà Pylance suy ra — nó đoán đúng bao nhiêu phần?

---

## 🔧 Troubleshooting

| Vấn đề | Vì sao xảy ra | Hướng xử lý |
|---|---|---|
| `uv: command not found` | Chưa cài hoặc chưa vào PATH | `curl -LsSf https://astral.sh/uv/install.sh \| sh` rồi mở lại shell |
| VS Code dùng nhầm interpreter | Chưa trỏ tới `.venv` | Ctrl+Shift+P → Python: Select Interpreter |
| `ruff` và formatter khác đánh nhau | Bật cả black lẫn ruff-format | Chọn một; `ruff format` đã đủ |
| mypy không thấy thư viện | Thiếu stub | `uv add --dev types-<lib>`, hoặc `ignore_missing_imports` cho riêng module đó |
| `uv sync` cài khác máy đồng nghiệp | Không commit lock file | Commit `uv.lock`, dùng `uv sync --frozen` trong CI |
