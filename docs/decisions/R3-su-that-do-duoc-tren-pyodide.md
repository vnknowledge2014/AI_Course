# R3.T3.1 — những con số phải ĐO, không được nhớ

Đo ngày 2026-08-30 trên đúng runtime của khoá: Pyodide 0.29.4 → **CPython
3.13.2**, biên dịch cho **WASM 32-bit**. Mọi bài của T3.1 phải dùng những con
số dưới đây, không dùng con số trong sách.

## Hai chỗ sách giáo khoa nói khác runtime này

### 1. `x = 1000; y = 1000; x is y` cho `True`, không phải `False`

Bài học kinh điển về "small int cache" dạy rằng Python dùng chung đối tượng
cho số nguyên nhỏ (−5..256) nên `256 is 256` đúng, còn `1000 is 1000` sai.

Đo thật: **cả hai đều `True`.** Lý do không phải cache số nhỏ mà là CPython
gộp hằng số trùng nhau trong CÙNG một code object. Hai dòng `x = 1000` và
`y = 1000` nằm trong cùng một khối mã nên chỉ có một hằng số 1000.

> **Hệ quả cho người viết bài: KHÔNG dạy `is` như cách phân biệt "cùng một
> vật" cho số nguyên.** Trên runtime này nó cho ra kết quả làm người học rút
> ra một luật sai. Dạy `is` bằng `list` — nơi `a is b` và `a is c` khác nhau
> thật (đo được: aliasing `True`, sau `a[:]` là `False`) — đúng như R1 đã làm
> với `core.list-aliasing`.

### 2. `sys.getsizeof` ở đây là số của máy 32-bit

| biểu thức | đo trên Pyodide | sách thường ghi (CPython 64-bit) |
|---|---|---|
| `sys.getsizeof(0)` | **16** | 28 |
| `sys.getsizeof(2**30)` | **20** | 32 |
| `sys.getsizeof(2**300)` | **56** | 68 |
| `sys.getsizeof([])` | **28** | 56 |
| `sys.getsizeof([1,2,3])` | **44** | 80 |
| `sys.getsizeof('')` | **21** | 49 |
| `sys.getsizeof('a')` | **22** | 50 |
| `sys.getsizeof('ở')` | **32** | 76 |

Chép con số từ sách vào bài là dạy một điều mà máy của người học nói khác —
đúng chế độ hỏng mà cả dự án này lấy làm kẻ thù.

Cách viết an toàn: đừng ghim con số tuyệt đối trong `expect`. Ghim **quan hệ**
— `getsizeof(2**300) > getsizeof(2**30) > getsizeof(0)` — vì quan hệ ấy đúng
trên mọi bản, còn con số thì không.

## Những con số dùng được, đã đo

- `(255).bit_length()` → `8`
- `bin(255)` → `'0b11111111'` · `hex(255)` → `'0xff'` · `0xff == 255` → `True`
- `-5 & 0xff` → `251` → `0b11111011` (bù hai trong một byte)
- `int.from_bytes(b'\xff\x00', 'big')` → `65280`
- `int.from_bytes(b'\xff\x00', 'little')` → `255`
- `0.1 + 0.2` → `0.30000000000000004`, và `== 0.3` là `False`
- `(0.1).hex()` → `'0x1.999999999999ap-4'`
- `'a'.encode('utf-8')` → 1 byte · `'ở'.encode('utf-8')` → **3 byte**,
  `[225, 187, 159]`

Con số cuối nối thẳng về R0.T1 bài 6: bảng 128 dòng đủ cho tiếng Anh, không đủ
cho tiếng Việt. Ở đó người học mới được nghe; ở T3.1 họ đếm được.
