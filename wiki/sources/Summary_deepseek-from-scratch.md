# Tóm tắt: deepseek-from-scratch

Đây là trang tóm tắt tự động cho repository `deepseek-from-scratch`.

**Bản trích xuất tự động từ README:**

```text
# DeepSeek from Scratch

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Python 3.10+](https://img.shields.io/badge/python-3.10+-blue.svg)](https://www.python.org/downloads/)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)

Educational implementations of **DeepSeek-V3.2** and **DeepSeek-R1** architectures in **Rust** (using Candle) and **Python** (using PyTorch/MLX).

This repository provides from-scratch implementations of the key innovations that make DeepSeek models state-of-the-art:

### 🧠 Attention Mechanisms
- **Multi-Query Attention (MQA)** - Single KV head for memory-efficient inference
- **Grouped-Query Attention (GQA)** - Balanced KV sharing across head groups
- **Multi-Head Latent Attention (MLA)** - Compressed KV cache for efficient inference
- **DeepSeek Sparse Attention (DSA)** - Hybrid local + dilated global attention patterns

### 🔀 Mixture of Experts
- **Standard MoE** - Top-k expert routing with load balancing
- **DeepSeek MoE** - Fine-grained experts with shared expert isolation
- **256-Expert MoE** - Hierarchical routing for massive expert scaling

### 🎯 Prediction & Quantization
- **Multi-Token Prediction (MTP)** - Predict multiple future tokens simultaneously
- **FP8 Mixed-Precision** - Low-precision training with dynamic scaling
- **FP8 Quantization** - Simulated 8-bit inference for deployment

### 🏋️ Training & Alignment
- **GRPO Training** - Group Relative Policy Optimizat
...
```

*Nguồn gốc: [repos/deepseek-from-scratch](../../repos/deepseek-from-scratch)*
