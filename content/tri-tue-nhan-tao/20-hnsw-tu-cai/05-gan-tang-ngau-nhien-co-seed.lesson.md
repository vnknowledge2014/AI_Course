---
id: tri-tue-nhan-tao.hnsw-tu-cai.gan-tang-ngau-nhien-co-seed
title: "Gán tầng ngẫu nhiên có seed: tầng 0 đông, tầng cao thưa dần"
summary: "gan_tang_ngau_nhien(seed, n_diem, xac_suat) dung random.Random(seed) (seed CO DINH, tat dinh): moi diem tung xu (xac suat=0,5) lien tuc, dung lai o lan dau tien KHONG qua nguong -- tang cang cao cang it diem toi duoc (xac suat toi tang t la 0,5^t). Voi seed=29, N=20: phan bo SO DIEM MOI TANG (tang t chua moi diem co tang gan >= t) la [20, 12, 4, 2, 2] -- giam dan tu tang 0 (ca 20 diem) len tang 4 (chi 2 diem), dung 1 seed co dinh, lap lai duoc y het moi lan chay."
locale: vi
track: tri-tue-nhan-tao
module: hnsw-tu-cai
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 9
teaches: [ai.gan-tang-ngau-nhien-co-seed]
requires: [ai.gioi-han-mot-tang-va-y-tuong-nhieu-tang]
concepts: [ai.gan-tang-ngau-nhien-co-seed]
gradingMatrix:
  web-chrome: [static, run, tests, output]
  web-firefox: [static, run, tests, output]
  macos: [static, run, tests, output]
  windows: [static, run, tests, output]
  linux: [static, run, tests, output]
  android: [static, run, tests, output]
  ios: [static, run, tests, output]
provenance:
  authoredBy: llm-assisted
  reviewed: false
---

::::byte{trigger=enter mood=curious pose=lean-in}
Bài trước đóng lại bằng một ý tưởng: nhiều tầng, tầng trên thưa, tầng dưới
đông. Nhưng "thưa" thế nào? Bài này gán CHO MỖI điểm một tầng — ngẫu nhiên,
nhưng tất định.
::::

