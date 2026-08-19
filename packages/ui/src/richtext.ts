/**
 * Kết xuất RichText thành HTML.
 *
 * RichText đã được biên dịch sẵn lúc build (xem `content-compiler/richtext.ts`),
 * nên ở đây không có parser markdown nào — chỉ duyệt cây và sinh HTML. Đó là lý
 * do bundle của app không kèm thư viện markdown, và vì sao lỗi định dạng nội
 * dung là lỗi BUILD chứ không phải lỗi lúc người học mở bài.
 */

import type { Inline, RichNode } from '@byte/content-schema';

/** Thoát ký tự HTML. Nội dung là dữ liệu, không bao giờ được coi là mã. */
function thoat(s: string): string {
  return s
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;');
}

export function ket_xuat_inline(cac: Inline[]): string {
  return cac
    .map((n) => {
      switch (n.t) {
        case 'txt':
          return thoat(n.v);
        case 'b':
          return `<strong>${noi_dung(n.c)}</strong>`;
        case 'i':
          return `<em>${noi_dung(n.c)}</em>`;
        case 'code':
          return `<code class="ma-nho">${noi_dung(n.c)}</code>`;
        case 'link':
          // `rel` bắt buộc: bài học có thể dẫn ra ngoài, và tab mới không được
          // giữ tham chiếu ngược tới app.
          return `<a href="${thoat(n.href)}" target="_blank" rel="noopener noreferrer">${ket_xuat_inline(n.c)}</a>`;
        case 'concept':
          return `<button class="khai-niem" data-khai-niem="${thoat(n.id)}">${ket_xuat_inline(n.c)}</button>`;
        case 'kbd':
          return `<kbd>${thoat(n.v)}</kbd>`;
        case 'math':
          return `<span class="toan" data-tex="${thoat(n.tex)}">${thoat(n.tex)}</span>`;
        default:
          return '';
      }
    })
    .join('');
}

function noi_dung(c: Inline[] | string): string {
  return typeof c === 'string' ? thoat(c) : ket_xuat_inline(c);
}

const NHAN_CALLOUT: Record<string, string> = {
  tip: 'Mẹo',
  warning: 'Cẩn thận',
  important: 'Quan trọng',
  note: 'Ghi chú',
  pitfall: 'Chỗ hay vấp',
};

export function ket_xuat(cac: RichNode[]): string {
  return cac
    .map((n) => {
      switch (n.t) {
        case 'p':
          return `<p>${ket_xuat_inline(n.c)}</p>`;
        case 'h':
          return `<h${n.lvl}>${ket_xuat_inline(n.c)}</h${n.lvl}>`;
        case 'ul':
        case 'ol': {
          const items = n.items.map((i) => `<li>${ket_xuat(i)}</li>`).join('');
          return `<${n.t}>${items}</${n.t}>`;
        }
        case 'code':
          return `<pre class="khoi-ma" data-ngon-ngu="${thoat(n.lang)}"><code>${thoat(n.src)}</code></pre>`;
        case 'callout':
          return `<aside class="callout callout-${thoat(n.variant)}"><span class="callout-nhan">${
            NHAN_CALLOUT[n.variant] ?? 'Ghi chú'
          }</span>${ket_xuat(n.c)}</aside>`;
        case 'table': {
          const head = n.head.map((c) => `<th>${ket_xuat_inline(c)}</th>`).join('');
          const rows = n.rows
            .map((r) => `<tr>${r.map((c) => `<td>${ket_xuat_inline(c)}</td>`).join('')}</tr>`)
            .join('');
          // Bảng phải tự cuộn ngang; trang KHÔNG bao giờ được cuộn ngang.
          return `<div class="bang-cuon"><table><thead><tr>${head}</tr></thead><tbody>${rows}</tbody></table></div>`;
        }
        case 'img':
          return `<img src="asset:${thoat(n.assetId)}" alt="${thoat(n.alt)}" loading="lazy">`;
        case 'math':
          return n.display
            ? `<div class="toan-khoi" data-tex="${thoat(n.tex)}">${thoat(n.tex)}</div>`
            : `<span class="toan" data-tex="${thoat(n.tex)}">${thoat(n.tex)}</span>`;
        case 'figure':
          return `<figure class="the-gioi" data-world="${thoat(JSON.stringify(n.world))}"></figure>`;
        default:
          return '';
      }
    })
    .join('');
}
