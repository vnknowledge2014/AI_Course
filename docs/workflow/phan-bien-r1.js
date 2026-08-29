export const meta = {
  name: 'phan-bien-r1',
  description: 'Phản biện sư phạm một track của Realm 1, trả phát hiện về cho người gọi ghi ra đĩa',
  phases: [{ title: 'Phản biện', detail: 'một agent mỗi 3 bài liền kề' }],
}

// `args` = { track: 'T1.1 …', thu_muc: 'content/nen-tang/01-…', bai: ['01-…', …] }
//
// Chạy TỪNG TRACK MỘT, không gộp cả 150 bài.
//
// Vòng T2.3 hỏng ba lần liền vì hạn mức — mỗi lần bung 22 agent rồi chết sạch,
// không viết nổi một bài. Một track 30 bài là 10 agent phản biện; gộp cả Realm
// 1 thành 50 agent là tự dựng lại đúng cái thất bại ấy.
const A = typeof args === 'string' ? JSON.parse(args) : args
if (!A || !A.thu_muc || !Array.isArray(A.bai) || A.bai.length === 0) {
  throw new Error(`args phải có {track, thu_muc, bai[]}; nhận được ${typeof args}`)
}

const CUM = []
for (let i = 0; i < A.bai.length; i += 3) CUM.push(A.bai.slice(i, i + 3))

const LOI_SCHEMA = {
  type: 'object',
  required: ['loi'],
  properties: {
    loi: {
      type: 'array',
      items: {
        type: 'object',
        required: ['tep', 'dong', 'muc', 'loi', 'sua_the_nao', 'trich'],
        properties: {
          tep: { type: 'string', description: 'đường dẫn từ gốc repo' },
          dong: { type: 'integer' },
          muc: { enum: ['chan', 'nen-sua', 'nho'] },
          loai: {
            enum: [
              'sai-su-that',
              'mau-thuan-bai-truoc',
              'buoc-nhay-qua-xa',
              'khai-niem-chua-day',
              'cham-diem-ho',
              'dien-dat',
            ],
          },
          loi: { type: 'string', description: 'sai cái gì, nói cụ thể' },
          trich: { type: 'string', description: 'trích NGUYÊN VĂN câu/dòng sai trong file' },
          sua_the_nao: {
            type: 'string',
            description:
              'sửa thành gì, cụ thể tới mức người khác sửa được mà không cần đọc lại toàn bài',
          },
        },
      },
    },
  },
}

const NEN = `Bạn đang phản biện học liệu LẬP TRÌNH tiếng Việt cho người ZERO nền tảng, phong cách Swift Playground (nhân vật Byte). Track: ${A.track}.

Mạch thiết kế ở \`content/nen-tang/MACH.md\` — ĐỌC MỤC CỦA TRACK NÀY TRƯỚC, kể cả phần "Vì sao thứ tự này đúng" bên dưới bảng.

Luật của mạch: mỗi bài đúng MỘT khái niệm mới; mỗi bài kết bằng một câu hỏi bỏ ngỏ mà bài kế trả lời; công cụ mới chỉ xuất hiện SAU khi bài trước đã tạo ra sự bất tiện cần nó.

## MƯỜI HAI CỔNG MÁY ĐÃ XANH trên các bài này

Đừng báo lại thứ máy đã bắt được. Những thứ sau ĐÃ kiểm và KHÔNG cần bạn nhìn:

- biên dịch, thang colon, frontmatter
- hiến chương sư phạm (\`::why\` cho mọi phương án sai, thang ba nấc gợi ý, Byte không buồn, không chữ hạ thấp người học, mọi \`assert\` có thông điệp)
- đồ thị tiền đề (mọi skill \`requires\` đều được một bài trước \`teaches\`)
- lời giải chạy thật dưới Pyodide, mọi \`assert\` đạt, output khớp
- mọi khối mã còn nguyên khi biên dịch
- mọi đẳng thức SỐ HỌC trong văn xuôi đã được tính lại
- hằng số của thế giới hư cấu không trôi giữa các bài
- cách chấm trượt được đáp án gần đúng (đột biến)

## Thứ bạn PHẢI tìm — máy mù trước chúng

Vòng phản biện tương ứng ở mạch Toán T2.2 tìm ra **10 lỗi mức CHẶN** trong 36 bài, trong khi mọi cổng máy đều xanh. Ví dụ THẬT từ vòng ấy, để bạn biết độ sâu cần đạt:

- một bài dạy "khác hình dạng thì khác giá trị" như một LUẬT — mà ba bài sau dựng riêng một phản ví dụ để bác nó, và phản ví dụ ấy dùng đúng tờ giấy của bài đầu;
- "8n + 6 không còn gì để đặt ra trước ngoặc", trong khi 8n + 6 = 2 × (4n + 3);
- "muốn bắt lỗi thì phải thử một số ở XA mốc" — ngược hẳn thực hành đúng, và cái bảng ngay trên nó tự bác;
- bài BOSS hứa "vẫn cái xe cũ" rồi đổi ba con số của chính cái xe ấy;
- sơ đồ "cân LỆCH" vẽ hai đĩa NGANG BẰNG nhau;
- một lời \`assert\` rơi mất chữ phủ định nên nói ngược hẳn sự thật;
- một khối \`predict\` bắt đoán kết quả của đúng đoạn mã đã in kèm đáp án 15 dòng trên;
- cách chấm đánh trượt một lời giải ĐÚNG — tệ hơn bỏ lọt đáp án sai, vì người học làm đúng mà máy nói sai, và không có cách nào biết vì sao.

## Sáu lớp, xếp theo độ nặng

1. **sai-su-that** — khẳng định sai về Python, về máy, hoặc về đời thật. Tự chạy lại mọi con số và mọi câu nói "máy sẽ in ra…".
2. **mau-thuan-bai-truoc** — đổi nghĩa một cái tên, một con số, một quy ước mà bài trước đã chốt; hoặc dạy một luật mà bài sau sẽ bác. MỞ bài trước ra đọc, đừng nhớ.
3. **cham-diem-ho** — chỗ trống không assert nào chạm tới, HOẶC cách chấm đánh trượt một đáp án đúng. Với mỗi chỗ trống, chỉ ra câu nào vỡ nếu điền sai, và thử một cách viết đúng nhưng khác lời giải mẫu.
4. **khai-niem-chua-day** — dùng một thứ chưa bài nào dạy, hoặc trích sai bài/luật.
5. **buoc-nhay-qua-xa** — lý lẽ được rao rộng hơn thứ nó chứng minh được.
6. **dien-dat** — đếm lệch, lỏng đơn vị, dùng sai thuật ngữ, hình vẽ không khớp lời.

## Cách báo

Mỗi phát hiện phải TRÍCH NGUYÊN VĂN câu sai (trường \`trich\`) — số dòng sẽ trôi khi ai đó sửa bài, câu trích thì không.

Đừng bịa phát hiện cho đủ số. Một cụm ba bài sạch thì trả mảng rỗng — đó là một câu trả lời hợp lệ và hữu ích.`

