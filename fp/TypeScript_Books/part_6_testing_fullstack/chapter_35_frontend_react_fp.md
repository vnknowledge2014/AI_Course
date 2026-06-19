# Chapter 35 — Frontend: React + FP

> **Bạn sẽ học được**:
> - Nhìn nhận React Component dưới lăng kính FP: `f(props) = JSX`.
> - Quản lý State bằng `useReducer` kết hợp với Discriminated Unions (State Machine).
> - Chấm dứt thảm họa `isLoading`, `error`, `data` bằng kiểu dữ liệu `RemoteData`.
> - Tách biệt hoàn toàn Logic (Core) và Giao diện (Shell) trong React.
>
> **Yêu cầu trước**: Chapter 13 (ADTs), Chapter 20 (Discriminated Unions).
> **Thời gian đọc**: ~45 phút | **Level**: Intermediate-Advanced
> **Kết quả cuối cùng**: Code React không bao giờ rơi vào các trạng thái vô lý (impossible states), dễ dàng viết Unit Test mà không cần render DOM.

---

Bạn biết nghệ sĩ gấp giấy Origami chứ? Họ nhận một tờ giấy (Props), gấp theo CÙNG một cách (Function), và cho ra CÙNG một hình thù (JSX). 
Cho cùng tờ giấy màu đỏ -> ra con hạc đỏ. Hình dạng con hạc được quyết định bởi CÁCH GẤP, chứ không phải do tờ giấy tự đổi màu.

React được THIẾT KẾ theo triết lý Functional Programming. Cha đẻ của Redux và thành viên cốt cán của React Core Team (Dan Abramov) từng nói: *"React components should be pure functions of props"*.

## 35.1 — React Components as Pure Functions

Pure function là hàm không có Side Effects và luôn trả về cùng kết quả với cùng đầu vào.

```tsx
// ❌ IMPURE: Component này đọc thời gian thực (Side effect ngầm)
// Cùng props {name: "An"}, nhưng gọi lúc 1h và 2h sẽ ra giao diện khác nhau!
const BadCard = ({ name }: { name: string }) => (
    <div>{name} - {new Date().toISOString()}</div>
);

// ✅ PURE: Same props -> Same JSX
const UserCard = ({ name, email, isActive }: { name: string, email: string, isActive: boolean }) => (
    <div className="card">
        <h3>{name}</h3>
        <p>{email}</p>
        <span>{isActive ? "Active" : "Inactive"}</span>
    </div>
);
```
Ngay cả với Hook, nếu bạn tuân thủ đúng quy tắc: Không trực tiếp gọi API trong render, Không trực tiếp thay đổi DOM (`document.getElementById`). JSX sinh ra là một kết quả "tất yếu" của State và Props hiện tại.

---

## 35.2 — Impossible States và useReducer

Giả sử bạn làm một Form đăng ký. Thông thường, bạn sẽ dùng `useState`:

```tsx
// ❌ CÁCH LÀM PHỔ BIẾN: Quá nhiều cờ (flags)
const [isLoading, setIsLoading] = useState(false);
const [error, setError] = useState<string | null>(null);
const [success, setSuccess] = useState(false);
```
Vấn đề là gì? Bạn có thể vô tình tạo ra các **Trạng thái không tưởng (Impossible States)**:
- `isLoading = true` VÀ `error = "Lỗi mạng"` VÀ `success = true` (Đang tải, nhưng vừa báo lỗi, lại vừa báo thành công???)

### Giải pháp: Discriminated Unions + useReducer
Dùng Discriminated Unions (Ch20) để ép buộc form chỉ được nằm ở MỘT VÀ CHỈ MỘT trạng thái ở cùng một thời điểm (State Machine).

