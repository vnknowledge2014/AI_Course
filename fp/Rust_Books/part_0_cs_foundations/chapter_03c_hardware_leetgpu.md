# Chapter 3C — Hardware, Compute: LeetCPU & LeetGPU

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



## 3C.5 — LeetCPU: Tối ưu hóa cực hạn trên CPU

Mặc dù GPU thống trị AI, CPU vẫn đảm nhận khâu tiền xử lý dữ liệu (Data Preprocessing, Tokenization, RAG chunking). Nền tảng `LeetCPU` dạy ta cách vắt kiệt sức mạnh của CPU thông qua:

1. **Vectorization (SIMD trên CPU)**: Thay vì cộng từng cặp số trong một mảng bằng vòng lặp `for`, các tập lệnh AVX-512 (Advanced Vector Extensions) cho phép CPU cộng 16 cặp số (32-bit) trong đúng 1 chu kỳ máy.
2. **Branch Prediction (Dự đoán rẽ nhánh)**: CPU có một bộ phận chuyên đoán xem lệnh `if` sẽ rẽ đi đâu. Nếu đoán sai (Branch Misprediction), CPU phải vứt bỏ toàn bộ luồng xử lý và làm lại từ đầu. Kỹ thuật LeetCPU hướng dẫn ta hạn chế dùng `if/else` trong vòng lặp lớn (dùng bitwise operations thay thế).
3. **Loop Unrolling**: Tự động mở cuộn vòng lặp để giảm bớt chi phí kiểm tra điều kiện nhảy (jump condition) của CPU.

Để CPU chạy nhanh không kém GPU trong các tác vụ nhất định, bạn phải viết code sao cho trình biên dịch (Compiler) có thể tự động áp dụng các tối ưu này!

## 3C.6 — Viết Kernel GPU trực tiếp bằng Rust

Một sai lầm phổ biến là nghĩ rằng lập trình GPU bắt buộc phải dùng C++. Thực tế, Rust có hệ sinh thái GPU cực kỳ mạnh mẽ nhờ sự an toàn bộ nhớ. Bạn có thể dùng `wgpu` (WebGPU chuẩn Rust) để chạy Compute Shaders ở bất cứ đâu (Mac, Windows, Linux) hoặc dùng `cudarc` để gọi CUDA trực tiếp.

```rust
// Ví dụ khái niệm dùng wgpu để chạy Compute Shader (WGSL)
// Khởi tạo thiết bị GPU
let adapter = instance.request_adapter(&Default::default()).await.unwrap();
let (device, queue) = adapter.request_device(&Default::default(), None).await.unwrap();

// Khai báo shader code chạy trực tiếp trên GPU
let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
    label: Some("Add Shader"),
    source: wgpu::ShaderSource::Wgsl(r#"
        @group(0) @binding(0) var<storage, read_write> data: array<i32>;
        
        @compute @workgroup_size(1)
        fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
            // Hàng ngàn thread sẽ chạy hàm này cùng lúc!
            data[global_id.x] = data[global_id.x] * 2;
        }
    "#.into()),
});
```
Rust giúp kiểm soát vòng đời của buffer bộ nhớ VRAM cực kỳ chặt chẽ, loại bỏ hoàn toàn lỗi Segmentation Faults thường thấy ở C++ CUDA.


## 3C.7 — Nút thắt cổ chai: Compute Bound vs Memory Bound

Đây là kiến thức quan trọng nhất khi triển khai mô hình LLM thực tế.

### 1. Compute Bound (Bị giới hạn bởi khả năng tính toán)
- **Định nghĩa**: Thời gian để GPU xử lý xong toán học **lâu hơn** thời gian để tải dữ liệu vào GPU.
- **Khi nào xảy ra**: Khi Train model, hoặc khi Inference ở giai đoạn "Prefill" (Khi bạn ném 1 prompt dài 10,000 từ vào mô hình để nó đọc lần đầu tiên).
- **Cách giải quyết**: Mua GPU xịn hơn (có nhiều FLOPS hơn), dùng quantization để giảm độ chính xác (FP32 -> FP16 -> INT8) nhằm tính toán nhanh hơn.

### 2. Memory Bandwidth Bound (Bị giới hạn bởi băng thông bộ nhớ)
- **Định nghĩa**: Toán học xử lý quá nhanh, GPU phải ngồi "chơi xơi nước" để chờ dữ liệu tải từ VRAM (Video RAM) sang chip xử lý. Băng thông (Bandwidth) không đủ lớn.
- **Khi nào xảy ra**: Khi model bắt đầu **Generate token từng chữ một** (Decoding phase). Mỗi lần sinh ra 1 chữ, nó phải tải TOÀN BỘ trọng số (Weights) của mô hình (hàng chục GB) từ VRAM qua chip xử lý. 
- **Cách giải quyết**: Tăng Batch Size (sinh ra cho 10 người dùng cùng lúc để tái sử dụng một lần nạp Weights), sử dụng **PagedAttention** hoặc FlashAttention để tối ưu hóa bộ nhớ KV Cache.

> Kỹ thuật **vLLM** mà chúng ta sẽ học ở phần Production (Chapter 43B) ra đời chính là để giải quyết bài toán Memory Bandwidth Bound này!

---

## Tóm tắt

- **Cache Locality**: Đặt dữ liệu liên tiếp trong RAM (`Vec`) giúp CPU đọc nhanh hơn.
- **CPU vs GPU**: CPU giỏi logic phức tạp, rẽ nhánh. GPU là ông hoàng nhân ma trận song song (SIMD).
- **VRAM Bandwidth**: Trong thế giới LLMs, nút thắt lớn nhất không phải là chip tính chậm, mà là đường ống bơm dữ liệu từ bộ nhớ VRAM vào chip không đủ to (Memory Bound ở Decoding Phase).

