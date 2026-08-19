# MASTERPLAN.md — Byte Academy

**Dự án:** Học liệu tương tác dạy lập trình từ số 0 → thành thạo, 9 lĩnh vực, 3 ngôn ngữ, 7 nền tảng
**Repo gốc:** `/Volumes/SEAGATE/Personal_Projects/AI_Course`
**Trạng thái tài liệu:** BẢN CHỐT v1 — thay thế toàn bộ 7 thiết kế mảng và mọi ADR mâu thuẫn
**Ngày:** 2026-08-19
**Tên sản phẩm:** **Byte Academy** (bỏ tên "Code Odyssey" trong `fp/game/package.json`)

---

## 0. TÓM TẮT ĐIỀU HÀNH — 14 quyết định chốt

| # | Vấn đề | QUYẾT ĐỊNH CHỐT | Bác bỏ |
|---|---|---|---|
| 1 | Shell 7 nền tảng | **Tauri 2.9** cho 5 native (macOS/Win/Linux/Android/iOS-iPadOS) + **PWA** cho Chrome/Firefox, MỘT app Svelte 5 duy nhất | Electron+Capacitor, Flutter, React Native, PWA thuần |
| 2 | Backend | **KHÔNG có backend bắt buộc.** Tauri host process = Rust, nhưng mọi tính năng học tập chạy 100% client. `services/sync` là TÙY CHỌN, tắt mặc định | Server chấm bài, proxy LLM, remote compile làm đường chính |
| 3 | Monorepo | **pnpm 10 (qua corepack) + Turborepo 2** ở gốc repo; `packages/` = TS, `crates/` = Rust, `content/` = nguồn nội dung | npm workspaces, bun, Nx, `fp/platform`, `fp/curriculum` |
| 4 | Runtime Rust | **`crates/byte-rust`** (đã có 3.653 dòng) = lex→parse(có error recovery)→typecheck-lite→interpreter. **KHÔNG borrow checker đầy đủ.** 3 kết cục: `Pass \| Fail(chẩn đoán) \| ChuaHoTro(tính năng)`. Ownership dạy bằng world `memory-city` + `cargo` thật trên Desktop | ferrite (tyck+MIR+NLL 14 người-tháng), rustc-WASM, rust-analyzer-wasm (đã archive), regex giả, Rhai |
| 5 | Runtime TypeScript | **`typescript` 5.x compiler API trong Worker** cho `tier: compile` (diagnostic thật) + thực thi transpiled code trong Worker khoá global. QuickJS-ng CHỈ opt-in cho bài đo độ phức tạp | Sucrase làm trọng tài (không typecheck), `new Function` main thread, QuickJS mặc định |
| 6 | Runtime Python | **Pyodide 0.28.x trong Web Worker**, wheel vendor offline (numpy/returns/pyrsistent/toolz/hypothesis), `SimDisk` thay `open()` cho bài storage | Pyodide main thread, `micropip` từ PyPI lúc chạy, MicroPython |
| 7 | Dừng code treo | **`worker.terminate()` + worker dự phòng ấm** là cơ chế CHÍNH trên cả 7 nền tảng. `setInterruptBuffer`/SAB là tối ưu khi `crossOriginIsolated===true` | Iframe removal (chết trên WebKit), SAB làm cơ chế chính |
| 8 | Cách ly | Iframe **origin THẬT khác** + `allow-scripts allow-same-origin`, CSP `connect-src 'none'`; Worker bên trong. Self-test khởi động là **release gate**, không phải "chế độ hạn chế" | Opaque origin (`Worker` ném SecurityError), same-origin Worker đơn thuần |
| 9 | Nguồn nội dung | **`content/**/*.lesson.md`** (markdown + frontmatter + remark-directive, thang colon `::::`/`:::`). 159 chương **COPY** sang `content/legacy/`, `fp/*_Books` thành READ-ONLY | MDX (RCE + không validate tĩnh), YAML thuần, sửa file tại chỗ, ánh xạ theo dải dòng |
| 10 | Chấm bài | **Assertion tất định trên artifact của học viên.** Không bao giờ chấm bằng free-text LLM, không so khớp trace với hệ thật, không so stdout với runtime khác | golden-trace matching, LLM-as-judge, byte-sim pass@all cho prompt |
| 11 | Self-test compiler | Chạy solution trong **ĐÚNG runtime shipped** (Pyodide dưới Node, byte-rust WASM, TS worker) — CI bắt buộc | CPython/Node native (sai lệch ngữ nghĩa) |
| 12 | Sao/huy hiệu | **4 huy hiệu không thứ bậc** (Hoàn thành · Tự lực · Tinh gọn · Giải thích được), cộng dồn, không mất. Xoá cơ chế "3 sao = không dùng hint" | Phạt việc dùng gợi ý, tụt hạng khi sai |
| 13 | Phạm vi v1.0 | **~180 lesson TIER-A** (Realm 0 + Realm 1 Python + Math T2.1–T2.3) + **toàn bộ 159 chương ở TIER-C reading mode**. Mọi thứ khác vào `roadmap.yaml`, KHÔNG schema-validate, KHÔNG vào build | Đóng băng 282 quest / 1.680 lesson, "ship khi đủ 10 realm" |
| 14 | App Store 2.5.2 | Content pack **hai lớp**: PROSE (OTA tự do) và EXEC (trong bundle app, iOS từ chối EXEC đến từ OTA). Tab "Test" hiện `testCode` **xem và sửa được** | OTA toàn bộ content pack kèm `testCode` ẩn |

---

## 1. SỰ THẬT NỀN — số liệu đo trực tiếp trên đĩa (2026-08-19)

Mọi con số dưới đây đã kiểm chứng bằng lệnh, KHÔNG lấy từ header outline. Đây là cơ sở của mọi ước lượng trong tài liệu.

```
Repo:            /Volumes/SEAGATE/Personal_Projects/AI_Course
Nhánh hiện tại:  chore/audit-fp-books   (main là nhánh đích)
Cây làm việc:    SẠCH — chỉ 2 mục untracked: docs/ và packages/
                 → packages/byte-rust CHƯA ĐƯỢC COMMIT (di chuyển miễn phí)

Chương markdown: Python 46 · Rust 63 · TypeScript 50 = 159 chapter_*.md
Tổng file .md:   299 (159 chương + 140 outline/appendix/README)
Tổng dòng .md:   115.640

fp/game:         Svelte 5.56.8 · Vite 8.1.5 · PixiJS 8.5.2 · CodeMirror 6 ·
                 Dexie 4.0.8 · sucrase 3.35 · marked 18 · dompurify 3.4.12
                 KHÔNG có pnpm workspace, KHÔNG có Worker (grep = 0 kết quả)

fp/game/dist:    assets/ + pyodide/ + index.html + favicon + icons
                 → KHÔNG CÓ MỘT FILE BÀI HỌC NÀO. Bản build là app RỖNG.
                 Nguyên nhân: db.ts fetch '/src/data/manifest.json'
                 (đường dẫn chỉ tồn tại ở vite dev server)

vite.config.ts:  COOP/COEP nằm trong `server.headers` → CHỈ dev server.
                 Bản build và mọi vỏ đóng gói KHÔNG có header này.
                 → crossOriginIsolated === false, SharedArrayBuffer undefined.

public/pyodide:  pyodide.asm.wasm 10.087.885 B · python_stdlib.zip 2.341.761 B
                 pyodide.asm.js 1.229.628 B · KHÔNG có một file .whl nào
                 → numpy KHÔNG chạy được offline.

runner.ts:       runRust() trả {success:true,"Compilation successful.\nTests passed."}
                 vô điều kiện. Pyodide chạy MAIN THREAD. TS chạy new Function()
                 cùng realm với app (chạm được DOM/Dexie/localStorage).

.gitignore:      ignore `game/src/data/` và `game/public/pyodide/`

packages/byte-rust (untracked, 0 dependency, crate-type cdylib+rlib):
  lexer.rs   558 | parser.rs 1443 | ast.rs   417 | interp.rs 458
  value.rs   291 | diag.rs    263 | span.rs  154 | lib.rs     52   = 3.653 dòng
  → parser VÀ evaluator đã bắt đầu. Chưa có typecheck, chưa có move-check.

docs/decisions/ADR-001-rust-runtime.md (untracked):
  Cam kết "Ownership/borrowing được mô phỏng và KIỂM TRA ... phải báo lỗi mượn
  giống compiler thật" — CAM KẾT NÀY BỊ HUỶ, xem §4.3 và ADR-002.

Toolchain máy:   node 26.7.0 · npm 11.19.0 · bun 1.3.14 · KHÔNG có pnpm
                 rustc 1.97.1 · cargo 1.97.1 · python 3.14.7
Rust targets đã cài: wasm32-unknown-unknown, x86_64-apple-darwin,
                     x86_64-pc-windows-msvc
                 → THIẾU: aarch64-apple-darwin, aarch64-apple-ios,
                   aarch64-apple-ios-sim, aarch64-linux-android,
                   armv7-linux-androideabi, x86_64-linux-android, i686-linux-android
```

**Hệ quả bắt buộc ghi vào M0:** con số "80.731 dòng" trong các thiết kế là cũ; con số "68 chương Rust" trong outline là sai (thực tế 63). Mọi bảng kiểm kê phải sinh từ `find` trên đĩa bằng `tools/inventory`, không gõ tay.

---

## 2. MƯỜI NGUYÊN TẮC BẤT DI BẤT DỊCH

Đây là các bất biến. Mọi PR vi phạm bị CI chặn; mọi thiết kế tương lai phải phục tùng.

1. **KHÔNG BAO GIỜ DEFAULT-PASS.** Mọi grader chỉ có 3 kết cục: `Pass`, `Fail(chẩn đoán cụ thể)`, `ChuaHoTro(tên tính năng)`. Không có nhánh "không biết → coi như đúng". Registry AST-query gặp `kind` chưa đăng ký → **BUILD FAIL**, không phải trả `true`.
2. **Thà không chấm còn hơn chấm sai.** Nội dung không chấm được thì hiển thị badge trung thực ("Bài này chấm cấu trúc, chưa chạy thật trên thiết bị của bạn") và không cấp huy hiệu `Tự lực`.
3. **Self-test chạy trong runtime shipped.** Solution phải pass validation của chính nó trong đúng engine mà học viên dùng. Build fail nếu không.
4. **Offline-first là ràng buộc được cưỡng chế, không phải lời hứa.** CSP của origin sandbox là `connect-src 'none'`. Không lesson core nào yêu cầu mạng.
5. **Tất định là thuộc tính kiến trúc, không phải kỷ luật.** Thời gian và ngẫu nhiên chỉ đến từ `ctx` do host cấp. Kiểm chứng bằng cách chạy 2 lần cùng seed rồi so hash trace — bắt được cả nguồn phi tất định chưa biết.
6. **Không phạt khi sai ≠ mọi lần thử đều tính điểm.** Tiến độ luôn cho đi tiếp; mastery chỉ tính `firstAttemptCorrect` trên `variantId` chưa gặp.
7. **Mỗi lesson tối đa 1 khái niệm mới** (Realm 0–1) / 2 (từ Realm 3), tối đa 3 thuật ngữ mới, 7–12 phút, trần cứng 15. Lint cưỡng chế.
8. **Ký hiệu xuất hiện SAU trực giác.** Chuỗi Concrete → Representational → Abstract; lint chặn `math` node xuất hiện trước task `manipulate`/`classify` đầu tiên trong lesson.
9. **Không có hai nguồn sự thật.** Nội dung ở `content/`; `fp/*_Books` là bản sách read-only; `dist/` không bao giờ được sửa tay và nằm trong `.gitignore`.
10. **Mọi con số trong tài liệu phải sinh được bằng script.** Kiểm kê chương, độ phủ, ngân sách dung lượng, tỉ lệ conformance — đều có script sinh và đều in vào `docs/generated/`.

---

## 3. GIẢI QUYẾT MÂU THUẪN GIỮA 7 MẢNG

### 3.1 Bảng phân xử

| Mâu thuẫn | Mảng A | Mảng B | PHÁN QUYẾT | Lý do |
|---|---|---|---|---|
| Vị trí monorepo | `app-platform`: gốc repo | `rust-runtime`: `fp/platform/`; `curriculum-master`: `fp/curriculum/` | **Gốc repo** | `packages/byte-rust` đã nằm ở gốc; `fp/` phải giữ vai trò "thư viện sách", không được biến thành thư mục build |
| Rust ở đâu | `packages/byte-rust` (thực tế) | `crates/` (app-platform, curriculum-ai) | **`crates/`** — `mv packages/byte-rust crates/byte-rust` | Untracked nên di chuyển miễn phí; tách rõ TS/Rust cho Cargo workspace |
| Có backend không | `app-platform`: Tauri, không backend | `rust-runtime`: "gọi cargo sidecar", `curriculum-db`: `apps/desktop-lab` | **Không backend mạng. Tauri host process (Rust) = "sidecar cục bộ"** | Tauri host vốn là Rust; sidecar chỉ là tiến trình con trên máy học viên, không phải server. Điều này thoả offline-first và giải thích được sự "giả định có backend" |
| Chạy Rust thật ở mobile | `curriculum-master`: Tauri sidecar từ M3 | Phản biện `curriculum-ai`: Tauri KHÔNG hỗ trợ sidecar trên iOS/Android | **Sidecar CHỈ desktop.** Mobile/web dùng `byte-rust` WASM + expected-output | Đã kiểm chứng: tài liệu sidecar Tauri chỉ có phần desktop; App Store cấm ship compiler |
| Dừng vòng lặp | `app-platform`: bỏ SAB | `rust-runtime`: SAB là chính; phản biện: SAB không có trên 4/7 | **`terminate()` là chính, SAB là tối ưu**, khai báo qua `EngineCapabilities.cancellation` | Chỉ `terminate()` chạy trên đủ 7 nền tảng |
| Sandbox | `rust-runtime`: opaque-origin iframe + Worker | Phản biện: `new Worker` ném `SecurityError` trên origin `null` | **Origin thật khác + `allow-same-origin`** | Đây là cách duy nhất vừa cách ly IndexedDB vừa tạo được Worker vừa bật được COI |
| Parser Rust | `app-platform`/`rust-runtime`: `syn` 2 | Phản biện: `syn` dừng ở lỗi đầu tiên, không recovery; đề xuất `ra_ap_syntax` | **`byte-rust` parser tự có + BẮT BUỘC error recovery**; `syn` chỉ dùng phía CI (native) | 2.001 dòng lexer+parser đã viết, 0 dependency, chẩn đoán tiếng Việt; `ra_ap_syntax` kéo theo rowan + không cho chẩn đoán tiếng Việt |
| TS: Sucrase hay không | `curriculum-master`: "Sucrase chi phí bằng 0" | Phản biện: Sucrase không typecheck → 126 lesson typed-FP vô nghĩa | **`typescript` compiler API trong Worker** (1,6 MB gz, lazy) | TS là home language của ADT/type-level; không có diagnostic thì không dạy được |
| QuickJS | `rust-runtime`: QuickJS mặc định | Phản biện: rủi ro Effect-TS | **Worker engine thật mặc định; QuickJS opt-in per-lesson** sau spike đo pass-rate test suite Effect | Effect-TS là trục của bộ TS; không được đánh cược |
| Nguồn sự thật nội dung | `curriculum-master`: `skill-tree.yaml` hút markdown theo dải dòng | `content-schema`: `.lesson.md` là nguồn | **`.lesson.md` là nguồn; `skill-tree.v1.yaml` chỉ khai báo ID + prereq**, compiler LINK hai bên | Dải dòng chết ngay commit đầu; hai nguồn sự thật là chống chỉ định |
| Migrate tại chỗ hay copy | `content-schema`: "sửa tại chỗ" + `sourceHash` | Phản biện: hai thứ loại trừ nhau | **COPY sang `content/legacy/`, `fp/*_Books` READ-ONLY** | `sourceHash` chỉ có nghĩa khi bản gốc còn tồn tại; giữ được "sách đọc trên GitHub" |
| Đơn vị tương tác | `content-schema`: 9 `Step.kind` | `pedagogy`: 9 `TaskKind` | **Hợp nhất thành 12 `Step.kind`** (§5) | Hai danh sách bổ sung nhau, không mâu thuẫn |
| Sao vs huy hiệu | `content-schema`/`curriculum-master`: 1–3 sao | `pedagogy`: 4 huy hiệu | **Huy hiệu.** `stars` giữ trong schema như trường suy diễn (số huy hiệu) để không phá Dexie cũ | Sao trừ vì dùng hint là độc hại; nhưng UI cần một con số hiển thị |
| FSRS ưu tiên | `curriculum-master`/`pedagogy`: P0 | Phản biện: hàng chờ rỗng, bank 2.000–3.600 micro-task không tồn tại | **P1, gate: ≥80 skill × ≥3 biến thể** | Đúng thứ tự nhân quả |
| Fixture DB | `curriculum-db`: so trace với hệ thật | Phản biện: ScyllaDB phi tất định; VOPR chỉ TigerBeetle tái tạo được | **Đổi tên `referenceTraceRef`, chỉ để ĐỐI CHIẾU. Chấm bằng invariant trên trace của chính học viên** | Toán học không cho phép cách kia |
| Embedding tiếng Việt | `curriculum-ai`: MiniLM-L6-v2 23 MB | Phản biện: vocab 30.522 token chỉ có 2 token có dấu, `strip_accents` xoá dấu | **`multilingual-e5-small` + vocabulary trimming → ~30 MB**, có gold set đo trước | Nếu không, RAG dạy sai trực giác cốt lõi |
| Chấm prompt | `curriculum-ai`: pass@all trên byte-sim | Phản biện: mâu thuẫn với chính risk #1 | **Structure linter + cassette ranking**; prompt tự do chỉ ở tier ≥2 và không cấp huy hiệu | byte-sim không phải trọng tài chất lượng văn bản |
| Số quest DB | `curriculum-db`: 21 quest | Phản biện: pha (9) "secondary index/key-encoding" bị nuốt | **22 quest** — chèn `q09 Những lối tắt của Byte` | "from scratch" mà không tự xây index thì mất đỉnh expressiveness |

