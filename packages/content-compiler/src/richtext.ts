/**
 * Markdown → RichText đã biên dịch sẵn.
 *
 * Runtime **không parse markdown**. Lý do: parse lúc chạy tốn thời gian trên
 * mobile, kéo theo một thư viện markdown vào bundle, và — quan trọng nhất — làm
 * lỗi nội dung chỉ lộ ra khi người học đã mở bài. Biên dịch sẵn lúc build khiến
 * mọi lỗi định dạng thành lỗi build.
 */

import type { Inline, RichNode } from '@byte/content-schema';

/** Chuyển một khối markdown thành RichText. Chỉ hỗ trợ tập cú pháp bài học dùng. */
export function doc_richtext(md: string): RichNode[] {
  const ra: RichNode[] = [];
  const dong = md.split('\n');
  let i = 0;

  while (i < dong.length) {
    const l = dong[i] ?? '';
    if (l.trim() === '') {
      i++;
      continue;
    }

    // Code fence
    const fence = /^\s*```([a-zA-Z0-9_+-]*)(.*)$/.exec(l);
    if (fence) {
      const lang = (fence[1] || 'text').toLowerCase();
      const src: string[] = [];
      i++;
      while (i < dong.length && !/^\s*```\s*$/.test(dong[i] ?? '')) {
        src.push(dong[i] ?? '');
        i++;
      }
      i++;
      ra.push({ t: 'code', lang: lang as never, src: src.join('\n') });
      continue;
    }

    // Tiêu đề
    const h = /^(#{2,4})\s+(.*)$/.exec(l);
    if (h) {
      ra.push({ t: 'h', lvl: h[1]!.length as 2 | 3 | 4, c: doc_inline(h[2]!) });
      i++;
      continue;
    }

    // Trích dẫn → callout
    if (l.trimStart().startsWith('>')) {
      const buf: string[] = [];
      while (i < dong.length && (dong[i] ?? '').trimStart().startsWith('>')) {
        buf.push((dong[i] ?? '').replace(/^\s*>\s?/, ''));
        i++;
      }
      const noi_dung = buf.join('\n');
      const variant = doan_variant(noi_dung);
      ra.push({ t: 'callout', variant, c: doc_richtext(noi_dung) });
      continue;
    }

    // Danh sách
    const ul = /^\s*[-*+]\s+/.test(l);
    const ol = /^\s*\d+[.)]\s+/.test(l);
    if (ul || ol) {
      const items: RichNode[][] = [];
      while (i < dong.length) {
        const cur = dong[i] ?? '';
        const m = ul ? /^\s*[-*+]\s+(.*)$/.exec(cur) : /^\s*\d+[.)]\s+(.*)$/.exec(cur);
        if (!m) break;
        items.push([{ t: 'p', c: doc_inline(m[1]!) }]);
        i++;
      }
      ra.push({ t: ul ? 'ul' : 'ol', items });
      continue;
    }

    // Bảng
    if (l.includes('|') && /^\s*\|?[\s:|-]+\|/.test(dong[i + 1] ?? '')) {
      const head = tach_o(l);
      i += 2;
      const rows: Inline[][][] = [];
      while (i < dong.length && (dong[i] ?? '').includes('|')) {
        rows.push(tach_o(dong[i] ?? '').map((c) => doc_inline(c)));
        i++;
      }
      ra.push({ t: 'table', head: head.map((c) => doc_inline(c)), rows });
      continue;
    }

    // Đoạn văn
    const buf: string[] = [];
    while (i < dong.length && (dong[i] ?? '').trim() !== '' && !la_dau_khoi(dong[i] ?? '')) {
      buf.push(dong[i] ?? '');
      i++;
    }
    ra.push({ t: 'p', c: doc_inline(buf.join(' ')) });
  }
  return ra;
}

function la_dau_khoi(l: string): boolean {
  return (
    /^\s*```/.test(l) ||
    /^#{2,4}\s/.test(l) ||
    l.trimStart().startsWith('>') ||
    /^\s*[-*+]\s+/.test(l) ||
    /^\s*\d+[.)]\s+/.test(l)
  );
}

function tach_o(l: string): string[] {
  return l
    .replace(/^\s*\|/, '')
    .replace(/\|\s*$/, '')
    .split('|')
    .map((s) => s.trim());
}

function doan_variant(s: string): 'tip' | 'warning' | 'important' | 'note' | 'pitfall' {
  const d = s.toLowerCase();
  if (d.includes('⚠️') || d.includes('cẩn thận') || d.includes('cảnh báo')) return 'warning';
  if (d.includes('bẫy') || d.includes('dễ nhầm') || d.includes('hay vấp')) return 'pitfall';
  if (d.includes('💡') || d.includes('mẹo')) return 'tip';
  if (d.includes('quan trọng') || d.includes('bắt buộc')) return 'important';
  return 'note';
}

/** Markdown nội tuyến: **đậm**, *nghiêng*, `mã`, [liên kết](url), :concept[…]{#id} */
export function doc_inline(s: string): Inline[] {
  const ra: Inline[] = [];
  let buf = '';
  let i = 0;

  const xa = () => {
    if (buf !== '') {
      ra.push({ t: 'txt', v: buf });
      buf = '';
    }
  };

  while (i < s.length) {
    // :concept[chữ hiện]{#id}
    if (s.startsWith(':concept[', i)) {
      const dong_ngoac = s.indexOf(']', i);
      const mo_nhon = s.indexOf('{', dong_ngoac);
      const dong_nhon = s.indexOf('}', mo_nhon);
      if (dong_ngoac > 0 && mo_nhon === dong_ngoac + 1 && dong_nhon > 0) {
        xa();
        const chu = s.slice(i + ':concept['.length, dong_ngoac);
        const id = s.slice(mo_nhon + 1, dong_nhon).replace(/^#/, '');
        ra.push({ t: 'concept', id, c: doc_inline(chu) });
        i = dong_nhon + 1;
        continue;
      }
    }
    // `mã`
    if (s[i] === '`') {
      const ket = s.indexOf('`', i + 1);
      if (ket > 0) {
        xa();
        ra.push({ t: 'code', c: s.slice(i + 1, ket) });
        i = ket + 1;
        continue;
      }
    }
    // **đậm**
    if (s.startsWith('**', i)) {
      const ket = s.indexOf('**', i + 2);
      if (ket > 0) {
        xa();
        ra.push({ t: 'b', c: doc_inline(s.slice(i + 2, ket)) });
        i = ket + 2;
        continue;
      }
    }
    // *nghiêng*
    if (s[i] === '*') {
      const ket = s.indexOf('*', i + 1);
      if (ket > 0) {
        xa();
        ra.push({ t: 'i', c: doc_inline(s.slice(i + 1, ket)) });
        i = ket + 1;
        continue;
      }
    }
    // [chữ](url)
    if (s[i] === '[') {
      const dong_ngoac = s.indexOf(']', i);
      if (dong_ngoac > 0 && s[dong_ngoac + 1] === '(') {
        const dong_tron = s.indexOf(')', dong_ngoac);
        if (dong_tron > 0) {
          xa();
          ra.push({
            t: 'link',
            href: s.slice(dong_ngoac + 2, dong_tron),
            c: doc_inline(s.slice(i + 1, dong_ngoac)),
          });
          i = dong_tron + 1;
          continue;
        }
      }
    }
    buf += s[i];
    i++;
  }
  xa();
  return ra;
}
