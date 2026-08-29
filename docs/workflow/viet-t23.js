export const meta = {
  name: 'viet-t23',
  description: 'Viết 32 bài R2.T2.3 — Logic & chứng minh (Python CHỈ để KIỂM), mỗi cụm tự chạy cổng tới xanh',
  phases: [
    { title: 'Viết', detail: '10 agent, mỗi agent 3 bài liền kề, tự chạy 4 cổng' },
    { title: 'Vá cổng', detail: 'cụm nào còn đỏ thì một agent khác gỡ' },
  ],
}

// `args` có thể tới dưới dạng chuỗi JSON thay vì mảng. Lần chạy trước nó tới
// dạng chuỗi, `slice(i, i+3)` cắt theo KÝ TỰ, và workflow sinh ra 3776 "cụm"
// mỗi cụm là ba chữ cái — 3776 lần thất bại thay vì một lần.
const BAI = typeof args === 'string' ? JSON.parse(args) : args
// Chặn ở đây chứ không để nó chạy tiếp: một workflow bung ra hàng nghìn agent
// vì đọc sai đầu vào thì đắt hơn nhiều so với một lỗi ngay dòng đầu.
if (!Array.isArray(BAI) || BAI.length !== 32 || !BAI[0]?.slug || !BAI[0]?.so) {
  throw new Error(`args phải là mảng 32 bài có trường slug; nhận được ${typeof args}, dài ${BAI?.length}`)
}
const THU_MUC = 'content/toan/03-logic-va-chung-minh'

const CUM = []
for (let i = 0; i < BAI.length; i += 3) CUM.push(BAI.slice(i, i + 3))

