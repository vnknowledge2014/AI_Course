# Chapter 30b — Advanced Effect: Context (DI), Fibers & Scheduling

> **Bạn sẽ học được**:
> - Bí mật của tham số `R` trong `Effect<A, E, R>` — Dependency Injection được nhúng thẳng vào hệ thống Type.
> - **Context & Layers**: Cách định nghĩa Service (Database, Logger) và tiêm (inject) chúng vào ứng dụng.
> - **Fibers**: Luồng (threads) siêu nhẹ của Effect. Cách spawn, join và interrupt an toàn.
> - **Concurrency**: Chạy song song hàng ngàn tác vụ với `Effect.all` và kiểm soát độ trễ.
> - **Schedules**: Viết logic retry (exponential backoff) phức tạp chỉ trong 1 dòng code.
>
> **Yêu cầu trước**: Chapter 25 (Intro to Effect), Chapter 27 (Monads).
> **Thời gian đọc**: ~45 phút | **Level**: Expert
> **Kết quả cuối cùng**: Có khả năng thiết kế hệ thống thực tế (Functional System Design) kết nối Database, API với Retry và Concurrency hoàn chỉnh.

---

Bạn xây dựng một ứng dụng. Hàm `getUser(id)` cần kết nối Database. Ở OOP, bạn dùng Constructor Injection (hoặc DI Container như NestJS). Ở các thư viện FP cũ, bạn phải truyền tay connection: `getUser(db, id)`, hoặc dùng một Monad phức tạp tên là `Reader`.

Effect giải quyết DI một cách thanh lịch tuyệt đối thông qua tham số **`R` (Requirements)** trong `Effect<A, E, R>`. Nếu một hàm cần Database, type của nó sẽ là `Effect<User, DbError, Database>`. Trình biên dịch TypeScript sẽ GÀO THÉT nếu bạn cố chạy hàm này mà quên cung cấp Database!

Và khi bạn có một hệ thống lớn, bạn cần chạy song song, cần retry khi rớt mạng, cần hủy tác vụ (cancel) khi user bấm nút "Hủy". Promise/async-await thuần của JS vô cùng kém cỏi trong việc cancel (bạn phải truyền `AbortSignal` khắp nơi). Effect cung cấp **Fibers** — giải quyết mọi thứ.

---

## 30b.1 — Context và Dependency Injection

### The `R` in `Effect<A, E, R>`

Hãy bắt đầu bằng việc tạo một Service đơn giản.

```typescript
// filename: src/effect_context.ts
import { Effect, Context } from "effect";
import assert from "node:assert/strict";

// ── 1. Khai báo Service (Interface) ──
// Đây là "Token" đại diện cho service.
interface RandomGenerator {
    readonly nextInt: Effect.Effect<number>;
}

// Tạo cái Tag (nhãn) để Effect biết cách tìm service này ở runtime.
const RandomGenerator = Context.GenericTag<RandomGenerator>("RandomGenerator");


// ── 2. Xây dựng Chương Trình yêu cầu Service ──
// Chương trình tung xúc xắc.
// Chú ý TYPE: Effect<string, never, RandomGenerator>
// R = RandomGenerator! Nó "đòi" service này.
const rollDice = Effect.gen(function* () {
    // yield* gọi Service Tag! Effect tự động lấy instance từ Context.
    const random = yield* RandomGenerator;
    
    const value = yield* random.nextInt;
    return `You rolled a ${value % 6 + 1}!`;
});


// ── 3. Cung cấp Implementation (Provide) ──
// Trình biên dịch sẽ cản bạn nếu bạn gọi Effect.runPromise(rollDice) bây giờ:
// Lỗi: "Argument of type 'Effect<..., ..., RandomGenerator>' is not assignable to Effect<..., ..., never>"

// Tạo một layer cung cấp implementation thực sự
const liveRandom = Context.make(RandomGenerator, {
    nextInt: Effect.sync(() => Math.floor(Math.random() * 100))
});

// Chạy thử với Live implementation
const program1 = Effect.provide(rollDice, liveRandom);
// Lúc này program1 có type: Effect<string, never, never>. R đã trở thành never!

// Chạy thử với Test implementation (Mocking cực kỳ dễ!)
const testRandom = Context.make(RandomGenerator, {
    nextInt: Effect.succeed(3) // Luôn trả về 3
});
const program2 = Effect.provide(rollDice, testRandom);

async function main() {
    const result2 = await Effect.runPromise(program2);
    assert.strictEqual(result2, "You rolled a 4!"); // (3 % 6) + 1
    console.log("Context DI OK ✅");
}
main();
```

