# Chapter 3C — Hardware, Compute & LeetGPU Basics

> **Bạn sẽ học được**:
> - Memory Hierarchy (L1, L2, RAM, VRAM)
> - CPU vs GPU Architecture
> - Sự kỳ diệu của PyTorch và CUDA
> - Memory Bandwidth Bound vs Compute Bound
>
> **Yêu cầu trước**: Chapter 3B
> **Thời gian đọc**: ~45 phút | **Level**: Advanced

---

## 3C.1 — Tại sao Python chạy AI lại nhanh?

Python nổi tiếng là ngôn ngữ chậm (vì GIL và Dynamic Typing). Nhưng thực tế, khi bạn chạy mô hình AI bằng Python, **Python không hề tính toán**. Nó chỉ đóng vai trò là một "người quản lý" gọi các hàm viết bằng C++ và CUDA chạy trực tiếp trên GPU.

Để thành thạo AI Engineering, bạn phải hiểu được "đám công nhân" (Hardware) bên dưới người quản lý Python hoạt động như thế nào.

---

## 3C.2 — Memory Hierarchy (Phân cấp bộ nhớ)

Dữ liệu không bao giờ nhảy thẳng từ ổ cứng vào chip xử lý. Nó phải đi qua các trạm trung chuyển.

1. **CPU Registers**: Cực nhỏ, tốc độ 1 cycle.
2. **L1 / L2 / L3 Cache**: Vài chục MB. Nằm ngay trên chip CPU. Tốc độ ánh sáng.
3. **RAM (DRAM)**: Vài chục đến hàng trăm GB. (Chậm hơn L1 hàng trăm lần).
4. **VRAM (Video RAM)**: Bộ nhớ nằm trên GPU (VD: 80GB trên H100). Đây là "ngôi nhà" của các Model AI.
5. **Disk / SSD**: Rất chậm. Tải Model file 100GB từ ổ cứng lên VRAM tốn vài chục giây.

**Nguyên lý vàng**: Đặt dữ liệu liên tục cạnh nhau trên bộ nhớ để tận dụng tốc độ Cache (Cache Locality). Trong Python, kiểu `list` lưu các object rời rạc trong bộ nhớ (rất chậm). Thư viện `numpy` và `torch.Tensor` bọc các mảng C/C++ liền kề, giúp CPU/GPU đọc siêu tốc.

---

## 3C.3 — CPU vs GPU: Siêu xe và Xe chở hàng

- **CPU**: Cấu tạo bởi vài chục "siêu lõi" (vd: Intel Core i9, AMD EPYC). Nó sinh ra để giải quyết rẽ nhánh if-else phức tạp, đa nhiệm hệ điều hành (Context Switch).
- **GPU**: Gồm hàng vạn lõi "ngu ngốc". Nó không biết làm toán phức tạp, nhưng nó có thể cộng 10,000 cặp số trong **cùng 1 giây**. Kiến trúc này gọi là SIMD (Single Instruction, Multiple Data).

Vì mô hình Transformer 99% thời gian là làm phép nhân ma trận khổng lồ, GPU chính là thiết bị hoàn hảo.

---

## 3C.4 — CUDA và Tensor Cores

Khi bạn gọi `model.to("cuda")` trong PyTorch, điều gì xảy ra?

- **CUDA**: Nền tảng lập trình song song của NVIDIA. PyTorch dịch lệnh của bạn xuống các Kernel CUDA để chạy đồng loạt trên GPU.
- **Tensor Cores**: Lõi phần cứng vật lý chuyên dụng có mặt trên các thế hệ GPU mới (từ Volta). Nó thiết kế đặc biệt để thực hiện phép toán `A * B + C` (FMA) của các ma trận nhỏ (VD: 4x4) chỉ trong **đúng 1 chu kỳ máy**. Sức mạnh của LLMs phần lớn đến từ cụm Tensor Cores này.

---

## 3C.5 — Compute Bound vs Memory Bound

Đây là kiến thức đắt giá nhất khi thiết kế hạ tầng AI cho doanh nghiệp.

### 1. Compute Bound (Kẹt ở tính toán)
- **Định nghĩa**: Khả năng tính toán của GPU đạt giới hạn (100% FLOPS utilization) nhưng bộ nhớ truyền dữ liệu rất thoải mái.
- **Xảy ra khi**: Train mô hình từ đầu, hoặc khi đưa vào 1 prompt khổng lồ (Prefill phase). 

### 2. Memory Bandwidth Bound (Kẹt ở đường ống truyền dữ liệu)
- **Định nghĩa**: GPU rảnh rỗi (chip xử lý mới chạy 10-20% công suất) nhưng quá trình lại chậm vì **băng thông bộ nhớ VRAM không nạp kịp dữ liệu lên chip**.
- **Xảy ra khi**: Quá trình sinh chữ (Decoding phase). Khi LLM sinh ra từng chữ một, nó buộc phải kéo lại toàn bộ trọng số khổng lồ (Weights) của chính nó từ VRAM qua chip để xử lý tính toán. Việc này làm VRAM Bandwidth quá tải cực kỳ nhanh.
- **Cách giải quyết**: 
  - **Batching**: Gộp nhiều người dùng lại xử lý cùng lúc để GPU chỉ nạp Weights 1 lần mà tính cho 10 người.
  - **KV Cache Optimization**: Công nghệ PagedAttention (vLLM) để quản lý RAM tốt hơn.

---

## Tóm tắt

- Viết Python giỏi là chưa đủ. Bạn cần hiểu dữ liệu đang nằm ở RAM hay VRAM, và chip nào đang xử lý.
- GPU thống trị AI nhờ hàng vạn lõi SIMD và kiến trúc Tensor Cores chuyên dùng để nhân ma trận.
- Điểm yếu "chí mạng" của quá trình chạy mô hình LLM (Inference) không phải là nó tính toán chậm, mà là băng thông bộ nhớ truyền dữ liệu lên chip không đủ (Memory Bound).

## Tiếp theo
Chúng ta đã đi qua toàn bộ phần nền tảng khoa học máy tính và phần cứng lõi. Kể từ Chapter 4 trở đi, bạn sẽ chính thức bước vào thế giới thiết kế phần mềm, học cách dùng Python với tư duy Hàm (Functional Programming).
