/**
 * Kiểm kiểu TypeScript THẬT, bằng chính trình biên dịch `typescript`.
 *
 * Vì sao không dùng Sucrase một mình: Sucrase **bóc** chú thích kiểu đi rồi
 * chạy, nó không kiểm gì cả. Nghĩa là `let n: number = "ba"` chạy trót lọt và
 * bài học báo ĐẠT. Đó đúng là chế độ hỏng đã làm engine Rust giả trước đây vô
 * dụng — và với TypeScript nó còn tệ hơn, vì toàn bộ lý do người ta học
 * TypeScript là hệ thống kiểu.
 *
 * Nên đường đi phải giống `byte-rust`: kiểm tĩnh TRƯỚC, chỉ chạy khi sạch.
 *
 * Nạp `lib.d.ts` được tiêm từ ngoài vào (`docLib`) vì nơi lấy chúng khác nhau
 * giữa Node (`typescript/lib/`) và trình duyệt (fetch từ `public/`), còn gói
 * này thì không được biết mình đang chạy ở đâu.
 */

import type ts from 'typescript';
import type { ChanDoan } from '@byte/exec-core';

export const TEN_TEP = 'bai-hoc.ts';

/** Khai báo môi trường tối thiểu cho bài học.
 *
 *  Bài học dùng `console.log` để in ra, nhưng `console` nằm trong
 *  `lib.dom.d.ts` — 2 MB phải tải về trình duyệt chỉ để lấy một cái tên, kèm
 *  theo hàng nghìn API DOM mà người học không có (mã chạy trong Worker, không
 *  có `document`). Khai báo tay đúng thứ cần dùng thì gọn hơn và trung thực
 *  hơn về những gì thật sự có mặt lúc chạy.
 */
export const TEN_MOI_TRUONG = 'moi-truong.d.ts';

const MOI_TRUONG = `
declare const console: {
  log(...v: unknown[]): void;
  info(...v: unknown[]): void;
  warn(...v: unknown[]): void;
  error(...v: unknown[]): void;
  debug(...v: unknown[]): void;
};
`;

/** Đọc nội dung một file `lib.*.d.ts` theo tên, `undefined` nếu không có. */
export type DocLib = (ten: string) => string | undefined;

/**
 * Tuỳ chọn biên dịch cho bài học.
 *
 * `strict` bật hết là cố ý. Dạy TypeScript ở chế độ lỏng nghĩa là dạy một
 * ngôn ngữ khác với ngôn ngữ người học sẽ gặp ở nơi làm việc, và tệ hơn: mọi
 * bài học về `null` và `undefined` — phần khó nhất — sẽ không bao giờ bắt được
 * lỗi nào.
 */
export function tuyChon(TS: typeof ts): ts.CompilerOptions {
  return {
    target: TS.ScriptTarget.ES2022,
    module: TS.ModuleKind.ESNext,
    moduleResolution: TS.ModuleResolutionKind.Bundler,
    strict: true,
    noUncheckedIndexedAccess: true,
    noImplicitOverride: true,
    noFallthroughCasesInSwitch: true,
    exactOptionalPropertyTypes: true,
    lib: ['lib.es2022.d.ts'],
    skipLibCheck: true,
    noEmit: false,
  };
}

/** Chuyển chẩn đoán của `tsc` sang dạng ba tầng của Byte. */
export function doiChanDoan(TS: typeof ts, d: ts.Diagnostic): ChanDoan {
  const thongDiep = TS.flattenDiagnosticMessageText(d.messageText, ' ');
  let dong = 1;
  let cot = 1;
  if (d.file && d.start !== undefined) {
    const v = d.file.getLineAndCharacterOfPosition(d.start);
    dong = v.line + 1;
    cot = v.character + 1;
  }
  const giai = giaiThich(d.code);
  return {
    ma: `TS${d.code}`,
    muc: d.category === TS.DiagnosticCategory.Error ? 'loi' : 'canh_bao',
    thongDiep,
    dong,
    cot,
    doDai: d.length ?? 1,
    viSao: giai?.viSao ?? null,
    cachSua: giai?.cachSua ?? [],
    khaiNiem: giai?.khaiNiem ?? null,
    vanBan: `TS${d.code} (dòng ${dong}, cột ${cot}): ${thongDiep}`,
  };
}

/**
 * Tầng "vì sao" cho những mã lỗi người mới gặp nhiều nhất.
 *
 * Thông báo gốc của `tsc` nói ĐÚNG nhưng nói với người đã hiểu hệ thống kiểu.
 * Bảng này không thay nó — nó bổ sung tầng giải thích và tầng cách sửa; mã lỗi
 * gốc vẫn hiện nguyên để người học tra cứu được ở ngoài.
 */