const NEN = `Bạn viết học liệu lập trình tiếng Việt cho người ZERO nền tảng, theo phong cách Swift Playground (nhân vật Byte). Mọi văn bản — kể cả comment trong mã — bằng TIẾNG VIỆT.

## Đọc trước khi viết (bắt buộc, theo đúng thứ tự)

1. \`content/toan/MACH.md\` — mục "## Mạch T2.3 — Logic & chứng minh". Đọc CẢ phần "Vì sao thứ tự này đúng" bên dưới bảng. Nó nói vì sao mỗi bài đứng đúng chỗ nó đứng, và bạn phải viết sao cho lý do đó thành thật.
2. \`content/toan/02-dai-so-va-ham-so/19-khi-so-nao-cung-dung.lesson.md\` và \`content/toan/02-dai-so-va-ham-so/33-khi-khong-chay-nguoc-duoc.lesson.md\` — hai bài MẪU cùng realm, đã qua đủ 12 cổng. Chép đúng hình dạng: thang colon, frontmatter, cách Byte nói, độ dài, nhịp.
3. \`docs/HIEN-CHUONG-SU-PHAM.md\` nếu có; nếu không thì đọc \`tools/kiem_bai_hoc.py\` — mọi luật hiến chương nằm trong đó dưới dạng mã.
4. \`content/toan/MACH.md\` mục T2.2 (36 bài liền trước) và cả phần dẫn nhập của T2.3 — nó nêu HIỆN VẬT xuyên suốt (CLB cờ vua lớp 6A, 6 thành viên) và nói rõ Python trong track này KHÔNG BAO GIỜ là lời giải.
5. \`content/onboarding/MACH.md\` — để biết chính xác Realm 0 đã dạy gì (tham chiếu \`R0-nn\`). ĐỪNG dạy lại thứ R0 đã dạy; dùng lại thì được và nên.

## Luật của mạch (vi phạm là hỏng bài)

- **Mỗi bài đúng MỘT khái niệm mới.** Không hai, không "tiện thể nói thêm".
- **Mỗi bài kết bằng câu hỏi bỏ ngỏ, và bài KẾ TIẾP trả lời đúng câu đó.** Cột \`reflect\` trong mạch là câu hỏi ấy — dùng gần như nguyên văn.
- **Công cụ mới chỉ xuất hiện SAU khi bài trước đã tạo ra sự bất tiện cần nó.** Đừng bao giờ mở bài bằng "hôm nay ta học X". Mở bằng chỗ đau bài trước để lại.
- **Không dùng khái niệm chưa bài nào dạy.** Cả Realm 1 đã xong nên list/dict/set/tuple, hàm, file, module đều dùng được. Nhưng **Python trong track này KHÔNG BAO GIỜ là lời giải** — nó dựng bảng chân lý (\`itertools.product\`), kiểm một câu trên 6 thành viên (\`all\`/\`any\`), đi săn phản ví dụ, và cuối cùng viết bất biến thành \`assert\` trong thân vòng. Máy nói "chưa thấy sai", không bao giờ nói "đúng" — bài 21 dạy thẳng điều đó.
- **Chậm mà chắc.** Người học chưa từng lập trình. Ẩn dụ trước, thuật ngữ sau.

## Hình dạng file (schema v2, thang colon)

\`::::step\` (4 dấu) → \`:::child\` (3) → \`::leaf\` (2). Các loại step: \`byte\`, \`explain\`, \`example\`, \`predict\`, \`code\`, \`sandbox\`, \`reflect\`, \`checkpoint\`. Chép chính xác từ bài mẫu — đừng suy diễn cú pháp.

Frontmatter: \`id\` là \`toan.logic-va-chung-minh.<slug>\` (KHÔNG mang số thứ tự), \`module: logic-va-chung-minh\`, \`track: toan\`, \`order\` = số bài, \`languages: [python]\`, \`teaches\`/\`requires\` là id skill — nhìn bài mẫu để theo đúng quy ước đặt tên skill, và mọi id trong \`requires\` phải được một bài ĐỨNG TRƯỚC \`teaches\`.

## Hiến chương sư phạm (cổng máy sẽ bắt)

- Mỗi lựa chọn SAI trong \`predict\` phải có \`::why\` — giải thích vì sao nghĩ thế là tự nhiên và chỗ nào lệch. Đó là chỗ dạy, không phải chỗ phạt.
- \`predict\` phải đứng TRƯỚC \`code\`.
- Mỗi \`code\` có thang ba nấc gợi ý đúng thứ tự: \`attention\` → \`strategy\` → \`one-line\`. Nấc đầu KHÔNG được cho đáp án.
- Byte không bao giờ buồn hay thất vọng. Không có mood \`sad\`/\`disappointed\`.
- Không dùng chữ hạ thấp người học: "hiển nhiên", "quá dễ", "chỉ cần"...
- Mọi \`assert\` trong khối test phải có thông điệp lỗi bằng tiếng Việt, nói rõ sai ở đâu.
- Mỗi bài phải có \`reflect\` và \`checkpoint\`.
- Không để lại chữ \`TODO —\` nào.

## Sáu lớp lỗi mà vòng phản biện vừa bắt được (đọc kỹ — đây là lỗi THẬT của mạch trước)

Mạch T2.2 viết đúng theo bản mô tả này, chạy sạch mọi cổng máy, rồi vẫn dính 10 lỗi
mức CHẶN. Cổng không bắt được vì không cổng nào đọc hiểu nghĩa. Sáu lớp ấy:

1. **Toán sai trong văn xuôi.** Ví dụ thật: một bài viết "8n + 6 không còn gì để đặt
   ra trước ngoặc", mà 8n + 6 = 2 × (4n + 3). Một bài khác nói 2ⁿ và 2 × n "trùng nhau
   đúng một chỗ" — trùng ở hai. Tự tính lại MỌI con số bạn viết ra, kể cả con số nằm
   trong một câu giải thích.
2. **Mâu thuẫn với bài trước.** Một bài định nghĩa "hoà vốn" là 5 ổ; bài sau gọi mốc
   khác là "hoà vốn" mà không ai để ý. Trước khi dùng lại một thuật ngữ hay một con
   số của bài trước, MỞ bài ấy ra đọc, đừng nhớ.
3. **Dạy một luật mà bài sau sẽ bác.** Một bài dạy "khác hình dạng thì khác giá trị";
   ba bài sau dựng riêng phản ví dụ để bác nó. Nếu mạch có bài phản bác, bài trước
   phải phát biểu ĐÚNG CHIỀU đúng của mệnh đề, không phát biểu thừa.
4. **Chấm điểm hổng.** Một chỗ trống không câu \`assert\` nào chạm tới — làm sai vẫn
   qua hết. Với MỖI chỗ trống, chỉ ra được câu assert nào sẽ vỡ nếu điền sai.
5. **Lời báo lỗi phát biểu thành luật tổng quát.** Câu \`assert\` đúng cho ca cụ thể,
   nhưng thông điệp của nó nói thành một luật sai. Thông điệp chỉ được nói về đúng
   những giá trị trong ca ấy.
6. **Bài BOSS đổi số liệu mà vẫn hứa "vẫn cái xe cũ".** Đổi số thì nói thẳng ra là
   đổi. Người học chép đáp án cũ sang là trật, và họ không có cách nào biết.

## Cách chấm phải TRƯỢT được một đáp án gần đúng

Cổng \`tools/kiem_dot_bien.mjs\` lấy chính lời giải của bạn, sửa một Ý trong
phần người học phải điền — đổi MỌI dấu \`>\` thành \`>=\`, MỌI phép nhân thành
cộng, MỌI hằng số \`n\` thành \`n+1\` — rồi hỏi cách chấm có bắt được không.
Bắt không được nghĩa là bài đang cho điểm một thứ nó không hề kiểm.

Nên khi viết khối test, đừng dừng ở câu hỏi "điền \`True\` có qua không". Hỏi:

- Đổi \`>\` thành \`>=\` thì có câu chấm nào vỡ không? Nếu bài dạy chỗ khác nhau
  giữa hai dấu ấy thì **dữ liệu chấm phải có một giá trị rơi ĐÚNG vào cái mốc**
  — không có thì bài đang dạy một điều nó không kiểm.
- Đổi một hằng số ngưỡng đi 1 thì có gì vỡ không? Không thì dữ liệu chấm chưa
  bao giờ chạm tới ngưỡng ấy.
- Đổi phép nhân thành phép cộng thì có gì vỡ không?

Bài mẫu làm đúng chuyện này: \`onboarding/02-ra-lenh-cho-byte/06-dung-hay-sai\`
chấm \`>=\` bằng BA tình huống, và tình huống thứ ba — khách đưa vừa đúng
45 000 — tồn tại chỉ để phân biệt \`>\` với \`>=\`. Đọc khối test của nó.

## Sổ sự thật của thế giới

\`content/curriculum/su-that-the-gioi.yaml\` khai hằng số của thế giới hư cấu và
\`tools/kiem_su_that.py\` (đã nằm trong \`cong.sh\`) đối chiếu mọi bài. Track này
dùng lại cuốn sổ chi tiêu của T1.4, nên **đừng đặt số mới cho thứ đã có số**.
Cần đổi thì khai ngoại lệ kèm lý do VÀ nói thẳng trong bài rằng số đã đổi.

## Chấm điểm phải THẬT

Khối test phải bắt được người làm sai. Tự hỏi: điền \`True\`, \`0\`, \`1\`, hay một hằng số bất kỳ vào chỗ trống thì có qua được không? Nếu có thì test hỏng. Ngược lại: một lời giải ĐÚNG nhưng viết khác cách bạn nghĩ có bị đánh trượt không? Nếu có thì test cũng hỏng.

## Chạy cổng (bắt buộc, tới khi xanh)

Sau khi viết xong cả ba bài, chạy đủ bốn lệnh trên CHÍNH ba file của bạn:

    node packages/content-compiler/dist/cli.js build content dist/content
    python3 tools/kiem_bai_hoc.py <ba file>
    node tools/kiem_ma_bai_hoc.mjs      # nhận THƯ MỤC json (mặc định dist/content),
                                       # KHÔNG nhận .lesson.md. NHIỀU NGƯỜI VIẾT
                                       # SONG SONG: dist/content có cả bài dở của
                                       # người khác nên cổng đỏ vì lý do không
                                       # phải của bạn. Chép JSON của riêng bạn
                                       # sang thư mục tạm rồi trỏ cổng vào đó.
    python3 tools/kiem_khong_mat.py    # khối mã nào biến mất khi biên dịch
    python3 tools/kiem_so_hoc.py <ba file>
    python3 tools/kiem_su_that.py                       # hằng số thế giới không tự đổi
    node tools/kiem_dot_bien.mjs <thư mục tạm của bạn>   # chấm có trượt được đáp án gần đúng không

Cổng đỏ thì SỬA rồi chạy lại. Đừng báo xong khi còn đỏ — báo xong khi còn đỏ là thứ tệ nhất bạn có thể làm ở đây, vì nó biến một lỗi thấy được thành một lỗi ẩn.

Nếu mạch yêu cầu một thứ mà schema hay cổng không cho phép, ĐỪNG bịa cách lách. Viết bài theo cách hợp lệ gần nhất và báo lại chỗ vướng trong trường \`vuong\`.`