### 3.2 Ba mâu thuẫn lớn cần giải thích dài

**(a) "Mảng app chọn Tauri nhưng mảng runtime giả định có backend."**
Không có backend mạng nào cả. Nguồn gốc hiểu nhầm: Tauri **host process viết bằng Rust**, và trên desktop nó spawn được tiến trình con (`cargo`, `docker`, `tigerbeetle`). Đó là "backend cục bộ trên máy học viên", không phải server. Ba hệ quả chốt cứng:
- `crates/odyssey-tauri` chỉ phơi ra 4 command: đọc resource content pack, dò `rustup`/`cargo`, chạy sidecar có timeout+rlimit, đọc/ghi secure store. Không có command nào ra Internet.
- Mọi tính năng cần sidecar phải khai báo `requiresHost: 'native-toolchain'` trong schema và **bắt buộc có `fallback`**; compiler fail nếu thiếu.
- `services/sync/` (SurrealDB + axum) là repo con TÙY CHỌN, không nằm trong bất kỳ mốc phát hành nào, tồn tại chủ yếu như case study cho track Database.

**(b) "Nội dung tương tác không tồn tại" (phản biện chí mạng của mảng app).**
Đo được: 1.777 page, **0 page có validation**, 1 page có `code-editable`. Nguyên nhân gốc đã xác định: `ingest-v3.cjs:214` dùng regex `/\*\*Bài \d+\*\*:/` trong khi markdown viết `**Bài 1** (5 phút):` → 234/235 bài tập bị nuốt. **Sửa regex KHÔNG giải quyết vấn đề** — nó chỉ tạo ra 235 bài tập không có `solution`, không có `testCode`, không có thang hint. Phán quyết:
- Nội dung tương tác là **P0**, runtime là **P1**. Đảo ngược thứ tự đầu tư.
- `ingest-v4` chỉ được sinh **TIER-B/C**, gắn `reviewed: false`, và TIER-B/C **không bao giờ** cấp mastery Proficient/Mastered.
- TIER-A (bài tập thật) **viết tay hoặc sinh từ reference implementation**, không sinh từ markdown.
- Tuyên bố độ phủ được sửa lại: 159 chương ≈ **20–24%** cây học tập (không phải 36%), vì 88 chương ROSETTA không sinh lesson mà chỉ thành 2–4 step trong lesson home.

**(c) "Cổng build tự khoá chính nó."**
`check_coverage` + `RETIRE = 0` khiến build chỉ xanh khi cả 115.640 dòng được nhận hết. Phán quyết:
- `validate` = **BLOCKING** (DAG, ID, prereq, schema, self-test, Rust guard, AST registry).
- `coverage` = **REPORT-ONLY**, in `docs/generated/coverage.md` + xu hướng, không fail build.
- Cho phép `disposition: RETIRE`, chỉ tiêu mềm ≤15% dòng. "Không xoá file, chỉ hạ vai trò."

---

## 4. KIẾN TRÚC HỆ THỐNG

### 4.1 Shell cross-platform — CHỐT

**Quyết định:** Một app Svelte 5 ở `packages/app`, hai entry: `apps/web` (PWA) và `apps/shell` (Tauri 2.9).

| Target | Vỏ | WebView engine | Pyodide | byte-rust | cargo thật | COI (SAB) |
|---|---|---|---|---|---|---|
| macOS 13+ (universal) | Tauri 2.9 | WKWebView | bundle | WASM bundle | ✅ nếu có rustup | ✅ qua `app.security.headers` |
| Windows 10+ | Tauri 2.9 | WebView2 (Chromium) | bundle | WASM bundle | ✅ | ✅ |
| Linux (deb/rpm/AppImage) | Tauri 2.9 | WebKitGTK 4.1 ≥2.44 | bundle | WASM bundle | ✅ | ✅ |
| Chrome 120+ | PWA | Blink | lazy + Cache Storage | WASM lazy | ❌ | ✅ qua `_headers` |
| Firefox 128+ | PWA | Gecko | lazy + Cache Storage | WASM lazy | ❌ | ✅ |
| Android 8+ (minSdk 26, targetSdk 36) | Tauri 2.9 | Android System WebView ≥96 | bundle | WASM bundle | ❌ | ⚠️ phải kiểm chứng ở M2-gate |
| iOS/iPadOS 16+ | Tauri 2.9 | WKWebView | bundle | WASM bundle | ❌ | ⚠️ phải kiểm chứng ở M2-gate |

**Đường lui đã lượng hoá:** nếu M2-gate cho thấy Tauri mobile không dựng được (build fail hoặc COI không set được), chuyển **chỉ phần mobile** sang Capacitor 7. Chi phí ước tính 1–2 tuần **với điều kiện** mọi truy cập native đi qua đúng một file `packages/platform/src/adapter.ts`. File này là ràng buộc kiến trúc, CI lint chặn mọi `import '@tauri-apps/...'` ngoài nó.

**App Store 2.5.2 — chiến lược chốt:**
- Viện dẫn **mệnh đề giáo dục** trong chính 2.5.2, không dựa vào ngoại lệ riêng của Swift Playgrounds.
- Điều kiện bắt buộc thoả: *"must make the source code provided by the App completely viewable and editable by the user"* → **thêm tab "Test"** hiện nguyên văn `testCode`, cho sửa. Đây vừa là tuân thủ vừa là giá trị sư phạm (dạy đọc test).
- **Content pack hai lớp** (§4.7): lớp EXEC (`testCode`, `starterCode`, `solution`, `prelude`) đóng trong app bundle, ký số, khoá theo `appBuildId`. Trên iOS, `content-source` **từ chối** nạp lớp EXEC đến từ OTA. Lớp PROSE cập nhật OTA tự do.
- ATS whitelist rỗng ở bản submit đầu; Playground tự do tắt bằng feature flag.
- Kế hoạch B nếu bị từ chối: thay engine Python trên iOS bằng "trace-based replay" (kết quả chạy sẵn ở CI, phát lại), giữ interpreter thật trên 6 target còn lại. Kiến trúc `ExecutionEngine` đã tách nên chỉ đổi adapter.

**iOS storage eviction — rủi ro bị bỏ quên trong mọi thiết kế:** WebKit xoá toàn bộ script-writable storage (IndexedDB/OPFS/Cache/SW) sau **7 ngày không tương tác**, trừ khi web app được **Add to Home Screen**. Bắt buộc:
- Bản web trên iOS: onboarding yêu cầu A2HS **trước** khi cho tải bất kỳ asset >5 MB nào; gọi `navigator.storage.persist()` và **kiểm tra giá trị trả về**; nếu false → chặn nút tải model/pack lớn và hướng sang bản App Store.
- Bản Tauri iOS không bị ảnh hưởng (dữ liệu nằm trong app container).

### 4.2 Execution & sandbox — CHỐT

```
┌─ App realm (origin A: https://byteacademy.app | tauri://localhost) ──────┐
│  Svelte 5 UI · PixiJS · CodeMirror 6 · Dexie (TIẾN ĐỘ HỌC)              │
│                                                                          │
│  <iframe src="https://sandbox.byteacademy.app/frame.html"                │
│          sandbox="allow-scripts allow-same-origin">                      │
│  ┌─ Sandbox realm (origin B) ─────────────────────────────────────────┐  │
│  │ CSP: default-src 'none'; script-src 'self' 'wasm-unsafe-eval';     │  │
│  │      connect-src 'none'; img-src 'none'; frame-ancestors <origin A>│  │
│  │                                                                     │  │
│  │  new Worker('runner.js')  ← dedicated, 1 worker / 1 lần chạy       │  │
│  │    ├ pyodide-worker   (Pyodide 0.28, SimDisk, seeded random)        │  │
│  │    ├ ts-worker        (tsc diagnostics + transpile + exec)          │  │
│  │    ├ rust-worker      (byte_rust.wasm)                              │  │
│  │    ├ surreal-worker   (@surrealdb/wasm, mem://)                     │  │
│  │    └ embed-worker     (ONNX e5-small-trimmed)                       │  │
│  │  watchdog: wallClockMs → worker.terminate() → hoán đổi worker ấm    │  │
│  └─────────────────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────────────┘
```

Cách tạo origin B trên từng nền tảng:
| Nền tảng | Cơ chế |
|---|---|
| Web | Subdomain `sandbox.<domain>`, file `_headers` riêng |
| Tauri desktop | `register_uri_scheme_protocol("byte-sandbox", …)` + set header trong `get_response` |
| Tauri iOS | `WKURLSchemeHandler` thứ hai |
| Tauri Android | `WebViewAssetLoader` với host thứ hai |

**`packages/sandbox-host/src/selftest.ts` — RELEASE GATE, không phải cảnh báo.** Lúc khởi động app, khẳng định TẤT CẢ:
1. `new Worker()` bên trong sandbox **thành công**;
2. `indexedDB.databases()` bên trong sandbox **không thấy** DB của app;
3. `fetch('https://example.com')` bên trong sandbox **bị chặn**;
4. In `crossOriginIsolated` + `typeof SharedArrayBuffer` vào log khởi động.
Nếu (1)(2)(3) sai trên bất kỳ nền tảng nào ở CI phát hành → **fail build cho nền tảng đó**. Tuyệt đối không có "chế độ hạn chế im lặng".

**Hợp đồng huỷ (cancellation) trong `EngineCapabilities`:**
```ts
cancellation: 'interrupt' | 'terminate-only'
```
- `terminate-only` (mặc định, mọi nền tảng): watchdog → `terminate()` → hoán đổi worker **đã warm sẵn** → UI trả về trong <500 ms. Tiêu chí nghiệm thu M2: đo trên iPhone SE 2020 bản Tauri, không phải trên dev server.
- `interrupt` (khi `crossOriginIsolated === true`): `pyodide.setInterruptBuffer` → `KeyboardInterrupt`, giữ nguyên state interpreter, không phải nạp lại 13 MB.
- Lesson cần `input()` khai báo `requiresCaps: ['sab']`; content-lint **chặn** lesson như vậy nằm trên đường chính trừ khi có biến thể không-stdin.

### 4.3 Runtime Rust — CHỐT (ADR-002, thay thế ADR-001)

Đây là quyết định gây tranh cãi nhất giữa 7 mảng. Phán quyết dựa trên thực tế đã đo: `crates/byte-rust` có 3.653 dòng, 0 dependency, biên dịch được `wasm32-unknown-unknown`; và trên sự thật kỹ thuật: **NLL được định nghĩa trên MIR (CFG + region liveness), không thể tái tạo trung thực trên AST.** Một "NLL-lite trên AST" sẽ từ chối chính những chương trình mà rustc đã cho phép từ 2018 — tức dạy học viên một luật ownership KHÔNG TỒN TẠI.

#### 4.3.1 Bốn tầng, khai báo tường minh

| Tầng | Cơ chế | Nền tảng | Chấm được gì | Huy hiệu tối đa |
|---|---|---|---|---|
| **R-A** | `byte-rust` WASM: lex → parse (error recovery, nhiều lỗi/lần) → resolve → typecheck-lite → interpreter (fuel + heap cap + trace) → **move-check hẹp** | 7/7 | Chạy thật, output thật, panic thật, lỗi kiểu cơ bản, use-after-move trên code thẳng hàng | 4/4 |
| **R-B** | Đối sánh output: CI biên dịch `solution.rs` bằng **rustc thật**, ghi stdout + exit code + mã lỗi vào content pack **như DỮ LIỆU** | 7/7 | Đúng/sai theo output cho bài mà R-A trả `ChuaHoTro` | 3/4 (không có `Tinh gọn`) |
| **R-C** | AST-query trên cây cú pháp của `byte-rust` (thay regex `requiredPatterns`/`forbiddenPatterns`) | 7/7 | "phải dùng `match`", "cấm `unwrap()`", "cấm `.clone()` trong vòng lặp", "phải có trait bound `Ord`" | 2/4 |
| **R-D** | `cargo` thật qua Tauri sidecar (timeout + thư mục tạm + rlimit) | Desktop 3/7 | **Borrow checker thật**, lifetime, trait resolution, diagnostic gốc của rustc | 4/4, kèm badge "đã đối chiếu rustc" |

#### 4.3.2 Phạm vi `byte-rust` v1.0 — HỢP ĐỒNG ĐÓNG

**CÓ:** `let`/`mut`/shadowing · mọi kiểu số nguyên + float, overflow panic ở debug · `char`/`&str`/`String` · tuple/array/slice · `struct` (named/tuple/unit) · `enum` + generic enum · `impl` block · `fn` + closure `Fn`/`FnMut`/`FnOnce` + `move` · `if`/`if let`/`while`/`while let`/`loop`/`for`/`let else` · `match` đầy đủ + exhaustiveness · `?` · `Option`/`Result` + combinator · `Vec`/`VecDeque`/`HashMap`/`BTreeMap`/`HashSet`/`String` · `Box` · `Iterator` (30 adapter) + `IntoIterator`/`FromIterator` · `derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash,Default)` · `From`/`Into`/`Display` · `mod`/`use` · `const`/`static` · macro có sẵn `println!/format!/vec!/assert!/assert_eq!/assert_ne!/matches!/panic!/todo!/unreachable!` · `#[test]` + `#[cfg(test)] mod tests` + `#[should_panic]`.

**CÓ NHƯNG HẸN v1.1:** trait + default method + associated type · generic + `where` + trait bound · `dyn Trait` · `impl Trait` · `Rc`/`RefCell` với `strong_count` đúng.

**KHÔNG BAO GIỜ trong v1.x (trả `ChuaHoTro` kèm tên tính năng):** `async`/`await`/`Future`/`tokio` · `std::thread` thật · `unsafe`/raw pointer · `macro_rules!` do người học viết · const generics · GAT/HRTB/specialization · crate ngoài (serde/rig/reqwest/lancedb) · **borrow checker đầy đủ (NLL, reborrow, lifetime tường minh)**.

**Move-check hẹp (điều duy nhất về ownership mà R-A kiểm):** chỉ hai luật, cả hai quyết định được không cần CFG:
1. Dùng một binding không-`Copy` sau khi nó đã bị move, **trong cùng một block, trên chuỗi statement thẳng hàng, không có nhánh** → `E0382`.
2. Hai `&mut` tới cùng một place trong **cùng một statement** → `E0499`.
Mọi tình huống ownership khác (qua nhánh, qua vòng lặp, qua lifetime, reborrow) → `ChuaHoTro("borrow-check-cfg")` kèm câu tiếng Việt: *"Byte chưa đủ giỏi để kiểm tra luật mượn trong tình huống này. Mở bản Desktop và bấm 'Đối chiếu với cargo' để có câu trả lời chính xác."*

#### 4.3.3 Ownership được dạy ở đâu, nếu không phải bằng borrow checker

Bằng **world `memory-city`** — một micro-world tất định trong đó move/borrow là **DỮ LIỆU của world**, không phải kết quả phân tích code tuỳ ý:
- Ô nhớ = một ô lưới có chủ; "thẻ mượn" `&` = thẻ vàng (nhiều thẻ cùng lúc được), `&mut` = thẻ đỏ (độc quyền); `move` = chuyển chìa khoá.
- Task `assemble`/`predict`/`trace`/`classify` trên **tập tình huống viết tay** — học viên dự đoán "dòng nào sẽ bị compiler chặn và vì sao", rồi world diễn lại.
- Bài `code` ở tầng R-D (Desktop) cho học viên đối chiếu với `rustc` thật: **đây là con đường duy nhất có phản hồi borrow-check thật, và tài liệu phải nói thẳng điều đó.**

