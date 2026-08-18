# Chapter 3C — Hardware, Compute: LeetCPU & LeetGPU

> **Bạn sẽ học được**:
> - Phân cấp bộ nhớ: Từ Cache đến VRAM
> - Sự khác biệt giữa môi trường Node.js/V8 và GPU Compute
> - LeetCPU (SIMD, Branch Prediction)
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



## 3C.5 — LeetCPU: Tối ưu hóa cực hạn trên CPU

Mặc dù GPU thống trị AI, CPU vẫn đảm nhận khâu tiền xử lý dữ liệu (Data Preprocessing, Tokenization, RAG chunking). Nền tảng `LeetCPU` dạy ta cách vắt kiệt sức mạnh của CPU thông qua:

1. **Vectorization (SIMD trên CPU)**: Thay vì cộng từng cặp số trong một mảng bằng vòng lặp `for`, các tập lệnh AVX-512 (Advanced Vector Extensions) cho phép CPU cộng 16 cặp số (32-bit) trong đúng 1 chu kỳ máy.
2. **Branch Prediction (Dự đoán rẽ nhánh)**: CPU có một bộ phận chuyên đoán xem lệnh `if` sẽ rẽ đi đâu. Nếu đoán sai (Branch Misprediction), CPU phải vứt bỏ toàn bộ luồng xử lý và làm lại từ đầu. Kỹ thuật LeetCPU hướng dẫn ta hạn chế dùng `if/else` trong vòng lặp lớn (dùng bitwise operations thay thế).
3. **Loop Unrolling**: Tự động mở cuộn vòng lặp để giảm bớt chi phí kiểm tra điều kiện nhảy (jump condition) của CPU.

Để CPU chạy nhanh không kém GPU trong các tác vụ nhất định, bạn phải viết code sao cho trình biên dịch (Compiler) có thể tự động áp dụng các tối ưu này!

## 3C.6 — Viết Kernel GPU trực tiếp bằng TypeScript

Rất nhiều người lầm tưởng TypeScript/JavaScript chỉ chạy trên CPU thông qua V8 Engine. Nhưng với sự ra đời của **WebGPU** (đã có mặt trên Node.js và Trình duyệt), bạn hoàn toàn có thể viết Compute Shaders chạy trực tiếp trên card đồ họa bằng TypeScript!

```typescript
// Ví dụ khái niệm dùng WebGPU API trong TypeScript
async function runGPU() {
    const adapter = await navigator.gpu.requestAdapter();
    const device = await adapter.requestDevice();

    // Viết WGSL (WebGPU Shading Language) trực tiếp trong TS
    const shaderModule = device.createShaderModule({
        code: `
            @group(0) @binding(0) var<storage, read_write> output: array<f32>;

            @compute @workgroup_size(64)
            fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
                // Phép toán này sẽ chạy song song trên hàng ngàn lõi GPU!
                output[global_id.x] = output[global_id.x] * 2.0;
            }
        `
    });

    // ... Khởi tạo pipeline và buffers để chuyển data từ RAM sang VRAM ...
}
```
Nhờ WebGPU, TypeScript đang dần trở thành một thế lực mới trong việc triển khai suy luận AI (Inference) ngay tại thiết bị của người dùng (Edge/Browser AI) mà không cần server!


## 3C.7 — Compute Bound vs Memory Bound

Điều tối quan trọng khi triển khai LLM cho người dùng:

1. **Compute Bound**: Hệ thống bị thắt cổ chai ở năng lực tính toán.
   - Thường xảy ra khi người dùng gửi một đoạn text siêu dài (10,000 từ) và LLM cần đọc nó (Prefill phase). Nó phải nhân ma trận điên cuồng.

2. **Memory Bandwidth Bound**: Bị thắt cổ chai ở đường truyền dữ liệu.
   - Khi LLM sinh ra từng chữ (Decoding phase), để sinh ra 1 token, nó cần tải TOÀN BỘ weights (mấy chục GB) từ VRAM vào chip xử lý. Việc này đòi hỏi cáp băng thông cực khủng (hàng TB/s). Các chip tính toán lúc này rảnh rỗi chờ dữ liệu nạp lên.
   - Giải pháp: Batching (gộp nhiều user lại tính chung), PagedAttention (quản lý bộ nhớ VRAM thông minh).

---

---

## ✅ Checkpoint 3C

1. Memory-bandwidth-bound khác compute-bound thế nào? Sinh token của LLM thuộc loại nào?
2. Vì sao rẽ nhánh làm GPU chậm nhưng gần như không ảnh hưởng CPU?
3. JavaScript chạy trên V8 — bạn còn kiểm soát được bao nhiêu về cache locality?

<details>
<summary>Đáp án</summary>

