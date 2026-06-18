# Chapter 3C — Hardware, Compute & LeetGPU Basics

> **Bạn sẽ học được**:
> - Phân cấp bộ nhớ: Từ Cache đến VRAM
> - Sự khác biệt giữa môi trường Node.js/V8 và GPU Compute
> - Tại sao GPU lại vô địch trong AI
> - Memory Bandwidth Bound vs Compute Bound
>
> **Yêu cầu trước**: Chapter 3B
> **Thời gian đọc**: ~35 phút | **Level**: Advanced

---

## 3C.1 — Tại sao JS/TS Dev cần hiểu Hardware?

Node.js nổi tiếng với kiến trúc Single-threaded Event Loop, xử lý I/O bất đồng bộ tuyệt vời. Nhưng khi đối mặt với mô hình AI nặng hàng chục Gigabytes thực hiện hàng nghìn tỷ phép tính mỗi giây, quy tắc trò chơi thay đổi hoàn toàn.

JS Developer khi lấn sân sang AI Engineer cần biết: giới hạn của hệ thống AI không nằm ở Event Loop, mà nằm ở **Giới hạn vật lý của RAM và kiến trúc lõi GPU**.

---

## 3C.2 — Memory Hierarchy (Phân cấp bộ nhớ)

Dữ liệu đi từ ổ cứng vào RAM, từ RAM vào Cache, rồi mới tới vi xử lý.

1. **CPU Registers**: Tốc độ ánh sáng (1 ns). Chứa ngay dữ liệu đang xử lý.
2. **L1 / L2 / L3 Cache**: Vài MB, nằm ngay trên chip.
3. **RAM (Main Memory)**: ~100 ns. Đây là nơi V8 lưu trữ toàn bộ các object trong bộ nhớ Heap.
4. **Disk (SSD)**: Rất chậm.

Trong TS/JS, mọi Object/Array thực tế được cấp phát tự do trên Heap (bị phân mảnh). V8 tối ưu khá tốt, nhưng để đạt **Cache Locality** tuyệt đối (dữ liệu nằm kề nhau trên RAM), các ngôn ngữ như C++/Rust có lợi thế hơn vì chúng cho phép cấp phát mảng liền kề (contiguous array) thực sự. Đó là lý do thư viện như `numpy` (Python) dùng C dưới nền.

---

## 3C.3 — CPU vs GPU trong AI

### CPU (Central Processing Unit)
Giống như vài vị giáo sư toán siêu việt. Họ giải quyết các bài toán logic if-else, rẽ nhánh phức tạp cực nhanh. JS chạy trên một "giáo sư" (Single Thread).

### GPU (Graphics Processing Unit)
Giống như một đội quân 10,000 học sinh cấp 1. Mỗi em chỉ biết cộng trừ nhân chia cơ bản.
Mạng Nơ-ron AI, thực chất là một chuỗi các phép tính ma trận khổng lồ. Việc tính toán ma trận hoàn toàn có thể chia nhỏ ra và tính song song cùng lúc (SIMD - Single Instruction, Multiple Data). Hàng ngàn "học sinh" GPU cùng nhân ma trận sẽ đè bẹp các "giáo sư" CPU.

---

## 3C.4 — Khái niệm CUDA và Tensor Cores

- **CUDA**: Nền tảng của NVIDIA cho phép ta ra lệnh cho GPU. Các frame AI (PyTorch/Tensorflow.js) đều dịch lệnh xuống CUDA C++.
- **Tensor Cores**: Trong GPU NVIDIA hiện đại, có các lõi chuyên dụng gọi là Tensor Core. Thay vì mất nhiều bước để tính `A * B + C`, Tensor Core làm xong trọn vẹn phép toán nhân-cộng ma trận nhỏ (4x4) chỉ trong **1 xung nhịp (clock cycle)**.

---

## 3C.5 — Compute Bound vs Memory Bound

Điều tối quan trọng khi triển khai LLM cho người dùng:

1. **Compute Bound**: Hệ thống bị thắt cổ chai ở năng lực tính toán.
   - Thường xảy ra khi người dùng gửi một đoạn text siêu dài (10,000 từ) và LLM cần đọc nó (Prefill phase). Nó phải nhân ma trận điên cuồng.

2. **Memory Bandwidth Bound**: Bị thắt cổ chai ở đường truyền dữ liệu.
   - Khi LLM sinh ra từng chữ (Decoding phase), để sinh ra 1 token, nó cần tải TOÀN BỘ weights (mấy chục GB) từ VRAM vào chip xử lý. Việc này đòi hỏi cáp băng thông cực khủng (hàng TB/s). Các chip tính toán lúc này rảnh rỗi chờ dữ liệu nạp lên.
   - Giải pháp: Batching (gộp nhiều user lại tính chung), PagedAttention (quản lý bộ nhớ VRAM thông minh).

---

## Tóm tắt

- Node/TS tuyệt vời ở I/O và điều phối, nhưng cực yếu ở tính toán toán học dày đặc. Đó là lý do ta thường gọi API đến các Inference Servers (GPU).
- GPU mạnh mẽ nhờ kiến trúc SIMD xử lý song song hàng ngàn luồng.
- "Sát thủ" của LLM trên Production thường không phải tính toán chậm, mà là băng thông VRAM không tải kịp dữ liệu (Memory Bound).

## Tiếp theo
Khóa nền tảng CS của bạn đã hoàn chỉnh! Từ bài sau, ta sẽ bắt đầu tìm hiểu sâu về cú pháp và sức mạnh thực sự của TypeScript Type System.