#### 4.3.4 Chống "runner giả thứ hai" — corpus ÂM là cổng, không phải corpus dương

Bài học từ phản biện: CI gate chỉ chạy trên lời giải đúng và 713 snippet trích từ sách (toàn code đúng) **không đo được** chế độ hỏng nguy hiểm nhất (false accept trên code sai của học viên).

`crates/byte-rust-conformance` (chỉ chạy trên CI native, có rustc thật) gồm **3 corpus**:
1. **Dương** — mọi code fence ```rust trong `content/legacy/rust/`, trích bằng `tools/inventory` (chạy TRƯỚC khi chốt phạm vi, không phải sau). So `stdout` + exit code.
2. **ÂM (cổng merge)** — mutation testing sinh tự động từ corpus dương: xoá `&`, đổi `&`→`&mut`, xoá `.clone()`, dùng biến sau move, đổi kiểu số, xoá nhánh `match`, hoán vị thứ tự drop. Mỗi mutant chạy qua **rustc thật**, ghi mã lỗi; `byte-rust` phải **hoặc** phát đúng mã đó **hoặc** trả `ChuaHoTro`. **Trả `Pass` cho một mutant mà rustc từ chối = FAIL MERGE.**
3. **UI subset** — lọc từ `rust-lang/rust/tests/ui` giới hạn trong feature set §4.3.2 (có sẵn expected stderr).

**Metric công bố** (in vào `docs/generated/rust-conformance.md`, hiển thị trong app ở màn hình "Byte biết gì về Rust"):
```
accept/reject agreement (âm)  : X %   ← CỔNG, ngưỡng ≥ 99 %, false-accept phải = 0
error-code agreement (âm)     : Y %   ← theo dõi, ngưỡng ≥ 70 %
stdout agreement (dương)      : Z %   ← theo dõi
ChuaHoTro rate                : W %   ← theo dõi, giảm dần theo phiên bản
```

#### 4.3.5 Hai cái bẫy ngữ nghĩa phải chốt ngay trong value model

- **`usize`/`isize` = 64-bit CỐ ĐỊNH cho MỌI build.** `struct Target { pointer_width: 64 }` trong `value.rs`. Tuyệt đối không dùng `usize` của host để biểu diễn `usize` của chương trình được thông dịch — nếu không, `usize::MAX`/`size_of`/overflow sẽ cho 3 kết quả khác nhau giữa WASM32, native 64-bit và rustc CI. Có test conformance riêng.
- **`HashMap` phải xáo trộn thứ tự duyệt theo seed ĐỔI MỖI LẦN CHẠY**, đúng như Rust thật. Nếu làm nó tất định để phục vụ chấm bài, học viên sẽ học sai rằng thứ tự duyệt `HashMap` ổn định. Conformance chỉ so sánh sau khi sort. Có một lesson riêng dạy đúng điểm này.

#### 4.3.6 Sửa ADR-001

`docs/decisions/ADR-001-rust-runtime.md` giữ nguyên (lịch sử), thêm header `**Trạng thái: SUPERSEDED bởi ADR-002**`. `ADR-002-rust-scope.md` ghi lại §4.3 này, đặc biệt:
- huỷ câu *"Ownership/borrowing được mô phỏng và KIỂM TRA — interpreter bắt buộc phải báo lỗi mượn giống compiler thật"*;
- sửa ước lượng: `byte-rust` v1 ≈ **9.000–13.000 dòng** (đã có 3.653), không phải 1.200; v1.1 (trait/generic/dyn) thêm ~5.000.

### 4.4 Runtime TypeScript

- **Type check** = `typescript@5.9` compiler API trong Worker: `ts.createProgram` với in-memory `CompilerHost`, bộ `lib.*.d.ts` rút gọn (chỉ `es2022` + `dom.iterable` khi cần), `getPreEmitDiagnostics()`. Đo được: `node_modules/typescript/lib/typescript.js` = 8,7 MB thô / ~1,6 MB gz → **lazy-load, tách khỏi bundle chính**, khai báo `requiresRuntime: ['ts-checker']` trong `PackMeta`.
- **Thực thi** = `ts.transpileModule` (hoặc `esbuild-wasm` 0.25 nếu đo nhanh hơn) → chạy trong Worker với global đã khoá: không `indexedDB`, không `fetch`, không `importScripts`, không `localStorage`. `console.log` bắt qua `postMessage`.
- **Thư viện** pre-bundle offline tại `tools/tslibs/`: `effect`, `fp-ts`, `zod`, `immutable`, `fast-check` → ESM bundle + import map. Không tải code lúc chạy (điều kiện 2.5.2).
- **QuickJS-ng** (`@jitl/quickjs-ng-wasmfile-release-sync`, ~1 MB) chỉ dùng cho lesson khai báo `determinism: true` + `stepMetering: true` (bài đo độ phức tạp). **Gate M3:** chạy test suite core + Fiber + Schedule của Effect bên trong QuickJS, ghi tỉ lệ pass vào `docs/generated/quickjs-effect.md`. Nếu <95%, bỏ QuickJS và đo step bằng **instrumented transpile** (esbuild plugin chèn bộ đếm ở mỗi backedge + call) — chạy trên mọi engine.
- **Bài tập cần DOM thật**: chạy trong iframe, và **chỉ ở chế độ preview không chấm** trên WebKit (macOS/iOS/Linux WebKitGTK), vì iframe cùng process không dừng được vòng lặp vô hạn. Content-lint chặn `kind:'code'` + `needsDom:true` + `gradable:true`.

### 4.5 Runtime Python

- Pyodide **0.28.x** (nâng từ 0.26.2), chạy trong Worker.
- `tools/pyodide/build-pyodide-pack.mjs`: tải + verify sha256, **nướng sẵn wheel pure-Python** vào `pyodide-lock.json` tuỳ biến: `numpy`, `returns`, `pyrsistent`, `toolz`, `hypothesis`, `micropip`, `packaging`. Không `micropip.install` từ PyPI lúc chạy (bắt buộc cho iOS 2.5.2 và cho offline). Xuất 2 biến thể: `bundle` (native) và `brotli` (web).
- **`SimDisk`** — phát hiện chí mạng của phản biện DB: Pyodide dùng MEMFS trong RAM, `os.fsync()` là no-op, không có sector, và `terminate()` xoá luôn MEMFS nên **sau "crash" không còn gì để recover**. Do đó `packages/simdisk` cung cấp một block device mô phỏng, có bản TS và bản Python song sinh cùng ngữ nghĩa: `read(sector)/write(sector)/fsync()`, hàng đợi ghi chưa fsync, và bơm được: torn write theo ranh giới sector, fsync bị mất, latent sector error, misdirected write, crash tại điểm tất định thứ N. **Mọi bài storage (Realm 6) lập trình trên `SimDisk`, không trên `open()`.** Kèm một page bắt buộc: "vì sao ta không dùng file thật".
- Tất định: `PYTHONHASHSEED=0`, `random`/`time` monkey-patch theo seed do host cấp, `sys.settrace` dùng cho `byte.probe`.

### 4.6 Runtime Database & AI

**SurrealDB — chạy THẬT.** `@surrealdb/wasm` 3.0.x, engine `mem://` cho bài học (reset giữa hai lần chạy, chấm bằng so sánh result set đã sort) và `indxdb://` cho capstone. Chạy trong Worker. Lazy-load theo quest, đóng gói offline như asset trong bản native. **Nhiệm vụ M0:** xác minh license bằng `npm view @surrealdb/wasm license` + đọc LICENSE trong tarball, ghi kết quả vào `docs/generated/licenses.md`; quy tắc quyết định: nếu không phải Apache-2.0/MIT thì hạ SurrealDB xuống mô phỏng và ghi ADR.

**ScyllaDB & TigerBeetle — KHÔNG chạy được trong WebView (Seastar cần io_uring/hugepage/pin-core; TigerBeetle cần io_uring/O_DIRECT).** Biến hạn chế thành nội dung: học viên **tự xây simulator** (TypeScript), và:
- **Chấm bằng invariant/property trên trace do CHÍNH code học viên phát ra**, không so khớp với hệ thật. Ví dụ: `sum(debits) == sum(credits)` sau mọi event; "mọi key đọc trả về version mới nhất"; "sau `compact`, số SSTable ở L1 giảm"; "write amplification nằm trong [X,Y]"; linearizability checker rút gọn cho VSR.
- Trace ghi từ hệ thật đổi tên thành **`referenceTraceRef`** và chỉ dùng để **hiển thị đối chiếu cạnh trace học viên** + so số liệu theo khoảng sai số khai báo trước. **Không dùng để pass/fail.** (ScyllaDB không tất định; VOPR seed 42 chỉ TigerBeetle tái tạo được.)
- Mỗi quest mô phỏng kết bằng page **"Mô phỏng này nói dối chỗ nào"**.
- Cluster thật chỉ ở `q20 Phòng thí nghiệm thật`, khai báo `requiresHost: 'native-toolchain'` + **ONLINE**, nằm NGOÀI mọi mốc phát hành bắt buộc. Ghi nhận: TigerBeetle có binary macOS/Windows chính thức → **không cần Docker**; chỉ ScyllaDB cần Docker.

**LLM — 4 tier sau một interface `LlmProvider`:**
| Tier | Tên | Tất định | Dung lượng | Dùng cho |
|---|---|---|---|---|
| T0 | `byte-sim` | ✅ | ~180 KB | Mặc định mọi lesson chấm điểm: cơ chế (budget, thứ tự, grounding, schema, vòng lặp) |
| T1 | `cassette` | ✅ | 1–3 MB/world | "Kiểm chứng thực tế" mỗi 4 lesson; ranking prompt |
| T2 | `local` | ❌ | 30 MB (embed) / 270 MB–1,1 GB (gen) | RAG thật; sinh chữ opt-in |
| T3 | `byok` | ❌ | 0 | Chỉ AI-7/8/10, không lesson core nào bắt buộc |

- **`byte-sim` được thiết kế để FAIL ĐÚNG CHỖ**: đếm token bằng BPE thật, cắt cụt khi tràn, mô phỏng lost-in-the-middle, bịa câu trả lời sai cố định khi fact vắng mặt, JSON hỏng khi prompt thiếu schema, tool-calling thật, suy giảm theo số turn nếu không compaction. Mỗi hành vi phải trích dẫn một hiện tượng có thật trong `packages/llm-sim/BEHAVIOR.md`.
- **`byte-sim` TUYỆT ĐỐI KHÔNG là trọng tài chất lượng văn bản.** Chấm prompt = (a) **structure linter tất định** (có khai báo output schema? few-shot nhất quán format? có negative constraint / điều kiện dừng? có đại từ mơ hồ? có nêu decoding params kèm lý do?) + (b) **ranking trên ma trận cassette** (học viên chọn/xếp hạng biến thể prompt đã ghi response thật và giải thích). Prompt tự do chỉ mở ở tier ≥2 và **không cấp huy hiệu**.
- **Embedding**: `multilingual-e5-small` (vocab XLM-R 250k, phủ tiếng Việt) + **vocabulary trimming** xuống ~30k token thực xuất hiện trong `corpus-vi` → ma trận embedding từ 250002×384 (96 M tham số) xuống ~30k×384 (11,5 M) → int8 ≈ **30 MB**. `tools/trim-vocab/` là deliverable P0 của track AI. **Gate bắt buộc trước khi viết lesson RAG:** dựng gold set ~200 cặp query-doc tiếng Việt, đo recall@5 của 3 phương án (MiniLM 23 MB / e5-small 118 MB / e5-small-trimmed 30 MB), công bố số vào `docs/generated/embed-bench.md`. Lý do: `all-MiniLM-L6-v2` có vocab 30.522 token trong đó **chỉ 2 token chứa dấu tiếng Việt**, và `strip_accents` theo `do_lower_case=true` sẽ **xoá dấu** → cosine chỉ còn tương quan với trùng lặp ký tự bề mặt.
- **Khoá API (T3)**: Desktop/Mobile → OS keychain qua Tauri; Web → **CHỈ `sessionStorage`**, không bao giờ IndexedDB/localStorage, kèm banner. Mọi lời gọi ra ngoài hiện dialog nêu endpoint + ước lượng token + chi phí, ghi audit log cục bộ xem được trong app. **Không bao giờ** nhúng key dùng chung của dự án, không bao giờ dựng proxy.

### 4.7 Content pipeline & content pack

**Pipeline 7 pha, chạy LÚC BUILD** (`packages/content-compiler`):

```
1 DISCOVER  glob content/**/*.lesson.md + *.yaml, cache theo mtime+sha
2 PARSE     unified 11 + remark-parse 11 + remark-directive 4 + remark-gfm 4
            → mdast → Step[] / RichText[]   (KHÔNG giữ markdown thô)
3 LINK      resolve ConceptId + SkillId, dựng DAG, phát hiện chu trình,
            mọi `requires` phải có ít nhất 1 lesson `teaches`, asset → sha256,
            LINK lesson ↔ skill-tree.v1.yaml (ID phải khớp hai chiều)
4 VALIDATE  (a) Zod 4 toàn bộ
            (b) content-lint (§7.4) — nhịp, khái niệm mới, CRA, hint, viewport
            (c) SELF-TEST trong RUNTIME SHIPPED (Pyodide-dưới-Node, byte-rust
                WASM, ts-worker, surreal-wasm). Solution không pass validation
                của chính nó → BUILD FAIL
            (d) RUST GUARD — lesson Rust khai tier compile/run/tests/output
                mà thiếu `requiresHost` + `fallback` → BUILD FAIL
            (e) AST REGISTRY — `AstQuery.kind` chưa đăng ký cho ngôn ngữ đó
                → BUILD FAIL (không có nhánh mặc định trả true)
5 EMIT      1 JSON/lesson theo locale; TÁCH thành 2 lớp: prose + exec
6 PACK      gộp theo MODULE → .json.gz + .json.br + sha256 + ký Ed25519;
            kiểm ngân sách maxPackKb; sinh index.json
7 REPORT    docs/generated/{coverage,inventory,budget,rust-conformance}.md
```

**Runtime KHÔNG parse markdown.** Gỡ `marked` 18 + `dompurify` 3.4.12 khỏi bundle app. `RichText` là cây JSON đã compile; Svelte 5 render trực tiếp từ node union. Ba lợi ích: không tốn CPU trên mobile, xoá bề mặt XSS, và mở được inline widget (`:concept[...]` tooltip, `$math$`, code annotation theo dòng) mà markdown không có.

**Content pack hai lớp — bắt buộc cho App Store:**
```
dist/content/vi/
├── index.json                          ~150 KB gz   tải lúc mở app
├── prose/<packId>.v<N>.json.br         150–500 KB   OTA tự do (GitHub Release)
└── exec/<packId>.v<N>.json.br          20–80 KB     TRONG app bundle, ký số,
                                                     khoá theo appBuildId
```
`packages/content-runtime` có 2 adapter: `fetch('/content/...')` cho web, `convertFileSrc` + Tauri asset protocol cho native. **Trên iOS, adapter từ chối nạp lớp `exec` nếu `origin !== 'bundle'`.**

**Cache offline có ghim (sửa lỗi `evictLRU` phá offline-first):**
```ts
interface PackCacheEntry {
  packId: string; pinned: boolean;
  source: 'prefetch' | 'user-download';
  bytes: number; lastUsed: number; sha256: string;
}
```
`evictLRU` chỉ được đụng `pinned:false`. Pack `user-download` mặc định `pinned:true`, chỉ xoá khi người dùng chủ động. API: `downloadTrackOffline(trackId)`, `offlineFootprint()`. `TrackMeta.totalPackBytes` để UI báo trước "Tải cả track: 48 MB".

---

## 5. SCHEMA NỘI DUNG v2 — CHỐT (`packages/content-schema/src/v2.ts`)

Đây là hợp đồng trung tâm. Mọi package khác phụ thuộc vào nó. Đóng băng trước khi viết lesson đầu tiên.

