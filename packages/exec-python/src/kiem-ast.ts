/**
 * Đánh giá luật chấm `tier: static` cho Python.
 *
 * Chấm bằng `output` không phân biệt được "đáp án đúng" với "đáp án tình cờ
 * ra đúng số": bài dạy dấu ngoặc, starter là `if ___:`, và `if True:` cũng in
 * ra câu mà bài mong đợi. Tầng `static` hỏi thẳng vào HÌNH DẠNG của mã người
 * học viết, nên nó bắt được đúng chỗ mà output bỏ qua.
 *
 * Dùng module `ast` của chính CPython chứ không tự viết bộ phân tích. Tự viết
 * nghĩa là có một bộ phân tích Python thứ hai trong dự án, và hai bộ phân tích
 * thì sớm muộn cũng lệch nhau — mà lệch ở đây nghĩa là chấm sai.
 *
 * MASTERPLAN §5 nói rõ: AST query, KHÔNG regex trên mã nguồn. Regex trên mã
 * không phân biệt nổi `not (a and b)` với chuỗi `"not (a and b)"` trong một
 * comment.
 */

import type { Pyodide } from './worker-body.js';

/** Một truy vấn AST, đúng hình dạng `AstQuery` của schema v2. */
export interface TruyVanAst {
  lang: string;
  kind: string;
  target?: string;
  min?: number;
  /**
   * Số lần TỐI ĐA được phép. Có \`max\` mới nói được câu "đừng khai triển ra".
   *
   * Bài 8 của T2.2 dạy GẤP \`15000 * n + 15000 * 2\` lại thành
   * \`15000 * (n + 2)\`. Hai câu ấy bằng nhau ở mọi \`n\`, nên không assert nào
   * và không tier \`output\` nào phân biệt được — người học làm ĐÚNG NGƯỢC
   * bài học (mở ngoặc ra, tức bài 7) vẫn qua sạch. Chỉ đếm số dấu nhân mới
   * thấy: dạng gấp có 1, dạng khai triển có 2. Mà \`min\` thì không nói được
   * "nhiều nhất bấy nhiêu".
   */
  max?: number;
}

export interface KetQuaAst {
  /** Mọi truy vấn `requireAst` đều thoả và không truy vấn `forbidAst` nào thoả. */
  dat: boolean;
  /** Truy vấn không thoả, để nói cho người học biết còn thiếu gì. */
  thieu: TruyVanAst[];
  /** Truy vấn bị cấm mà vẫn xuất hiện. */
  cam: TruyVanAst[];
  /** Mã không phân tích được — khác hẳn "phân tích được nhưng sai hình dạng". */
  loi_cu_phap: string | null;
}

/**
 * Đoạn Python đếm số lần mỗi hình dạng xuất hiện.
 *
 * Chạy trong Pyodide nên nó là CPython thật; `ast.parse` ở đây là đúng cùng
 * hàm mà `python3` dùng.
 */
