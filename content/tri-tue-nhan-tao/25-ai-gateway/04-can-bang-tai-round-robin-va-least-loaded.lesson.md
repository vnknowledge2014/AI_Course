---
id: tri-tue-nhan-tao.ai-gateway.can-bang-tai-round-robin-va-least-loaded
title: "Cân bằng tải: round-robin có thể chọn nhầm node quá tải, least-loaded thì không"
summary: "round_robin(danh_sach_node, chi_so_hien_tai) = (chi_so_hien_tai + 1) % len(danh_sach_node) -- luon chon NODE KE TIEP theo vong, quay lai dau khi het danh sach, KHONG can biet tai hien tai. least_loaded(danh_sach_tai) tra ve CHI SO cua phan tu NHO NHAT (node it tai nhat), giu chi so DAU TIEN khi hoa. Tren DANH_SACH_TAI=[5,2,8,2] (khong deu), voi chi_so_hien_tai=1: round_robin tra ve chi so 2 (tai=8, node DANG QUA TAI NHAT), trong khi least_loaded LUON tra ve chi so 1 (tai=2, node NHE NHAT) bat ke chi_so_hien_tai la gi -- round_robin_chon_qua_tai = (8 > 2) = True."
locale: vi
track: tri-tue-nhan-tao
module: ai-gateway
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.can-bang-tai-round-robin-va-least-loaded]
requires: [ai.xac-thuc-api-key-tra-bang]
concepts: [ai.can-bang-tai-round-robin-va-least-loaded]
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
Request đã qua xác thực VÀ còn đủ token. Nhưng `Chương 39.3` liệt kê MỘT
tầng `Inference` không phải LÀ một GPU duy nhất — nó LÀ nhiều "node GPU"
đứng song song. Cổng phải chọn: request này đi tới NODE NÀO?
::::

