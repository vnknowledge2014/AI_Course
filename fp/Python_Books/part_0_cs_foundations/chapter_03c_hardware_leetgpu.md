# Chapter 3C — Hardware, Compute: LeetCPU & LeetGPU

> **Bạn sẽ học được**:
> - Memory Hierarchy (L1, L2, RAM, VRAM)
> - CPU vs GPU Architecture
> - LeetCPU (SIMD, Branch Prediction)
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



## 3C.5 — LeetCPU: Tối ưu hóa cực hạn trên CPU

Mặc dù GPU thống trị AI, CPU vẫn đảm nhận khâu tiền xử lý dữ liệu (Data Preprocessing, Tokenization, RAG chunking). Nền tảng `LeetCPU` dạy ta cách vắt kiệt sức mạnh của CPU thông qua:

1. **Vectorization (SIMD trên CPU)**: Thay vì cộng từng cặp số trong một mảng bằng vòng lặp `for`, các tập lệnh AVX-512 (Advanced Vector Extensions) cho phép CPU cộng 16 cặp số (32-bit) trong đúng 1 chu kỳ máy.
2. **Branch Prediction (Dự đoán rẽ nhánh)**: CPU có một bộ phận chuyên đoán xem lệnh `if` sẽ rẽ đi đâu. Nếu đoán sai (Branch Misprediction), CPU phải vứt bỏ toàn bộ luồng xử lý và làm lại từ đầu. Kỹ thuật LeetCPU hướng dẫn ta hạn chế dùng `if/else` trong vòng lặp lớn (dùng bitwise operations thay thế).
3. **Loop Unrolling**: Tự động mở cuộn vòng lặp để giảm bớt chi phí kiểm tra điều kiện nhảy (jump condition) của CPU.

Để CPU chạy nhanh không kém GPU trong các tác vụ nhất định, bạn phải viết code sao cho trình biên dịch (Compiler) có thể tự động áp dụng các tối ưu này!

## 3C.6 — Viết Kernel GPU trực tiếp bằng Python

Đúng là PyTorch dùng C++ ở dưới nền, nhưng hiện tại bạn **hoàn toàn có thể viết code GPU (Kernel) bằng Python thuần túy**. 

Công cụ đình đám nhất hiện nay là **Triton** (do OpenAI phát triển). Triton cho phép bạn viết Python code, sau đó nó sẽ được JIT Compile (biên dịch theo thời gian thực) thẳng thành mã máy GPU mà không cần động đến một dòng C++ nào!

```python
import triton
import triton.language as tl

# Decorator này báo hiệu hàm này sẽ chạy trên GPU!
@triton.jit
def add_kernel(
    x_ptr,  # Con trỏ tới mảng X trên VRAM
    y_ptr,  # Con trỏ tới mảng Y trên VRAM
    output_ptr, # Con trỏ tới mảng Output
    n_elements,
    BLOCK_SIZE: tl.constexpr,
):
    # Lấy ID của thread hiện tại
    pid = tl.program_id(axis=0)
    
    # Tính toán vị trí bộ nhớ mà thread này chịu trách nhiệm
    block_start = pid * BLOCK_SIZE
    offsets = block_start + tl.arange(0, BLOCK_SIZE)
    mask = offsets < n_elements

    # Load dữ liệu từ VRAM, cộng song song, và lưu lại
    x = tl.load(x_ptr + offsets, mask=mask)
    y = tl.load(y_ptr + offsets, mask=mask)
    output = x + y
    tl.store(output_ptr + offsets, output, mask=mask)
```
Code Triton chạy nhanh ngang ngửa, thậm chí **nhanh hơn cả CUDA C++** do trình biên dịch của nó tự động tối ưu hóa bộ nhớ Cache (SRAM). Hầu hết các thư viện AI hiện đại như `xFormers` hay thuật toán `FlashAttention` đều đang chuyển sang viết bằng Triton (Python)!


## 3C.7 — Compute Bound vs Memory Bound

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

---

## ✅ Checkpoint 3C

1. Memory-bandwidth-bound và compute-bound khác nhau ra sao? Tại sao inference LLM thường rơi vào loại đầu?
2. Vì sao GPU nhanh hơn CPU cho phép nhân ma trận nhưng chậm hơn cho logic nhiều rẽ nhánh?
3. Cache L1 nhanh hơn RAM khoảng bao nhiêu lần, và điều đó ảnh hưởng gì tới cách bạn duyệt mảng?

<details>
<summary>Đáp án</summary>