## 3C.8 — Thử thách tối ưu Hardware: LeetCPU & LeetGPU

Để thực sự nắm bắt giới hạn của phần cứng, bạn nên luyện tập các bài toán vi kiến trúc (micro-architecture) thông qua **LeetCPU** và **LeetGPU**. Thay vì viết thuật toán chuẩn O(N), mục tiêu là viết code chạy *nhanh nhất có thể trên máy tính*.

### LeetCPU: Tối ưu kiến trúc CPU
Khắc phục các điểm yếu về Pipeline và Cache (Rất phù hợp cho Rust compiler và `slice`):
- **[Easy] Stable Partition / Grade Bands**: Xóa bỏ Branch Prediction mispredicts bằng cách dùng Branchless code hoặc Lookup Tables.
- **[Medium] Matrix Multiply — Cache Tiling**: Cải thiện L1/L2 Cache Locality khi duyệt ma trận (vốn là thế mạnh tuyệt đối của Rust `Vec`).
- **[Medium] Histogram / SAXPY**: Tối ưu ILP (Instruction-Level Parallelism) và bẻ gãy Write Dependency Chains.

### LeetGPU: Làm chủ GPGPU bằng Rust
Sử dụng `wgpu` (WebGPU) hoặc `cudarc` (Rust bọc CUDA) để thực thi Compute Shaders:
- **[Easy] Vector Add & Relu**: Viết Compute Shader tính toán phép cộng/hàm kích hoạt song song.
- **[Medium] GEMM (General Matrix Multiplication)**: Thuật toán kinh điển nhất trong AI, tính trên WGSL hoặc PTX.
- **[Medium] RMS Normalization / Batched Matmul**: Các block hiện diện trong mọi Transformer model.
- **[Hard] Casual (Masked) Self-Attention**: Hiểu cặn kẽ Memory Bandwidth khi chạy Attention.

---

## Tiếp theo
Bạn đã có đủ hành trang CS Foundations (Toán, Thuật toán, Phần cứng). Bước tiếp theo, chúng ta sẽ bắt đầu học ngôn ngữ lập trình cụ thể để hiện thực hóa những kiến thức này (Part 1).

---

## ✅ Checkpoint 3C

1. Memory-bandwidth-bound khác compute-bound thế nào? Sinh token của LLM thuộc loại nào?
2. Vì sao rẽ nhánh làm GPU chậm nhưng gần như không ảnh hưởng CPU hiện đại?
3. Cache line thường là 64 byte. Điều đó ảnh hưởng gì tới cách bố trí struct?

<details>
<summary>Đáp án</summary>

1. Compute-bound nghẽn ở số phép tính; bandwidth-bound nghẽn ở việc chuyển dữ liệu. Sinh **một** token phải đọc **toàn bộ** trọng số model — cực nặng về băng thông, rất nhẹ về tính toán. Đó là lý do batching hiệu quả đến vậy: cùng một lần đọc trọng số phục vụ nhiều token.
2. GPU chạy theo SIMT: các luồng trong một warp thực thi cùng một lệnh. Rẽ nhánh khiến chúng phân kỳ và phải chạy tuần tự từng nhánh. CPU thì có branch predictor rất tốt, đoán đúng trên 95% nên gần như không mất gì.
3. Nên nhóm các field hay dùng cùng nhau vào cùng một cache line, và cân nhắc struct-of-arrays thay vì array-of-structs khi chỉ duyệt một field. Duyệt `Vec<f32>` nhanh hơn nhiều so với duyệt `Vec<StructBéo>` chỉ để lấy một field.
</details>

---

## 🏋️ Bài tập

**Bài 1 (10 phút).** Duyệt ma trận 2D theo hàng và theo cột trên mảng 4000×4000. Đo chênh lệch và giải thích bằng cache line.

**Bài 2 (15 phút).** So sánh Array-of-Structs và Struct-of-Arrays khi chỉ cần cộng một field. Đo trên 10 triệu phần tử ở bản `--release`.

**Bài 3 (15 phút).** Ước lượng: model 7B ở fp16 cần bao nhiêu băng thông để sinh 50 token/giây? So với băng thông thực tế của GPU bạn có.

<details>
<summary>Đáp án bài 3</summary>

7 tỷ × 2 byte = **14 GB** đọc cho **mỗi** token. 50 token/s ⇒ **700 GB/s**.
RTX 4090 có ~1.000 GB/s. Con số cho thấy rõ: đây là bài toán băng thông, không
phải bài toán sức tính. Vì thế quantize (giảm số byte mỗi tham số) hiệu quả hơn
hẳn so với mua GPU nhiều TFLOPS hơn.
</details>

---

## 🔧 Troubleshooting

| Vấn đề | Vì sao xảy ra | Hướng xử lý |
|---|---|---|
| Benchmark cho kết quả vô lý | Đang chạy debug build | Luôn `--release`; dùng `criterion` để đo tử tế |
| Compiler tối ưu mất cả vòng lặp benchmark | Kết quả không được dùng | `std::hint::black_box(...)` |
| SIMD không được sinh ra | Vòng lặp có rẽ nhánh hoặc phụ thuộc | Viết vòng lặp phẳng; kiểm tra bằng `cargo asm` |
| GPU dùng thấp mà vẫn chậm | Bandwidth-bound | Tăng batch; quantize model |