```typescript
// 1. Định nghĩa các State có thể xảy ra
type FormState =
    | { status: "idle" }
    | { status: "editing"; name: string; email: string }
    | { status: "submitting"; name: string; email: string }
    | { status: "success"; userId: string }
    | { status: "error"; message: string; name: string; email: string };

// 2. Định nghĩa các Hành động (Events/Actions)
type FormAction =
    | { type: "start_editing" }
    | { type: "update_field"; field: "name" | "email"; value: string }
    | { type: "submit" }
    | { type: "submit_success"; userId: string }
    | { type: "submit_error"; message: string }
    | { type: "reset" };

// 3. PURE REDUCER: (State, Action) => NewState
// Không API, Không DOM -> Test cực dễ!
export const formReducer = (state: FormState, action: FormAction): FormState => {
    switch (action.type) {
        case "start_editing":
            return { status: "editing", name: "", email: "" };

        case "update_field":
            // Ngăn chặn: Chỉ cho gõ phím khi đang editing hoặc error
            if (state.status !== "editing" && state.status !== "error") return state;
            return { ...state, status: "editing", [action.field]: action.value };

        case "submit":
            if (state.status !== "editing") return state;
            return { status: "submitting", name: state.name, email: state.email };

        // ... Xử lý success, error tương tự
        default:
            return state;
    }
};
```
Nhờ cơ chế này, bạn không thể nào gõ phím (`update_field`) khi Form đang ở trạng thái `submitting`. Nếu cố tình gửi Action đó, Reducer sẽ bắt `if (state.status !== "editing") return state;` và không làm gì cả!

---

## 35.3 — Loại bỏ cờ Loading với RemoteData

Cũng tương tự như Form, khi gọi API lấy dữ liệu (ví dụ lấy Thông tin User), bạn thường sinh ra đống cờ `isLoading`, `isError`.
Trong FP, cộng đồng đã chuẩn hóa nó thành cấu trúc dữ liệu **RemoteData** (một sự tiến hóa của kiểu Option/Either chuyên dùng cho UI).

```typescript
// Định nghĩa RemoteData (4 trạng thái bất biến)
export type RemoteData<E, A> =
    | { status: "idle" }
    | { status: "loading" }
    | { status: "success"; data: A }
    | { status: "error"; error: E };

// Hàm Pattern Matching (fold/match) để ép buộc lập trình viên phải render đủ 4 trường hợp
export const foldRemoteData = <E, A, R>(
    rd: RemoteData<E, A>,
    cases: {
        idle: () => R;
        loading: () => R;
        success: (data: A) => R;
        error: (error: E) => R;
    },
): R => {
    switch (rd.status) {
        case "idle": return cases.idle();
        case "loading": return cases.loading();
        case "success": return cases.success(rd.data);
        case "error": return cases.error(rd.error);
    }
};
```

### Áp dụng vào React Component

```tsx
const UserProfile = ({ userId }: { userId: string }) => {
    // Chỉ 1 state duy nhất!
    const [user, setUser] = useState<RemoteData<string, User>>({ status: "idle" });

    useEffect(() => {
        setUser({ status: "loading" });
        fetchUser(userId)
            .then(data => setUser({ status: "success", data }))
            .catch(err => setUser({ status: "error", error: err.message }));
    }, [userId]);

    // ÉP BUỘC phải xử lý đủ 4 case. Quên 1 case là TypeScript chửi ngay!
    return foldRemoteData(user, {
        idle: () => <div>Nhấn nút để tải...</div>,
        loading: () => <Spinner />,
        success: (data) => <UserCard name={data.name} />,
        error: (msg) => <ErrorBanner message={msg} />,
    });
};
```

---

## Tóm tắt

- ✅ **React Components = Pure Functions**: UI là kết quả tất yếu của việc mapping (biến đổi) State/Props.
- ✅ **State Machine**: Dùng `useReducer` + Type Unions thay cho hàng chục biến `useState` boolean (cờ). Loại bỏ hoàn toàn các Impossible States.
- ✅ **RemoteData**: Kiểu dữ liệu tối thượng cho việc fetch API. Thay thế cụm `isLoading/error/data` và ép buộc lập trình viên phải render đủ mọi trạng thái thông qua `fold` (Pattern Matching).

## Tiếp theo

Bạn đã có đủ hành trang cho cả Backend và Frontend với TypeScript. Ở chương tiếp theo, chúng ta sẽ bước lên mây (Cloud) để thiết kế một hệ thống chịu tải lớn. Mời bạn đến với **Chapter 42: System Design & Architecture**.
