# R3.T3.4 — những sự thật đã đo trước khi viết

Đo ngày 2026-08-31 trên đúng runtime của khoá: Pyodide 0.29.4 → **CPython
3.13.2**, biên dịch cho **WASM 32-bit** (cùng runtime T3.1 đã đo, chưa đổi).

## 1. `dis` module CHẠY ĐƯỢC trên Pyodide — đã xác nhận, không phải đoán

```python
import dis
def cong(a, b):
    return a + b
dis.dis(cong)
```
cho ra:
```
  5           RESUME                   0

  6           LOAD_FAST_LOAD_FAST      1 (a, b)
              BINARY_OP                0 (+)
              RETURN_VALUE
```

Track này DÙNG ĐƯỢC `dis.dis()` thật trong sandbox — không cần giả lập.

## 2. Bytecode CPython 3.13 có lệnh GỘP — sách cũ nói khác

Textbook cũ (CPython ≤3.10) dạy bytecode "cổ điển": `LOAD_FAST a`,
`LOAD_FAST b`, `BINARY_ADD` — ba lệnh tách rời. CPython 3.11+ có trình
thông dịch "chuyên biệt hoá thích ứng" (adaptive specializing interpreter),
và 3.13 gộp hai `LOAD_FAST` liền nhau thành **một** lệnh `LOAD_FAST_LOAD_FAST`
khi cả hai đọc từ biến cục bộ. Đếm SỐ LỆNH theo sách cũ (3 lệnh) sẽ SAI trên
runtime này (2 lệnh: `LOAD_FAST_LOAD_FAST`, `BINARY_OP`).

> **Hệ quả cho người viết bài**: đừng ghim SỐ LỆNH tuyệt đối đọc được từ
> sách. Ghim những gì đo được thật bằng `dis.dis()` trên chính runtime này,
> giống cách T3.1 đã làm với `sys.getsizeof`.

## 3. `dis.dis()` của một vòng `for` — có nhãn nhảy tường minh

```python
def dem_toi(n):
    tong = 0
    for i in range(n):
        tong += i
    return tong
```
cho ra (rút gọn):
```
  6           LOAD_GLOBAL   1 (range + NULL)
              LOAD_FAST     0 (n)
              CALL          1
              GET_ITER
      L1:     FOR_ITER      7 (to L2)
              STORE_FAST    2 (i)

  7           LOAD_FAST_LOAD_FAST  18 (tong, i)
              BINARY_OP     13 (+=)
              STORE_FAST    1 (tong)
              JUMP_BACKWARD 9 (to L1)

  6   L2:     END_FOR
              POP_TOP
```

Nhãn `L1:`/`L2:` là định dạng dis.dis() phiên bản 3.13 — RẤT hợp cho bài dạy
"vòng lặp là nhảy lùi": chỉ cần chỉ vào `JUMP_BACKWARD 9 (to L1)` là thấy
ngay đường quay lại đầu vòng, không cần suy diễn từ số offset thô như bản
dis cũ.

## 4. KHÔNG có hiệu ứng "chuyên biệt hoá" lộ ra qua `dis.dis()` mặc định

Đã đo: gọi cùng một hàm 2000 lần rồi `dis.dis()` lại — output GIỐNG HỆT lúc
đầu (`same_before_after: true`). CPython 3.11+ có cơ chế "quickening" nội bộ
(PEP 659) làm bytecode tự thay đổi sau nhiều lần gọi, nhưng cơ chế đó KHÔNG
lộ ra qua lời gọi `dis.dis()` mặc định (cần cờ khác để thấy, và hành vi này
không ổn định giữa các bản CPython).

> **Hệ quả cho người viết bài**: KHÔNG dạy hiện tượng "chuyên biệt hoá" —
> track này không cần tới nó, và dạy sai một cơ chế phức tạp còn tệ hơn
> không dạy. `dis.dis()` trong track này luôn cho MỘT kết quả ổn định, xem
> được lặp lại được.

## 5. `sys.platform` là `"emscripten"`, không phải `"linux"`/`"darwin"`

Một sự thật nhỏ nhưng đáng nói: ngay cả TÊN hệ điều hành mà Python "nghĩ"
mình đang chạy trên đó cũng khác — sandbox không giả vờ là Linux thật.

## 6. `sys.maxsize` xác nhận lại 32-bit — khớp T3.1

`sys.maxsize == 2147483647` (= 2³¹−1). CPython 64-bit thật cho
`9223372036854775807` (= 2⁶³−1). Cùng sự thật T3.1 đã đo qua `getsizeof`,
đo lại bằng một con đường khác — nhất quán.

## 7. `time.perf_counter()` có độ phân giải nano giây

`time.get_clock_info('perf_counter').resolution == 1e-09`. Dùng được cho
bài đo thời gian THẬT (không chỉ đếm bước như T3.3), nếu track cần đối
chiếu "đếm lệnh nhiều hơn" với "chạy lâu hơn thật".

## Quyết định sư phạm: "ngăn xếp"/"đống" Ở HAI TẦNG NGHĨA — phải phân biệt

T3.2 dạy `ds.stack`/`ds.heap` như CẤU TRÚC DỮ LIỆU (một `list` bạn tự quản
bằng `.append`/`.pop`). Track này phải giới thiệu **ngăn xếp gọi hàm** (call
stack — vùng bộ nhớ tiến trình giữ khung mỗi lời gọi, do interpreter tự
quản) và **đống cấp phát** (heap memory — vùng bộ nhớ chứa mọi object Python
sống, do bộ quản lý bộ nhớ tự quản). BỐN khái niệm, hai cặp trùng TÊN, khác
NGHĨA:

| Tên gọi | T3.2 dạy (cấu trúc dữ liệu) | T3.4 dạy (vùng bộ nhớ) |
|---|---|---|
| "ngăn xếp" | một `list` bạn tự đẩy/lấy bằng tay | vùng bộ nhớ giữ khung gọi hàm, interpreter tự quản |
| "đống" | cây nhị phân đặc biệt, cài trong mảng | vùng bộ nhớ chứa MỌI object Python đang sống |

Đây KHÔNG phải trùng tên tình cờ — cả hai cặp đều "vào sau ra trước" (ngăn
xếp) hoặc "không theo thứ tự cố định, cấp phát theo nhu cầu" (đống) ở tinh
thần chung, nên tên gọi mượn đúng chỗ. Nhưng người học ĐÃ học nghĩa thứ nhất
ở T3.2 trước, nên track này PHẢI nói thẳng ra sự trùng tên này ở bài mở đầu
cụm cuối, không được lờ đi rồi hy vọng người học tự suy ra.
