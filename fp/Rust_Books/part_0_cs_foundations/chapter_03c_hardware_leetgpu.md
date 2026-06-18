# Chapter 3C — Hardware, Compute & LeetGPU Basics

> **Bạn sẽ học được**:
> - Memory Hierarchy (Phân cấp bộ nhớ): Từ L1 Cache đến Disk
> - Kiến trúc CPU vs GPU: Tại sao GPU lại vô đối trong AI?
> - Khái niệm CUDA, Tensor Cores & SIMD
> - Compute Bound vs Memory Bandwidth Bound
>
> **Yêu cầu trước**: Chapter 3B
> **Thời gian đọc**: ~45 phút | **Level**: Advanced

---

## 3C.1 — Tại sao Software Engineer cần hiểu Hardware?

Khi lập trình web truyền thống, bạn hiếm khi quan tâm ứng dụng chạy trên con chip nào. Nhưng trong AI Engineering, **phần cứng quyết định tất cả**. Việc tải một mô hình LLM lên bộ nhớ, phân chia batch size, hay thiết kế hệ thống suy luận (Inference) đều chịu giới hạn vật lý của RAM và GPU.

Nắm được kiến trúc phần cứng là cách duy nhất để chuyển từ "người gọi API" (API wrapper) thành một "kỹ sư AI" (AI Engineer) thực thụ.

---

## 3C.2 — Memory Hierarchy (Phân cấp bộ nhớ)

Máy tính không nạp dữ liệu trực tiếp từ ổ cứng vào CPU. Dữ liệu phải đi qua nhiều tầng, mỗi tầng càng gần CPU thì càng nhanh nhưng dung lượng càng nhỏ.

1. **CPU Registers**: Tốc độ ánh sáng (1 chu kỳ xung nhịp). Chứa biến hiện tại đang xử lý.
2. **L1 / L2 Cache**: (Vài chục KB đến vài MB). Nằm ngay trên lõi CPU. Đọc mất ~1-10 ns.
3. **L3 Cache**: (Vài chục MB). Dùng chung cho nhiều lõi CPU.
4. **RAM (Main Memory)**: (Hàng chục đến hàng trăm GB). Đọc mất ~100 ns. (Chậm gấp 100 lần L1).
5. **SSD / NVMe**: Đọc mất hàng chục micro-giây (Chậm gấp 10,000 lần RAM).

**Tại sao điều này quan trọng? (Cache Locality)**
Trong Rust, một mảng `Vec<T>` là một dải bộ nhớ **liên tục**. Khi CPU đọc phần tử `A[0]`, nó sẽ load luôn `A[1]`, `A[2]` vào L1 Cache. Do đó, việc duyệt qua `Vec<T>` nhanh hơn hàng chục lần so với duyệt qua `LinkedList<T>` (vì các node của LinkedList nằm rải rác trong RAM -> Gây ra **Cache Miss** liên tục).

---

## 3C.3 — CPU vs GPU: Xe Công thức 1 và Xe Tải Hạng Nặng

Tại sao CPU mạnh như Intel Core i9 / AMD Ryzen lại không dùng để train mô hình AI?

### CPU (Central Processing Unit)
- **Kiến trúc**: Có ít lõi (vd: 8, 16, 64 lõi), nhưng mỗi lõi **cực kỳ thông minh và mạnh mẽ**.
- **Thế mạnh**: Xử lý logic rẽ nhánh phức tạp (if/else), đa nhiệm hệ điều hành (Context Switching).
- **So sánh**: Giống như 10 chiếc siêu xe đua F1. Chạy từ A đến B siêu nhanh nhưng chở được rất ít hành khách (dữ liệu).

### GPU (Graphics Processing Unit)
- **Kiến trúc**: Chứa **hàng ngàn lõi** (ví dụ: NVIDIA H100 có hàng chục ngàn lõi). Mỗi lõi thì khá "ngu" và chậm, chỉ làm được toán cộng/nhân đơn giản.
- **Mô hình tính toán**: SIMD (Single Instruction, Multiple Data). Hàng ngàn lõi cùng làm chung 1 phép toán trên những mảnh dữ liệu khác nhau cùng một lúc.
- **So sánh**: Giống như một đoàn tàu chở hàng. Khởi động thì chậm, nhưng mỗi chuyến chở được 10,000 người (dữ liệu lớn).
- **Sự phù hợp với AI**: Mạng nơ-ron thực chất chỉ là một ma trận số khổng lồ. Mọi tính toán trong Neural Networks đều quy về phép nhân ma trận (MatMul - Matrix Multiplication). GPU có thể tính hàng triệu phép nhân ma trận trong 1 tích tắc.