const RA = {
  type: 'object',
  required: ['da_viet', 'cong_xanh', 'vuong'],
  properties: {
    da_viet: { type: 'array', items: { type: 'string' }, description: 'đường dẫn các file đã viết' },
    cong_xanh: { type: 'boolean', description: 'cả bốn cổng xanh trên các file này' },
    cong_do: { type: 'string', description: 'nếu chưa xanh: cổng nào, lỗi gì' },
    vuong: { type: 'array', items: { type: 'string' }, description: 'chỗ mạch đòi mà schema/cổng không cho; rỗng nếu không có' },
  },
}

// Chỉ đưa SỐ và SLUG. Nội dung hàng lấy từ MACH.md — bản chép trong lời gọi
// sẽ lệch với nguồn ngay lần đầu ai đó sửa mạch, và không ai biết bên nào đúng.
const mo_ta = (b) => `### Bài ${b.so} — \`${b.slug}\`
Đọc hàng số ${b.so} trong bảng T2.3 trong \`content/toan/MACH.md\`: tiêu đề, khái niệm mới (ĐÚNG MỘT),
câu hỏi bỏ ngỏ cuối bài (dùng gần như nguyên văn cho \`reflect\`), và cột "Dựa trên".`

// ── Chim hoàng yến: thả MỘT cụm vào trước ───────────────────────────────────
//
// Track này hỏng BA LẦN liên tiếp vì hạn mức, mỗi lần bung 22 agent rồi chết
// sạch: 0,74M rồi 1,65M token cho 0 bài. Cộng cả phiên, các lần hỏng vì hạn
// mức đốt 4,24M token — 37% tổng số đã tiêu — và không sinh ra một bài nào.
//
// Chúng chết vì cùng một lẽ, và lẽ ấy biết được từ con đầu tiên. Thả một con
// vào trước: sống thì thả nốt, chết thì dừng ngay.
phase('Viết')
const lam_cum = (cum, i) => {
    const truoc = i > 0 ? BAI[i * 3 - 1] : null
    const sau = i * 3 + 3 < BAI.length ? BAI[i * 3 + 3] : null
    return agent(
      `${NEN}

## Việc của bạn: ba bài LIỀN KỀ trong ${THU_MUC}/

Tên file là \`<số hai chữ số>-<slug>.lesson.md\`, ví dụ \`${String(cum[0].so).padStart(2, '0')}-${cum[0].slug}.lesson.md\`.

${cum.map(mo_ta).join('\n\n')}

${truoc ? `## Bài ĐỨNG NGAY TRƯỚC cụm của bạn (người khác viết — đừng sửa)
Bài ${truoc.so} \`${truoc.slug}\` — đọc cột \`reflect\` của hàng ${truoc.so} trong MACH.md.
Bài ĐẦU TIÊN của bạn phải trả lời đúng câu hỏi đó, và mở bài bằng chính chỗ đau ấy.` : `## Cụm của bạn MỞ MÀN cả track
T2.2 vừa khép bằng BOSS xe bánh mì. Track này đổi hiện vật hẳn: **CLB cờ vua lớp 6A, 6 thành viên (Nam, Lan, Minh, Hoa, Tú, Khanh), một bảng nội quy, một sổ quỹ.** Đừng mở bài bằng "hôm nay ta học mệnh đề" — mở bằng một câu cãi nhau có thật trong CLB mà không ai phân xử được.`}

${sau ? `## Bài ĐỨNG NGAY SAU cụm của bạn (người khác viết — đừng sửa)
Bài ${sau.so} \`${sau.slug}\` — đọc cột "Khái niệm mới" của hàng ${sau.so} trong MACH.md.
Nên bài CUỐI của bạn phải kết bằng đúng chỗ đau mà bài ${sau.so} sẽ chữa — đừng chữa hộ nó, và đừng nhắc trước công cụ của nó.` : `## Cụm của bạn KHÉP cả track
Bài cuối là BOSS: không khái niệm mới. Nó khép lại v1.0 — người học viết được một BẰNG CHỨNG cho một vòng lặp, bằng bất biến, đúng cái vòng \`while\` cộng dồn của T1.2.`}

Viết đủ ba bài, chạy đủ bốn cổng tới xanh, rồi trả kết quả.`,
      { label: `viet:${String(cum[0].so).padStart(2, '0')}-${String(cum[cum.length - 1].so).padStart(2, '0')}`, phase: 'Viết', schema: RA, effort: 'high' },
    )
  }

const canh = await lam_cum(CUM[0], 0)
if (!canh) {
  throw new Error(
    'Cụm thăm dò chết ngay — nhiều khả năng hết hạn mức. DỪNG, không thả các ' +
      'agent còn lại vào chết cùng. Chạy lại khi hạn mức mở.',
  )
}
log(`Cụm thăm dò sống — thả nốt ${CUM.length - 1} cụm.`)

// `i + 1` vì đã cắt mất cụm đầu; sai chỗ này thì mọi bài đọc nhầm hàng xóm.
const ket = [canh, ...(await pipeline(CUM.slice(1), (cum, _c, i) => lam_cum(cum, i + 1)))]

const xanh = ket.filter((r) => r && r.cong_xanh)
const do_ = ket.map((r, i) => ({ r, cum: CUM[i] })).filter((x) => !x.r || !x.r.cong_xanh)
log(`Viết xong: ${xanh.length}/${CUM.length} cụm xanh.`)

phase('Vá cổng')
const va = do_.length ? await parallel(do_.map((x) => () => agent(
  `Ba bài học sau vừa được viết nhưng cổng còn ĐỎ. Gỡ cho xanh.

File: ${x.cum.map((b) => `${THU_MUC}/${String(b.so).padStart(2, '0')}-${b.slug}.lesson.md`).join(', ')}
Báo cáo của người viết: ${x.r ? x.r.cong_do || '(không nói rõ)' : '(agent chết giữa chừng — file có thể thiếu hoặc viết dở)'}

${NEN}

Việc của bạn:
1. Kiểm file nào thiếu hoặc viết dở — viết nốt theo mạch.
2. Chạy đủ bốn cổng, đọc lỗi thật, sửa tận gốc.
3. ĐỪNG lách cổng. Nếu một cổng bắt sai, nói rõ trong \`vuong\` thay vì né nó.

Mô tả mạch của ba bài:

${x.cum.map(mo_ta).join('\n\n')}`,
  { label: `va:${String(x.cum[0].so).padStart(2, '0')}`, phase: 'Vá cổng', schema: RA, effort: 'high' },
))) : []

return {
  cum_xanh: `${xanh.length + va.filter((r) => r && r.cong_xanh).length}/${CUM.length}`,
  vuong: ket.filter(Boolean).flatMap((r) => r.vuong ?? []).concat(va.filter(Boolean).flatMap((r) => r.vuong ?? [])),
  con_do: va.filter((r) => !r || !r.cong_xanh).length,
}
