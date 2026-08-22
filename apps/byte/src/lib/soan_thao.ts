/** Xử lý phím trong ô soạn mã.
 *
 *  Python quyết định khối lệnh bằng THỤT LỀ, nên một ô soạn không xử lý được
 *  thụt lề thì không dùng được cho Python. Mặc định của trình duyệt là Tab
 *  chuyển focus — người học viết `if`, bấm Tab, và con trỏ nhảy sang nút bấm.
 *  Họ không biết mình vừa gặp một quy ước của trình duyệt chứ không phải một
 *  luật của Python.
 *
 *  Tách khỏi component để test được: đây là logic văn bản thuần, và nó có đủ
 *  ca biên (chọn nhiều dòng, giảm lề, con trỏ giữa khoảng trắng) để đáng
 *  kiểm riêng.
 */

const LE = 4;

export interface TrangThai {
  van: string;
  dau: number;
  cuoi: number;
}

/** Đầu dòng chứa vị trí `i`. */
function dau_dong(van: string, i: number): number {
  return van.lastIndexOf('\n', Math.max(0, i - 1)) + 1;
}

/** Chèn `LE` dấu cách, hoặc thụt cả khối nếu đang chọn nhiều dòng. */
export function tab(t: TrangThai): TrangThai {
  const { van, dau, cuoi } = t;
  if (van.slice(dau, cuoi).includes('\n')) return thut_khoi(t, +1);
  // Chèn cho tròn mốc lề, không phải luôn 4: con trỏ ở cột 2 thì Tab đưa tới
  // cột 4, giống mọi trình soạn thảo người học sẽ gặp về sau.
  const cot = dau - dau_dong(van, dau);
  const n = LE - (cot % LE) || LE;
  const cach = ' '.repeat(n);
  return { van: van.slice(0, dau) + cach + van.slice(cuoi), dau: dau + n, cuoi: dau + n };
}

/** Shift+Tab — giảm lề. */
export function tab_nguoc(t: TrangThai): TrangThai {
  return thut_khoi(t, -1);
}

function thut_khoi(t: TrangThai, chieu: 1 | -1): TrangThai {
  const { van, dau, cuoi } = t;
  const d = dau_dong(van, dau);
  const c = van.indexOf('\n', cuoi) === -1 ? van.length : van.indexOf('\n', cuoi);
  const khoi = van.slice(d, c);
  let bot_dau = 0;
  let bot_tong = 0;

  const moi = khoi
    .split('\n')
    .map((dong, i) => {
      if (chieu === 1) {
        if (i === 0) bot_dau = LE;
        bot_tong += LE;
        return ' '.repeat(LE) + dong;
      }
      const co = dong.length - dong.trimStart().length;
      const bot = Math.min(LE, co);
      if (i === 0) bot_dau = -bot;
      bot_tong -= bot;
      return dong.slice(bot);
    })
    .join('\n');

  return {
    van: van.slice(0, d) + moi + van.slice(c),
    dau: Math.max(d, dau + bot_dau),
    cuoi: Math.max(d, cuoi + bot_tong),
  };
}

/** Enter — giữ lề dòng trước, và thụt thêm một mức sau dấu hai chấm.
 *
 *  Đây là chỗ Python khác hẳn: `if x:` xuống dòng thì DÒNG SAU BẮT BUỘC thụt
 *  vào. Bắt người học tự gõ bốn dấu cách mỗi lần là bắt họ làm một việc mà
 *  cú pháp đã nói trước là phải làm.
 */
export function enter(t: TrangThai): TrangThai {
  const { van, dau, cuoi } = t;
  const d = dau_dong(van, dau);
  const dong = van.slice(d, dau);
  const le = dong.length - dong.trimStart().length;
  const them = dong.trimEnd().endsWith(':') ? LE : 0;
  const cach = '\n' + ' '.repeat(le + them);
  const i = dau + cach.length;
  return { van: van.slice(0, dau) + cach + van.slice(cuoi), dau: i, cuoi: i };
}

/** Backspace ở đầu dòng đang thụt — xoá cả một mức lề, không phải một dấu cách.
 *
 *  Xoá từng dấu cách nghĩa là bấm bốn lần cho một mức, và ba lần đầu để lại
 *  một mức lề không hợp lệ mà Python sẽ từ chối.
 */
export function backspace(t: TrangThai): TrangThai | null {
  const { van, dau, cuoi } = t;
  if (dau !== cuoi || dau === 0) return null;
  const d = dau_dong(van, dau);
  const truoc = van.slice(d, dau);
  if (truoc === '' || truoc.trim() !== '') return null;
  const bot = truoc.length % LE || LE;
  return { van: van.slice(0, dau - bot) + van.slice(dau), dau: dau - bot, cuoi: dau - bot };
}
