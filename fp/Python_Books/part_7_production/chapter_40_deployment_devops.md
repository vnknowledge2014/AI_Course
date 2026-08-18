# Chapter 40 — Deployment & DevOps

> **Bạn sẽ học được**:
> - Đóng gói ứng dụng Python bằng Docker (Multi-stage build) để tối ưu kích thước và bảo mật.
> - Tại sao phải chạy ứng dụng với Non-root user và kết hợp Gunicorn + Uvicorn.
> - Triết lý CI/CD (Continuous Integration / Continuous Deployment) qua GitHub Actions.
> - Lời kết & Tổng kết lại toàn bộ hành trình cuốn sách.
>
> **Yêu cầu trước**: Chapter 39 (AI System Design)
> **Thời gian đọc**: ~30 phút | **Level**: Principal
> **Kết quả cuối cùng**: Mã nguồn của bạn rời khỏi máy tính cá nhân và an toàn vận hành trên môi trường Production thực tế.

---

Code chạy ngon trên máy tính của bạn (Local) không có nghĩa là nó sẽ chạy trên máy chủ (Production). 
Khác biệt về hệ điều hành, phiên bản Python, hay các thư viện C++ cài ngầm có thể làm hỏng ứng dụng. Giải pháp tối thượng là **Docker**. Hơn nữa, chúng ta cần tự động hóa việc test và build mỗi khi có code mới thông qua **CI/CD**.

## 40.1 — Tối ưu hóa Dockerfile (Multi-stage & Security)

Một Dockerfile nghiệp dư thường chỉ có 1 `FROM python:3.12`. Hậu quả:
1. **Dung lượng khổng lồ** (có thể lên tới 1GB) vì nó chứa cả các công cụ biên dịch (gcc, make) cần thiết lúc cài thư viện.
2. **Kém bảo mật** vì mặc định nó chạy ứng dụng bằng quyền `root`. Nếu hacker khai thác được ứng dụng, họ chiếm luôn cả Container!

Dưới đây là chuẩn **Multi-stage Build** (Build nhiều giai đoạn) dành cho Production:

```dockerfile
# =======================================================
# STAGE 1: BUILDER
# Dùng bản Python đầy đủ để cài đặt thư viện (có thể cần compile C-extensions)
# =======================================================
FROM python:3.12-slim AS builder

# Tắt cảnh báo và ngăn Python ghi file .pyc dư thừa
ENV PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1

WORKDIR /app

# CHỈ COPY pyproject.toml / requirements.txt trước
# Tại sao? Để tận dụng Docker Cache. Nếu bạn không đổi thư viện, 
# Docker sẽ bỏ qua bước cài đặt rất lâu này dù bạn có đổi code.
COPY pyproject.toml .

# Cài đặt thư viện bằng `uv` - Package manager nhanh gấp 10 lần pip
RUN pip install uv
RUN uv pip install --system -r pyproject.toml

# =======================================================
# STAGE 2: PRODUCTION RUNNER
# =======================================================
FROM python:3.12-slim

# BẢO MẬT: Tạo một user thường (non-root) để chạy ứng dụng
RUN groupadd -r appuser && useradd -r -g appuser appuser

WORKDIR /app

# CHỈ COPY thư viện đã cài xong từ Stage 1 sang Stage 2.
# Bỏ lại toàn bộ source code C++ hay tool biên dịch ở Stage 1!
COPY --from=builder /usr/local/lib/python3.12/site-packages/ /usr/local/lib/python3.12/site-packages/
COPY --from=builder /usr/local/bin/ /usr/local/bin/

# Lấy source code vào container
COPY ./src /app/src

# Trao quyền sở hữu folder cho user thường
RUN chown -R appuser:appuser /app

# Ép Container chạy bằng user thường từ đây trở xuống
USER appuser

EXPOSE 8000

# KHÔNG DÙNG `uvicorn main:app` trên Production!
# Dùng Gunicorn làm Process Manager để quản lý nhiều tiến trình Uvicorn (Workers).
# Nếu 1 worker bị sập, Gunicorn sẽ tự động tạo worker mới.
CMD ["gunicorn", "src.api.main:app", "--workers", "4", "--worker-class", "uvicorn.workers.UvicornWorker", "--bind", "0.0.0.0:8000"]
```