::::explain{#hai_chien_luoc_chon_node}
**Load Balancing** — cân bằng tải giữa nhiều node GPU — có nhiều chiến
lược; bài này xét HAI chiến lược đối lập nhau về mức độ "biết thông tin":

1. **Round-robin** — chọn node KẾ TIẾP theo vòng, không quan tâm node đó
   đang bận hay rảnh. Dùng phép chia lấy dư (`%`) để quay lại đầu danh sách
   khi đã hết:

```
round_robin(danh_sach_node, chi_so_hien_tai) = (chi_so_hien_tai + 1) % len(danh_sach_node)
```

2. **Least-loaded** — chọn node có TẢI HIỆN TẠI thấp nhất, tức PHẢI biết
   trạng thái của mọi node trước khi quyết định:

```
least_loaded(danh_sach_tai) = chi_so cua phan tu NHO NHAT trong danh_sach_tai
```

Round-robin đơn giản hơn HẲN — không cần đo tải của bất kỳ node nào, chỉ
cần nhớ CHỈ SỐ đã dùng lần trước. Nhưng chính vì KHÔNG nhìn vào tải, nó có
thể chọn nhầm một node ĐANG QUÁ TẢI, chỉ vì tới lượt nó theo vòng. Đó
CHÍNH LÀ điểm bài này đo bằng số cụ thể.
::::

::::example{#danh_sach_tai_khong_deu}
```python title=readonly
def round_robin(danh_sach_node, chi_so_hien_tai):
    return (chi_so_hien_tai + 1) % len(danh_sach_node)


def least_loaded(danh_sach_tai):
    chi_so_nhe_nhat = 0
    for i in range(1, len(danh_sach_tai)):
        if danh_sach_tai[i] < danh_sach_tai[chi_so_nhe_nhat]:
            chi_so_nhe_nhat = i
    return chi_so_nhe_nhat


DANH_SACH_NODE = ["gpu-0", "gpu-1", "gpu-2", "gpu-3"]
DANH_SACH_TAI = [5, 2, 8, 2]

idx_round_robin = round_robin(DANH_SACH_NODE, 1)
idx_least_loaded = least_loaded(DANH_SACH_TAI)

node_round_robin = DANH_SACH_NODE[idx_round_robin]
node_least_loaded = DANH_SACH_NODE[idx_least_loaded]

tai_cua_round_robin = DANH_SACH_TAI[idx_round_robin]
tai_cua_least_loaded = DANH_SACH_TAI[idx_least_loaded]

round_robin_chon_qua_tai = tai_cua_round_robin > tai_cua_least_loaded

print(idx_round_robin, node_round_robin, tai_cua_round_robin)
print(idx_least_loaded, node_least_loaded, tai_cua_least_loaded)
print(round_robin_chon_qua_tai)
```

```text title=readonly
2 gpu-2 8
1 gpu-1 2
True
```

Bốn node có tải HIỆN TẠI lần lượt `5, 2, 8, 2` — KHÔNG đều. Giả sử request
TRƯỚC vừa đi tới node chỉ số `1` (`chi_so_hien_tai = 1`). Round-robin:
`1 + 1 = 2`, rồi `2 % 4 = 2` — chọn ĐÚNG node chỉ số `2`, tải `8` — node
ĐANG QUÁ TẢI NHẤT trong cả bốn! Least-loaded: quét cả danh sách, tìm tải
nhỏ nhất — node chỉ số `1` (tải `2`, NHẸ NHẤT), KHÔNG quan tâm request
trước tới node nào. `round_robin_chon_qua_tai` (`8 > 2`) LÀ `True` — trên
danh sách tải cụ thể này, round-robin chọn PHẢI đúng node nặng nhất, trong
khi least-loaded luôn tìm được đúng node nhẹ nhất.
::::

::::predict{#doan_round_robin_quay_vong commitOnce}
Xét ĐÚNG `DANH_SACH_NODE = ["gpu-0", "gpu-1", "gpu-2", "gpu-3"]` (`4` node),
nhưng `chi_so_hien_tai = 3` — node CUỐI CÙNG (`gpu-3`) vừa được dùng.

**Trước khi chạy thử**, bạn đoán: `round_robin(DANH_SACH_NODE, 3)` trả về
chỉ số nào?

:::opt{correct}
`0` — `3 + 1 = 4`, rồi `4 % 4 = 0`; phép chia lấy dư quay CHỈ SỐ về ĐẦU danh
sách (`gpu-0`) khi vừa vượt qua node CUỐI CÙNG, đúng ý nghĩa "quay vòng"
:::

:::opt
`4` — chỉ cần cộng thêm `1` vào chỉ số hiện tại (`3 + 1 = 4`), không cần
chia lấy dư gì cả
::why
Gần đúng ở phép cộng `3 + 1 = 4` — bản thân phép cộng đó không sai.

Chỗ lệch: `4` KHÔNG PHẢI một chỉ số hợp lệ của `DANH_SACH_NODE` — danh sách
chỉ có `4` phần tử, chỉ số hợp lệ chạy từ `0` tới `3`. Bỏ qua phép `% 4` là
bỏ qua chính cơ chế "quay lại đầu khi hết danh sách" mà round-robin cần —
dùng chỉ số `4` để truy cập `DANH_SACH_NODE[4]` sẽ ném `IndexError`, vì
phần tử đó không tồn tại.
::
:::

:::opt
Chương trình sẽ báo lỗi, vì `chi_so_hien_tai = 3` đã LÀ chỉ số cuối cùng —
không còn node nào "kế tiếp" để chọn
::why
Gần đúng ở việc `3` THẬT SỰ là chỉ số CUỐI CÙNG của `DANH_SACH_NODE`
(`4` phần tử, chỉ số `0` đến `3`) — quan sát về vị trí đó đúng.

Chỗ lệch: round-robin được thiết kế CHÍNH XÁC để xử lý trường hợp này —
"hết danh sách" không phải một lỗi, mà LÀ tín hiệu quay VỀ ĐẦU. Phép chia
lấy dư (`%`) làm đúng việc đó: `4 % 4 = 0`, không NỔ, không dừng lại — nó
tiếp tục vòng lặp một cách êm thấm.
::
:::
::::

::::code{#viet_round_robin_va_least_loaded}
Hoàn thiện `round_robin` (chỉ số kế tiếp, quay vòng bằng `%`) và
`least_loaded` (điều kiện cập nhật chỉ số nhẹ nhất khi gặp một tải NHỎ HƠN).

```python title=starter
def round_robin(danh_sach_node, chi_so_hien_tai):
    return ___                                              # (chi_so_hien_tai + 1) % len(danh_sach_node)


def least_loaded(danh_sach_tai):
    chi_so_nhe_nhat = 0
    for i in range(1, len(danh_sach_tai)):
        if ___:                                             # danh_sach_tai[i] < danh_sach_tai[chi_so_nhe_nhat]
            chi_so_nhe_nhat = i
    return chi_so_nhe_nhat


DANH_SACH_NODE = ["gpu-0", "gpu-1", "gpu-2", "gpu-3"]
DANH_SACH_TAI = [5, 2, 8, 2]

idx_round_robin = round_robin(DANH_SACH_NODE, 1)
idx_least_loaded = least_loaded(DANH_SACH_TAI)

node_round_robin = DANH_SACH_NODE[idx_round_robin]
node_least_loaded = DANH_SACH_NODE[idx_least_loaded]

tai_cua_round_robin = DANH_SACH_TAI[idx_round_robin]
tai_cua_least_loaded = DANH_SACH_TAI[idx_least_loaded]

round_robin_chon_qua_tai = tai_cua_round_robin > tai_cua_least_loaded

print(idx_round_robin, node_round_robin, tai_cua_round_robin)
print(idx_least_loaded, node_least_loaded, tai_cua_least_loaded)
print(round_robin_chon_qua_tai)
```

```python title=solution
def round_robin(danh_sach_node, chi_so_hien_tai):
    return (chi_so_hien_tai + 1) % len(danh_sach_node)


def least_loaded(danh_sach_tai):
    chi_so_nhe_nhat = 0
    for i in range(1, len(danh_sach_tai)):
        if danh_sach_tai[i] < danh_sach_tai[chi_so_nhe_nhat]:
            chi_so_nhe_nhat = i
    return chi_so_nhe_nhat


DANH_SACH_NODE = ["gpu-0", "gpu-1", "gpu-2", "gpu-3"]
DANH_SACH_TAI = [5, 2, 8, 2]

idx_round_robin = round_robin(DANH_SACH_NODE, 1)
idx_least_loaded = least_loaded(DANH_SACH_TAI)

node_round_robin = DANH_SACH_NODE[idx_round_robin]
node_least_loaded = DANH_SACH_NODE[idx_least_loaded]

tai_cua_round_robin = DANH_SACH_TAI[idx_round_robin]
tai_cua_least_loaded = DANH_SACH_TAI[idx_least_loaded]

round_robin_chon_qua_tai = tai_cua_round_robin > tai_cua_least_loaded

print(idx_round_robin, node_round_robin, tai_cua_round_robin)
print(idx_least_loaded, node_least_loaded, tai_cua_least_loaded)
print(round_robin_chon_qua_tai)
```

```python title=test
assert round_robin(DANH_SACH_NODE, 1) == 2, f"chi_so_hien_tai=1 tren 4 node -- phai la 2 -- dang ra {round_robin(DANH_SACH_NODE, 1)}"
assert round_robin(DANH_SACH_NODE, 3) == 0, f"chi_so_hien_tai=3 (node cuoi) phai QUAY VE 0 -- dang ra {round_robin(DANH_SACH_NODE, 3)}"
assert round_robin(DANH_SACH_NODE, 0) == 1, f"chi_so_hien_tai=0 phai la 1 -- dang ra {round_robin(DANH_SACH_NODE, 0)}"
assert round_robin(["a", "b"], 1) == 0, f"chi voi 2 node, chi_so_hien_tai=1 phai QUAY VE 0 -- xac nhan do dai danh sach THAT SU rang buoc ket qua, dang ra {round_robin(['a', 'b'], 1)}"

assert least_loaded([5, 2, 8, 2]) == 1, f"tai khong deu [5,2,8,2] -- node nhe nhat la chi so 1 (tai 2) -- dang ra {least_loaded([5, 2, 8, 2])}"
assert least_loaded([2, 2, 2]) == 0, f"tai deu nhau het -- phai giu chi so DAU TIEN (0) -- dang ra {least_loaded([2, 2, 2])}"
assert least_loaded([9, 1]) == 1, f"tai [9,1] -- node nhe nhat la chi so 1 -- dang ra {least_loaded([9, 1])}"
assert least_loaded([1, 9]) == 0, f"tai [1,9] -- node nhe nhat la chi so 0 -- dang ra {least_loaded([1, 9])}"

assert idx_round_robin == 2, f"idx_round_robin (demo) phai la 2 -- dang ra {idx_round_robin}"
assert idx_least_loaded == 1, f"idx_least_loaded (demo) phai la 1 -- dang ra {idx_least_loaded}"
assert round_robin_chon_qua_tai is True, f"tren DANH_SACH_TAI nay, round-robin PHAI chon dung node qua tai hon least-loaded -- dang ra {round_robin_chon_qua_tai}"
```

:::hints
- kind: attention
  body: "Hai cho trong, o hai ham khac nhau. Cho dau (trong round_robin) la GIA TRI TRA VE -- chi so KE TIEP, quay vong bang phep chia lay du (%) voi DO DAI danh sach. Cho hai (trong least_loaded) la DIEU KIEN cua if, ben trong vong lap -- so sanh tai cua node hien tai (chi so i) voi tai cua node nhe nhat DA TIM DUOC cho toi gio (chi so chi_so_nhe_nhat), dung toan tu <."
- kind: strategy
  body: "Cho dau: (chi_so_hien_tai + 1) % len(danh_sach_node) -- cong 1 roi lay phan du cho do dai danh sach, de tu dong quay ve 0 khi vuot qua chi so cuoi. Cho hai: danh_sach_tai[i] < danh_sach_tai[chi_so_nhe_nhat] -- neu tai node i NHO HON tai node nhe nhat da luu, cap nhat chi_so_nhe_nhat = i (dong ngay duoi, da co san)."
- kind: one-line
  body: "Cho dau la (chi_so_hien_tai + 1) % len(danh_sach_node), cho hai la danh_sach_tai[i] < danh_sach_tai[chi_so_nhe_nhat]."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: cho trong dau phai dung phep chia lay du % voi len(danh_sach_node) (khong duoc bo qua phep quay vong); cho trong hai phai la phep so sanh < giua danh_sach_tai[i] va danh_sach_tai[chi_so_nhe_nhat]
  requireAst:
  - kind: uses-operator, target: "%", min: 1
  - kind: uses-operator, target: "<", min: 1
  # Da CHAY THAT qua kiemAst() (node + pyodide, trich solution tu chinh
  # file nay) -- ket qua dung du kien:
  #   "%"=1: DUY NHAT o cho trong dau ((chi_so_hien_tai + 1) %
  #   len(danh_sach_node)). Ham least_loaded khong dung % o dau ca.
  #   "<"=1: DUY NHAT o cho trong hai (danh_sach_tai[i] <
  #   danh_sach_tai[chi_so_nhe_nhat]). Ham round_robin khong dung < o dau
  #   ca; vong lap for dung range(1, ...) (mot cuoc goi ham, khong phai
  #   toan tu so sanh).
  # Dien bua "True" vao CA HAI cho trong ("return True" va "if True:") cho
  # "%"=0 VA "<"=0 -- CA HAI luat CHAN DUNG (da CHAY THAT xac nhan qua
  # kiemAst).
  #
  # GOTCHA "hoan doi ca cum" (da tu dung mutant hoan doi CA HAI cho trong --
  # xac dinh ranh gioi tu chinh khoi starter -- va CHAY THAT qua kiemAst va
  # python3): dien "danh_sach_tai[i] < danh_sach_tai[chi_so_nhe_nhat]" vao
  # cho trong dau (dong "return danh_sach_tai[i] <
  # danh_sach_tai[chi_so_nhe_nhat]" trong round_robin) VA dien
  # "(chi_so_hien_tai + 1) % len(danh_sach_node)" vao cho trong hai (dong
  # "if (chi_so_hien_tai + 1) % len(danh_sach_node):" trong least_loaded)
  # -- tong so lan "%" VA "<" tren TOAN BO solution KHONG DOI (van la 1 va
  # 1, chi doi VI TRI) -- da CHAY THAT xac nhan qua kiemAst: static KHONG
  # bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run' -- CA HAI HUONG deu nem NameError doc
  # lap: (a) ben trong round_robin (tham so la danh_sach_node,
  # chi_so_hien_tai -- KHONG co bien "danh_sach_tai", "i", hay
  # "chi_so_nhe_nhat" nao trong scope nay), bieu thuc moi
  # "danh_sach_tai[i] < danh_sach_tai[chi_so_nhe_nhat]" dung BA ten CHUA HE
  # TON TAI -- NameError ngay khi round_robin(DANH_SACH_NODE, 1) duoc goi
  # (dong dau tien cua phan demo); (b) ben trong least_loaded (tham so la
  # danh_sach_tai -- KHONG co bien "chi_so_hien_tai" hay "danh_sach_node"
  # nao trong scope nay), bieu thuc moi "(chi_so_hien_tai + 1) %
  # len(danh_sach_node)" dung HAI ten CHUA HE TON TAI. Da tu chay THAT qua
  # python3 CA HAI huong rieng biet: xac nhan "name 'danh_sach_tai' is not
  # defined" (goi round_robin truoc) va "name 'chi_so_hien_tai' is not
  # defined" (goi least_loaded truoc) -- bi chan boi tier 'run' theo CA HAI
  # duong, doc lap voi static.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^2 gpu-2 8\\n1 gpu-1 2\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Round-robin chọn `gpu-2` (tải `8`, nặng nhất); least-loaded LUÔN chọn
`gpu-1` (tải `2`, nhẹ nhất) — trên CÙNG một danh sách tải. Bốn mảnh đã
xong: đếm đúng theo token, xô token thực thi giới hạn, xác thực trước tiên,
định tuyến biết chọn node nhẹ. BOSS quý ráp cả bốn thành MỘT cổng AI hoàn
chỉnh.
::::

::::reflect{#nghi-lai}
Round-robin không phải một chiến lược TỆ — nó đơn giản HƠN HẲN
least-loaded (không cần đo tải của bất kỳ node nào, chỉ cần nhớ MỘT con số:
chỉ số đã dùng lần trước), và trong một hệ thống nơi mọi request tốn CÙNG
một lượng công việc, nó phân phối tải đều hệt như least-loaded. Cái nó
KHÔNG làm được LÀ phản ứng với tải KHÔNG ĐỀU — trên `DANH_SACH_TAI` của bài
này (`[5, 2, 8, 2]`), nó chọn ĐÚNG node nặng nhất chỉ vì tới lượt theo
vòng. Đánh đổi này — đơn giản hơn ĐỔI LẤY khả năng chọn nhầm node quá tải
— LÀ lý do các hệ thống production thật thường dùng least-loaded (hoặc các
biến thể phức tạp hơn) khi tải giữa các request không đồng đều. BOSS quý
tiếp theo ráp CẢ BỐN bài của `q8.6b` thành một cổng AI hoàn chỉnh: xác
thực, giới hạn theo token, định tuyến, VÀ ghi phí.
::::

::::checkpoint{mastery=0.78}
::::