const DEM = `
import ast, json

def _dem(nguon, cac_truy_van):
    cay = ast.parse(nguon)
    ra = []
    for tv in cac_truy_van:
        kind = tv.get("kind")
        tg = tv.get("target")
        n = 0
        for nut in ast.walk(cay):
            if kind == "uses-call":
                if isinstance(nut, ast.Call):
                    ten = getattr(nut.func, "id", None) or getattr(nut.func, "attr", None)
                    if tg is None or ten == tg:
                        n += 1
            elif kind == "uses-operator":
                n += _khop_toan_tu(nut, tg)
            elif kind == "uses-fstring":
                if isinstance(nut, ast.JoinedStr):
                    n += 1
            elif kind == "uses-name":
                # Chỉ đếm chỗ ĐỌC tên, không đếm chỗ gán nó.
                #
                # \`tien = 0\` là đặt tên, không phải dùng tên. Luật \`uses-name\`
                # tồn tại để hỏi "lời giải có THAM CHIẾU tới biến này không",
                # mà một dòng gán thì chưa tham chiếu gì cả — đếm nó vào sẽ cho
                # qua đúng những đáp án mà luật này sinh ra để chặn.
                if (isinstance(nut, ast.Name) and isinstance(nut.ctx, ast.Load)
                        and (tg is None or nut.id == tg)):
                    n += 1
            elif kind == "subscript-assign":
                # Gán VÀO MỘT Ô: \`chi[khoa] = gia_tri\`. Người viết T1.4 báo
                # thiếu đúng luật này, và phải lách bằng cách đếm số lần đọc
                # tên — một con số phụ thuộc số lệnh \`print\` trong khung, nên
                # sửa khung là luật chấm hỏng lặng lẽ.
                if isinstance(nut, (ast.Assign, ast.AugAssign)):
                    dich = nut.targets if isinstance(nut, ast.Assign) else [nut.target]
                    for d in dich:
                        if isinstance(d, ast.Subscript):
                            ten = getattr(d.value, "id", None)
                            if tg is None or ten == tg:
                                n += 1
            elif kind == "has-literal":
                if isinstance(nut, ast.Constant):
                    # So bằng CHUỖI HOÁ để tác giả viết target là văn bản thuần:
                    # target "10000" khớp cả 10000 lẫn "10000" — người viết bài
                    # không phải nhớ kiểu, và trong ngữ cảnh "chặn đáp án chép
                    # cứng" thì cả hai đều là chép cứng.
                    #
                    # \`str(tg)\` chứ không phải \`tg\`: bộ đọc \`.lesson.md\` đổi mọi
                    # giá trị toàn chữ số thành SỐ, nên \`target: 45\` tới đây là int
                    # 45 và \`str(nut.value) == tg\` không bao giờ đúng — luật im
                    # lặng không chạy, đúng thứ tệ nhất một luật chấm có thể làm.
                    if tg is None or str(nut.value) == str(tg):
                        n += 1
            elif kind == "nesting":
                n += _khop_long(nut, tg)
            elif kind == "comprehension":
                if isinstance(nut, (ast.ListComp, ast.SetComp, ast.DictComp, ast.GeneratorExp)):
                    n += 1
            elif kind == "lambda":
                if isinstance(nut, ast.Lambda):
                    n += 1
            elif kind == "match-stmt":
                if isinstance(nut, getattr(ast, "Match", ())):
                    n += 1
            elif kind == "no-import":
                if isinstance(nut, (ast.Import, ast.ImportFrom)):
                    n += 1
        ra.append(n)
    return ra

_TOAN_TU = {
    "+": ast.Add, "-": ast.Sub, "*": ast.Mult, "/": ast.Div,
    "//": ast.FloorDiv, "%": ast.Mod, "**": ast.Pow,
    "==": ast.Eq, "!=": ast.NotEq, "<": ast.Lt, "<=": ast.LtE,
    ">": ast.Gt, ">=": ast.GtE,
}

def _khop_toan_tu(nut, tg):
    # Dấu âm MỘT NGÔI (\`-3\`) là \`UnaryOp(USub)\`, không phải \`BinOp(Sub)\`, nên
    # target \`-\` không đếm nó. Trước khi có target riêng này, bài
    # \`khi-khong-chay-nguoc-duoc\` không có cách nào đòi người học viết ra dấu
    # âm: máy vuông nhả ĐÚNG CÙNG con số cho 3 và −3 — đó chính là điều bài dạy
    # — nên không assert nào và không tier \`output\` nào phân biệt nổi
    # \`dien_tich(3)\` với \`dien_tich(-3)\`. Chỉ nhìn vào MÃ mới thấy.
    if tg in ("neg", "dau-am"):
        return 1 if isinstance(nut, ast.UnaryOp) and isinstance(nut.op, ast.USub) else 0
    if tg == "and":
        return 1 if isinstance(nut, ast.BoolOp) and isinstance(nut.op, ast.And) else 0
    if tg == "or":
        return 1 if isinstance(nut, ast.BoolOp) and isinstance(nut.op, ast.Or) else 0
    if tg == "not":
        return 1 if isinstance(nut, ast.UnaryOp) and isinstance(nut.op, ast.Not) else 0
    lop = _TOAN_TU.get(tg)
    if lop is None:
        return 0
    if isinstance(nut, ast.BinOp) and isinstance(nut.op, lop):
        return 1
    if isinstance(nut, ast.Compare) and any(isinstance(o, lop) for o in nut.ops):
        return 1
    if isinstance(nut, ast.AugAssign) and isinstance(nut.op, lop):
        return 1
    return 0

def _loai(nut, ten):
    if ten in ("neg", "dau-am"):
        return isinstance(nut, ast.UnaryOp) and isinstance(nut.op, ast.USub)
    if ten in ("and", "or"):
        lop = ast.And if ten == "and" else ast.Or
        return isinstance(nut, ast.BoolOp) and isinstance(nut.op, lop)
    if ten == "not":
        return isinstance(nut, ast.UnaryOp) and isinstance(nut.op, ast.Not)
    if ten == "call":
        return isinstance(nut, ast.Call)
    if ten == "if":
        return isinstance(nut, (ast.If, ast.IfExp))
    if ten == "for":
        return isinstance(nut, (ast.For, ast.AsyncFor))
    if ten == "while":
        return isinstance(nut, ast.While)
    return False

def _khop_long(nut, tg):
    """\`ngoài/trong\` — nút ngoài chứa TRỰC TIẾP một nút trong.

    Trực tiếp, không phải bắc cầu: \`not (a and b)\` khớp \`not/and\`, còn
    \`not a and b\` thì không — đúng chỗ mà một cặp ngoặc thay đổi.
    """
    if not tg or "/" not in tg:
        return 0
    ngoai, trong = tg.split("/", 1)
    if not _loai(nut, ngoai):
        return 0
    for con in ast.iter_child_nodes(nut):
        if _loai(con, trong):
            return 1
    return 0
`;