---

## 40.2 — CI/CD Pipeline (Tự động hóa)

Nguyên tắc DevOps: **Con người không bao giờ deploy code bằng tay từ máy tính cá nhân**.
Quá trình đưa code lên Production phải trải qua một đường ống tự động (Pipeline) do máy chủ CI đảm nhiệm.

1. **Continuous Integration (CI)**: Mỗi khi bạn push code lên GitHub, máy chủ sẽ tự tải code về, chạy Linter, chạy Type Checker, và chạy TẤT CẢ Unit Tests.
2. **Continuous Deployment (CD)**: Nếu toàn bộ test đều PASS (Màu xanh), máy chủ sẽ tự đóng gói Docker Image và ném lên Kubernetes hoặc AWS.

Ví dụ file cấu hình cho GitHub Actions:

```yaml
# .github/workflows/ci.yml
name: CI Pipeline

on:
  push:
    branches: [ "main" ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    
    - name: Set up Python 3.12
      uses: actions/setup-python@v5
      with:
        python-version: "3.12"
        
    - name: Cài đặt Dependencies (siêu tốc với uv)
      run: |
        pip install uv
        uv pip install --system ".[dev]"
        
    - name: Kiểm tra Type (mypy)
      run: mypy src/ # Cực kỳ quan trọng để đảm bảo Domain Types luôn đúng
      
    - name: Linting & Formatting (ruff)
      run: ruff check src/
      
    - name: Chạy Unit/Integration Tests (pytest)
      run: pytest tests/

  build-and-push:
    needs: test # CHỈ CHẠY nếu Job "test" thành công
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    steps:
      - name: Xây dựng Docker Image và ném lên DockerHub...
        run: echo "Building and pushing to registry..."
```

---

## 40.3 — Lời kết (Conclusion)

Chúc mừng! Bạn đã hoàn thành chặng đường vô cùng thử thách từ việc chập chững bước vào **Tư duy hàm (Functional Thinking)**, đến việc làm chủ **Thiết kế Domain (DDD)** và cuối cùng là áp dụng tất cả vào một hệ thống **Production** mạnh mẽ bằng Python.

Hãy cùng nhìn lại những gì chúng ta đã làm được:
1. **Python as an FP Language**: Bạn không còn viết Python như viết kịch bản (Scripting) nữa. Bạn dùng `dataclass(frozen=True)`, `tuple`, Pydantic để đảm bảo tính Bất biến (Immutability).
2. **Domain-Driven Design (DDD)**: Mã nguồn của bạn giờ đây nói cùng một ngôn ngữ (Ubiquitous Language) với chuyên gia nghiệp vụ. Business Logic được bảo vệ tuyệt đối ở tầng Core.
3. **Hexagonal Architecture**: Bằng cách đẩy I/O (Database, API, Message Queue) ra ngoài rìa (Edges), mã nguồn của bạn trở nên trong suốt và cực kỳ dễ test.
4. **Railway-Oriented Programming (ROP)**: Thay vì tung `Exception` mù mịt và làm hệ thống sụp đổ dây chuyền, bạn đã biết trả về kiểu `Result[Success, Error]` giúp điều hướng luồng lỗi một cách thanh lịch.
5. **Scale & Reliability**: Bạn biết cách dùng Event-Driven Architecture, Caching, và Circuit Breaker để chịu tải, cùng với Observability để thấu thị từng ngóc ngách của hệ thống.

