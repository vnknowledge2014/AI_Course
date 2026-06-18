# Chapter 3C — Hardware, Compute & LeetGPU Basics

> **Bạn sẽ học được**:
> - Memory Hierarchy (L1, L2, RAM, VRAM)
> - CPU vs GPU Architecture
> - CUDA & Tensor Cores
> - Memory Bandwidth Bound vs Compute Bound
>
> **Yêu cầu trước**: Chapter 3B
> **Thời gian đọc**: ~30 phút | **Level**: Advanced

---

## 3C.1 — Memory Hierarchy

```text
CPU Registers (Nhanh nhất, cực nhỏ)
  ↓
L1/L2/L3 Cache (Rất nhanh, vài MB)
  ↓
RAM (Chậm hơn, hàng chục GB)
  ↓
Disk (Rất chậm, hàng TB)
```
TypeScript cho phép ta sắp xếp dữ liệu liên tục trong RAM (`Array<T>` thay vì `LinkedList<T>`) để tận dụng **Cache Locality**.

## 3C.2 — CPU vs GPU trong AI

- **CPU**: Vài chục Cores rất mạnh. Giỏi xử lý Logic phức tạp (If/Else, Branching). Giống như xe đua F1.
- **GPU**: Hàng ngàn Cores nhỏ. Giỏi tính toán ma trận song song (SIMD). Giống như xe tải chở ngàn món hàng cùng lúc.
- Trong AI (đặc biệt là Transformers), nhân ma trận (MatMul) chiếm 99% thời gian -> GPU là bắt buộc.

## 3C.3 — Tensor Cores & VRAM

Trong `LeetGPU` và lập trình CUDA, ta quan tâm:
- **Compute Bound**: Tính toán tốn nhiều thời gian hơn nạp dữ liệu. (Cần tối ưu phép tính, dùng Tensor Cores).
- **Memory Bandwidth Bound**: Tính thì nhanh nhưng nạp dữ liệu từ VRAM vào GPU quá chậm. (Cần tối ưu KV Cache, FlashAttention).

## Tóm tắt
Để hệ thống AI chạy nhanh, không chỉ cần Big-O tốt, mà phải hiểu code chạy trên phần cứng nào.