> **💡 Bản chất**: Thay vì nhồi nhét params vào hàm (`rollDice(random)`), bạn để tham số ở cấp độ Type. `yield* Random` sẽ tạm dừng chương trình, báo với runtime: "Lấy cho tôi thằng Random!". Tới cuối cùng, khi gọi `Effect.provide()`, bạn lấp đầy các lỗ hổng đó.

---

## 30b.2 — Layers: Xây dựng đồ thị Dependencies

Service thường phụ thuộc vào Service khác. `DatabaseService` cần `ConfigService`. `UserRepository` cần `DatabaseService`. Effect dùng `Layer` để giải quyết đồ thị này.

```typescript
// filename: src/effect_layers.ts
import { Effect, Context, Layer } from "effect";

// ── 1. Định nghĩa Tags ──
interface Config { readonly getDbUrl: Effect.Effect<string>; }
const Config = Context.GenericTag<Config>("Config");

interface Database { readonly query: (sql: string) => Effect.Effect<any[]>; }
const Database = Context.GenericTag<Database>("Database");

// ── 2. Tạo Layers ──
// Config không cần gì cả (R = never)
const ConfigLive = Layer.succeed(
    Config,
    Config.of({ getDbUrl: Effect.succeed("postgres://localhost:5432") })
);

// Database CẦN Config! (R = Config)
const DatabaseLive = Layer.effect(
    Database,
    Effect.gen(function* () {
        const config = yield* Config;
        const url = yield* config.getDbUrl;
        
        console.log(`Connecting to ${url}...`);
        
        return Database.of({
            query: (sql) => Effect.succeed([{ result: "mock data" }])
        });
    })
);

// ── 3. Gộp Layers (Compose) ──
// DatabaseLive yêu cầu Config. Ta cung cấp ConfigLive cho nó.
const MainLayer = Layer.provide(DatabaseLive, ConfigLive);

// ── 4. Ứng dụng ──
const app = Effect.gen(function* () {
    const db = yield* Database;
    const data = yield* db.query("SELECT * FROM users");
    return data;
});

// App đòi Database. MainLayer cung cấp Database.
const runnableApp = Effect.provide(app, MainLayer);
// Effect.runPromise(runnableApp);
```

> **💡 Layer**: Là công thức để tạo ra Service. Nó nhận vào Dependencies, xử lý (khởi tạo connection), và nhả ra Service. Bạn kết hợp các Layer lại thành một cây, sau đó cắm cái cây đó vào App. Quên NestJS module cồng kềnh đi, đây là DI thuần túy bằng hàm!

---

## ✅ Checkpoint 30b.1-30b.2

> Đến đây bạn phải hiểu:
> 1. **`Context.GenericTag`**: Là biển tên đại diện cho một Service.
> 2. Dùng **`yield* Tag`** để gọi Service ra dùng. Trình biên dịch sẽ tự động gắn nó vào tham số `R`.
> 3. **`Effect.provide`**: Cung cấp Service để biến tham số `R` thành `never` (sẵn sàng chạy).
> 4. **`Layer`**: Dùng khi việc khởi tạo Service cần chạy các Effect (async/chờ mạng) hoặc khi Service phụ thuộc chéo lên nhau.
>
> **Test nhanh**: App yêu cầu Database. Ta có `TestDbLayer`. Hàm nào được dùng để kết nối chúng?
> <details><summary>Đáp án</summary>`Effect.provide(app, TestDbLayer)`. Nó tiêm (inject) lớp layer giả lập vào app, giúp app sẵn sàng chạy trong môi trường test mà không sợ đụng database thật.</details>