1. Compute-bound = nghẽn ở số phép tính; bandwidth-bound = nghẽn ở việc chuyển dữ liệu vào/ra bộ nhớ. Sinh từng token của LLM phải đọc **toàn bộ** trọng số model cho **một** token — cực kỳ nặng về băng thông, rất nhẹ về tính toán. Đó chính là lý do batching giúp nhiều đến vậy.
2. GPU có hàng nghìn nhân đơn giản chạy theo lối SIMT — cùng một lệnh trên nhiều dữ liệu. Rẽ nhánh khiến các luồng trong một warp đi khác đường (divergence), buộc chúng chạy tuần tự và mất sạch lợi thế song song.
3. Khoảng 100 lần (≈1ns so với ≈100ns). Vì thế duyệt mảng **liên tục theo bộ nhớ** nhanh hơn nhiều so với đuổi con trỏ lung tung — cùng độ phức tạp Big-O nhưng khác hàng chục lần thời gian thực.
</details>

---

## 🏋️ Bài tập

**Bài 1 (10 phút).** Cộng hai mảng 10 triệu phần tử bằng vòng lặp Python thuần và bằng NumPy. Đo và giải thích khoảng cách.

**Bài 2 (15 phút).** Duyệt ma trận 2D theo hàng và theo cột. Đo chênh lệch thời gian và liên hệ với cache line.

**Bài 3 (15 phút).** Ước lượng: một model 7B ở fp16 cần bao nhiêu băng thông bộ nhớ để sinh 50 token/giây? So với băng thông thực tế của một GPU tiêu dùng.

<details>
<summary>Đáp án bài 3</summary>

7 tỷ tham số × 2 byte = **14 GB** phải đọc cho **mỗi** token. 50 token/giây ⇒
**700 GB/s**. Một RTX 4090 có khoảng 1.000 GB/s — vừa đủ, và điều đó cho thấy vì
sao con số này gần như hoàn toàn do băng thông quyết định, không phải do sức tính toán.
</details>

---

## 🔧 Troubleshooting

| Vấn đề | Vì sao xảy ra | Hướng xử lý |
|---|---|---|
| GPU dùng có 20% mà vẫn chậm | Bandwidth-bound, không phải compute-bound | Tăng batch size; dùng model đã quantize |
| Tăng batch không nhanh thêm | Đã chạm trần băng thông | Quantize, hoặc chuyển sang GPU có băng thông cao hơn |
| Vòng lặp NumPy vẫn chậm | Còn lặp ở tầng Python | Vector hoá; loại bỏ vòng `for` |
| CUDA OOM dù model vừa VRAM | KV cache tăng theo độ dài ngữ cảnh | Giới hạn `max_model_len`; dùng paged attention |

## Tóm tắt

- Viết Python giỏi là chưa đủ. Bạn cần hiểu dữ liệu đang nằm ở RAM hay VRAM, và chip nào đang xử lý.
- GPU thống trị AI nhờ hàng vạn lõi SIMD và kiến trúc Tensor Cores chuyên dùng để nhân ma trận.
- Điểm yếu "chí mạng" của quá trình chạy mô hình LLM (Inference) không phải là nó tính toán chậm, mà là băng thông bộ nhớ truyền dữ liệu lên chip không đủ (Memory Bound).

## 3C.8 — Thử thách tối ưu Hardware: LeetCPU & LeetGPU

Để thực sự nắm bắt giới hạn của phần cứng, bạn nên luyện tập các bài toán vi kiến trúc (micro-architecture) thông qua **LeetCPU** và **LeetGPU**. Thay vì viết thuật toán chuẩn O(N), mục tiêu là viết code chạy *nhanh nhất có thể trên máy tính*.

### LeetCPU: Tối ưu kiến trúc CPU
Khắc phục các điểm yếu về Pipeline và Cache:
- **[Easy] Stable Partition / Grade Bands**: Xóa bỏ Branch Prediction mispredicts bằng cách dùng Branchless code hoặc Lookup Tables.
- **[Medium] Matrix Multiply — Cache Tiling**: Cải thiện L1/L2 Cache Locality khi duyệt ma trận.
- **[Medium] Histogram / SAXPY**: Tối ưu ILP (Instruction-Level Parallelism) và bẻ gãy Write Dependency Chains.

### LeetGPU: Làm chủ CUDA/Triton
Làm quen với tính toán Tensor Core và xử lý siêu song song, đặc biệt dùng Triton trong Python:
- **[Easy] Vector Add & Relu**: Viết Kernel tính toán phép cộng/hàm kích hoạt song song cơ bản.
- **[Medium] GEMM (General Matrix Multiplication)**: Thuật toán kinh điển nhất trong AI. 
- **[Medium] RMS Normalization / Batched Matmul**: Các kernel hiện diện trong mọi Transformer model.
- **[Hard] Casual (Masked) Self-Attention / Multi-Head Attention**: Thử thách cốt lõi để thấu hiểu cách Attention layer thực sự hoạt động trên VRAM.

---

## Tiếp theo
Chúng ta đã đi qua toàn bộ phần nền tảng khoa học máy tính và phần cứng lõi. Kể từ Chapter 4 trở đi, bạn sẽ chính thức bước vào thế giới thiết kế phần mềm, học cách dùng Python với tư duy Hàm (Functional Programming).
