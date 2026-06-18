# Chapter 39 — Deployment & DevOps

> **Bạn sẽ học được**:
> - Đóng gói ứng dụng Python bằng Docker (Multi-stage build)
> - Best practices bảo mật Docker
> - CI/CD pipeline basics (GitHub Actions)
> - Lời kết & Bước tiếp theo
>
> **Yêu cầu trước**: Chapter 38 (Observability)
> **Thời gian đọc**: ~25 phút | **Level**: Principal

---

## 39.1 — Dockerizing Python (Multi-stage Build)

```dockerfile
# Sử dụng builder stage để cài đặt dependencies (giảm size ảnh cuối cùng)
FROM python:3.12-slim AS builder

# Thiết lập biến môi trường
ENV PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1

WORKDIR /app

# Chỉ copy file requirements trước để tận dụng Docker cache
COPY pyproject.toml .

# Cài đặt uv (package manager nhanh cho Python)
RUN pip install uv
RUN uv pip install --system -r pyproject.toml

# =======================================================
# Stage chạy ứng dụng (Minimal size & Secure)
FROM python:3.12-slim

# Tạo non-root user để tăng bảo mật
RUN groupadd -r appuser && useradd -r -g appuser appuser

WORKDIR /app

# Copy thư viện đã cài từ builder stage
COPY --from=builder /usr/local/lib/python3.12/site-packages/ /usr/local/lib/python3.12/site-packages/
COPY --from=builder /usr/local/bin/ /usr/local/bin/

# Copy mã nguồn
COPY ./src /app/src

# Đổi quyền sở hữu cho non-root user
RUN chown -R appuser:appuser /app
USER appuser

# Expose port (FastAPI mặc định)
EXPOSE 8000

# Chạy ứng dụng bằng gunicorn + uvicorn worker
CMD ["gunicorn", "src.api.main:app", "--workers", "4", "--worker-class", "uvicorn.workers.UvicornWorker", "--bind", "0.0.0.0:8000"]
```

## 39.2 — CI/CD Pipeline (GitHub Actions)

```yaml
# .github/workflows/ci.yml
name: CI Pipeline

on:
  push:
    branches: [ "main" ]
  pull_request:
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
        
    - name: Install dependencies (using uv)
      run: |
        pip install uv
        uv pip install --system ".[dev]"
        
    - name: Type checking (mypy)
      run: mypy src/
      
    - name: Linting (ruff)
      run: ruff check src/
      
    - name: Run tests (pytest)
      run: pytest tests/

  build:
    needs: test
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    steps:
      # Build and Push Docker image to Registry...
```

## 39.3 — Lời kết (Conclusion)

Chúc mừng! Bạn đã hoàn thành hành trình từ **Cơ bản (Math, Data Structures)** đến **Tư duy hàm (Functional Thinking, FP Patterns)**, làm chủ **Thiết kế Domain (DDD)** và áp dụng vào **Production (FastAPI, SQL, DevOps)** trong Python.

**Bạn đã học được:**
1. **Python as FP Language:** Dùng `dataclass(frozen=True)`, `tuple`, Pydantic để có Immutability.
2. **Domain-Driven Design:** Ubiquitous Language, Value Objects, Entities.
3. **Architecture:** Tách biệt Pure Domain Logic (Core) khỏi I/O (Ports & Adapters).
4. **Resilience:** Railway-Oriented Programming (Result type) thay cho Exceptions.
5. **Scale:** CQRS, Event-Driven, Observability.

**Bước tiếp theo (Next Steps):**
- Đọc cuốn *Domain-Driven Design* (Eric Evans).
- Thử nghiệm ngôn ngữ thuần FP (F#, Haskell) để mở rộng tư duy.
- Áp dụng các patterns này vào dự án thực tế của bạn, bắt đầu từ những module nhỏ.

> *"Make illegal states unrepresentable, and push I/O to the edges."*

Cảm ơn bạn đã đọc cuốn sách này!