---

## 30b.3 — Fibers: Trái tim của Concurrency

JavaScript chạy trên 1 thread (Event Loop). Promise là cách JS xử lý bất đồng bộ. Nhưng Promise có 2 yếu điểm sinh tử:
1. **Khởi chạy ngay lập tức (Eager)**: Cứ tạo Promise là nó chạy.
2. **Gần như không thể hủy (Uncancelable)**: Nếu bạn ném một request API, xong người dùng đổi ý chuyển trang, request đó VẪN CHẠY ngầm và tiêu thụ tài nguyên.

Effect tạo ra **Fibers**. Chúng giống hệt Goroutines trong Go, hay Virtual Threads trong Java 21. Chúng là "Luồng ảo".
Một Fiber đại diện cho một Effect đang được thực thi. Bạn có thể `fork` (chạy ngầm), `join` (đợi kết quả), hoặc **`interrupt`** (giết nó không thương tiếc).

```typescript
// filename: src/effect_fibers.ts
import { Effect, Fiber } from "effect";

const slowTask = Effect.gen(function* () {
    console.log("Bắt đầu tác vụ nặng...");
    yield* Effect.sleep("2 seconds");
    console.log("Tác vụ nặng hoàn thành!");
    return "Done!";
});

const main = Effect.gen(function* () {
    // 1. Fork: Tạo một Fiber chạy ngầm (không block)
    const fiber = yield* Effect.fork(slowTask);
    
    console.log("Fiber đang chạy ngầm, làm việc khác...");
    yield* Effect.sleep("500 millis");
    
    // 2. Interrupt: Hủy Fiber!
    console.log("Hủy Fiber!");
    yield* Fiber.interrupt(fiber);
    
    // 3. Join: Đợi Fiber (nó đã bị hủy, ta sẽ nhận Exit state)
    // const result = yield* Fiber.join(fiber);
});

// Chạy thử:
// Effect.runPromise(main);
// Kết quả in ra:
// "Bắt đầu tác vụ nặng..."
// "Fiber đang chạy ngầm, làm việc khác..."
// "Hủy Fiber!"
// (Không bao giờ in ra "Tác vụ nặng hoàn thành!")
```

**Sự Hủy Tự Động (Automatic Interruption)**
Đây là thứ khiến Effect là phép màu. Khi một Fiber kết thúc (do lỗi), Effect TỰ ĐỘNG dọn dẹp các Fiber con của nó! Không có memory leak. Không có ghost threads.

---

## 30b.4 — Chạy Song Song: `Effect.all`

`Effect.all` là phiên bản nâng cấp hoàn hảo của `Promise.all` và `asyncio.gather`.
Khác biệt: Nếu 1 task fail, `Effect.all` TỰ ĐỘNG hủy tất cả các task còn lại ngay lập tức (Fail-fast + Cleanup)!

```typescript
// filename: src/effect_all_concurrency.ts
import { Effect } from "effect";

const fetchUser = (id: number) => Effect.gen(function* () {
    yield* Effect.sleep("1 second");
    if (id === 2) return yield* Effect.fail("Error on User 2!");
    return `User ${id}`;
});

const parallelFetch = Effect.gen(function* () {
    const tasks = [fetchUser(1), fetchUser(2), fetchUser(3)];
    
    // Chạy song song (Mặc định Effect.all chạy tuần tự! Phải chỉ định concurrency)
    const results = yield* Effect.all(tasks, { 
        concurrency: "unbounded" // Chạy tất cả song song
    });
    
    return results;
});

// Effect.runPromise(parallelFetch).catch(console.error);
// Ngay tại giây thứ 1, User 2 fail.
// User 1 và User 3 lập tức bị interrupt, không tiêu tốn thêm tài nguyên!
```