1. Compute-bound nghẽn ở số phép tính; bandwidth-bound nghẽn ở việc chuyển dữ liệu. Sinh **một** token phải đọc **toàn bộ** trọng số model — cực nặng băng thông, rất nhẹ tính toán. Đó là lý do batching hiệu quả đến vậy.
2. GPU chạy SIMT: các luồng trong warp thực thi cùng lệnh; rẽ nhánh làm chúng phân kỳ và phải chạy tuần tự. CPU có branch predictor đoán đúng trên 95% nên gần như không mất gì.
3. Ít hơn nhiều so với C/Rust, nhưng không phải bằng không: `TypedArray` (`Float32Array`) cho bộ nhớ **liên tục** thay vì mảng con trỏ, và duyệt tuần tự vẫn nhanh hơn hẳn duyệt ngẫu nhiên. Đó là lý do thư viện tính toán JS luôn dùng TypedArray.
</details>

---

## 🏋️ Bài tập

**Bài 1 (10 phút).** Cộng hai mảng 10 triệu phần tử bằng `Array` thường và bằng `Float32Array`. Đo và giải thích khoảng cách.

**Bài 2 (15 phút).** Duyệt ma trận 2D theo hàng và theo cột. Đo chênh lệch và liên hệ với cache line.

**Bài 3 (15 phút).** Ước lượng: model 7B ở fp16 cần bao nhiêu băng thông để sinh 50 token/giây?

<details>
<summary>Đáp án bài 3</summary>

7 tỷ × 2 byte = **14 GB** đọc cho **mỗi** token. 50 token/s ⇒ **700 GB/s**. Một
RTX 4090 có ~1.000 GB/s. Con số cho thấy đây là bài toán băng thông chứ không phải
sức tính — nên quantize hiệu quả hơn nhiều so với mua GPU nhiều TFLOPS hơn.
</details>

---

## 🔧 Troubleshooting

| Vấn đề | Vì sao xảy ra | Hướng xử lý |
|---|---|---|
| Benchmark JS dao động mạnh | JIT chưa warm up, GC xen vào | Chạy nhiều vòng, bỏ vòng đầu; dùng `tinybench` |
| `Array` chậm hơn `TypedArray` nhiều lần | `Array` lưu con trỏ, không liên tục | Dùng `Float32Array`/`Int32Array` cho dữ liệu số |
| WebGPU không khả dụng | Trình duyệt chưa hỗ trợ | Kiểm tra `navigator.gpu` trước khi dùng |
| GPU dùng thấp mà vẫn chậm | Bandwidth-bound | Tăng batch; quantize model |

## Tóm tắt

- Node/TS tuyệt vời ở I/O và điều phối, nhưng cực yếu ở tính toán toán học dày đặc. Đó là lý do ta thường gọi API đến các Inference Servers (GPU).
- GPU mạnh mẽ nhờ kiến trúc SIMD xử lý song song hàng ngàn luồng.
- "Sát thủ" của LLM trên Production thường không phải tính toán chậm, mà là băng thông VRAM không tải kịp dữ liệu (Memory Bound).

## 3C.8 — Thử thách tối ưu Hardware: LeetCPU & LeetGPU

Để thực sự nắm bắt giới hạn của phần cứng, bạn nên luyện tập các bài toán vi kiến trúc thông qua **LeetCPU** và **LeetGPU**. 

### LeetCPU: Tối ưu V8 Engine
Node.js và V8 Engine có trình biên dịch JIT, nhưng bạn vẫn có thể giúp nó chạy nhanh hơn:
- **[Easy] Stable Partition / Grade Bands**: Tối ưu Branch Prediction (Dự đoán rẽ nhánh) khi duyệt mảng.
- **[Medium] Matrix Multiply — Cache Tiling**: Truy cập mảng 1D một cách có thứ tự bằng TypedArrays (`Float32Array`) để tận dụng L1/L2 Cache.
- **[Medium] SAXPY**: Hiểu cách V8 thực hiện vòng lặp và tối ưu ILP.

### LeetGPU: Trở thành chiến thần WebGPU
Với WebGPU API chuẩn bị bùng nổ, TypeScript/JS sẽ thành ngôn ngữ có thể chạy inference AI mạnh mẽ ngay trên trình duyệt:
- **[Easy] Vector Add & Relu**: Viết mã WGSL (WebGPU Shading Language) để tính toán song song trực tiếp từ trình duyệt.
- **[Medium] GEMM (General Matrix Multiplication)**: Thuật toán cốt lõi để chạy LLM cục bộ trên máy client.
- **[Medium] RMS Normalization / Batched Matmul**: Hiện thực hóa Transformer pipeline trên GPU qua TS.
- **[Hard] Multi-Head Attention**: Đỉnh cao của WebGPU Compute Shader để chạy LLaMA trực tiếp trên Chrome.

---

## Tiếp theo
Khóa nền tảng CS của bạn đã hoàn chỉnh! Từ bài sau, ta sẽ bắt đầu tìm hiểu sâu về cú pháp và sức mạnh thực sự của TypeScript Type System.