::::explain{#gan_tang_ngau_nhien}
HNSW quyết định điểm nào xuất hiện ở tầng nào bằng một phép NGẪU NHIÊN CÓ
KIỂM SOÁT: mỗi điểm, khi được chèn, "tung xu" liên tiếp (xác suất `p` mỗi
lần) — CÒN "trúng" (dưới ngưỡng `p`) thì tăng tầng lên `1` và tung tiếp;
"trượt" (đạt ngưỡng `p` trở lên) thì DỪNG LẠI, và tầng hiện tại là "tầng cao
nhất" của điểm đó (điểm này xuất hiện ở MỌI tầng từ `0` tới tầng đó).

Kết quả là một **phân phối giảm dần theo cấp số nhân**: xác suất một điểm
đạt tới tầng `≥ 1` là `p`, tới tầng `≥ 2` là `p²`, tới tầng `≥ t` là `pᵗ` —
càng lên cao, càng ít điểm "sống sót" tới đó. Với `p = 0,5`: khoảng MỘT NỬA
số điểm dừng lại ở tầng `0`, một phần tư dừng ở tầng `≤1`... nhưng đây là kỳ
vọng XÁC SUẤT, không phải một con số cố định — số liệu THẬT trên một lần
chạy cụ thể có thể lệch đi ít nhiều, và bài này đo con số THẬT đó.

**Bắt buộc**: mọi phép ngẫu nhiên ở đây PHẢI qua `random.Random(seed)` với
`seed` CỐ ĐỊNH, KHÔNG dùng module `random` toàn cục không seed — để kết quả
TẤT ĐỊNH, lặp lại y hệt ở mọi lần chạy, trên mọi máy (cùng một `seed` luôn
cho cùng một phân bố tầng).
::::

::::example{#do_phan_bo_tang}
Gán tầng cho `20` điểm, `seed=29`, xác suất `p=0,5`:

```python title=readonly
import random
from collections import Counter


def gan_tang_ngau_nhien(seed, n_diem, xac_suat):
    rng = random.Random(seed)
    tang = []
    for i in range(n_diem):
        t = 0
        while rng.random() < xac_suat:
            t += 1
        tang.append(t)
    return tang


def so_diem_moi_tang(tang_diem, tang_toi_da):
    return [sum(1 for t in tang_diem if t >= tang) for tang in range(tang_toi_da + 1)]


N = 20
tang_diem = gan_tang_ngau_nhien(29, N, 0.5)
dem = Counter(tang_diem)
tang_toi_da = max(tang_diem)
so_luong_theo_tang = so_diem_moi_tang(tang_diem, tang_toi_da)

print(tang_diem)
print(dict(sorted(dem.items())))
print(tang_toi_da)
print(so_luong_theo_tang)
```

```text title=readonly
[0, 1, 1, 2, 4, 1, 1, 4, 0, 1, 1, 2, 0, 0, 0, 0, 0, 1, 0, 1]
{0: 8, 1: 8, 2: 2, 4: 2}
4
[20, 12, 4, 2, 2]
```

`tang_toi_da = 4` — với `seed=29`, không điểm nào tung xu "trúng" quá `4`
lần liên tiếp. `so_luong_theo_tang[t]` đếm SỐ ĐIỂM CÓ MẶT tại tầng `t` (tầng
gán của chúng `≥ t`) — tầng `0` LUÔN có đủ cả `20` điểm (mọi điểm đều xuất
hiện ở tầng `0`), tầng `1` còn `12` điểm, tầng `2` còn `4`, tầng `3` VÀ tầng
`4` đều còn `2` điểm (ở lần chạy CỤ THỂ này, không điểm nào dừng lại ĐÚNG ở
tầng `3` — cả `2` điểm cao nhất đều "nhảy thẳng" từ tầng `2` lên tầng `4`,
một kết quả THẬT của phép ngẫu nhiên, không phải lỗi). Xu hướng chung RÕ:
tầng `0` đông nhất (`20`), giảm dần khi lên cao (`12 → 4 → 2 → 2`) — không
bao giờ TĂNG khi lên tầng cao hơn.
::::

::::predict{#doan_tang_0_dong_nhat commitOnce}
Xét đúng ví dụ trên: `N=20` điểm, `seed=29`, `xac_suat=0,5`.

**Trước khi chạy thử**, bạn đoán: `so_luong_theo_tang[0]` (số điểm có mặt ở
tầng `0`) so với `so_luong_theo_tang[4]` (số điểm có mặt ở tầng cao nhất) —
cái nào LỚN HƠN?

:::opt{correct}
`so_luong_theo_tang[0]` lớn hơn HẲN (`20` so với `2`) — MỌI điểm đều xuất
hiện ở tầng `0` (không có điều kiện nào loại một điểm khỏi tầng `0`), còn
lên tới tầng cao thì phải "sống sót" qua nhiều lần tung xu liên tiếp, càng
lên cao càng ít điểm làm được điều đó
:::

:::opt
Bằng nhau — vì `gan_tang_ngau_nhien` dùng CÙNG một xác suất `0,5` cho mọi
lần tung xu, nên số điểm ở mỗi tầng phải xấp xỉ đều nhau
::why
Gần đúng ở việc xác suất MỖI LẦN tung xu là CỐ ĐỊNH (`0,5`), không đổi theo
tầng — quan sát đó đúng.

Chỗ lệch: xác suất "mỗi lần tung" cố định không có nghĩa là số điểm SỐNG
SÓT tới một tầng cao là như nhau — để tới tầng `4`, một điểm phải "trúng"
LIÊN TIẾP `4` lần (xác suất `0,5⁴ = 0,0625`, khoảng `6%`), trong khi để ở
tầng `0` không cần trúng lần nào (`100%`). Xác suất giảm theo CẤP SỐ NHÂN
khi tầng tăng, không phải đều nhau.
::
:::

:::opt
`so_luong_theo_tang[4]` lớn hơn — vì những điểm "may mắn" tới được tầng cao
là những điểm ĐẶC BIỆT (láng giềng gần trung tâm dữ liệu hơn), nên tầng cao
thường tập trung nhiều điểm quan trọng hơn
::why
Gần đúng ở trực giác "tầng cao chứa điểm quan trọng hơn" — một ý tưởng có
liên quan tới VAI TRÒ của tầng cao trong HNSW (giúp nhảy xa nhanh).

Chỗ lệch: `gan_tang_ngau_nhien` KHÔNG biết gì về vị trí hay "tầm quan
trọng" của một điểm trong không gian vector — nó chỉ tung xu THUẦN NGẪU
NHIÊN, độc lập với nội dung điểm đó. Việc một điểm tới được tầng cao hoàn
toàn do MAY MẮN trong chuỗi tung xu, không do đặc điểm dữ liệu — và càng
lên cao, số điểm may mắn như vậy càng ÍT, không nhiều hơn tầng `0`.
::
:::
::::

::::code{#viet_gan_tang_ngau_nhien}
Hoàn thiện `gan_tang_ngau_nhien`: dùng `random.Random(seed)` (KHÔNG dùng
`random` toàn cục), và dừng tung xu ở lần đầu tiên KHÔNG đạt ngưỡng
`xac_suat`.

```python title=starter
import random
from collections import Counter


def gan_tang_ngau_nhien(seed, n_diem, xac_suat):
    rng = random.___(seed)                       # Random
    tang = []
    for i in range(n_diem):
        t = 0
        while rng.random() ___ xac_suat:          # <
            t += 1
        tang.append(t)
    return tang


def so_diem_moi_tang(tang_diem, tang_toi_da):
    return [sum(1 for t in tang_diem if t >= tang) for tang in range(tang_toi_da + 1)]


N = 20
tang_diem = gan_tang_ngau_nhien(29, N, 0.5)
dem = Counter(tang_diem)
tang_toi_da = max(tang_diem)
so_luong_theo_tang = so_diem_moi_tang(tang_diem, tang_toi_da)

print(tang_diem)
print(dict(sorted(dem.items())))
print(tang_toi_da)
print(so_luong_theo_tang)
```

```python title=solution
import random
from collections import Counter


def gan_tang_ngau_nhien(seed, n_diem, xac_suat):
    rng = random.Random(seed)
    tang = []
    for i in range(n_diem):
        t = 0
        while rng.random() < xac_suat:
            t += 1
        tang.append(t)
    return tang


def so_diem_moi_tang(tang_diem, tang_toi_da):
    return [sum(1 for t in tang_diem if t >= tang) for tang in range(tang_toi_da + 1)]


N = 20
tang_diem = gan_tang_ngau_nhien(29, N, 0.5)
dem = Counter(tang_diem)
tang_toi_da = max(tang_diem)
so_luong_theo_tang = so_diem_moi_tang(tang_diem, tang_toi_da)

print(tang_diem)
print(dict(sorted(dem.items())))
print(tang_toi_da)
print(so_luong_theo_tang)
```

```python title=test
assert tang_diem == [0, 1, 1, 2, 4, 1, 1, 4, 0, 1, 1, 2, 0, 0, 0, 0, 0, 1, 0, 1], f"tang_diem sai -- dang ra {tang_diem}"
assert tang_toi_da == 4, f"tang cao nhat phai la 4 -- dang ra {tang_toi_da}"
assert so_luong_theo_tang == [20, 12, 4, 2, 2], f"phan bo so diem moi tang sai -- dang ra {so_luong_theo_tang}"
assert so_luong_theo_tang[0] == N, "tang 0 phai chua DU ca N diem"
assert all(so_luong_theo_tang[t] >= so_luong_theo_tang[t + 1] for t in range(len(so_luong_theo_tang) - 1)), "so diem moi tang khong duoc TANG khi len tang cao hon"

# kiem tra tat dinh: CUNG mot seed phai cho CUNG mot ket qua, hai lan chay doc lap
assert gan_tang_ngau_nhien(29, 20, 0.5) == gan_tang_ngau_nhien(29, 20, 0.5), "cung seed phai cho DUNG cung ket qua moi lan goi"

# kiem tra tren mot vi du nho, tu tinh tay duoc: xac_suat=0.0 -- KHONG lan tung nao trung, moi diem o tang 0
assert gan_tang_ngau_nhien(1, 5, 0.0) == [0, 0, 0, 0, 0], "xac_suat=0.0 nghia la KHONG bao gio 'trung' (0.0 < 0.0 la False ngay lan dau) -- moi diem o tang 0"

# bien: seed KHAC nhau (cung tham so con lai) phai cho ket qua co the KHAC nhau
assert gan_tang_ngau_nhien(29, 20, 0.5) != gan_tang_ngau_nhien(0, 20, 0.5), "hai seed khac nhau (29 va 0) phai cho hai phan bo tang KHAC nhau tren du lieu nay"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai dòng khác nhau. Chỗ đầu là TÊN LỚP tạo bộ sinh số ngẫu nhiên CÓ SEED, gọi qua `random.___`. Chỗ hai là toán tử SO SÁNH điều khiển vòng lặp `while` — tiếp tục tăng tầng khi số ngẫu nhiên vừa sinh ra CÒN NHỎ HƠN `xac_suat`.
- kind: strategy
  body: 'Chỗ đầu: `Random` — `random.Random(seed)`, KHÔNG phải `random.random` hay `random.seed`. Chỗ hai: `<` — `rng.random() < xac_suat`.'
- kind: one-line
  body: 'Chỗ đầu là `Random`, chỗ hai là `<`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai GOI random.Random(seed) (lop tao bo sinh so ngau nhien CO SEED, khong duoc dung random toan cuc khong seed); VA cho trong hai phai SO SANH NGHIEM NGAT bang toan tu '<' giua rng.random() va xac_suat
  requireAst:
  - kind: uses-call, target: Random, min: 1
  - kind: uses-operator, target: "<", min: 1
  # Da thu that (dung ban sao _dem trich tu kiem-ast.ts, chay qua python3 TREN
  # CHINH van ban solution da trich tu file nay) -- ket qua [1, 1] cho hai
  # luat theo dung thu tu khai bao o tren. Khong co lan goi 'Random' hay
  # toan tu '<' nao khac trong toan bo solution (chi co hai ham ngan gon,
  # khong boilerplate them) -- ca hai nguong deu la TONG THAT, khong co rui
  # ro "boilerplate-threshold-masking" o day.
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant va CHAY THAT qua
  # python3 de xac nhan, khong doan tay): dien "<" vao cho trong dau
  # ("rng = random.<(seed)") VA dien "Random" vao cho trong hai ("while
  # rng.random() Random xac_suat:") -- CA HAI deu SAI CU PHAP NGAY LAP TUC
  # (khong the dung mot toan tu lam ten thuoc tinh sau dau cham, khong the
  # dung mot ten (khong phai tu khoa toan tu) lam toan tu so sanh giua hai
  # bieu thuc) -- da tu chay qua python3, xac nhan nem SyntaxError ngay o
  # buoc phan tich cu phap, TRUOC CA khi kiemAst() hay 'run' kip chay. Rui
  # ro hoan doi o day khong co thuc, da xac nhan bang chay that chu khong
  # doan.
  # Da tu ra soat gotcha bien (#10): mutant dung '<=' thay vi '<' se KHONG
  # bi bat boi static (khac toan tu, uses-operator '<' se dem 0, duoi
  # nguong min=1 -- VAN bi chan, vi Lt va LtE la hai lop AST khac nhau).
  # Rieng ve HANH VI: xac suat rng.random() tra ve DUNG BANG 0.5 tuyet doi
  # gan nhu khong xay ra voi bo sinh so thuc lien tuc, nen '<=' co the cho
  # ket qua GIONG HET '<' tren du lieu nay -- nhung dieu do KHONG quan trong,
  # vi static da chan mutant nay TRUOC ca khi chay toi output, doc lap voi
  # hanh vi runtime co trung hop hay khong.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[0, 1, 1, 2, 4, 1, 1, 4, 0, 1, 1, 2, 0, 0, 0, 0, 0, 1, 0, 1\\]\\n\\{0: 8, 1: 8, 2: 2, 4: 2\\}\\n4\\n\\[20, 12, 4, 2, 2\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tầng `0` có đủ cả `20` điểm, tầng `4` chỉ còn `2` — giảm dần, tất định, lặp
lại được. Bài sau ráp phép gán tầng này vào thuật toán CHÈN nhiều tầng đầy
đủ.
::::

::::reflect{#nghi-lai}
`gan_tang_ngau_nhien` không cần một công thức phức tạp — chỉ là một vòng
`while` tung xu liên tiếp, dừng lại ở lần đầu tiên "trượt". Chính cơ chế
đơn giản đó tạo ra đúng tính chất cần cho HNSW: PHẦN LỚN điểm dừng ở tầng
thấp (đủ để tầng `0` chứa MỌI điểm, dùng để tinh chỉnh kết quả), một số ít
điểm "may mắn" trèo lên tầng cao (dùng để nhảy xa nhanh). Điều bắt buộc
không được quên: `seed` CỐ ĐỊNH qua `random.Random(seed)` — không có nó,
mỗi lần chạy cho một đồ thị khác nhau, không thể đối chiếu hay gỡ lỗi được.
Hai bài tiếp theo dùng ĐÚNG hàm này (không viết lại) để chèn và tìm kiếm
trên một đồ thị nhiều tầng đầy đủ.
::::

::::checkpoint{mastery=0.85}
::::