### Bước tiếp theo (Next Steps)
- Đọc cuốn kinh điển **"Domain-Driven Design: Tackling Complexity in the Heart of Software"** của Eric Evans.
- Hãy thử nghiệm một ngôn ngữ thuần Functional (như F# hoặc Haskell) để mở rộng biên giới tư duy của bạn, sau đó mang những triết lý đó quay lại áp dụng cho Python.
- Hãy bắt đầu áp dụng ROP và Hexagonal Architecture vào một module nhỏ trong dự án hiện tại của bạn. Đừng cố gắng đập đi xây lại toàn bộ hệ thống ngay lập tức.

> *"Make illegal states unrepresentable, and push I/O to the edges."*
> *(Hãy khiến những trạng thái sai trái không thể tồn tại, và đẩy mọi thao tác đọc/ghi ra rìa của hệ thống.)*

Cảm ơn bạn đã đồng hành cùng cuốn sách này. Chúc bạn viết ra những dòng code tĩnh lặng, bền bỉ và đẹp đẽ!

---

## ✅ Checkpoint 40

1. Vì sao multi-stage build cho image nhỏ hơn hẳn, và vì sao "nhỏ hơn" cũng là "an toàn hơn"?
2. Vì sao production dùng Gunicorn quản lý Uvicorn worker thay vì chạy `uvicorn` trực tiếp?
3. Vì sao container phải chạy bằng user thường, không phải root?

<details>
<summary>Đáp án</summary>

1. Vì stage cuối chỉ copy **kết quả** (thư viện đã cài), bỏ lại toàn bộ compiler, header C và cache build. An toàn hơn vì mỗi công cụ còn sót trong image là một công cụ kẻ tấn công dùng được sau khi vào được container.
2. Vì Gunicorn là process manager: nó khởi động lại worker chết, quản lý graceful shutdown, và tận dụng nhiều core (điều mà một process Python không làm được vì GIL). `uvicorn` một mình không có cơ chế hồi phục worker.
3. Nguyên tắc đặc quyền tối thiểu. Nếu có lỗ hổng thoát container, tiến trình root trong container ánh xạ tới root trên host (khi không bật user namespace). User thường thu hẹp đáng kể thiệt hại.
</details>

---

## 🏋️ Bài tập

**Bài 1 (10 phút).** Build image theo Dockerfile trong chương, rồi build lại bằng bản một-stage. So sánh kích thước bằng `docker images`.

**Bài 2 (15 phút).** Thêm `HEALTHCHECK` vào Dockerfile trỏ tới `/healthz`. Rồi trả lời: nên trỏ vào `/healthz` hay `/readyz`, và vì sao (xem lại Chapter 38).

**Bài 3 (25 phút).** Bổ sung vào pipeline CI: chạy `ruff`, `mypy`, `pytest --cov`, và một bước quét bảo mật image (`trivy`). Cho pipeline fail khi coverage tụt dưới 80%.

<details>
<summary>Gợi ý bài 2</summary>

`HEALTHCHECK` của Docker gây **restart** container khi fail, nên nó phải dùng
`/healthz` (liveness). Dùng `/readyz` ở đây sẽ khiến container restart mỗi lần
database chớp tắt — trong khi restart app chẳng sửa được database.
</details>

---

## 🔧 Troubleshooting

| Triệu chứng | Nguyên nhân | Cách xử lý |
|---|---|---|
| Image nặng hơn 1 GB | Base image `python:3.12` bản đầy đủ | Dùng `-slim`, và multi-stage |
| Build lại rất chậm mỗi lần đổi code | `COPY . .` đặt trước bước cài thư viện | Copy `pyproject.toml` và cài trước, copy source sau |
| `Permission denied` sau khi đổi sang non-root | Quên `chown` thư mục app | `chown -R appuser:appuser /app` trước `USER appuser` |
| Container bị kill lúc deploy | Không xử lý SIGTERM | Đặt `--graceful-timeout`, đóng connection pool khi shutdown |
| CI xanh nhưng production hỏng | Version thư viện khác nhau | Commit lock file, dùng `uv sync --frozen` |

---

## Tóm tắt

- **Multi-stage build** tách môi trường build khỏi môi trường chạy: image nhỏ hơn,
  bề mặt tấn công hẹp hơn.
- **Non-root user** là chi phí bằng không nhưng thu hẹp thiệt hại rất nhiều.
- **Gunicorn + UvicornWorker**: Gunicorn lo vòng đời process (GIL buộc phải đa
  tiến trình), Uvicorn lo ASGI trong mỗi process.
- **Thứ tự lệnh trong Dockerfile là quyết định về cache**, không phải chuyện thẩm mỹ.
- **CI/CD là lưới an toàn**, nhưng chỉ khi nó thực sự chặn được merge — pipeline
  cảnh báo mà không fail thì chẳng khác gì không có.