```ts
export const SCHEMA_VERSION = '2.0.0' as const;

/* ═══════════════ 1. ĐỊNH DANH ═══════════════ */

/** Mở rộng được: track DB cần surrealql/cql, không phá hợp đồng về sau. */
export type LangId =
  | 'python' | 'typescript' | 'rust'
  | 'surrealql' | 'cql' | 'sql' | 'text';

export type LocaleId = 'vi' | 'en';

export type Platform =
  | 'web-chrome' | 'web-firefox'
  | 'macos' | 'windows' | 'linux'
  | 'android' | 'ios';

export type TrackId =
  | 'onboarding' | 'programming-101' | 'math-discrete' | 'cs-core'
  | 'fp-core' | 'software-eng' | 'database-scratch'
  | 'system-design' | 'ai-genai-rag' | 'ai-applied';

export type Level = 'intro' | 'beginner' | 'intermediate' | 'advanced' | 'expert';

export type ConceptId = string;  // 'fp.functor.map'  — dotted, ổn định, không dịch
export type SkillId   = string;  // 'math.set.subset' — đơn vị đo mastery
export type LessonId  = string;  // '<track>.<module>.<slug>'
export type StepId    = string;  // '<lessonId>#<slug>'
export type ModuleId  = string;

/* ═══════════════ 2. RICHTEXT (compile sẵn, runtime không parse markdown) ═══════════════ */

export type RichNode =
  | { t: 'p';  c: Inline[] }
  | { t: 'h';  lvl: 2 | 3 | 4; c: Inline[] }
  | { t: 'ul' | 'ol'; items: RichNode[][] }
  | { t: 'code'; lang: LangId | 'bash' | 'json'; src: string;
      highlight?: [number, number][]; annotations?: { line: number; note: Inline[] }[] }
  | { t: 'callout'; variant: 'tip'|'warning'|'important'|'note'|'pitfall'; c: RichNode[] }
  | { t: 'table'; head: Inline[][]; rows: Inline[][][] }
  | { t: 'math'; tex: string; display: boolean }   // render: SVG pre-render lúc build
  | { t: 'img'; assetId: string; alt: string }     // alt BẮT BUỘC
  | { t: 'figure'; world: WorldRef; caption?: Inline[] };

export type Inline =
  | { t: 'txt'; v: string }
  | { t: 'b' | 'i' | 'code'; c: Inline[] | string }
  | { t: 'link'; href: string; c: Inline[] }
  | { t: 'concept'; id: ConceptId; c: Inline[] }   // tooltip + "ôn lại"
  | { t: 'kbd'; v: string }
  | { t: 'math'; tex: string };

export type RichText = RichNode[];

/* ═══════════════ 3. BYTE ═══════════════ */

export type ByteMood =
  | 'idle' | 'thinking' | 'happy' | 'blocked' | 'dizzy' | 'curious';
// CỐ Ý KHÔNG CÓ 'sad' / 'disappointed'. Sai là dữ liệu, không phải thất bại.

export type BytePose =
  | 'idle' | 'lean-in' | 'point-editor' | 'point-stage' | 'jump';

export type ByteTrigger =
  | { on: 'enter' }
  | { on: 'success' }
  | { on: 'fail-same-error'; nth: 2 }        // CHỈ khi lỗi GIỐNG NHAU lần thứ 2
  | { on: 'idle'; afterMs: 90_000 }
  | { on: 'suboptimal'; ratioOverOptimal: number }
  | { on: 'hint'; rung: number };

export interface ByteBeat {
  trigger: ByteTrigger;
  mood: ByteMood;
  pose?: BytePose;
  line: RichText;                    // ≤ 90 ký tự, đã là tiếng Việt
  spotlight?: 'editor' | 'stage' | 'console' | 'hint-button';
  /** Không bao giờ dùng thuật ngữ chưa dạy. Director lọc theo mastery. */
  requiresConcepts?: ConceptId[];
  cooldownMs: number;
  maxPerPage: 1;
}

/* ═══════════════ 4. WORLD & LIVE VIEW ═══════════════ */

export type RendererId = 'pixi' | 'svg';

export type WorldFamily =
  | 'grid-bot'        // lập trình nền tảng
  | 'machine-belt'    // FP: hàm thuần, compose, functor/monad
  | 'memory-city'     // CS + ownership Rust
  | 'set-garden'      // tập hợp, Venn, số học
  | 'graph-lab'       // đồ thị, quan hệ, ma trận kề
  | 'proof-bench'     // logic, luật suy diễn (bước sai → dây KHÔNG nối được)
  | 'number-line'     // số, phân số, toạ độ
  | 'blueprint-board' // DDD: sum type hở → bug BÒ RA
  | 'storage'         // page 4KB, B+Tree split/merge
  | 'lsm'             // memtable → flush → level, đếm amplification
  | 'ring'            // token ring, vnode, shard lane, quorum
  | 'ledger'          // double-entry, pending vs posted
  | 'vsr'             // replica, message arrow, view number, partition
  | 'traffic-city'    // system design: p50/p95/p99 + $/tháng
  | 'context-window'  // cửa sổ token như thanh nhiên liệu, thấy block bị evict
  | 'agent-trace';    // vòng lặp/graph agent chạy thật

export interface WorldRef {
  family: WorldFamily;
  params: Record<string, unknown>;
  seed: number;
}

export interface LiveViewConfig {
  world: WorldRef;
  renderer: RendererId;
  /** Số biểu diễn LIÊN KẾT ĐỘNG. >=2 ⇒ mobile BẮT BUỘC stack dọc, CẤM tab. */
  linkedViews: number;
  /** Sàn theo renderer: pixi >= 320x240 ; svg >= 360x420 (chiều CAO là ràng buộc thật) */
  minViewport: { w: number; h: number };
  onSuccess: 'pulse' | 'confetti' | 'flow' | 'none';
  onFailure: 'freeze-at-step' | 'shake' | 'none';
}

/** Hợp đồng code-học-viên → world. Code KHÔNG BAO GIỜ gọi PixiJS trực tiếp. */
export interface TraceEvent {
  t: number;                          // step counter, KHÔNG phải wall clock
  kind: 'call' | 'return' | 'alloc' | 'drop' | 'move' | 'borrow' | 'release'
      | 'print' | 'assign' | 'panic' | 'flush' | 'compact' | 'send' | 'recv'
      | 'tool-call' | 'evict' | 'retrieve';
  payload: Record<string, unknown>;   // ĐÃ CHUẨN HOÁ + SẮP XẾP (xem probe)
  depth: number;
}

/* ═══════════════ 5. CODE & PROBE ═══════════════ */

export type PolyCode = Partial<Record<LangId, string>>;

export interface CodeSlot {
  language: LangId;
  starter: string;
  /** BẮT BUỘC. Compiler dùng để self-test trong runtime shipped. */
  solution: string;
  test?: string;
  prelude?: string;                   // chạy trước, ẩn khỏi editor
  epilogue?: string;
  /** Chế độ "thử-đi": chỉ vùng này sửa được, phần còn lại readonly */
  editableRegions?: [number, number][];
  suggestions: string[];              // snippet cho suggestion bar (mobile)
  /** Biến global mà probe chiếu ra WorldPatch. Chuẩn hoá + SORT trước khi serialize. */
  probeVars?: string[];
  needsDom?: boolean;
}

/* ═══════════════ 6. VALIDATION NHIỀU TẦNG ═══════════════ */

export type ValidationTier =
  | 'static'     // AST query (KHÔNG regex trên source)
  | 'compile'    // tsc --noEmit / byte-rust typecheck / mypy
  | 'run'        // chạy được, không throw
  | 'tests'      // assertion trong test fence pass
  | 'output'     // stdout khớp (exact|trim|regex|json-deep)
  | 'trace'      // invariant/property trên TraceEvent[] của CHÍNH học viên
  | 'property'   // ∀-property + shrinking, seed đổi mỗi lần nộp
  | 'complexity' // hồi quy log-log trên step counter
  | 'style';     // lint/idiom — advisory, không chặn

export type Severity = 'blocking' | 'advisory';
export type HostReq  = 'wasm' | 'native-toolchain';
export type CapReq   = 'sab' | 'webgpu' | 'ts-checker' | 'surreal-wasm' | 'embed-model';

/** Discriminated theo NGÔN NGỮ — chống lỗi "no-unwrap trong lesson Python". */
export type AstQuery =
  | { lang: 'python'; kind: PyAstKind; target?: string; min?: number }
  | { lang: 'rust';   kind: RsAstKind; target?: string; min?: number }
  | { lang: 'typescript'; kind: TsAstKind; target?: string; min?: number };

export type PyAstKind =
  | 'match-stmt' | 'comprehension' | 'lambda' | 'recursion'
  | 'frozen-dataclass' | 'no-mutation' | 'pure-fn' | 'no-global'
  | 'uses-generator' | 'no-import';
export type RsAstKind =
  | 'match-expr' | 'enum-def' | 'impl-trait' | 'no-unwrap'
  | 'no-clone-in-loop' | 'uses-iterator-chain' | 'trait-bound' | 'no-mut-binding';
export type TsAstKind =
  | 'discriminated-union' | 'readonly-modifier' | 'no-let'
  | 'no-any' | 'exhaustive-switch' | 'uses-pipe' | 'no-nonnull-assert';

export type ValidationRule =
  | { tier: 'static'; id: string; severity?: Severity;
      requireAst?: AstQuery[]; forbidAst?: AstQuery[]; onFail: string }
  | { tier: 'compile'; id: string; severity?: Severity; strict?: boolean;
      requiresHost?: HostReq; requiresCaps?: CapReq[];
      /** Bài "phải làm compiler báo lỗi" — dạy được nhờ trường này. */
      expectDiagnostics?: { code: string; line?: number }[]; onFail?: string }
  | { tier: 'run'; id: string; timeoutMs: number; maxSteps?: number;
      requiresHost?: HostReq; onFail?: string }
  | { tier: 'tests'; id: string; testCode?: PolyCode; timeoutMs: number;
      requiresHost?: HostReq; onFail?: string }
  | { tier: 'output'; id: string; expected: string;
      match: 'exact' | 'trim' | 'regex' | 'json-deep';
      /** R-B: expected do CI sinh bằng rustc/CPython THẬT, lưu như DỮ LIỆU. */
      generatedBy?: { toolchain: string; version: string; at: string };
      onFail?: string }
  | { tier: 'trace'; id: string; assertions: TraceAssertion[]; onFail?: string }
  | { tier: 'property'; id: string; generator: PolyCode; runs: number;
      invariant: PolyCode; onFail?: string }
  | { tier: 'complexity'; id: string;
      /** Hồi quy log(steps) theo log(n) trên n ∈ ns; so alpha kỳ vọng ±0.35.
       *  CẤM dùng để phân biệt O(n) với O(n log n) — hai lớp đó KHÔNG tách được. */
      ns: number[]; expectedAlpha: number; tolerance: 0.35;
      /** Chỉ đếm thao tác MIỀN do std phát ra, không đếm step nội bộ interpreter. */
      countOnly: 'domain-ops'; severity?: Severity; onFail?: string }
  | { tier: 'style'; id: string; lint: 'ruff' | 'clippy' | 'eslint';
      maxWarnings: number; severity: 'advisory'; onFail?: string };

/** Vị từ trên chuỗi TraceEvent — KHÔNG so khớp bằng nhau với trace hệ thật. */
export interface TraceAssertion {
  id: string;
  desc: string;                          // tiếng Việt, hiện cho học viên
  kind: 'invariant' | 'ordering' | 'count' | 'ratio' | 'absence';
  expr: string;                          // DSL nhỏ, đánh giá trong sandbox
  tolerance?: number;
}

export interface ValidationSpec {
  rules: ValidationRule[];
  stopOnFirstBlocking: boolean;
  /** Rule advisory fail ⇒ mất huy hiệu `Tinh gọn`, KHÔNG chặn qua bài. */
  advisoryAffectsBadge: 'tinh-gon' | 'none';
}

/* ═══════════════ 7. TOÁN — Khan Academy / New Math ═══════════════ */

export type AnswerSpec =
  | { kind: 'numeric'; value: number; tolerance: number; unit?: string }
  | { kind: 'math-expr'; canonical: string; vars: string[];
      /** Tương đương ký hiệu bằng sampling: đánh giá 2 biểu thức tại N điểm ngẫu
       *  nhiên theo seed. Thuần TS, offline, không cần CAS. */
      equivalence: 'symbolic-sampling'; samples: number }
  | { kind: 'choice'; correct: string[]; multi: boolean }
  | { kind: 'ordering'; correct: string[] }
  | { kind: 'matching'; pairs: [string, string][] }
  | { kind: 'set-membership'; correct: string[] }   // "kéo vật vào rổ"
  | { kind: 'free-text'; gradable: false };         // KHÔNG BAO GIỜ cấp mastery

/** Sinh đề tham số hoá tất định theo seed — cùng seed = cùng đề. */
export interface GeneratorSpec {
  id: string;
  params: Record<string, { min: number; max: number; step?: number }>;
  /** Sinh {đề, đáp án, lời giải từng bước} — code TS thuần, có test riêng. */
  moduleId: string;
}

/* ═══════════════ 8. HINT (thang 5 bậc, thay hints[] phẳng) ═══════════════ */

export type HintKind =
  | 'attention'     // "Nhìn xem Byte đang quay mặt về hướng nào"
  | 'strategy'      // "Cần lặp 4 lần — có cách nào không phải chép 4 dòng?"
  | 'watch-byte'    // ★ MỚI: phát ANIMATION lời giải, KHÔNG hiện code
  | 'one-line'      // một dòng code cụ thể
  | 'solution';     // đầy đủ — BẮT BUỘC kèm task `trace` ngay sau

export interface HintRung {
  kind: HintKind;
  body: RichText;
  autoRevealAfterFailures?: number;   // chống bế tắc; solution KHÔNG BAO GIỜ auto
  byte?: ByteBeat;
}

export interface HintLadder {
  rungs: HintRung[];                  // index 0 = ít spoiler nhất
  /** Sau bậc `solution`, học viên PHẢI qua một step `trace` mới đi tiếp. */
  requireTraceAfterSolution: true;
}

/* ═══════════════ 9. STEP — union 12 nhánh ═══════════════ */

interface StepBase {
  id: StepId;
  title?: string;
  byte?: ByteBeat[];
  liveView?: LiveViewConfig;
  teaches?: SkillId[];
  requires?: SkillId[];
  concepts?: ConceptId[];
  estimatedSeconds: number;
  /** Item ôn tập FSRS chỉ lấy từ step có cờ này + có >=3 biến thể. */
  reviewable?: boolean;
  variantId?: string;
  determinism?: boolean;              // bật determinism harness (clock ảo + PRNG seed)
}

interface GradableBase extends StepBase {
  validation: ValidationSpec;
  hints: HintLadder;
  /** Số hành động tối ưu — dùng cho huy hiệu `Tinh gọn` VÀ chống brute-force. */
  optimalActions?: number;
  timeoutMs: number;
  maxSteps?: number;
}

export interface ExplainStep    extends StepBase { kind: 'explain'; body: RichText }
export interface ExampleStep    extends StepBase { kind: 'example'; body?: RichText;
                                                   code: CodeSlot; runnable: boolean }
export interface AssembleStep   extends GradableBase { kind: 'assemble';  // kéo block, không gõ
                                                   body: RichText; blocks: string[];
                                                   code: CodeSlot }
export interface CodeStep       extends GradableBase { kind: 'code';
                                                   body: RichText; code: CodeSlot }
export interface PredictStep    extends GradableBase { kind: 'predict';   // cam kết TRƯỚC khi chạy
                                                   body: RichText; answer: AnswerSpec;
                                                   code?: CodeSlot;
                                                   commitOnce: true }     // chỉ lần đầu tính điểm
export interface ManipulateStep extends GradableBase { kind: 'manipulate';
                                                   body: RichText; answer: AnswerSpec }
export interface TuneStep       extends GradableBase { kind: 'tune';
                                                   body: RichText;
                                                   knobs: { id: string; min: number;
                                                            max: number; step: number }[] }
export interface TraceStep      extends GradableBase { kind: 'trace';     // điền bảng trạng thái
                                                   body: RichText; code: CodeSlot;
                                                   rows: { at: number; vars: string[] }[] }
export interface RepairStep     extends GradableBase { kind: 'repair';
                                                   body: RichText; code: CodeSlot }
export interface RefactorStep   extends GradableBase { kind: 'refactor';  // đúng rồi, làm gọn hơn
                                                   body: RichText; code: CodeSlot }
export interface ClassifyStep   extends GradableBase { kind: 'classify';  // ĐỊNH NGHĨA ĐƯỢC PHÁT HIỆN
                                                   body: RichText;
                                                   items: { id: string; label: RichText;
                                                            correct: boolean }[];
                                                   requireWhy: boolean }
export interface CheckpointStep extends StepBase { kind: 'checkpoint';
                                                   quizzes: (PredictStep | ClassifyStep)[];
                                                   masteryThreshold: number;
                                                   gatesProgress: boolean }
export interface SandboxStep    extends StepBase { kind: 'sandbox'; code: CodeSlot }
export interface ReflectStep    extends StepBase { kind: 'reflect'; prompt: RichText;
                                                   rubric: RichText; gradable: false }

export type Step =
  | ExplainStep | ExampleStep | AssembleStep | CodeStep | PredictStep
  | ManipulateStep | TuneStep | TraceStep | RepairStep | RefactorStep
  | ClassifyStep | CheckpointStep | SandboxStep | ReflectStep;

/* ═══════════════ 10. LESSON ═══════════════ */

export type ContentTier = 'A' | 'B' | 'C';
// A = tương tác đầy đủ, viết tay, chấm thật, cấp mastery
// B = prose từ legacy + 2-3 task predict/trace sinh bán tự động, reviewed=false
//     ⇒ KHÔNG cấp Proficient/Mastered
// C = chế độ đọc thuần + glossary, mastery mức chương ⇒ KHÔNG lĩnh vực nào trống

export interface Lesson {
  schemaVersion: typeof SCHEMA_VERSION;
  id: LessonId;
  locale: LocaleId;
  track: TrackId;
  module: ModuleId;
  order: number;
  title: string;
  summary: string;
  level: Level;
  tier: ContentTier;
  languages: LangId[];
  defaultLanguage: LangId;
  /** Ngôn ngữ CHỦ của khái niệm này. 2 ngôn ngữ kia chỉ có Bước Rosetta. */
  homeLanguage?: LangId;
  rosettaOf?: LessonId;
  estimatedMinutes: number;            // 7..15, lint cưỡng chế
  steps: Step[];
  teaches: SkillId[];
  practices: SkillId[];
  requires: SkillId[];
  concepts: ConceptId[];
  assets: string[];
  /** Nền tảng nào chấm được tier nào. Thiếu ⇒ BUILD FAIL cho lesson Rust. */
  gradingMatrix: Partial<Record<Platform, ValidationTier[]>>;
  /** Bắt buộc khi lesson có step requiresHost:'native-toolchain'. */
  fallback?: { kind: 'recorded-trace' | 'reading-only'; ref?: string };
  requiresCaps?: CapReq[];
  provenance: {
    migratedFrom?: string;             // 'fp/Rust_Books/.../chapter_12_*.md'
    anchorId?: string;                 // neo ổn định, KHÔNG phải dải dòng
    sourceHash?: string;               // sha256 khối gốc — phát hiện drift
    authoredBy: 'human' | 'migration' | 'llm-assisted';
    reviewed: boolean;                 // false ⇒ chỉ vào bundle `beta`
  };
}

/* ═══════════════ 11. SKILL / CONCEPT GRAPH ═══════════════ */

export interface Skill {
  id: SkillId;
  label: string;
  /** ĐỊNH NGHĨA CHUẨN TIẾNG VIỆT DUY NHẤT. >1 định nghĩa ⇒ BUILD FAIL. */
  definition: RichText;
  track: TrackId;
  requires: SkillId[];
  languageAgnostic: boolean;
  bloom: 'remember' | 'understand' | 'apply' | 'analyze' | 'create';
  /** FSRS chỉ bật cho skill có >=3 biến thể review. Lint cưỡng chế. */
  reviewVariants: string[];
}

export interface SkillGraph { version: string; skills: Skill[] } // compiler đảm bảo DAG

/* ═══════════════ 12. MASTERY & TIẾN ĐỘ ═══════════════ */

export type MasteryLevel = 'not-started' | 'attempted' | 'familiar' | 'proficient' | 'mastered';
export type Badge = 'hoan-thanh' | 'tu-luc' | 'tinh-gon' | 'giai-thich-duoc';

export interface StepAttempt {
  stepId: StepId; lessonId: LessonId; at: number;
  /** ★ Chống đoán mò: mastery CHỈ tính lần trả lời đầu tiên. */
  firstAttemptCorrect: boolean;
  attemptsBeforeFirstPass: number;
  actionCount: number;
  variantId: string;
  hintsUsed: number;
  badges: Badge[];
  language: LangId;
  durationMs: number;
  platform: Platform;
}

export interface SkillMastery {
  skillId: SkillId;
  level: MasteryLevel;
  score: 0 | 50 | 80 | 100;
  /** proficient: firstAttemptCorrect trên variantId CHƯA GẶP + actionCount <= 2*optimal.
   *  mastered:   đúng skill đó trong "Chuyến tuần tra", >= 16h sau khi đạt proficient. */
  proficientAt?: number;
  // FSRS-5
  stability: number; difficulty: number; due: number; lapses: number;
}
// KHÔNG CÓ MŨI TÊN ĐI XUỐNG. Sai chỉ khiến FSRS xếp lịch ôn sớm hơn.

export interface LessonProgress {
  lessonId: LessonId; currentStep: number; completedSteps: StepId[];
  badges: Badge[]; stars: 0|1|2|3;      // suy diễn = số badge, giữ tương thích Dexie cũ
  lastAccessed: number;
}

/* ═══════════════ 13. PACK / INDEX ═══════════════ */

export interface PackMeta {
  id: string; locale: LocaleId;
  proseUrl: string; execUrl: string;
  proseBytes: number; execBytes: number;
  proseSha256: string; execSha256: string;
  signature: string;                    // Ed25519
  lessonCount: number;
  requiresRuntime: ('pyodide'|'byte-rust'|'ts-checker'|'surreal-wasm'|'embed-model')[];
  maxPackKb: number;                    // vượt ⇒ BUILD FAIL, buộc tách module
}

export interface ContentIndex {
  schemaVersion: typeof SCHEMA_VERSION;
  builtAt: string; appBuildId: string;
  locale: LocaleId;
  tracks: TrackMeta[];
  skills: SkillGraph;
  packs: PackMeta[];
}

export interface TrackMeta {
  id: TrackId; order: number; title: string; summary: string;
  coverEmoji: string; coverColor: string;
  languages: LangId[]; modules: ModuleMeta[]; requiresTracks: TrackId[];
  totalPackBytes: number;               // để UI báo "Tải cả track: 48 MB"
}

export interface ModuleMeta {
  id: ModuleId; track: TrackId; order: number;
  title: string; summary: string; coverEmoji: string;
  lessonIds: LessonId[]; packId: string; estimatedMinutes: number;
}

/* ═══════════════ 14. KẾT QUẢ THỰC THI ═══════════════ */

export type ExecStatus =
  | 'ok' | 'parse_error' | 'type_error' | 'move_error'
  | 'runtime_panic' | 'assertion_failed'
  | 'timeout' | 'step_limit' | 'oom' | 'output_limit'
  | 'forbidden_api'
  | 'unsupported_feature'      // ★ ChuaHoTro — KHÔNG BAO GIỜ được coi là pass
  | 'engine_crash';

export interface Diagnostic {
  severity: 'error' | 'warning' | 'help' | 'note';
  code?: string;                        // 'E0382', 'TS2345', 'TypeError'
  messageEn: string;                    // nguyên văn — để học viên Google được
  messageVi: string;                    // từ bảng ánh xạ theo MÃ LỖI, không dịch máy
  primarySpan?: { file: string; startLine: number; startCol: number;
                  endLine: number; endCol: number };
  labels: { span: NonNullable<Diagnostic['primarySpan']>; text: string }[];
  suggestion?: { span: NonNullable<Diagnostic['primarySpan']>;
                 replacement: string; label: string };   // quick-fix cho CodeMirror 6
}

export interface StepResult {
  status: ExecStatus;
  passed: boolean;
  badges: Badge[];
  ruleResults: { id: string; tier: ValidationTier; passed: boolean;
                 severity: Severity; message?: string }[];
  diagnostics: Diagnostic[];
  stdout: string; stderr: string;
  trace?: TraceEvent[];
  metrics: { wallMs: number; steps?: number; domainOps?: number;
             peakHeapBytes?: number; allocCount?: number };
  engineId: string; engineVersion: string;
  revealHintRung?: number;
}

export interface EngineCapabilities {
  compileDiagnostics: boolean;
  typeCheck: boolean;
  moveCheck: 'none' | 'narrow' | 'full';
  stepMetering: boolean;
  deterministic: boolean;
  cancellation: 'interrupt' | 'terminate-only';   // ★ khai báo, không giả định
  coldStartMsEstimate: number;
  residentBytesEstimate: number;
  supportedFeatures: string[];
  unsupportedFeatures: string[];                   // hiện trong "Byte biết gì"
}
```