/** Những kind mang nghĩa PHỦ ĐỊNH: xuất hiện là hỏng, dù nằm ở `requireAst`. */
const KIND_PHU_DINH = new Set(['no-import', 'no-mutation', 'no-global']);

export function kiemAst(
  py: Pyodide,
  ma: string,
  yeu_cau: TruyVanAst[] = [],
  cam: TruyVanAst[] = [],
): KetQuaAst {
  py.runPython(DEM);
  const dem = (cac: TruyVanAst[]): number[] => {
    if (cac.length === 0) return [];
    py.globals.set('_nguon', ma);
    py.globals.set('_tv', JSON.stringify(cac));
    py.runPython('_kq = json.dumps(_dem(_nguon, json.loads(_tv)))');
    return JSON.parse(String(py.globals.get('_kq'))) as number[];
  };

  try {
    const n_yeu = dem(yeu_cau);
    const n_cam = dem(cam);
    const thieu = yeu_cau.filter((q, i) => {
      const n = n_yeu[i] ?? 0;
      // `no-*` đảo chiều: có mặt là hỏng.
      if (KIND_PHU_DINH.has(q.kind)) return n > 0;
      // `max` là trần: quá là hỏng. Một truy vấn khai cả `min` lẫn `max` thì
      // phải lọt vào giữa — đó là cách nói "đúng chừng này, đừng khai triển
      // thêm".
      if (q.max !== undefined && n > q.max) return true;
      return n < (q.min ?? (q.max !== undefined ? 0 : 1));
    });
    const vi_pham = cam.filter((_, i) => (n_cam[i] ?? 0) > 0);
    return {
      dat: thieu.length === 0 && vi_pham.length === 0,
      thieu,
      cam: vi_pham,
      loi_cu_phap: null,
    };
  } catch (e) {
    // Mã không phân tích được thì KHÔNG kết luận là sai hình dạng — đó là lỗi
    // cú pháp, và tầng `run` sẽ báo nó bằng thông báo dễ hiểu hơn nhiều.
    return {
      dat: false,
      thieu: [],
      cam: [],
      loi_cu_phap: e instanceof Error ? e.message : String(e),
    };
  }
}
