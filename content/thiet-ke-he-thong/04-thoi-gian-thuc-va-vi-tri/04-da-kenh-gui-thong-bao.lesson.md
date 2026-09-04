---
id: thiet-ke-he-thong.thoi-gian-thuc-va-vi-tri.da-kenh-gui-thong-bao
title: "Đa kênh gửi thông báo: thử tiếp KHI kênh trước thất bại"
summary: "guiThongBaoDaKenh thu theo THU TU uu tien (push, email, sms), THU kenh ke tiep CHI khi kenh truoc that bai -- push loi thi thu email, email thanh cong thi DUNG NGAY (sms KHONG duoc thu). Khong kenh nao loi thi CHI thu dung 1 kenh dau tien roi dung, khong lang phi thu them kenh con lai du chung van con trong danh sach."
locale: vi
track: thiet-ke-he-thong
module: thoi-gian-thuc-va-vi-tri
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [sd.da-kenh-gui-thong-bao]
requires: [sd.xac-nhan-da-doc]
concepts: [sd.da-kenh-gui-thong-bao]
gradingMatrix:
  web-chrome: [run, tests, output]
  web-firefox: [run, tests, output]
  macos: [run, tests, output]
  windows: [run, tests, output]
  linux: [run, tests, output]
  android: [run, tests, output]
  ios: [run, tests, output]
provenance:
  authoredBy: llm-assisted
  reviewed: false
---

::::byte{trigger=enter mood=curious pose=lean-in}
Ba bài đầu xong phần Chat. Mảnh tiếp theo — Notification — bắt đầu từ một
sự thật khác: ứng dụng của người dùng không LUÔN mở. Push có thể tới
thiết bị đang tắt mạng. Cần một kế hoạch DỰ PHÒNG khi kênh đầu tiên chịu
thua.
::::