---

## 6. ĐỊNH DẠNG NGUỒN `.lesson.md` — CHỐT (đã sửa lỗi parse)

**Lỗi chí mạng trong thiết kế cũ:** remark-directive yêu cầu container NGOÀI phải có **nhiều dấu hai chấm HƠN** container trong. `:::challenge` bọc `:::hints` cùng 3 dấu → `:::hints` ĐÓNG luôn `:::challenge`, và `:::validate` rơi ra ngoài. Golden lesson của thiết kế cũ **không parse được**.

**Thang colon chốt cứng** (lint chặn nếu con ≥ cha):
- Cấp Step: `::::` (4 dấu)
- Cấp con (`hints`, `validate`, `opt`, `why`, `solution`): `:::` (3 dấu)

```markdown
---
id: fp-core.py-fundamentals.control-flow
title: Rẽ nhánh và so khớp mẫu
locale: vi
track: fp-core
module: py-fundamentals
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: beginner
estimatedMinutes: 11
teaches: [ctrl.conditional, ctrl.match-destructure]
requires: [core.values-types]
gradingMatrix:
  web-chrome: [static, run, tests, output]
  ios:        [static, run, tests, output]
provenance:
  migratedFrom: content/legacy/python/part_1_fundamentals/chapter_06_control_flow.md
  anchorId: py-ch06.ternary-intro
  sourceHash: sha256-9f2a…
  authoredBy: human
  reviewed: true
---

::::byte{trigger=enter mood=curious pose=lean-in}
Máy tính cần biết *khi nào* rẽ trái, *khi nào* rẽ phải. Đi xem nhé!
::::

::::explain{#ternary}
`if/elif/else` là **statement**. Nhưng Python còn có dạng **expression**…
::::

::::predict{#guess-label commitOnce}
Byte sắp chạy đoạn này. Bạn đoán nó in ra gì?

```python title=readonly
x = 42
label = "chẵn" if x % 2 == 0 else "lẻ"
print(label)
```

:::opt{correct}
chẵn
:::
:::opt
lẻ
:::why
`42 % 2` bằng `0`, mà `0` là điều kiện SAI, nên… khoan, thử lại nhé.
:::
::::

::::code{#classify}
Điền vào chỗ trống để `classify(95)` trả về `"A"`.

```python title=starter
def classify(score: int) -> str:
    if score >= 90:
        return ___
    return "F"
```
```python title=solution
def classify(score: int) -> str:
    if score >= 90:
        return "A"
    return "F"
```
```python title=test
assert classify(95) == "A"
assert classify(10) == "F"
```

:::hints
- kind: attention
  body: Hàm phải TRẢ VỀ một chuỗi. Chuỗi trong Python viết trong dấu nháy.
- kind: strategy
  body: Điểm >= 90 thì xếp loại gì? Viết đúng chữ đó, có nháy kép.
- kind: one-line
  body: "`return \"A\"`"
:::

:::validate
- tier: static
  requireAst: [{ lang: python, kind: match-stmt }]
  onFail: Bài này yêu cầu dùng if/else, đừng gọi thư viện ngoài.
- tier: tests
  timeoutMs: 4000
:::
::::

::::checkpoint{mastery=0.8}
::::
```

**Ba quy tắc thi công bắt buộc:**
1. `content:new <track>/<module>/<slug>` sinh scaffold đúng thang colon — tác giả không bao giờ gõ tay `::::`.
2. Lint pha PARSE **fail build** nếu directive con có số colon ≥ directive cha.
3. **Không có text-directive tự chế** (`:why` là sai cú pháp). Giải thích option nằm trong container `:::why` bên trong `:::opt`.

---

## 7. CHẤM BÀI, MASTERY, LINT

### 7.1 Adjudicator — 7 tầng (`packages/grader`)

```
S0 GUARD       cỡ file, ký tự lạ, thời gian gõ bất thường → cảnh báo mềm
S1 STATIC      AST query THẬT (byte-rust AST / Python `ast` / ts-morph).
               KHÔNG regex trên source. kind chưa đăng ký → BUILD FAIL.
S2 COMPILE     tsc diagnostics / byte-rust typecheck / mypy.
               Lỗi ở đây = phản hồi DẠY HỌC, không mất huy hiệu.
S3 EXAMPLES    3–5 ca CÔNG KHAI (học viên xem được) — để debug.
S4 RANDOMIZED  sinh input từ GeneratorSpec bằng seed 128-bit MỚI mỗi lần nộp,
               so với reference impl chạy trong CÙNG engine (differential).
               + constant-answer detector: quét AST tìm literal trùng đáp án vừa sinh.
S5 PROPERTY    ∀-property + shrinking (round-trip, giao hoán, idempotent, invariant).
S6 TRACE/COMPLEXITY
               invariant trên TraceEvent[] của chính học viên;
               độ phức tạp bằng HỒI QUY log-log, KHÔNG bằng ngưỡng hằng số.