---

## 3C.4 — Lập trình GPU: CUDA & Tensor Cores

### CUDA (Compute Unified Device Architecture)
CUDA là nền tảng (và ngôn ngữ) của NVIDIA cho phép lập trình viên viết code chạy trực tiếp trên các lõi GPU. 
- Khi dùng PyTorch hay TensorFlow, bên dưới chúng đã tự động gọi các hàm CUDA viết bằng C++.
- Bạn định nghĩa một hàm (gọi là **Kernel**) và yêu cầu GPU chạy kernel đó đồng thời trên hàng triệu luồng (Threads).

### Tensor Cores
GPU hiện đại (từ kiến trúc Volta trở đi) có một thành phần phần cứng chuyên dụng gọi là **Tensor Core**. 
- Một CUDA core thông thường thực hiện phép `A * B + C` (FMA) trong 1 chu kỳ.
- Một Tensor Core có thể thực hiện một phép nhân ma trận `4x4` chỉ trong 1 chu kỳ! Nó làm AI tăng tốc lên gấp 10-30 lần.

---

## 3C.5 — Nút thắt cổ chai: Compute Bound vs Memory Bound

Đây là kiến thức quan trọng nhất khi triển khai mô hình LLM thực tế.

### 1. Compute Bound (Bị giới hạn bởi khả năng tính toán)
- **Định nghĩa**: Thời gian để GPU xử lý xong toán học **lâu hơn** thời gian để tải dữ liệu vào GPU.
- **Khi nào xảy ra**: Khi Train model, hoặc khi Inference ở giai đoạn "Prefill" (Khi bạn ném 1 prompt dài 10,000 từ vào mô hình để nó đọc lần đầu tiên).
- **Cách giải quyết**: Mua GPU xịn hơn (có nhiều FLOPS hơn), dùng quantization để giảm độ chính xác (FP32 -> FP16 -> INT8) nhằm tính toán nhanh hơn.

### 2. Memory Bandwidth Bound (Bị giới hạn bởi băng thông bộ nhớ)
- **Định nghĩa**: Toán học xử lý quá nhanh, GPU phải ngồi "chơi xơi nước" để chờ dữ liệu tải từ VRAM (Video RAM) sang chip xử lý. Băng thông (Bandwidth) không đủ lớn.
- **Khi nào xảy ra**: Khi model bắt đầu **Generate token từng chữ một** (Decoding phase). Mỗi lần sinh ra 1 chữ, nó phải tải TOÀN BỘ trọng số (Weights) của mô hình (hàng chục GB) từ VRAM qua chip xử lý. 
- **Cách giải quyết**: Tăng Batch Size (sinh ra cho 10 người dùng cùng lúc để tái sử dụng một lần nạp Weights), sử dụng **PagedAttention** hoặc FlashAttention để tối ưu hóa bộ nhớ KV Cache.

> Kỹ thuật **vLLM** mà chúng ta sẽ học ở phần Production (Chapter 44) ra đời chính là để giải quyết bài toán Memory Bandwidth Bound này!

---

## Tóm tắt

- **Cache Locality**: Đặt dữ liệu liên tiếp trong RAM (`Vec`) giúp CPU đọc nhanh hơn.
- **CPU vs GPU**: CPU giỏi logic phức tạp, rẽ nhánh. GPU là ông hoàng nhân ma trận song song (SIMD).
- **VRAM Bandwidth**: Trong thế giới LLMs, nút thắt lớn nhất không phải là chip tính chậm, mà là đường ống bơm dữ liệu từ bộ nhớ VRAM vào chip không đủ to (Memory Bound ở Decoding Phase).

## Tiếp theo
Bạn đã có đủ hành trang CS Foundations (Toán, Thuật toán, Phần cứng). Bước tiếp theo, chúng ta sẽ bắt đầu học ngôn ngữ lập trình cụ thể để hiện thực hóa những kiến thức này (Part 1).