**Quản lý tài nguyên (Concurrency Limit)**
Với `Promise.all`, nếu bạn ném 10,000 requests, bạn sập server.
Với `Effect.all(tasks, { concurrency: 10 })`, Effect tự động quản lý pool, chạy đúng 10 request cùng lúc.

---

## 30b.5 — Schedule: Retries không đổ mồ hôi

Viết hàm retry thủ công cho API call cực kỳ mệt mỏi: vòng lặp `while`, `sleep` đợi, đếm số lần thử, tính thời gian tăng dần (exponential backoff).
Effect coi Schedule là một Data Type (giống Functor). Bạn có thể kết hợp chúng!

```typescript
// filename: src/effect_schedule.ts
import { Effect, Schedule } from "effect";

// Giả lập API lúc được lúc không
let counter = 0;
const flakyApi = Effect.gen(function* () {
    counter++;
    console.log(`Lần thử thứ ${counter}...`);
    if (counter < 3) {
        return yield* Effect.fail("Network error!");
    }
    return "Thành công!";
});

// 1. Tạo Schedule: Tối đa 5 lần.
const maxRetries = Schedule.recurs(5);

// 2. Tạo Schedule: Giãn cách theo hàm số mũ (10ms -> 20ms -> 40ms)
const expoBackoff = Schedule.exponential("10 millis");

// 3. KẾT HỢP hai Schedule (Intersection)
// -> Chạy giãn cách Mũ, NHƯNG dừng lại nếu quá 5 lần.
const policy = Schedule.intersect(expoBackoff, maxRetries);

// 4. Áp dụng vào Effect
const robustApi = Effect.retry(flakyApi, policy);

// async function test() {
//     const res = await Effect.runPromise(robustApi);
//     console.log(res); 
//     // Sẽ in ra: Lần thử 1, 2, 3... "Thành công!"
// }
// test();
```

> **💡 Tính đại số (Algebraic)**: `Schedule` là một thứ có thể được compose. Bạn lấy `recurs` kết hợp (intersect) với `exponential`. Giống y hệt cách bạn dùng `Order.combine` ở Chapter 25b. FP biến mọi thứ thành các building block có thể ghép nối.

---

## ✅ Checkpoint 30b.3-30b.5

> Đến đây bạn phải hiểu:
> 1. **Fibers**: Luồng ảo trong Effect. Chạy ngầm (`fork`), đợi (`join`), hủy (`interrupt`).
> 2. **Automatic Interruption**: Khi cha chết hoặc có lỗi, con chết theo. Tránh rò rỉ tài nguyên.
> 3. **`Effect.all`**: Chạy mảng Effect. Dùng tham số `{ concurrency: N }` để tránh sập server.
> 4. **`Schedule`**: Cấu trúc dữ liệu mô tả quy tắc lặp. Ghép nối được. Áp dụng qua `Effect.retry` hoặc `Effect.repeat`.
>
> **Test nhanh**: Bạn tạo một mảng gồm 1 triệu `Effect` (mỗi cái insert vào DB). Bạn chạy `Effect.all(arr)`. Có sợ sập DB không?
> <details><summary>Đáp án</summary>Không! Mặc định `Effect.all` chạy TUẦN TỰ (Sequential). Nó chỉ chạy song song nếu bạn cố tình khai báo `{ concurrency: N }` hoặc `"unbounded"`.</details>

---

## 🏋️ Bài tập

**Bài 1** (15 phút): Hệ thống Retry toàn diện