```

**Sửa lỗi ngưỡng độ phức tạp (phản biện đúng, tính lại):** với O(n log n), tỉ lệ step khi n: 100→1000 là `(1000·log₂1000)/(100·log₂100) = 9966/664 = 15,00`. Ngưỡng `< 15` **đánh trượt chính lời giải đúng**. Thay bằng:
- chạy `n ∈ {100, 200, 400, 800, 1600}`;
- fit `log(steps)` theo `log(n)`, lấy hệ số góc α;
- so với α kỳ vọng ±0,35 (O(n)→1,0; O(n log n)→~1,1; O(n²)→2,0);
- **CẤM dùng tiêu chí này để phân biệt O(n) với O(n log n)** — hai lớp đó cách nhau 0,1, không tách được an toàn;
- step counter chuẩn hoá: **chỉ đếm thao tác miền** (so sánh, hoán vị) do std phát ra, không đếm step nội bộ interpreter;
- `allocCount == 0` là bất khả thi với code Rust cho người mới dùng `Vec`/`String` → đổi thành `allocCount <= k·n`, `k` khai báo trong step.

### 7.2 Chống gian lận (không dựa vào bí mật)

App là client offline — giấu test là ảo tưởng và không nên giả vờ. Thay vào đó làm cho hardcode **vô dụng**: seed mới mỗi lần nộp → test case sinh ra từ `GeneratorSpec` → so với reference impl chạy cùng engine; cộng property ∀ có shrinking; cộng constant-answer detector. Mã hoá test trong JSON bị bác bỏ (khoá cũng nằm trong client).

### 7.3 Chống đoán mò (phản biện `pedagogy`, bắt buộc)

Tách **CHÍNH SÁCH TIẾN ĐỘ** khỏi **CHÍNH SÁCH ĐO LƯỜNG**: "không phạt khi sai" = luôn được đi tiếp; **không** có nghĩa mọi lần thử đều tính điểm.
- `predict`/`classify`: **chỉ lần trả lời ĐẦU TIÊN** tính mastery (`commitOnce: true`); lần sau vẫn học và vẫn qua nhưng ghi rating `Again(1)`.
- `Proficient` yêu cầu `firstAttemptCorrect === true` trên một `variantId` học viên **chưa gặp** — không chỉ "không dùng gợi ý".
- `assemble`/`manipulate`/`proof-bench`: thêm `actionCount <= 2 * optimalActions` (chính là điều kiện huy hiệu `Tinh gọn`, tái dùng được) — vì action space đã bị cắt còn toàn nước đi hợp lệ nên brute-force click chắc chắn về đích nếu không chặn.
- FSRS rating tính từ `firstAttemptCorrect` + `attemptsBeforeFirstPass`, không phải từ "pass/không gợi ý".

### 7.4 `packages/content-lint` — mỗi rule trong SPEC.md phải có một lint

| Rule | Ngưỡng | Mức |
|---|---|---|
| Thời lượng lesson | 7–12 phút, trần cứng 15 | error >15 |
| Khái niệm mới/lesson | ≤1 (Realm 0–1), ≤2 (từ Realm 3) | error |
| Thuật ngữ mới/lesson | ≤3, mỗi cái có glossary + cách đọc tiếng Việt | error |
| Prose giữa 2 task | >150 từ warning, >250 từ error | |
| Chuỗi CRA | node `math`/ký hiệu không được xuất hiện trước task `manipulate`/`classify` đầu tiên | error |
| `linkedViews >= 2` | mọi view phải ở CÙNG live-view pane; mobile stack dọc, CẤM tab | error |
| `minViewport` | pixi ≥320×240, svg ≥360×420 | error |
| Hint ladder | `challenge`/`code` phải có ≥2 rung; nếu có `solution` thì phải có `trace` ngay sau | error |
| Rust guard | lesson Rust có tier compile/run/tests/output mà thiếu `requiresHost`+`fallback` | error |
| AST registry | `AstQuery.kind` chưa đăng ký cho `lang` đó | error |
| Skill definition | mỗi `SkillId` đúng 1 định nghĩa | error |
| Ranh giới khái niệm AI | lesson dạy khái niệm không thuộc `concept_owner` của world mình | error |
| Determinism | `Math.random`/`Date.now`/`crypto.getRandomValues` trong `packages/worldkit/src/worlds/**` và `packages/sim-*/src/**` | error |
| Review bank | skill đánh dấu `reviewable` mà có <3 `reviewVariants` | error |
| Alt text | mọi `img` node phải có `alt` | error |

### 7.5 `byte.probe` — giao thức code ↔ world (thay "cùng một state")

Phản biện đúng: `ro = {"xoài",…}` tạo một `set` CPython trong heap Pyodide, nó **không thể LÀ** state của world SVG bên TS. Giao thức chốt (`packages/worldkit/src/probe.ts`):

1. World **LÀ nguồn chân lý duy nhất** cho render. Code học viên phát ra **PATCH**, không chia sẻ state.
2. Worker chạy code với `sys.settrace` (Python) / instrumented transpile (TS) / trace hook (byte-rust), bắt sự kiện `line`.
3. Sau mỗi dòng, serialize một **PROJECTION CHUẨN HOÁ** của các global khai trong `CodeSlot.probeVars`. Với `set`/`dict`/`HashMap`: **SẮP XẾP theo khoá chuẩn hoá**, tuyệt đối không dùng thứ tự lặp.
4. Sinh `WorldPatch[]` — chính là `TraceEvent[]`.
5. `PYTHONHASHSEED=0` + test CI chạy 2 lần so hash chuỗi state.
6. Cùng một giao thức cho `machine-belt` và `byte.probe.tree()` của track DB — **không phát minh cơ chế thứ hai**.

---

## 8. SƯ PHẠM & GAMEPLAY

### 8.1 Byte — nhân vật

**Byte là BẠN HỌC, không phải giáo viên.** Bốn vai có vị trí hiển thị khác nhau:
- **Byte-Diễn viên** — nhân vật trong world, KHÔNG nói (mặc định).
- **Byte-Tường thuật** — bubble cạnh live view, ≤90 ký tự, mô tả việc vừa xảy ra.
- **Byte-Bạn học** — đặt câu hỏi ngược, chỉ trong `predict`.
- **Byte-Thợ máy** — chỉ khi bấm nút Gợi ý.

Xưng "mình", gọi người học là "bạn". **Byte LUÔN nhận lỗi về mình**: *"Mình đâm vào tường ở bước 3"*, không bao giờ *"Bạn sai rồi"*. Kho thoại `packages/byte/src/dialogue/vi.json` **không có dòng nào chứa chữ "sai"**.

**Ngân sách phát ngôn — cưỡng chế bằng pure reducer, unit-test được** (`packages/byte/src/director.ts`):
- ≤1 phát ngôn chủ động / lesson-page ngoài phản hồi do học viên kích hoạt;
- im lặng tuyệt đối 3 giây sau keystroke cuối;
- bubble không bao giờ modal, tự tan sau 8 s hoặc khi gõ phím;
- sau 2 lần bị bỏ qua → tắt chế độ chủ động hết phiên;
- Settings 3 mức "Đầy đủ / Ít nói / Tắt"; khi **Tắt**, gợi ý vẫn còn nguyên trong hộp trung tính — tắt Byte không bao giờ đồng nghĩa với học kém đi;
- lọc theo `requiresConcepts` — không bao giờ dùng thuật ngữ chưa được dạy.

**Bốn trigger chủ động duy nhất:** (a) fail **cùng một lỗi** 2 lần liên tiếp → NGỎ Ý gợi ý bậc 1, không tự mở; (b) 90 s không gõ và chưa chạy được → tóm tắt mục tiêu 1 dòng; (c) pass nhưng >2× số bước tối ưu → `curious`, mời refactor, không bắt buộc, không mất gì; (d) pass lần đầu không gợi ý → `happy`, 1 câu rồi im.

*Lý do (a) đợi lỗi thứ hai GIỐNG NHAU: lỗi khác nhau nghĩa là học viên đang tiến bộ, can thiệp lúc đó là cướp mất khoảnh khắc tự khám phá.*

### 8.2 `World<S,A>` — trừu tượng cốt lõi

```ts
interface World<S, A> {
  id: WorldFamily;
  init(seed: number): S;
  step(s: S, a: A): { state: S; events: TraceEvent[] };   // thuần, tất định
  goal(s: S): 'unmet' | 'met' | 'violated';
  invariants?: Array<(s: S) => Violation | null>;
  describe(s: S): string;                 // a11y + chế độ "kể lại bằng lời"
  rendererId: RendererId;
  minViewport: { w: number; h: number };
}
type Trace<A> = A[];   // → replay từng bước, tua được, phát lại lời giải
```

Đây là điều kiện đủ cho cả 6 bất biến làm nên hiệu quả của Swift Playgrounds: state hữu hạn quan sát được; mục tiêu hiển nhiên (không cần đọc assert để biết mình đúng); lỗi hiện ở ĐÚNG BƯỚC sai; sai không tốn kém; nhiều lời giải đúng (nên refactor 4 lệnh lặp thành `for` chính là cách giới thiệu khái niệm kế tiếp); "Xem giải pháp" phát lại được.

**Hợp đồng cứng:** world mới phải viết được **dưới 400 dòng logic** và BẮT BUỘC tái dùng renderer primitive từ `packages/worldkit/src/render/` (node, edge, token, meter, grid, table). Mọi world có test snapshot trên **STATE** (không phải pixel).

**Hai renderer, một interface:** `pixi` cho world nhiều hạt/chuyển động (`grid-bot`, `machine-belt`, `traffic-city`); `svg` (Svelte 5 component) cho world dạng sơ đồ/bảng/ký hiệu (`set-garden`, `graph-lab`, `proof-bench`, `blueprint-board`, `storage`, `ledger`, `context-window`). Lý do: text trong canvas WebGL bị mờ khi zoom, không select được, vô hình với screen reader — mà đây là học liệu cho người mới trên màn hình 5 inch.

### 8.3 Nhịp một lesson

```
[Byte mở màn 1 câu]
  → Task 1  manipulate/assemble  (30–90 s)  ≤120 từ đọc
  → Task 2  predict              cam kết dự đoán TRƯỚC khi chạy
  → Task 3  predict/classify     ← ĐẶT TÊN khái niệm Ở ĐÂY, không sớm hơn
  → Task 4  explain (abstract)   ← KÝ HIỆU xuất hiện lần đầu, kèm cách đọc
  → Task 5  code/assemble        ← live view phản ứng với code qua byte.probe
  → Task 6  checkpoint (vì sao?)
[Sổ tay của Byte: 3 dòng, tự sinh từ teaches[]]
```
Tỉ lệ mục tiêu: đọc 25% / quan sát 20% / làm 50% / ôn 5%, đo bằng `packages/telemetry` (cục bộ, không gửi đi đâu).

**Bậc thang input cho người mới:** Lesson 1–20 chỉ `assemble` (kéo block — không sai được cú pháp, và gõ code trên điện thoại là rào cản vật lý); Lesson 21–40 gõ trong `editableRegions` (phần còn lại readonly như placeholder của Playgrounds); từ Lesson 41 gõ tự do. Trên di động, `assemble` là input mặc định ở mọi lesson có cả hai biến thể.

### 8.4 Năm quy tắc New Math + Common Core (cưỡng chế bằng lint)

1. **Chuỗi CRA bắt buộc** — Concrete → Representational → Abstract; **ký hiệu luôn cuối cùng**.
2. **Tối thiểu 2 biểu diễn liên kết động** — đổi bên này, bên kia sáng lên. **Chúng nằm TRONG CÙNG live-view pane** (SVG xếp chồng dọc), không phải các pane/tab khác nhau, nên chuyển tab trên mobile không bao giờ tách chúng ra.
3. **Định nghĩa được PHÁT HIỆN chứ không được PHÁT** — học viên phân loại ví dụ/phản ví dụ trước, hệ thống mới đặt tên.
4. **Quy mô nhỏ, bối cảnh Việt Nam** — 5–7 phần tử, đồ vật thật (xoài, ổi, thước kẻ), không ví dụ dịch máy.
5. **Nói thành lời trước khi viết ký hiệu** — mỗi ký hiệu mới kèm cách đọc ("∈" đọc là "thuộc").

**Unit mẫu vàng `math.set` — "Cái rổ của Byte"** (7 lesson, viết tay, khuôn cho mọi unit sau): rổ tre rỗng + 8 vật kéo được → `manipulate` "rổ CHỈ ĐỰNG TRÁI CÂY", không có nút Run, phản hồi tức thì, vật sai bị đẩy ra kèm *"Cái này mình không xếp vào rổ trái cây được"* (không có chữ "sai"). **Chỉ SAU KHI** rổ đủ 5 quả, hệ thống mới đặt tên "tập hợp (set)". → `predict` "bỏ thêm quả xoài nữa, rổ có 6 món không?" → xoài thứ hai TAN BIẾN ⇒ tự phát hiện tính duy nhất. → `predict` "xếp lại thứ tự, còn là cùng một rổ không?" ⇒ tính vô thứ tự. → **Bây giờ mới** hiện `A = {xoài, ổi, chuối, cam, sầu riêng}`, `bút chì ∉ A`, `|A| = 5`. → `code`: `ro = {...}` rồi `ro.add("xoài")` rồi `print(len(ro))`, và live view phản ứng qua `byte.probe`.

### 8.5 Thang trợ giúp 5 bậc

Bậc 1 hướng sự chú ý → Bậc 2 chiến lược → **Bậc 2.5 "XEM BYTE LÀM"** (phát animation lời giải trên live view, **KHÔNG hiện code**) → Bậc 3 một dòng code → Bậc 4 giải pháp đầy đủ, **BẮT BUỘC kèm step `trace` ngay sau** mới được đi tiếp.

*"Xem Byte làm" là cải tiến then chốt so với Swift Playgrounds:* lỗ hổng lớn nhất của "See a solution" là cho phép chép mù và đi tiếp. Tách phần WHAT (kết quả trông thế nào) khỏi phần HOW (code) giữ được giá trị gỡ bí mà không cướp mất phần suy nghĩ.

Dùng gợi ý **không mất** huy hiệu `Hoàn thành`; chỉ ảnh hưởng huy hiệu `Tự lực` và rating FSRS.

---

## 9. SKILL TREE ĐẦY ĐỦ — CHỐT

**Hai artifact, hai chính sách khác nhau** (sửa lỗi "đóng băng 282 quest sẽ không bao giờ xây xong"):

| Artifact | Nội dung | Được validate? | Vào build? |
|---|---|---|---|
| `content/curriculum/skill-tree.v1.yaml` | **CHỈ R0 + R1 + R2.T1–T3 + R3.T1–T2**, mức lesson, ~180 lesson TIER-A | ✅ blocking | ✅ |
| `content/curriculum/roadmap.yaml` | R2.T4–T7, R4–R9, mức **TRACK**, không mức lesson | ❌ | ❌ |
| `content/curriculum/BACKLOG.md` | Mọi đề xuất mở rộng (ngôn ngữ thứ 4, realm mới) | ❌ | ❌ |

### 9.1 Cây đầy đủ 10 Realm (trạng thái đóng băng ghi rõ)

```
R0  BÃI ĐÁP CỦA BYTE                         [v1 FROZEN]  2 track ·  40 lesson · 100% MỚI
    T0.1 Máy tính nói gì (18)  — bit/byte, file/thư mục, terminal mô phỏng, chương trình là gì
    T0.2 Ra lệnh cho Byte (22) — print(), input(), đọc traceback, BOSS: máy trả lời tự động
    world: grid-bot · Ngôn ngữ: Python (6 lesson đầu KHÔNG cần code)

R1  NỀN TẢNG LẬP TRÌNH                       [v1 FROZEN]  5 track · 150 lesson
    T1.1 Giá trị, biến & kiểu (30)      T1.2 Rẽ nhánh & lặp (28)
    T1.3 Hàm — viên gạch (30)           T1.4 list/dict/set/tuple (34)
    T1.5 Chương trình thật: file, module, CLI, debugger (28)
    BOSS: "Sổ chi tiêu của Byte" — CLI đọc/ghi file, 30 test tự động
    Nguồn: PY Ch4–10 (1.477 dòng) → VIẾT LẠI mở rộng ×4-5. Đây là nợ nội dung lớn nhất.

R2  TOÁN & TOÁN RỜI RẠC                      [T1–T3 v1 FROZEN · T4–T7 roadmap]
    T2.1 Cảm nhận số (44)      T2.2 Đại số & hàm số (36)   T2.3 Logic & chứng minh (32)   ← v1
    T2.4 Tập hợp, quan hệ, ánh xạ (28)   T2.5 Tổ hợp, xác suất, thống kê (34)
    T2.6 Đồ thị · modular · đại số trừu tượng (28)  T2.7 ĐSTT & giải tích cho AI (28)
    world: set-garden, number-line, graph-lab, proof-bench
    Mọi bài luyện là GeneratorSpec tham số hoá; mastery Khan = 5 câu đúng liên tiếp.
    Ch1 Math Foundations ×3 (lambda calculus, Curry–Howard) HẠ KHỎI vị trí mở đầu
    → tái định vị làm ĐỈNH T2.6.

R3  KHOA HỌC MÁY TÍNH                        [T1–T2 v1 FROZEN · T3–T5 roadmap]
    T3.1 Bit, byte & bộ nhớ (28)  ← dọn đường cho ownership   T3.2 Cấu trúc dữ liệu (36)
    T3.3 Thuật toán & độ phức tạp (34)   T3.4 Máy chạy thế nào (28)
    T3.5 Ngôn ngữ & máy trừu tượng (24)  ← khoá liên thông tới parser (R4.T6) và SQL (R6.T3)
    world: memory-city
    Nguồn TÁI DÙNG: Ch2/Ch3/Ch3B/Ch3C ×3 sách

R4  FUNCTIONAL PROGRAMMING                   [roadmap]     8 track · 260 lesson
    T4.0a ⛩ CỔNG TYPESCRIPT (18)   T4.0b ⛩ CỔNG RUST (24)
    T4.1 Bất biến & thuần khiết (24) [PY]   T4.2 HOF & composition (30) [PY]
    T4.3 ADT & pattern matching (36) [TS]   T4.4 Generics · Traits · Type classes (36) [RS]
    T4.5 Đại số của chương trình (54) [TS] ← Monoid→Functor→Applicative→Monad→Traverse→Cata
    T4.6 Parser combinators & interpreter (38) [RS]
    ★ Khối TÁI DÙNG lớn nhất: 24 chương FP Patterns + 22 chương DDD

R5  SOFTWARE ENGINEERING                     [roadmap]     6 track · 170 lesson
    T5.1 Git (20, MỚI, git mô phỏng WASM offline)  T5.2 TDD→Property→Mutation (34)
    T5.3 DDD (42) [TS] ★ khối tái dùng lớn thứ 2   T5.4 Patterns & refactoring (24)
    T5.5 CI/CD & observability (24)                T5.6 Bảo mật ứng dụng (26)
    world: blueprint-board (sum type hở → bug BÒ RA)

R6  DATABASE TỪ SỐ 0                         [roadmap]     22 quest · ~227 lesson
    ★ LÃNH THỔ RUST. Chi tiết §9.2.

R7  SYSTEM DESIGN                            [roadmap]     4 track · 120 lesson
    T7.1 Nền tảng phân tán (30)   T7.2 Thiết kế thực chiến (34)
    T7.3 ★ SYSTEM DESIGN FOR FUNCTIONAL (30, ~70% MỚI):
         log bất biến = fold ở quy mô hệ thống · functional core/imperative shell cấp DỊCH VỤ ·
         idempotency & exactly-once như tính chất đại số · CRDT = commutative monoid ·
         streaming/backpressure/effect system ở biên
         skill namespace: sd.fp.*    world: traffic-city + blueprint-board
    T7.4 Triển khai & vận hành (26)

R8  AI · GENERATIVE AI · RAG                 [roadmap]     6 track · 190 lesson
    ★ LÃNH THỔ PYTHON (numpy thuần trong Pyodide, KHÔNG torch)
    T8.1 Máy học nền tảng (30)   T8.2 Mạng nơ-ron từ số 0 (32, autograd ~200 dòng)
    T8.3 Transformer từ số 0 (40, BPE tay → attention tính tay trên 4 token → micro-transformer
         2 tầng numpy, corpus 5KB, chạy <2s)  [+ lab TUỲ CHỌN GPT-2 đầy đủ, notebook, KHÔNG chấm]
    T8.4 Dùng LLM đúng cách (28)  T8.5 RAG từ số 0 (34, HNSW tự cài, BM25 hybrid)
    T8.6 Hạ tầng & vận hành AI (26) ← TÁI DÙNG NGUYÊN

R9  KỸ NGHỆ ỨNG DỤNG AI                      [roadmap]     7 track · 180 lesson
    ★ LÃNH THỔ TYPESCRIPT. Chi tiết §9.3.
```

**Sáu cơ chế chống nhàm chán khi dạy 3 ngôn ngữ:**
1. **Ngôn ngữ chủ** — mỗi khái niệm dạy ĐÚNG MỘT LẦN, ở một ngôn ngữ.
2. **Bước Rosetta** — 2–4 step cuối lesson chỉ nói ĐỘ LỆCH ở 2 ngôn ngữ kia.
3. **Thử thách dịch** — port code chạy được sang ngôn ngữ khác, chạy CÙNG bộ test.
4. **Đấu trường ba cổng** — boss quest giải 1 bài ở cả 3 ngôn ngữ, mỗi lần lộ một sự thật riêng.
5. **Bảng đối chiếu sống** — sinh từ 10 phụ lục sẵn có, tra cứu mọi lúc.
6. **Lãnh thổ độc quyền** — R6=Rust, R8=Python, R9=TypeScript. Học Rust vì KHÔNG THỂ xây storage engine bằng gì khác trong khoá này, chứ không phải vì bị bắt.

**Ràng buộc bảng ánh xạ 159 chương** (sửa mâu thuẫn 46-vs-64 và số liệu sai tới 5,4×):
- Bảng sinh từ `tools/inventory` (`find` trên đĩa), **không gõ tay**.
- Ràng buộc kiểm được trong `validate`: với mỗi `topic_id`, số chương `disposition=REUSE` phải **ĐÚNG BẰNG 1**; hai chương còn lại bắt buộc `ROSETTA` hoặc `REWRITE`. Rule này tự động phát hiện mâu thuẫn ở khối FP Patterns (đang liệt kê đủ cả 3 bản là REUSE) và khối DDD.
- Neo là `anchorId` (`<!-- anchor: py-ch11.purity-intro -->` chèn một lần vào **bản copy** trong `content/legacy/`) + `sha256` khối, **không phải dải dòng**.

### 9.2 Realm 6 — Database (22 quest, 3 mốc phát hành độc lập)

```
R6-1 (68 lesson)  q00 Chiếc hộp giày của Byte (6)  — byte/file/fixed-width record  [PY, SimDisk]
                  q01 Nhật ký không bao giờ quên (9) — append-only log + hash index Bitcask
                  q02 Khi điện mất (8)              — WAL, fsync, torn write, CRC32, recovery
                  q03 Cây của người thủ thư (12)    — B+Tree trên page 4KB, split/merge, free list
                  q04 Thác dữ liệu (14)             — LSM: skiplist, SSTable, bloom, compaction [TS]
                  q05 Bốn chữ cái ACID (8)          — lock table, 2PL, deadlock detection
                  q06 Nhiều dòng thời gian (11)     — MVCC, snapshot isolation, SSI, ARIES-lite
R6-2 (96 lesson)  q07 Ngôn ngữ của Byte (12)        — lexer + Pratt parser + AST mini-SurrealQL
                  q08 Bộ não của truy vấn (12)      — logical plan, pushdown, Volcano executor
                  q09 ★ Những lối tắt của Byte (10) — key-encoding có thứ tự, secondary index,
                        graph edge = 2 index entry hai chiều, HNSW-lite   [BỔ SUNG, thiếu ở bản cũ]
                  q10 Một hộp, nhiều hình (14)      — SurrealDB THẬT qua @surrealdb/wasm
                  q11 Vòng tròn quyền lực (9)       — murmur3, consistent hashing, vnode
                  q12 Mỗi lõi một vương quốc (10)   — shard-per-core, share-nothing
                  q13 Đếm phiếu (12)                — RF/CL quorum, hinted handoff, read repair, Merkle
                  q14 Bia mộ và người dọn rác (9)   — tombstone, gc_grace, lab STCS/LCS/TWCS
                  q15 Mô hình hoá theo câu hỏi (8)  — wide-column, partition key, hot partition
R6-3 (63 lesson)  q16 Sổ cái không sai một xu (12)  — double-entry, 128B DataView, two-phase, linked
                  q17 Vũ trụ tất định (8)           — event loop tất định, xorshift128+, clock ảo
                  q18 Kẻ phá hoại có chủ đích (11)  — simulated network + SimDisk fault, mini-VOPR
                  q19 Đồng thuận kiểu VSR (14)      — prepare/prepare_ok/commit, view change, ST
                  q20 Phòng thí nghiệm thật (8)     — ONLINE, desktop-only, ngoài mọi mốc bắt buộc
                  q21 ByteLedger — Capstone (10)
```
Tam giác ba hệ: **SurrealDB = expressiveness** (chạy THẬT, WASM, 7/7) · **ScyllaDB = scale** (mô phỏng, TS) · **TigerBeetle = correctness** (mô phỏng, TS). Mỗi hệ dạy đúng một thứ hai hệ kia không dạy được.

**BOSS R6 hạ mục tiêu** từ "TPC-C rút gọn" xuống **"heap file + B+Tree + WAL + MVCC 2 mức isolation"**; cắt hẳn "simulator WASM cho ScyllaDB/TigerBeetle" khỏi v1 (dùng `referenceTraceRef` + notebook phân tích).

### 9.3 Realm 9 — Kỹ nghệ ứng dụng AI (ranh giới 5 "engineering")

Đây là trục xương sống, phải chốt **trước** khi viết lesson vì nếu không, 82 lesson của 5 world sẽ trùng nội dung nhau. Định nghĩa theo **ĐƠN VỊ CÔNG VIỆC / LỖI NÓ CHỮA / CÁCH ĐO**:

| | Đơn vị | Tác giả | Chữa lỗi | Đo bằng | #L |
|---|---|---|---|---|---|
| **PROMPT** | MỘT lượt request/response | con người, viết tay | "model hiểu SAI nhiệm vụ" | pass@1 trên bộ test prompt cố định | 16 |
| **CONTEXT** | TOÀN BỘ cửa sổ token tại mỗi bước, xuyên phiên | chương trình (assembler) | "đúng chữ nhưng SAI thông tin / thừa / thiếu / context rot / vỡ ngân sách" | token/turn, recall-trong-cửa-sổ, % turn vượt budget | 18 |
| **HARNESS** | CHƯƠNG TRÌNH + REPO bao quanh model | kỹ sư | "model giỏi + prompt tốt + context đủ VẪN fail" | completion rate của CÙNG model khi đổi harness (A/B) | 16 |
| **LOOP** | HÌNH DẠNG vòng lặp + ĐIỀU KIỆN DỪNG | kỹ sư | "không dừng / dừng sớm / đốt tiền / lặp lại lỗi cũ" | steps-to-success, % chạm step cap, cost/task | 18 |
| **GRAPH** | TOPOLOGY + STATE SCHEMA | kỹ sư | "không resume / không song song / không audit / không test từng nhánh" | edge coverage, replay từ checkpoint | 14 |

Quan hệ: **Prompt ⊂ Context ⊂ Harness**; Loop và Graph là hai cách tạo hình control-flow BÊN TRONG Harness. Loop là DÒNG THỜI GIAN; Graph là KHÔNG GIAN TRẠNG THÁI. **MCP** chuẩn hoá biên agent↔tool (dọc); **A2A** chuẩn hoá biên agent↔agent (ngang); **Skill** là kỹ thuật context engineering đóng gói lại.

Mỗi lesson khai `concept_owner ∈ {prompt, context, harness, loop, graph, protocol}`; lint chặn build nếu lesson dạy khái niệm thuộc owner khác. **Nếu không nghĩ ra cách đo theo metric của world thì lesson đó thuộc world khác.**

**Tách BẤT BIẾN / PHIÊN BẢN cho MCP & A2A:** phần bất biến (vì sao cần chuẩn hoá biên tool; JSON-RPC; capability negotiation; vòng đời task; lớp mối đe doạ: tool poisoning / confused deputy / token passthrough) nằm trong lesson; phần phiên bản (tên field, endpoint, header) dồn hết vào **MỘT file** `packages/mcp-kit/src/spec-2026-06.ts` + bảng "phiên bản" cuối mỗi lesson. Conformance test chạy CI hàng tuần; test đỏ → sửa 1 file, không phải 18 lesson.

**Góc FP — 1 lesson/world**, nối thẳng sang 159 chương đã viết. Đây là lý do duy nhất khiến track này không thể thay bằng một khoá trên mạng:
- agent loop = fold trên State monad (`chapter_27_monads.md`)
- tool call = Kleisli arrow; chain tool = `>=>`
- retry/fallback = Alternative; gọi tool song song = Applicative (`chapter_27b_applicative_validation.md`)
- execution graph = Free monad + interpreter; catamorphism trên cây node (`chapter_28c_recursive_types_folds.md`)
- context assembly = Monoid **có ràng buộc ngân sách** (`chapter_25_abstract_algebra.md`)
- error trong agent = ROP, không phải try/catch (`chapter_24_rop.md`)
- tool schema = Domain Modeling Made Functional (`chapter_22_domain_modeling.md`)

**`MockProviderSpec` giữ `connect-src 'none'` mà vẫn dạy được AI:** provider tất định chạy HOÀN TOÀN trong sandbox, phục vụ từ cassette (JSON fixture: prompt hash → response) vendor kèm bài học, cộng "toy LLM" quy tắc. Nhờ vậy agent/MCP/RAG chạy được, tất định, **chấm được** (assert trên chuỗi tool-call, trên context window, trên retrieval hit).

---

## 10. CÂY THƯ MỤC MONOREPO CUỐI CÙNG

```
/Volumes/SEAGATE/Personal_Projects/AI_Course/
│
├── pnpm-workspace.yaml            # packages: ["apps/*","packages/*","tools/*"]
├── turbo.json                     # pipeline: inventory→content→typecheck→build→test
├── package.json                   # private, "packageManager":"pnpm@10.x", name "byte-academy"
├── Cargo.toml                     # [workspace] members = ["crates/*","apps/shell/src-tauri"]
├── rust-toolchain.toml            # channel = "1.97.1"; targets = 7 target native + wasm32
├── .gitignore                     # dist/, node_modules/, target/, .content-cache/
├── MASTERPLAN.md                  # ← FILE NÀY
├── AGENTS.md · README.md
│
├── content/                       ★ NGUỒN SỰ THẬT CỦA NỘI DUNG — git-tracked
│   ├── curriculum/
│   │   ├── skill-tree.v1.yaml     # ĐÓNG BĂNG: R0+R1+R2.T1-3+R3.T1-2 (~180 lesson)
│   │   ├── roadmap.yaml           # R2.T4-7, R4-R9 ở MỨC TRACK — không validate, không build
│   │   ├── BACKLOG.md             # chống phình phạm vi
│   │   └── schema/skill-tree.schema.json
│   ├── skills/                    # ~120 SkillId cho v1, 1 định nghĩa tiếng Việt/skill
│   │   ├── core.skills.yaml · math.skills.yaml · cs.skills.yaml
│   ├── concepts/                  # ConceptId + blurb cho tooltip :concept[...]
│   │   ├── fp.concepts.yaml · math.concepts.yaml · db.concepts.yaml · ai.concepts.yaml
│   │
│   ├── onboarding/                # R0 — 40 lesson TIER-A, 2 module
│   │   ├── track.yaml
│   │   ├── 01-may-tinh-noi-gi/    { module.yaml, NN-slug.lesson.md × 18 }
│   │   └── 02-ra-lenh-cho-byte/   { module.yaml, NN-slug.lesson.md × 22 }
│   ├── programming-101/           # R1 — 150 lesson TIER-A, 5 module
│   ├── math-discrete/             # R2 — T2.1-T2.3 TIER-A + generators/
│   │   ├── 01-cam-nhan-so/
│   │   ├── generators/            # ~600 bộ sinh đề tham số hoá (TS, có test riêng)
│   │   └── _gold/                 # đáp án tham chiếu để test generator
│   ├── cs-core/                   # R3 — T3.1, T3.2 TIER-A
│   ├── fp-core/          software-eng/     database-scratch/
│   ├── system-design/    ai-genai-rag/     ai-applied/     # roadmap, chỉ có track.yaml ở v1
│   │
│   ├── legacy/                    ★ BẢN COPY của 159 chương + 140 file phụ
│   │   ├── python/ (46 chương)  rust/ (63)  typescript/ (50)
│   │   ├── _reference/            # 10 phụ lục → lesson kind 'explain' thuần
│   │   └── ANCHORS.md             # danh mục anchorId đã chèn
│   │
│   └── _assets/
│       ├── <sha256>.{png,svg,woff2}
│       ├── corpus-vi/             # ~4 MB, giấy phép mở, dùng cho RAG + gold set recall@k
│       ├── cassettes/             # bản ghi LLM thật (T1), tách theo world
│       └── reference-traces/      # trace ghi từ ScyllaDB/TigerBeetle — CHỈ ĐỐI CHIẾU
│
├── packages/                      # TypeScript
│   ├── content-schema/            # v2.ts (§5) + zod.ts + expect-type test
│   ├── content-compiler/          # discover·parse·link·validate·emit·pack·report + cli.ts
│   ├── content-lint/              # cưỡng chế SPEC.md (§7.4)
│   ├── content-runtime/           # loadIndex·loadPack·selectPack·downloadTrackOffline·evictLRU(pinned)
│   ├── pedagogy/SPEC.md           # hiến chương sư phạm — nguồn chân lý của lint
│   ├── app/                       # ★ App Svelte 5 THỰC — dùng chung web + shell
│   │   └── src/{routes,screens/{Map,Lesson,Codex,Review}.svelte,lib/content-source.ts,state/}
│   ├── ui/                        # design system + Byte sprite; assets/fonts/ TỰ HOST
│   ├── tokens/                    # design token CSS vars, light/dark
│   ├── platform/                  # ★ ADAPTER NATIVE DUY NHẤT — lint chặn import tauri ngoài đây
│   ├── worldkit/
│   │   ├── src/world.ts · replay.ts · probe.ts · render/{node,edge,token,meter,grid,table}.ts
│   │   └── src/worlds/{grid-bot,set-garden,memory-city,machine-belt,       # v1
│   │                   number-line,graph-lab,proof-bench,blueprint-board,  # v1.1
│   │                   storage,lsm,ring,ledger,vsr,traffic-city,           # v2
│   │                   context-window,agent-trace}/                        # v2
│   ├── byte/                      # director.ts (pure reducer) + moods/ + dialogue/vi.json
│   ├── mastery/                   # levels·skill-graph·fsrs(FSRS-5 ~250 dòng)·challenge·badges
│   │   └── review-bank/           # micro-task biến thể — GATE: skill reviewable cần >=3
│   ├── exec-core/                 # ExecutionEngine·protocol·limits·diagnostic·trace·registry (0 dep)
│   ├── sandbox-host/              # host.ts·frame.html·csp.ts·pool.ts·watchdog.ts·selftest.ts
│   ├── exec-python/               # Pyodide 0.28 worker + vendor/wheels/*.whl
│   ├── exec-typescript/           # tsc-worker (diagnostics) + exec-worker + prelude/*.js
│   ├── exec-rust/                 # binding byte_rust.wasm + worker
│   ├── exec-surrealdb/            # @surrealdb/wasm, mem:// + indxdb://
│   ├── simdisk/                   # ★ block device mô phỏng — bản TS + bản Python song sinh
│   ├── grader/                    # adjudicator 7 tầng + evidence + antifraud + complexity-regression
│   ├── ast-query/                 # registry theo NGÔN NGỮ; kind chưa đăng ký ⇒ BUILD FAIL
│   ├── tokenizer-vi/              # BPE offline + đo chi phí token tiếng Việt vs Anh
│   ├── llm-provider/  llm-sim/  llm-cassette/  llm-local/  llm-byok/
│   ├── rag-kit/  agent-kit/  graph-kit/  mcp-kit/
│   ├── sim-db/  sim-arch/         # mô phỏng tất định có seed
│   ├── progress/                  # Dexie 4 + ProgressLog (Lamport LWW) + export .odyssey.json + QR
│   ├── a11y/                      # describe-world.ts — mô tả tiếng Việt cho screen reader
│   └── telemetry/                 # đo CỤC BỘ, không gửi đi đâu
│
├── crates/                        # Rust
│   ├── byte-rust/                 ← mv từ packages/byte-rust (untracked, miễn phí)
│   │   └── src/{lexer,parser,ast,resolve,tyck,interp,value,movecheck,diag,span,wasm}.rs
│   ├── byte-rust-conformance/     # CI-only: corpus dương + ÂM(mutation) + ui-subset vs rustc
│   ├── byte-index/                # HNSW + token budget accounting → wasm
│   ├── byte-mcp/                  # MCP server bằng rmcp → wasm32 artifact ship kèm app
│   ├── odyssey-core/              # đọc content pack, verify chữ ký Ed25519, ProgressLog
│   └── odyssey-tauri/             # 4 command: resource fs · rustup detect · sidecar · secure store
│
├── apps/
│   ├── web/                       # Vite 8 PWA entry
│   │   ├── vite.config.ts         # ★ server.headers VÀ preview.headers (COOP/COEP)
│   │   ├── public/_headers        # ★ COOP/COEP production cho Cloudflare Pages
│   │   ├── public/sandbox/        # frame.html cho origin B
│   │   └── src/sw.ts              # Workbox 7 precache + runtime cache theo track
│   └── shell/                     # Tauri 2.9 entry
│       └── src-tauri/{Cargo.toml,tauri.conf.json,capabilities/,icons/,resources/content/,
│                      gen/apple/, gen/android/}
│
├── tools/
│   ├── inventory/                 # ★ sinh bảng kiểm kê từ find; MỌI con số phải từ đây
│   ├── migrate-v1/                # 159 chương → content/legacy/*.lesson.md TIER-B/C + anchors
│   ├── derive-lessons/            # ★ khoét lỗ @lesson-hole trong reference impl → starter+test
│   ├── authoring-llm/             # sinh starter/hint/distractor/Byte beat; authoredBy:'llm-assisted'
│   ├── pyodide/                   # build-pyodide-pack.mjs + VERSION + wheel vendor
│   ├── tslibs/                    # pre-bundle effect/fp-ts/zod/immutable/fast-check + import map
│   ├── trim-vocab/                # ★ cắt vocab e5-small 250k → 30k, int8
│   ├── record-cassette/           # ghi T1 từ API thật lúc soạn bài
│   ├── record-reference-traces/   # ghi trace ScyllaDB/TigerBeetle (đối chiếu, không chấm)
│   └── icons/                     # sinh icon + splash 7 target từ 1 SVG Byte
│
├── docs/
│   ├── decisions/                 # ADR-001(SUPERSEDED)·ADR-002 rust-scope·ADR-003 sandbox·
│   │                              # ADR-004 content-format·ADR-005 appstore-2.5.2
│   ├── authoring/                 # LESSON_FORMAT.md · AST_QUERIES.md · GRADING-CONTRACT.md
│   ├── design/                    # BYTE_BIBLE.md · SCENE_CATALOG.md · CONCEPT-BOUNDARIES.md ·
│   │                              # STORAGE-BUDGET.md · engine-coverage.md
│   └── generated/                 # ★ inventory·coverage·budget·rust-conformance·embed-bench·
│                                  #   quickjs-effect·licenses — sinh bằng script, không gõ tay
│
├── services/sync/                 # TÙY CHỌN, ngoài mọi mốc: SurrealDB schema + axum gateway
│
├── fp/                            ★ READ-ONLY sau M1 — CODEOWNERS + CI cảnh báo
│   ├── Python_Books/ Rust_Books/ TypeScript_Books/   # 159 chương, giữ nguyên như một cuốn sách
│   └── game/                      # app cũ — XOÁ sau M5 khi packages/app thay thế xong
│
├── repos/  references/  wiki/     # tra cứu, không vào build
└── .github/workflows/
    ├── content.yml    # inventory + compile + lint + SELF-TEST + publish prose pack (Release)
    ├── web.yml        # build + Playwright chromium/firefox/webkit + COI smoke + Cloudflare Pages
    ├── desktop.yml    # macos-15(universal) / ubuntu-22.04(glibc 2.35 sàn) / windows-2022
    ├── mobile.yml     # macos-15 → TestFlight · ubuntu-24.04 → Play internal (AAB, targetSdk 36)
    ├── rust-conformance.yml   # corpus ÂM là cổng merge
    └── spec-watch.yml         # tuần: conformance MCP/A2A vs spec upstream
```

---

## 11. LỘ TRÌNH THI CÔNG — 13 MỐC, MỖI MỐC CÓ TIÊU CHÍ KIỂM CHỨNG ĐƯỢC

Nguyên tắc: **mỗi mốc phải ship được thứ gì đó dùng được**, và tiêu chí "xong" phải chạy được bằng một lệnh hoặc quan sát được bằng một số.

---

### M0 — SỰ THẬT NỀN & DỌN ĐƯỜNG *(1 tuần)*

**Làm:**
- `corepack enable && corepack prepare pnpm@10 --activate`.
- `rustup target add aarch64-apple-darwin aarch64-apple-ios aarch64-apple-ios-sim aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android`.
- Merge `chore/audit-fp-books` → `main`. Commit `packages/byte-rust` + `docs/` (đang untracked).
- Viết `tools/inventory` → sinh `docs/generated/inventory.md`: (path, lines, sha256, part, code-fence count theo ngôn ngữ, số `**Bài N**`, số `<details>`).
- Sửa header outline Rust từ "68 chương" về 63; đo lại bảng Python.
- Rà license: `@surrealdb/wasm`, `surrealdb` crate, ScyllaDB (AGPL — **không vendor source**, chỉ trích đoạn + link commit), TigerBeetle (Apache-2.0 — vendor thoải mái), output của Anthropic/OpenAI (cassette). Ghi `docs/generated/licenses.md`.
- Viết `docs/design/CONCEPT-BOUNDARIES.md` (§9.3) và `docs/design/STORAGE-BUDGET.md`.
- Viết `ADR-002-rust-scope.md`; stamp `ADR-001` là SUPERSEDED.

**XONG nghĩa là:**
- [ ] `pnpm -v` in ra `10.x`; `rustup target list --installed` có đủ 8 target.
- [ ] `git status --porcelain` rỗng trên `main`.
- [ ] `docs/generated/inventory.md` tồn tại, dòng tổng khớp `find … | wc -l` chạy tay.
- [ ] `docs/generated/licenses.md` có kết luận GO/NO-GO cho từng thư viện.
- [ ] `grep -c "68 chương" fp/Rust_Books/book_outline.md` trả về 0.

---

### M1 — MONOREPO + CONTENT PACK + SỬA BUG "APP RỖNG" *(2 tuần)*

**Làm:**
- Dựng `pnpm-workspace.yaml` + `turbo.json` + `Cargo.toml` workspace gốc + `rust-toolchain.toml`.
- `mv packages/byte-rust crates/byte-rust` (untracked → miễn phí).
- **COPY** 159 chương + 140 file phụ sang `content/legacy/` bằng `git mv`-free copy; chèn `anchorId` vào **bản copy** bằng `tools/migrate-v1 --anchors-only`. `fp/*_Books` thành READ-ONLY (CODEOWNERS + CI cảnh báo).
- `packages/content-schema/src/v2.ts` + `zod.ts` (§5) — **ĐÓNG BĂNG**.
- `packages/app/src/lib/content-source.ts`: 2 adapter (fetch cho web, `convertFileSrc` cho native) thay `db.ts` fetch `/src/data/…`.
- Tự host `Inter` + `JetBrains Mono` woff2 subset Latin+Vietnamese trong `packages/ui/assets/fonts/`; **xoá mọi `preconnect`/`link` tới fonts.googleapis.com**.
- `apps/shell/src-tauri` khởi tạo (Tauri 2.9), `bundle.resources` trỏ content pack.

**XONG nghĩa là:**
- [ ] `pnpm turbo build` xanh từ clean checkout.
- [ ] `pnpm build && ls apps/web/dist/content/vi/index.json` **tồn tại** (bug app-rỗng đã chết).
- [ ] Playwright smoke test chạy trên **`dist` thật, không phải dev server**: mở app → thấy ≥1 lesson → `manifest 404` = 0. Test này chặn merge.
- [ ] `grep -r "fonts.googleapis" apps/ packages/` trả về rỗng.
- [ ] `cargo build --workspace` xanh; `crates/byte-rust` build được `--target wasm32-unknown-unknown`.

---

### M2 — RUNTIME, SANDBOX & CROSS-ORIGIN ISOLATION TRÊN 7 NỀN TẢNG *(3 tuần)* ⚠️ GATE CHÍNH

**Làm:**
- **XOÁ `runRust()` giả ngay ngày đầu** → trả `{ status: 'unsupported_feature' }`; UI hiện "Bài này chưa chấm tự động được" + khoá huy hiệu.
- `packages/exec-core` (0 dependency) + `packages/sandbox-host` (origin B thật, CSP, worker pool, watchdog, **selftest.ts**).
- Chuyển Pyodide và TS sang Worker; worker dự phòng ấm; `terminate()` + hoán đổi.
- COOP/COEP **production** trên cả 7 vỏ: `apps/web/public/_headers`, `preview.headers` trong vite.config, `app.security.headers` + `on_web_resource_request` cho Tauri, `WKURLSchemeHandler` iOS, `WebViewAssetLoader` Android.
- Spike "hello byte-rust" chạy thật trên iPhone và Android **TRƯỚC khi viết nội dung**.

**XONG nghĩa là:**
- [ ] `grep -n "Tests passed" fp/game/src/engine/runner.ts packages/` trả về rỗng.
- [ ] Trên **cả 7 nền tảng**, `selftest.ts` báo PASS cả 3 khẳng định: `new Worker()` thành công · `indexedDB.databases()` không thấy DB app · `fetch` cross-origin bị chặn. Bảng kết quả in vào `docs/generated/sandbox-matrix.md`.
- [ ] Bảng `crossOriginIsolated` + `typeof SharedArrayBuffer` cho 7 nền tảng, đo trên **bản build/đóng gói**, không phải dev server.
- [ ] **Bấm Stop khi đang chạy `while True: pass` → UI trả về trong <500 ms trên iPhone SE 2020 bản Tauri.** Đây là tiêu chí nghiệm thu bằng đồng hồ, quay video.
- [ ] Nếu Tauri mobile fail: quyết định Capacitor được ghi vào ADR **trong M2**, không đợi.

---

### M3 — SCHEMA COMPILER + LESSON MẪU VÀNG *(3 tuần)*

**Làm:**
- `packages/content-compiler` 7 pha, đặc biệt `validate.ts` với **self-test trong runtime shipped** (Pyodide dưới Node, cùng phiên bản khoá trong `public/pyodide/package.json`).
- `packages/content-lint` với đủ 15 rule §7.4.
- `docs/authoring/LESSON_FORMAT.md` tiếng Việt cho người không-phải-dev; `content:new` scaffold.
- Viết TAY 3 lesson mẫu vàng: `onboarding/02/05-print`, `math-discrete/01/01-cai-ro-cua-byte`, `fp-core/py-fundamentals/06-control-flow`.
- Spike QuickJS × Effect-TS (2 ngày), ghi tỉ lệ pass.

**XONG nghĩa là:**
- [ ] `pnpm content:build` compile 3 lesson mẫu, self-test chạy trong Pyodide và **build fail** khi cố tình làm hỏng một `solution`.
- [ ] Golden lesson **parse đúng thang colon**: test snapshot khẳng định `challenge` chứa `hints` và `validate` là con của nó (đây là bug đã giết thiết kế cũ).
- [ ] Đặt `AstQuery {lang:'python', kind:'no-unwrap'}` vào một lesson → **BUILD FAIL** (không phải pass ngầm).
- [ ] `docs/generated/quickjs-effect.md` có con số; quyết định GO/NO-GO cho QuickJS được ghi vào ADR.
- [ ] **ĐO thời gian thực tế soạn 1 lesson TIER-A đầy đủ** và công bố trong `docs/generated/authoring-rate.md`. Con số này quyết định toàn bộ phạm vi v1.

---

### M4 — `byte-rust` v1 + CONFORMANCE ÂM *(6 tuần)*

**Làm:**
- Thêm **error recovery** vào parser (panic-mode sync trên `;` `}` `fn` `let`) → nhiều lỗi/lần chạy.
- `resolve.rs` + `tyck.rs` (typecheck-lite: kiểu vô hướng, struct/enum, generic đơn giản, integer literal defaulting, `?`).
- Hoàn thiện `interp.rs` + `value.rs`: **`Target { pointer_width: 64 }` cố định**, fuel counter, heap cap, stack depth, trace emit, `HashMap` xáo trộn theo seed đổi mỗi lần chạy.
- `movecheck.rs` — **chỉ 2 luật hẹp** §4.3.2, mọi thứ khác → `ChuaHoTro`.
- `crates/byte-rust-conformance`: 3 corpus, mutation generator, so với **rustc thật** trên CI.
- `diag_vi.rs` — bảng ánh xạ mã lỗi → tiếng Việt + quick-fix cho CodeMirror 6.

**XONG nghĩa là:**
- [ ] `byte_rust.wasm` ≤ **2 MB brotli** (đo trong CI, ngưỡng chặn merge).
- [ ] `cargo test -p byte-rust-conformance` xanh với: **false-accept trên corpus ÂM = 0** (bắt buộc), accept/reject agreement ≥99%, error-code agreement ≥70%.
- [ ] Test riêng cho `usize::MAX`, `size_of::<usize>()`, overflow panic — cho **cùng kết quả** ở wasm32 build và rustc x86_64.
- [ ] `docs/generated/rust-conformance.md` + màn hình "Byte biết gì về Rust" trong app liệt kê `supportedFeatures` / `unsupportedFeatures`.
- [ ] Một chương trình dùng `async fn` trả về `unsupported_feature("async")`, **không phải** pass và **không phải** fail vô cớ.

---

### M5 — WORLDKIT + BYTE + LÁT DỌC R0 → **v0.1 nội bộ** *(5 tuần)*

**Làm:**
- `packages/worldkit`: `World<S,A>`, `replay.ts` (scrubber tua/step), `probe.ts`, render primitive.
- 2 world đầu: `grid-bot` (pixi) và `set-garden` (svg).
- `packages/byte`: `director.ts` pure reducer + `dialogue/vi.json` (~120 dòng, không dòng nào chứa "sai").
- `packages/app/src/screens/Lesson.svelte` viết lại: 3 pane desktop / stack dọc mobile, render 6 `Step.kind` đầu (`explain`, `example`, `assemble`, `predict`, `code`, `checkpoint`), thang hint 5 bậc, timeline scrubber. **Bỏ hẳn** đoạn regex `res.output.match(/\d+/g)` đoán số để bơm animation.
- `packages/mastery`: `levels.ts`, `skill-graph.ts`, `badges.ts` (**chưa có fsrs**).
- Viết trọn 40 lesson R0 TIER-A.
- `docs/design/BYTE_BIBLE.md` + `SCENE_CATALOG.md`.

**XONG nghĩa là:**
- [ ] 40 lesson R0 chạy end-to-end trên cả 7 nền tảng: `assemble` → `predict` → `code` → chấm thật qua Pyodide worker → world phản ứng qua `byte.probe`.
- [ ] Test determinism: mỗi world chạy 2 lần cùng seed → **hash chuỗi state bằng nhau**; chạy cho toàn bộ world trong CI.
- [ ] Test `director.ts` trên chuỗi event mô phỏng: không quá 1 phát ngôn chủ động/page; không phát ngôn trong 3 s sau keystroke.
- [ ] Ảnh chụp PR ở **375×667 / 768 / 1440** cho mỗi world; lesson có `linkedViews>=2` render stack dọc, **không có tab**.
- [ ] **Chạy thử với 3 người thật chưa biết lập trình**, ghi lại nơi họ tắc. Đây là tiêu chí chất, không phải test tự động.

---

### M6 — DI TRÚ 159 CHƯƠNG → TIER-B/C → **v0.5 "Thư viện của Byte"** *(4 tuần)*

**Làm:**
- `tools/migrate-v1`: `content/legacy/**` → `.lesson.md` TIER-B/C. Cắt theo `## N.M` thành `explain`/`example`; code fence chứa `assert` → `example{runnable}` + tier `tests` (khai thác **1.772 dòng assert sẵn có**); `**Bài N** (…)::` (regex ĐÃ SỬA) → `code` step; `<details>…Lời giải…` → fence `title=solution`; `## ✅ Checkpoint` → `checkpoint`; `## 🔧 