function giaiThich(ma: number): { viSao: string; cachSua: string[]; khaiNiem: string } | null {
  switch (ma) {
    case 2322:
      return {
        viSao:
          'TypeScript ghim kiểu cho mỗi cái tên ngay lúc bạn khai báo. Đưa vào một giá trị khác kiểu là phá lời hứa đó — và lời hứa ấy chính là thứ giúp trình soạn thảo gợi ý đúng cho bạn ở mọi dòng sau.',
        khaiNiem: 'kiểu dữ liệu',
        cachSua: ['sửa giá trị cho khớp kiểu đã khai báo', 'hoặc sửa kiểu khai báo cho khớp giá trị'],
      };
    case 2345:
      return {
        viSao:
          'Chữ ký hàm là lời hứa với mọi người gọi nó. TypeScript kiểm lời hứa đó lúc biên dịch, nên hàm không bao giờ nhận về thứ khác với thứ nó khai báo.',
        khaiNiem: 'hàm',
        cachSua: ['đưa vào đối số đúng kiểu mà hàm khai báo'],
      };
    case 2532:
    case 18048:
      return {
        viSao:
          'Giá trị này CÓ THỂ là `undefined`, nên dùng thẳng nó là chấp nhận rủi ro chương trình vỡ lúc chạy. Đây đúng là loại lỗi mà TypeScript sinh ra để chặn.',
        khaiNiem: 'undefined',
        cachSua: ['kiểm tra trước khi dùng: `if (x) { ... }`', 'hoặc cho một giá trị mặc định: `x ?? mac_dinh`'],
      };
    case 2531:
    case 18047:
      return {
        viSao: 'Giá trị này có thể là `null`. Dùng thẳng nó thì chương trình sẽ vỡ đúng lúc nó là `null`.',
        khaiNiem: 'null',
        cachSua: ['kiểm tra `if (x !== null)` trước khi dùng'],
      };
    case 2304:
      return {
        viSao:
          'TypeScript phân giải mọi cái tên lúc biên dịch. Kể cả khi dòng này nằm trong nhánh không bao giờ chạy tới, cái tên vẫn phải tồn tại.',
        khaiNiem: 'phạm vi tên',
        cachSua: ['kiểm lại chính tả của tên', 'hoặc khai báo nó trước khi dùng'],
      };
    case 2554:
      return {
        viSao:
          'Số đối số phải khớp chính xác. TypeScript không tự điền tham số thiếu — tham số muốn có thể vắng thì phải đánh dấu `?` lúc khai báo.',
        khaiNiem: 'hàm',
        cachSua: ['truyền đủ số đối số hàm khai báo'],
      };
    case 7006:
      return {
        viSao:
          'Tham số này không có kiểu, và TypeScript cũng không suy ra được từ ngữ cảnh. Nó thành `any` — mà `any` tắt hết mọi phép kiểm cho mọi thứ chạm vào nó.',
        khaiNiem: 'kiểu dữ liệu',
        cachSua: ['ghi kiểu cho tham số: `(x: number) => ...`'],
      };
    default:
      return null;
  }
}

/**
 * Kiểm kiểu một đoạn mã, trả về chẩn đoán và bản JavaScript đã sinh.
 *
 * `js` là `null` khi có lỗi mức `loi`: đúng luật của `byte-rust` — không chạy
 * mã đã biết chắc là sai. Chạy nó rồi báo "đạt" vì tình cờ không nổ chính là
 * cái bẫy mà cả đường ống này được dựng để tránh.
 */
export function kiemKieu(
  TS: typeof ts,
  ma: string,
  docLib: DocLib,
): { chanDoan: ChanDoan[]; js: string | null } {
  const opts = tuyChon(TS);
  const dich = opts.target ?? TS.ScriptTarget.ES2022;
  const goc = TS.createSourceFile(TEN_TEP, ma, dich, true);
  const moi_truong = TS.createSourceFile(TEN_MOI_TRUONG, MOI_TRUONG, dich, true);

  const host: ts.CompilerHost = {
    getSourceFile: (ten) => {
      if (ten === TEN_TEP) return goc;
      if (ten === TEN_MOI_TRUONG) return moi_truong;
      const noi_dung = docLib(ten);
      return noi_dung === undefined
        ? undefined
        : TS.createSourceFile(ten, noi_dung, opts.target ?? TS.ScriptTarget.ES2022, true);
    },
    getDefaultLibFileName: () => 'lib.es2022.d.ts',
    writeFile: () => {},
    getCurrentDirectory: () => '/',
    getDirectories: () => [],
    fileExists: (ten) => ten === TEN_TEP || ten === TEN_MOI_TRUONG || docLib(ten) !== undefined,
    readFile: (ten) =>
      ten === TEN_TEP ? ma : ten === TEN_MOI_TRUONG ? MOI_TRUONG : docLib(ten),
    getCanonicalFileName: (ten) => ten,
    useCaseSensitiveFileNames: () => true,
    getNewLine: () => '\n',
  };

  const chuong_trinh = TS.createProgram([TEN_MOI_TRUONG, TEN_TEP], opts, host);
  const tho = [
    ...chuong_trinh.getSyntacticDiagnostics(goc),
    ...chuong_trinh.getSemanticDiagnostics(goc),
  ];
  const chanDoan = tho.map((d) => doiChanDoan(TS, d));

  if (chanDoan.some((c) => c.muc === 'loi')) return { chanDoan, js: null };

  // Emit chỉ để lấy JavaScript; kiểu đã kiểm xong ở trên nên `transpileModule`
  // ở bước này là đủ và nhanh hơn nhiều so với emit qua Program.
  const { outputText } = TS.transpileModule(ma, { compilerOptions: opts, fileName: TEN_TEP });
  return { chanDoan, js: outputText };
}