```typescript
// Bạn có một hàm gửi Email. Nó có tỷ lệ fail.
// YÊU CẦU:
// 1. Nếu fail, hãy retry.
// 2. Schedule retry: giãn cách bắt đầu từ 100ms, tăng theo hàm mũ (exponential).
// 3. Tối đa chỉ retry 3 lần.
// 4. GỢI Ý: dùng Schedule.exponential và Schedule.recurs. Giao cắt bằng Schedule.intersect (hoặc Schedule.andThen).

import { Effect, Schedule } from "effect";

let attempts = 0;
const sendEmail = Effect.gen(function* () {
    attempts++;
    if (attempts <= 3) {
        console.log(`Attempt ${attempts} failed`);
        return yield* Effect.fail(new Error("SMTP down"));
    }
    console.log(`Attempt ${attempts} success`);
    return "Email sent";
});

// THỰC HIỆN BÀI LÀM VÀO ĐÂY:
const mySchedule = Schedule.intersect(
    Schedule.exponential("100 millis"),
    Schedule.recurs(3)
);

const reliableSend = Effect.retry(sendEmail, mySchedule);

// Effect.runPromise(reliableSend).then(console.log);
```

<details><summary>✅ Lời giải Bài 1</summary>

Giải pháp chính xác được viết ngay trong phần gợi ý ở trên! 
`Schedule.intersect` là phép toán `AND`. Điều kiện dừng là khi MỘT TRONG HAI Schedule báo kết thúc (nghĩa là số lần > 3). Vì vậy việc kết hợp này rất an toàn. Chú ý: `sendEmail` fail 3 lần đầu, lần thứ 4 nó sẽ pass, và vì ta `recurs(3)` tức là cho phép retry tối đa 3 lần (chạy tổng cộng 4 lần), nên tác vụ sẽ vừa vặn thành công!
</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| Báo lỗi "Argument of type ... is not assignable to Effect<..., ..., never>" | Quên cung cấp Dependencies | App đang đòi service. Bạn phải dùng `Effect.provide(app, LayerCuaBan)` để giải quyết nó trước khi `runPromise`. |
| Màn hình đứng im, không có lỗi gì | Effect.all không trả về kết quả | `Effect.all` chờ TẤT CẢ các item hoàn thành. NẾU 1 fiber bị kẹt (không bao giờ kết thúc), toàn bộ lệnh all bị kẹt. Dùng `Effect.timeout` cho các tác vụ không đáng tin. |
| Code không chạy song song | Dùng `Effect.all` nhưng quên tham số | Khác với `Promise.all` (mặc định song song), `Effect.all` của Effect bản mới mặc định chạy tuần tự. Bạn PHẢI truyền `{ concurrency: N }` hoặc `"unbounded"`. |

---

## Tóm tắt

- ✅ **The `R` Type**: `Effect<A, E, R>`. TypeScript compiler giờ đây kiêm luôn vai trò DI Container. Nó kiểm tra xem bạn đã truyền đủ dependencies cho hệ thống chưa ngay lúc code!
- ✅ **Layer**: Cách bạn đóng gói việc khởi tạo các Service phức tạp (ví dụ: cần async connect DB).
- ✅ **Fibers**: Cơ chế chạy đa luồng cực nhẹ của Effect. An toàn tuyệt đối trước rò rỉ bộ nhớ nhờ Automatic Interruption (hủy dây chuyền).
- ✅ **Concurrency Limits**: Viết code lấy dữ liệu hàng loạt không còn sợ sập API nhờ `{ concurrency }`.
- ✅ **Schedule**: Sức mạnh của đại số (Algebra) được áp dụng vào logic Retry/Repeat. Ghép nối thời gian giãn cách hàm mũ với giới hạn số lần bằng một phép `intersect`!

## Tiếp theo

Bạn đã nắm vững TẤT CẢ công cụ thiết kế hệ thống tốt nhất thế giới TypeScript.
→ Chapter 30c (New): **Recursive Types & Folds** — Trở lại với toán học. Làm sao để bóc tách một cấu trúc AST phức tạp (Parser) mà không cần dùng hàm đệ quy bẩn? Dùng Catamorphisms!