::::explain{#thu-tuan-tu-theo-uu-tien}
Một hệ thống thông báo THẬT không gửi qua đúng MỘT kênh cố định — nó có
một DANH SÁCH kênh theo thứ tự ưu tiên (push trước, rồi email, rồi SMS),
VÀ chỉ chuyển sang kênh kế tiếp khi kênh hiện tại THẤT bại. Ngay khi một
kênh THÀNH công, vòng thử DỪNG lại:

```typescript title=readonly
type Kenh = "push" | "email" | "sms";

function guiQuaKenh(kenh: Kenh, dsKenhLoi: Kenh[]): boolean {
  return !dsKenhLoi.includes(kenh);
}

interface KetQuaGuiDaKenh { kenhThanhCong: Kenh | undefined; dsDaThu: Kenh[]; }

function guiThongBaoDaKenh(dsUuTien: Kenh[], dsKenhLoi: Kenh[]): KetQuaGuiDaKenh {
  const dsDaThu: Kenh[] = [];
  for (const kenh of dsUuTien) {
    dsDaThu.push(kenh);
    if (guiQuaKenh(kenh, dsKenhLoi)) {
      return { kenhThanhCong: kenh, dsDaThu };
    }
  }
  return { kenhThanhCong: undefined, dsDaThu };
}

const dsUuTien: Kenh[] = ["push", "email", "sms"];
const ketQua = guiThongBaoDaKenh(dsUuTien, ["push"]);
console.log("kenh thanh cong:", ketQua.kenhThanhCong);
console.log("cac kenh da thu (dung thu tu):", ketQua.dsDaThu);
```

```text title=readonly
kenh thanh cong: email
cac kenh da thu (dung thu tu): [ 'push', 'email' ]
```

`dsKenhLoi = ["push"]` mô phỏng kênh push đang GẶP sự cố. Vòng lặp thử
`push` trước — thất bại (nằm TRONG `dsKenhLoi`) — nên chuyển sang `email`
— thành công. `dsDaThu` chỉ ghi lại ĐÚNG hai kênh: `push` VÀ `email`.
`sms` không hề xuất hiện trong danh sách đã thử, vì vòng lặp đã DỪNG
ngay khi `email` thành công — nó không bao giờ chạy tới lượt `sms`.
::::

::::example{#thanh-cong-ngay-va-that-bai-toan-bo}
Hai trường hợp biên: kênh ĐẦU tiên thành công NGAY (dừng sau đúng một lần
thử), VÀ cả BA kênh đều thất bại (thử hết, không kênh nào thành công):

```typescript title=readonly
type Kenh = "push" | "email" | "sms";

function guiQuaKenh(kenh: Kenh, dsKenhLoi: Kenh[]): boolean {
  return !dsKenhLoi.includes(kenh);
}

interface KetQuaGuiDaKenh { kenhThanhCong: Kenh | undefined; dsDaThu: Kenh[]; }

function guiThongBaoDaKenh(dsUuTien: Kenh[], dsKenhLoi: Kenh[]): KetQuaGuiDaKenh {
  const dsDaThu: Kenh[] = [];
  for (const kenh of dsUuTien) {
    dsDaThu.push(kenh);
    if (guiQuaKenh(kenh, dsKenhLoi)) {
      return { kenhThanhCong: kenh, dsDaThu };
    }
  }
  return { kenhThanhCong: undefined, dsDaThu };
}

const dsUuTien: Kenh[] = ["push", "email", "sms"];

const ketQuaThanhCongNgay = guiThongBaoDaKenh(dsUuTien, []);
console.log("khong kenh nao loi -> kenh thanh cong:", ketQuaThanhCongNgay.kenhThanhCong);
console.log("khong kenh nao loi -> cac kenh da thu:", ketQuaThanhCongNgay.dsDaThu);

const ketQuaCaBaLoi = guiThongBaoDaKenh(dsUuTien, ["push", "email", "sms"]);
console.log("ca ba kenh deu loi -> kenh thanh cong:", ketQuaCaBaLoi.kenhThanhCong);
console.log("ca ba kenh deu loi -> cac kenh da thu:", ketQuaCaBaLoi.dsDaThu);
```

```text title=readonly
khong kenh nao loi -> kenh thanh cong: push
khong kenh nao loi -> cac kenh da thu: [ 'push' ]
ca ba kenh deu loi -> kenh thanh cong: undefined
ca ba kenh deu loi -> cac kenh da thu: [ 'push', 'email', 'sms' ]
```

Khi `push` thành công NGAY, `dsDaThu` chỉ CÓ đúng một phần tử — hệ thống
không hề "thử cho chắc" thêm `email` hay `sms` dù chúng vẫn nằm trong
`dsUuTien`. Khi cả ba đều thất bại, `dsDaThu` mới liệt kê ĐỦ cả ba, VÀ
`kenhThanhCong` trả về `undefined` — không có kênh nào thắng cuộc.
::::

::::predict{#doan-dung-ngay-khong-thu-them commitOnce}
`dsUuTien = ["push", "email", "sms"]`, VÀ không kênh nào bị liệt trong
`dsKenhLoi` (mảng rỗng — `push` chắc chắn thành công ở lượt thử ĐẦU
tiên). `sms` có xuất hiện trong `dsDaThu` của kết quả trả về KHÔNG?

:::opt{correct}
Không — `push` thành công NGAY ở lượt thử đầu tiên, vòng lặp `return`
ngay lập tức, nên `email` VÀ `sms` không bao giờ được thêm vào `dsDaThu`
:::
:::opt
Có — hệ thống vẫn thử ĐỦ cả ba kênh trong `dsUuTien` để ghi log đầy đủ,
dù kênh đầu tiên đã thành công
::why
Nhầm "duyệt hết danh sách để ghi log" VỚI hành vi THẬT của
`guiThongBaoDaKenh` — hàm này KHÔNG có mục tiêu ghi log đầy đủ, nó dừng
NGAY khi tìm được một kênh thắng.

Chỗ lệch: bên trong vòng `for`, câu lệnh `return { kenhThanhCong: kenh,
dsDaThu }` thoát khỏi HÀM ngay khi `guiQuaKenh(kenh, dsKenhLoi)` trả về
`true` — vòng lặp không có cơ hội chạy tới `email` hay `sms` nữa, dù
chúng vẫn còn TRONG mảng `dsUuTien` chưa duyệt tới.
::
:::
::::

::::code{#viet_gui_thong_bao_da_kenh}
Hoàn thiện `guiThongBaoDaKenh` — bên TRONG vòng lặp, sau khi ghi nhận
kênh đang thử VÀO `dsDaThu`, kiểm tra kênh đó có gửi thành công hay
không; nếu CÓ thì trả về NGAY, dừng vòng lặp.

```typescript title=starter
type Kenh = "push" | "email" | "sms";

function guiQuaKenh(kenh: Kenh, dsKenhLoi: Kenh[]): boolean {
  return !dsKenhLoi.includes(kenh);
}

interface KetQuaGuiDaKenh { kenhThanhCong: Kenh | undefined; dsDaThu: Kenh[]; }

function guiThongBaoDaKenh(dsUuTien: Kenh[], dsKenhLoi: Kenh[]): KetQuaGuiDaKenh {
  const dsDaThu: Kenh[] = [];
  for (const kenh of dsUuTien) {
    dsDaThu.push(kenh);
    ___
  }
  return { kenhThanhCong: undefined, dsDaThu };
}

const dsX: Kenh[] = ["email", "sms", "push"];
console.log(guiThongBaoDaKenh(dsX, ["email", "sms"]).kenhThanhCong);
```

```typescript title=solution
type Kenh = "push" | "email" | "sms";

function guiQuaKenh(kenh: Kenh, dsKenhLoi: Kenh[]): boolean {
  return !dsKenhLoi.includes(kenh);
}

interface KetQuaGuiDaKenh { kenhThanhCong: Kenh | undefined; dsDaThu: Kenh[]; }

function guiThongBaoDaKenh(dsUuTien: Kenh[], dsKenhLoi: Kenh[]): KetQuaGuiDaKenh {
  const dsDaThu: Kenh[] = [];
  for (const kenh of dsUuTien) {
    dsDaThu.push(kenh);
    if (guiQuaKenh(kenh, dsKenhLoi)) {
      return { kenhThanhCong: kenh, dsDaThu };
    }
  }
  return { kenhThanhCong: undefined, dsDaThu };
}

const dsX: Kenh[] = ["email", "sms", "push"];
console.log(guiThongBaoDaKenh(dsX, ["email", "sms"]).kenhThanhCong);
```

```typescript title=test
const uuTien: Kenh[] = ["push", "email", "sms"];

const kq1 = guiThongBaoDaKenh(uuTien, ["push"]);
if (kq1.kenhThanhCong !== "email") throw new Error("push loi, email phai la kenh thanh cong");
if (kq1.dsDaThu.join(",") !== "push,email") throw new Error("phai dung thu ('sms') KHONG duoc thu vi email da thanh cong");

const kq2 = guiThongBaoDaKenh(uuTien, []);
if (kq2.kenhThanhCong !== "push") throw new Error("khong kenh nao loi, push (dau tien) phai thanh cong");
if (kq2.dsDaThu.join(",") !== "push") throw new Error("chi duoc thu DUNG 1 kenh (push) khi no thanh cong ngay");

const kq3 = guiThongBaoDaKenh(uuTien, ["push", "email", "sms"]);
if (kq3.kenhThanhCong !== undefined) throw new Error("ca ba kenh deu loi, khong co kenh nao thanh cong");
if (kq3.dsDaThu.join(",") !== "push,email,sms") throw new Error("ca ba kenh deu loi thi phai thu DU CA BA theo dung thu tu");

const kq4 = guiThongBaoDaKenh(["email", "sms", "push"], ["email", "sms"]);
if (kq4.kenhThanhCong !== "push") throw new Error("thu tu uu tien email,sms,push voi email+sms loi thi push phai thanh cong");
if (kq4.dsDaThu.join(",") !== "email,sms,push") throw new Error("phai thu dung ca ba kenh theo thu tu uu tien truyen vao");
```

:::hints
- kind: attention
  body: "Sau khi push kenh dang xet vao dsDaThu, kiem tra guiQuaKenh(kenh, dsKenhLoi) -- neu tra ve true (thanh cong) thi return NGAY ca object ket qua, dung khong cho vong lap chay tiep."
- kind: strategy
  body: "if (guiQuaKenh(kenh, dsKenhLoi)) { return { kenhThanhCong: kenh, dsDaThu }; } -- return ben trong vong lap chinh la thu ngat vong lap som."
- kind: one-line
  body: "if (guiQuaKenh(kenh, dsKenhLoi)) { return { kenhThanhCong: kenh, dsDaThu }; }"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 5000
- tier: output
  match: contains
  expect: "push"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Thử tuần tự, dừng NGAY khi thắng — một quy tắc đơn giản, đủ để tránh gửi
thừa. Mảnh kế tiếp: khi MƯỜI sự kiện dồn về trong một nhịp ngắn, đừng gửi
mười thông báo riêng lẻ.
::::

::::reflect{#nghi-lai}
`guiThongBaoDaKenh` không hề PHỨC tạp — một vòng lặp VÀ một điều kiện
dừng sớm. Điều đáng nhớ LÀ chỗ đặt `return`: NGAY bên trong vòng lặp,
ngay khi có kênh thắng — không đợi duyệt hết mảng rồi mới quyết định.
Dừng sớm chính LÀ điều khiến hệ thống không lãng phí một lần gửi nào.
::::

::::checkpoint{mastery=0.72}
::::