// ── Chim hoàng yến: thả MỘT cụm vào trước ───────────────────────────────────
//
// Bốn lần chạy hỏng vì hạn mức trong phiên này đốt 4,24M token và sinh ra 0
// bài — 37% tổng token đã tiêu. Riêng vòng phản biện T1.1: 10 agent cùng lao
// vào, cùng chết, mỗi con đốt ~71k token trước khi tắt.
//
// Chúng chết vì cùng một lẽ, và lẽ ấy biết được từ con đầu tiên. Nên thả một
// con vào trước: sống thì thả nốt phần còn lại, chết thì dừng ngay. Một lần
// hỏng từ 709k xuống còn ~71k.
phase('Phản biện')
const lam = (cum) =>
  agent(
    `${NEN}

## Ba bài của bạn, trong \`${A.thu_muc}/\`

${cum.map((b) => `- \`${b}.lesson.md\``).join('\n')}

Đọc CẢ ba, và đọc thêm bài ĐỨNG NGAY TRƯỚC cụm (nếu có) để bắt được lỗi NỐI — phần lớn lỗi nặng của vòng trước là bài sau mâu thuẫn quy ước bài trước vừa dựng, trong khi từng bài đọc riêng thì chẳng sai gì.`,
    { label: `pb:${cum[0]}`, phase: 'Phản biện', schema: LOI_SCHEMA, effort: 'high' },
  )

const canh = await lam(CUM[0])
if (!canh) {
  throw new Error(
    'Cụm thăm dò chết ngay — nhiều khả năng hết hạn mức. DỪNG, không thả chín ' +
      'agent còn lại vào chết cùng. Chạy lại khi hạn mức mở.',
  )
}
log(`Cụm thăm dò sống (${canh.loi?.length ?? 0} phát hiện) — thả nốt ${CUM.length - 1} cụm.`)

const ket = [canh, ...(await pipeline(CUM.slice(1), (cum) => lam(cum)))]

// Agent chết thì `pipeline` trả `null`. Đếm TRƯỚC khi lọc.
//
// Lần chạy đầu cho T1.1: cả 10 agent trúng giới hạn phiên, `ket` toàn `null`,
// `flatMap` ra mảng rỗng, và workflow trả `{tong: 0}` — trông y hệt "đã đọc 30
// bài, không thấy lỗi nào". Một con số xanh nghĩa là KHÔNG CÓ GÌ CHẠY.
//
// Đây là chế độ hỏng tệ nhất của cả tập công cụ này, và lần ấy nó nằm ngay
// trong một workflow sinh ra để đi tìm đúng loại lỗi đó.
const chet = ket.filter((r) => !r).length
if (chet > 0) {
  throw new Error(
    `${chet}/${CUM.length} agent phản biện chết — KHÔNG kết quả nào đáng tin. ` +
      `Đừng đọc "0 phát hiện" như "sạch": nghĩa là chưa ai đọc bài nào.`,
  )
}

const loi = ket.flatMap((r) => r.loi ?? [])
const dem = (m) => loi.filter((x) => x.muc === m).length
log(`${loi.length} phát hiện: ${dem('chan')} chặn · ${dem('nen-sua')} nên sửa · ${dem('nho')} nhỏ`)

// Trả thẳng dữ liệu về cho người gọi ghi ra đĩa.
//
// Vòng T2.2 mất sạch 109 phát hiện một lần vì agent lo việc lưu trữ chết giữa
// chừng; cứu lại được là nhờ đọc `journal.jsonl`. Không nhờ một agent nữa làm
// hộ việc ghi — nó chỉ thêm một chỗ chết.
return {
  track: A.track,
  tong: loi.length,
  chan: dem('chan'),
  nen_sua: dem('nen-sua'),
  nho: dem('nho'),
  phat_hien: loi,
}
