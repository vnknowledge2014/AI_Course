# Chapter 42 — System Design Thinking

> **Bạn sẽ học được**:
> - Tính toán tải trọng (Capacity estimation) bằng nhẩm tính (back-of-the-envelope).
> - Hiểu rõ 4 lớp Caching để tăng tốc độ phản hồi gấp 100 lần.
> - So sánh sâu sắc giữa REST, gRPC, và GraphQL.
> - Tại sao bạn NÊN bắt đầu bằng Monolith thay vì Microservices (Conway's Law).
>
> **Yêu cầu trước**: Chapters 37-41 (Production Engineering).
> **Thời gian đọc**: ~50 phút | **Level**: Principal
> **Kết quả cuối cùng**: Chuyển đổi tư duy từ "Thợ code" sang "Kiến trúc sư", biết thiết kế hệ thống dựa trên sự cân bằng (trade-offs).

---

Bạn biết quy hoạch thành phố không? Trước khi xây đường, người ta phải tính: Sẽ có bao nhiêu nghìn dân? Cần ống nước to chừng nào?
**System Design** chính là quy hoạch phần mềm. Đừng vội cắm mặt vào code! Hãy tính toán (Capacity Estimation), vẽ luồng dữ liệu (Data Flow), và chọn công nghệ phù hợp. 

Trong System Design không có công nghệ "tốt nhất" — chỉ có sự "đánh đổi" (Trade-offs). Bạn chọn MySQL hay MongoDB? Monolith hay Microservices? Mỗi quyết định đều đi kèm cái giá phải trả.

## 42.1 — Capacity Estimation (Nhẩm tính tải trọng)

Bước đầu tiên của thiết kế hệ thống là TÍNH. 2 request/giây thì xài 1 con server rách là đủ. Nhưng 20,000 request/giây thì cần Load Balancer + hàng chục instances. Việc nhẩm tính (Back-of-the-envelope) giúp bạn biết mình đang ở **BẬC ĐỘ** nào.

```typescript
// Định lượng tải trọng cho một nền tảng E-Commerce
const estimate = {
    // 1. Traffic (Lưu lượng)
    dailyActiveUsers: 1_000_000,        // 1 Triệu DAU
    avgOrdersPerUser: 0.1,              // Tỉ lệ mua hàng 10%
    dailyOrders: 1_000_000 * 0.1,       // = 100,000 đơn/ngày
    
    // (1 ngày = 86,400 giây)
    ordersPerSecond: Math.ceil(100_000 / 86_400), // ~2 đơn/giây (Trung bình)
    peakOrdersPerSecond: 20,            // Ngày hội Sales (Gấp 10 lần trung bình)

    // 2. Storage (Lưu trữ)
    avgOrderSizeBytes: 2_000,           // 2KB cho mỗi đơn hàng
    dailyStorageBytes: 100_000 * 2_000, // = 200MB/ngày
    yearlyStorageGB: Math.ceil(200 * 365 / 1_000), // ~73GB/năm
};

// Phân tích kết quả:
// 2 ops/sec -> Database đơn giản là cân tốt!
// 73GB/year -> Vừa khít một ổ SSD rẻ tiền!
```
> **💡 Thần chú cần nhớ**: 
> - 1 ngày ≈ 100,000 giây.
> - 1 máy chủ bình thường xử lý được ~1000 kết nối đồng thời.
> - Network round-trip (gọi qua mạng) tốn ~10ms-100ms. Gọi Local/Redis tốn < 1ms.

---

## 42.2 — Caching: 4 Lớp siêu tốc

Caching là cách HIỆU QUẢ NHẤT để hệ thống chịu tải. Thay vì bắt Database tính lại một kết quả tốn 200ms, bạn đọc từ RAM (Cache) mất 0.5ms. Nhanh hơn 400 lần! 
Tuy nhiên, Cache mang theo lời nguyền khét tiếng: **Cache Invalidation** (Khi nào thì xóa Cache cũ đi?).

Có 4 lớp Cache từ ngoài vào trong:

1. **Browser Cache**: Nằm ngay trên máy người dùng (Chrome). Nhanh nhất (0ms). Dùng cho CSS, JS, ảnh tĩnh. Cấu hình bằng header `Cache-Control`.
2. **CDN (Content Delivery Network)**: Đặt máy chủ tại các vị trí địa lý gần người dùng (ví dụ máy chủ Cloudflare tại Việt Nam). Rất nhanh (10ms).
3. **Application Cache (Redis/Memcached)**: Cache kết quả Database (vd: Bảng xếp hạng, Thông tin giỏ hàng).
4. **Database Cache**: Chính PostgreSQL cũng tự động cache các câu lệnh SELECT hay dùng vào RAM của nó.

```typescript
// Một cơ chế Application Cache đơn giản bằng RAM (Thay bằng Redis cho Production)
const createTTLCache = <T>(defaultTTLMs = 60_000) => {
    const store = new Map<string, { value: T; expiresAt: number }>();

    return {
        get: (key: string): T | undefined => {
            const entry = store.get(key);
            if (!entry) return undefined;
            // Xóa và báo Miss nếu quá hạn (TTL - Time To Live)
            if (Date.now() > entry.expiresAt) { store.delete(key); return undefined; }
            return entry.value;
        },
        set: (key: string, value: T, ttlMs = defaultTTLMs): void => {
            store.set(key, { value, expiresAt: Date.now() + ttlMs });
        },
        invalidate: (key: string): boolean => store.delete(key)
    };
};
```
**Chiến lược Invalidation**:
- *TTL (Time-To-Live)*: Tự động hết hạn (VD: 5 phút).
- *Write-through*: Mỗi khi ghi DB, ghi luôn vào Cache. Đảm bảo dữ liệu luôn mới nhất, nhưng tốc độ Ghi bị chậm.

---

## 42.3 — Thiết kế API: REST vs GraphQL vs gRPC

Chọn giao thức kết nối nào?

1. **REST (JSON over HTTP/1.1)**: Chuẩn mực mặc định. 
   - *Ưu điểm*: Đơn giản, dễ cache, dễ dùng với mọi client. 
   - *Nhược điểm*: Over-fetching (trả về quá nhiều thứ không cần) và Under-fetching (phải gọi 3 APIs mới đủ dữ liệu gộp màn hình).
   
2. **GraphQL**: Sinh ra bởi Facebook để giải quyết nhược điểm của REST.
   - *Ưu điểm*: Client muốn trường nào, Server trả đúng trường đó, trong 1 cục duy nhất!
   - *Nhược điểm*: Cực kỳ khó thiết lập Caching ở tầng HTTP. Dễ dính lỗi N+1 Query làm sập DB.

3. **gRPC (Protobuf over HTTP/2)**: Vũ khí của Google.
   - *Ưu điểm*: Nhanh gấp 10 lần REST nhờ truyền dữ liệu dạng nhị phân (Binary). Có sẵn type-safety. Hỗ trợ Streaming 2 chiều.
   - *Nhược điểm*: Trình duyệt không đọc được nhị phân gRPC nguyên bản (phải qua Envoy proxy). Cực khó debug vì không mở Network tab lên đọc text được như JSON.

> 🛠️ **Best Practice**: Dùng REST hoặc GraphQL cho Client (Mobile, Web) giao tiếp với Server. Dùng gRPC cho các Microservices nội bộ giao tiếp với nhau để tối đa hóa tốc độ!

---

## 42.4 — Ảo mộng Microservices và Định luật Conway

Có một căn bệnh phổ biến: Cứ thấy project mới là lao vào đẻ ra 10 cái Microservices.
Lời khuyên từ Martin Fowler (Bậc thầy kiến trúc): **"You shouldn't start with microservices"** (Đừng bao giờ bắt đầu bằng microservices).

Microservices mang tới thảm họa vận hành: Bạn phải deploy 10 cái CI/CD, theo dõi log phân tán, debug siêu khó, dữ liệu mâu thuẫn.

Vậy khi nào mới chia Microservices?
- Quy định bởi Định luật Conway: Khi tổ chức có quá nhiều người (Team > 15-20 dev). Hai team đánh nhau vì conflict git liên tục trên cùng 1 repo.
- Yêu cầu Scale khác nhau: Service Resize Video cần rất nhiều CPU, trong khi Service Chat cần nhiều RAM.

### Giải pháp tối ưu: Modular Monolith
Chạy 1 cục Monolith duy nhất trên 1 server, nhưng chia thư mục cực kỳ nghiêm ngặt (Ch33).

```typescript
// Thay vì chia làm 3 Microservices (User, Product, Order)
// Hãy thiết kế thành 3 Module, nói chuyện với nhau thông qua Barrel Export (Public API)
type Module = {
    name: string;
    ownsTables: string[];   // Quyền sở hữu độc quyền (Module khác cấm chọc vào!)
    publicAPI: string[];    // Những Interface lộ ra cho module khác dùng
    dependsOn: string[];    // Phụ thuộc vào module nào
};

const modules: Module[] = [
    {
        name: "orders",
        ownsTables: ["orders", "order_items"],
        publicAPI: ["createOrder", "confirmOrder"],
        dependsOn: ["products", "users"], // Order phụ thuộc Product
    },
    {
        name: "products",
        ownsTables: ["products", "categories"],
        publicAPI: ["findProduct"],
        dependsOn: [],
    }
];

// NẾU SAU NÀY CẦN CHIA MICROSERVICES? 
// Cực kỳ dễ! Vì `orders` đã bị cách ly sẵn, không hề join thẳng vào bảng của `products`.
```

---

---

## ✅ Checkpoint 42

1. Node.js chạy đơn luồng. Điều đó ảnh hưởng gì tới quyết định thiết kế?
2. BFF (Backend for Frontend) giải quyết vấn đề gì mà một API chung không giải quyết được?
3. Khi nào serverless **đắt hơn** một server thường?

<details>
<summary>Đáp án</summary>

1. Một tác vụ CPU nặng sẽ **chặn event loop** và làm đứng mọi request khác trên process đó. Vì thế: đẩy việc nặng sang worker thread hoặc queue, và scale bằng nhiều process (`cluster`, PM2) chứ không bằng thread.
2. Mỗi client cần shape dữ liệu khác nhau: mobile cần payload nhỏ, web cần nhiều field, smart TV cần khác nữa. API chung buộc phải là mẫu số chung — hoặc over-fetch, hoặc phải gọi nhiều lần. BFF cho mỗi client một tầng ghép riêng.
3. Khi lưu lượng **đều và cao**. Serverless tính theo lượt gọi và thời gian chạy; ở mức tải ổn định 24/7, một VM thường rẻ hơn nhiều lần. Serverless thắng khi tải rất thất thường hoặc gần bằng không phần lớn thời gian.
</details>

---

## 🏋️ Bài tập

**Bài 1 (10 phút).** Tính peak QPS cho hệ 2 triệu DAU, mỗi người 15 request/ngày, hệ số peak ×4.

**Bài 2 (15 phút).** Viết một endpoint cố tình chặn event loop (vòng lặp bận 3 giây) rồi bắn 10 request đồng thời. Đo và giải thích. Sau đó chuyển sang `worker_threads`.

**Bài 3 (25 phút).** Thiết kế caching cho một trang sản phẩm: quyết định TTL cho từng tầng (browser, CDN, Redis) và chiến lược invalidate khi giá thay đổi.

---

## 🔧 Troubleshooting

| Vấn đề | Vì sao xảy ra | Hướng xử lý |
|---|---|---|
| Toàn bộ API đứng khi một request chạy | Tác vụ CPU chặn event loop | `worker_threads`, hoặc đẩy sang queue |
| Cold start chậm trên serverless | Bundle lớn, nhiều dependency | Tree-shaking, giảm dependency, cân nhắc edge runtime |
| Cache CDN không được xoá | Thiếu cơ chế purge khi deploy | Dùng URL có hash cho asset; purge API cho dữ liệu động |
| Memory leak sau vài ngày | Cache trong RAM không có giới hạn | Dùng LRU có `maxSize`; đo bằng heap snapshot |
| `EMFILE: too many open files` | Rò rỉ file descriptor / connection | Đóng stream; giới hạn concurrency |

## Tóm tắt

- ✅ **Capacity estimation**: Luôn nhẩm tính traffic và storage trước khi gõ phím. Đừng mua xe lu để giết muỗi.
- ✅ **Caching**: Là vũ khí mạnh nhất, nhưng hãy cẩn thận chọn chiến lược Invalidation.
- ✅ **APIs**: REST cho public, GraphQL cho tính linh hoạt Frontend, gRPC cho liên lạc nội bộ tốc độ bàn thờ.
- ✅ **Monolith-first**: Bắt đầu bằng Modular Monolith. Chỉ chia tách Microservices khi tổ chức nhân sự phình to.

## Tiếp theo

Bạn đã nắm vững TẤT CẢ các khái niệm từ Functional Programming, Database, đến System Design. Giờ là lúc ráp chúng lại thành một sản phẩm thực tế.
Mời bạn bước vào **Chapter 43: Capstone Part 2 — Production Deployment**!
