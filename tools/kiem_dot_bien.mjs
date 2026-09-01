// Cổng ĐỘT BIẾN: một cách chấm phải TRƯỢT được đáp án sai.
//
// `kiem_ma_bai_hoc.mjs` đã canh chiều xuôi — lời giải thật phải chạy, phải ra
// đúng output, mọi assert phải đạt. Nó cũng thử điền bừa `True`/`1`/`0`. Nhưng
// đáp án sai mà người học THẬT SỰ viết ra không bao giờ là `True`: nó là một
// biểu thức gần đúng.
//
// Vòng phản biện T2.2 bắt được đúng chỗ ấy. Bài 3 dạy phân biệt HÌNH DẠNG của
// ba tờ giấy, chấm bằng đúng một giá trị `n = 8`; viết `15000 * n + 30000` thay
// cho `15000 * (n + 2)` là sai hình dạng mà trùng giá trị tại n = 8, nên qua
// sạch cả bốn tầng chấm. `True` không bao giờ lộ ra chuyện đó.
//
// Nên cổng này làm một việc: lấy chính lời giải, sửa đi một chỗ nhỏ TRONG
// PHẦN NGƯỜI HỌC PHẢI ĐIỀN, rồi hỏi cách chấm có bắt được không. Bắt không
// được nghĩa là bài ấy đang cho điểm một thứ nó không thật sự kiểm.
//
// Chỉ đột biến trên những DÒNG có `___` ở mã khởi đầu. Sửa chỗ khác thì không
// công bằng: người học không gõ được vào đó, nên nó không phải lỗ chấm điểm.

import { readdirSync, readFileSync, existsSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { createRequire } from 'node:module';
import { pathToFileURL } from 'node:url';
import { Worker } from 'node:worker_threads';

const THU_MUC = process.argv[2] ?? 'dist/content';
const MIEN_TRU = 'content/curriculum/dot-bien-bo-qua.yaml';

const { kiemAst } = await import(new URL('../packages/exec-python/dist/kiem-ast.js', import.meta.url).href);
const require = createRequire(new URL('../packages/exec-python/package.json', import.meta.url));
const { loadPyodide } = await import(pathToFileURL(require.resolve('pyodide/pyodide.mjs')).href);
const py = await loadPyodide();

// ── TypeScript: cùng nguyên tắc "đột biến theo Ý", engine khác ────────────
//
// Trước bản vá này, cổng đột biến CHỈ chạy trên Python (`c.lang !== 'python'`
// → bỏ qua) — nghĩa là T4.0a (18 bài TypeScript) chưa từng được kiểm đột
// biến, dù `kiem_ma_bai_hoc.mjs` đã chạy được TypeScript từ lâu. Đúng lớp lỗ
// hổng "một cổng không đo GÌ CẢ cho cả một ngôn ngữ" đã vá cho
// `kiem_ma_bai_hoc.mjs` — giờ vá tương đương ở đây.
//
// Bộ sinh đột biến TypeScript dùng THẲNG TypeScript compiler API thật
// (`ts.createSourceFile` → duyệt/sửa cây → `ts.createPrinter().printFile`)
// — không phải quét chuỗi. Cùng lý do bản Python không thay chuỗi: một `+`
// trong một chuỗi ký tự cũng bị sửa nếu regex không hiểu ngữ cảnh.
const { BoThucThiTypeScript, kiemKieu } = await import(new URL('../packages/exec-typescript/dist/index.js', import.meta.url).href);
const require_ts = createRequire(new URL('../packages/exec-typescript/package.json', import.meta.url));
const TS = require_ts('typescript');
const TS_LIB_DIR = dirname(require_ts.resolve('typescript'));
const doc_lib_ts = (ten) => {
  try {
    return readFileSync(join(TS_LIB_DIR, ten), 'utf-8');
  } catch {
    return undefined;
  }
};
const kiem_kieu_truoc = (ma) => kiemKieu(TS, ma, doc_lib_ts);

const TS_WORKER_URL = new URL('../packages/exec-typescript/node-worker.mjs', import.meta.url);
function tao_cong_ts() {
  const w = new Worker(TS_WORKER_URL);
  let handler = null;
  w.on('message', (m) => handler?.(m));
  w.on('error', () => {});
  return {
    gui: (tin) => w.postMessage(tin),
    khiNhan: (f) => { handler = f; },
    giet: () => { w.terminate(); },
  };
}
const ts_engine = new BoThucThiTypeScript(tao_cong_ts, kiem_kieu_truoc);

/** Chạy một đoạn TypeScript, trả về {ok, xuat} — cùng hình dạng chay(). */
async function chay_ts(ma, ma_kiem_tra) {
  const r = await ts_engine.chay(ma, ma_kiem_tra ? { maKiemTra: ma_kiem_tra } : {});
  return { ok: r.ok, xuat: r.xuat };
}

const TRAN_DONG = 5000;

// Đột biến có thể biến một vòng lặp hữu hạn thành vòng lặp vô tận: đổi `<`
// thành `<=` ở đúng chỗ, hoặc đổi bước nhảy, là chương trình chạy mãi. Lượt
// chạy đầu tiên của cổng này treo ở bài thứ 120, 102% CPU, không nhả ra nữa.
//
// Nên mã đột biến chạy dưới một hạn mức BƯỚC, đếm bằng `sys.settrace`. Một
// cổng treo thì cũng vô dụng như một cổng báo sai — không ai đợi nó xong.
py.runPython(`
import sys

def chay_co_han(ma, tran=300000):
    dem = [0]
    def theo_doi(frame, su_kien, gt):
        dem[0] += 1
        if dem[0] > tran:
            raise RuntimeError('vượt hạn mức bước — đột biến làm chương trình chạy mãi')
        return theo_doi
    # \`globals\` riêng cho mỗi lần chạy: vừa là cách dọn sạch tên của lần
    # trước, vừa rẻ hơn quét cả globals() rồi pop từng khoá.
    sys.settrace(theo_doi)
    try:
        exec(ma, {'__name__': '__main__'})
    finally:
        sys.settrace(None)
`);
const chay_co_han = py.globals.get('chay_co_han');

function chay(ma) {
  const dong = [];
  const loi_goc = console.error;
  console.error = () => {};
  try {
    py.setStdout({ batched: (s) => { if (dong.length < TRAN_DONG) dong.push(s); } });
    chay_co_han(ma);
    return { ok: true, xuat: dong.join('\n') };
  } catch {
    return { ok: false, xuat: dong.join('\n') };
  } finally {
    console.error = loi_goc;
  }
}

function khop(thuc, luat) {
  const mong = String(luat.expected ?? '');
  switch (luat.match ?? 'contains') {
    case 'exact': return thuc === mong;
    case 'trim': return thuc.trim() === mong.trim();
    case 'regex': return new RegExp(mong).test(thuc);
    default: return thuc.includes(mong.trim());
  }
}

// Bộ sinh đột biến chạy bằng `ast` của chính Python — không phải bằng thay
// chuỗi. Thay chuỗi thì `+` trong một chuỗi văn bản cũng bị sửa, và cái đột
// biến ấy chỉ chứng minh được rằng công cụ hỏng.
py.runPython(`
import ast, json

# Đột biến sửa MỌI nút cùng loại trên các dòng người học phải điền, không phải
# một nút.
#
# Bản đầu sửa đúng một nút, và nó báo oan ngay bài đầu tiên soi kỹ:
# \`dung-hay-sai\` có ba chỗ trống điền GIỐNG HỆT nhau
# (\`tien_khach_dua >= gia_pho\`), chấm bằng ba tình huống mà tình huống thứ ba
# — đưa vừa đúng 45000 — mới là chỗ phân biệt \`>\` với \`>=\`. Sửa mỗi chỗ
# trống ĐẦU thì tình huống ấy vẫn chạy trên \`>=\` cũ, output không đổi, và cổng
# kết luận "bài không chấm được \`>\`" — trong khi bài chấm được, thậm chí có hẳn
# một đoạn chú thích giải thích vì sao nó chấm được.
#
# Người học không sai một chỗ. Họ hiểu sai MỘT ĐIỀU, rồi viết cái hiểu sai ấy
# ra ở mọi chỗ. Nên đột biến phải đi theo Ý, không theo vị trí.

class _Doi(ast.NodeTransformer):
    """Sửa mọi nút khớp \`(loai, khoa)\` nằm trên dòng người học điền."""
    def __init__(self, dong_duoc_sua, loai, khoa, thay_the=None):
        self.dong = dong_duoc_sua
        self.loai = loai
        self.khoa = khoa
        self.thay_the = thay_the
        self.dinh = 0

    def _duoc(self, node):
        return getattr(node, 'lineno', None) in self.dong

    def visit_BinOp(self, node):
        self.generic_visit(node)
        if self.loai != 'phep' or not self._duoc(node):
            return node
        if type(node.op).__name__ != self.khoa:
            return node
        moi = _PHEP.get(self.khoa)
        if not moi:
            return node
        self.dinh += 1
        return ast.BinOp(left=node.left, op=moi(), right=node.right)

    def visit_Constant(self, node):
        # Hai lớp cùng nằm ở Constant: hằng số nguyên và hằng CHUỖI.
        #
        # Người viết T1.5 báo cổng này sinh 0 đột biến trên nhiều bài của họ,
        # vì chỗ trống là chuỗi chế độ mở file, tên phương thức, tên lỗi — không
        # có phép toán hay số nguyên nào để lật. Cổng xanh mà không kiểm gì.
        if self.loai == 'chuoi' and self._duoc(node) and isinstance(node.value, str):
            if node.value == self.khoa and self.thay_the is not None:
                self.dinh += 1
                return ast.Constant(value=self.thay_the)
            return node
        if self.loai != 'hang' or not self._duoc(node):
            return node
        if not isinstance(node.value, int) or isinstance(node.value, bool):
            return node
        if node.value != self.khoa:
            return node
        self.dinh += 1
        return ast.Constant(value=node.value + 1)

    def visit_Name(self, node):
        # Dùng nhầm một cái tên đang bày ngay bên cạnh — lỗi thật hay gặp nhất.
        if self.loai != 'ten' or not self._duoc(node):
            return node
        if isinstance(node.ctx, ast.Load) and node.id == self.khoa and self.thay_the:
            self.dinh += 1
            return ast.Name(id=self.thay_the, ctx=ast.Load())
        return node

    def visit_Attribute(self, node):
        # .read() thay vì .readlines(), .rstrip() thay vì .strip().
        self.generic_visit(node)
        if self.loai != 'thuoc_tinh' or not self._duoc(node):
            return node
        if node.attr == self.khoa and self.thay_the:
            self.dinh += 1
            return ast.Attribute(value=node.value, attr=self.thay_the, ctx=node.ctx)
        return node

    def visit_Subscript(self, node):
        # Bỏ lát cắt trọn vẹn: so[:] thành so. Đúng cái lỗi bí danh mà mấy bài
        # về bản sao dựng lên để dạy, mà nó không đụng tới con số nào.
        self.generic_visit(node)
        if self.loai != 'bo_lat_cat' or not self._duoc(node):
            return node
        s = node.slice
        if (isinstance(s, ast.Slice) and s.lower is None and s.upper is None
                and s.step is None):
            self.dinh += 1
            return node.value
        return node

    def visit_BoolOp(self, node):
        # Đổi and thành or, hoặc ngược lại. Trong Python chúng là BoolOp chứ
        # không phải BinOp, nên bảng _PHEP không với tới.
        #
        # Người viết T2.3 báo đúng chỗ này: cả track LOGIC chỉ điền and, or,
        # not và vài cái tên — không hằng số, không phép so sánh. Cổng sinh ra
        # 17 đột biến toàn là đổi tên, bắt hết 17, rồi báo xanh — trong khi nó
        # KHÔNG hề thử được đúng cái lỗi trung tâm của mạch.
        self.generic_visit(node)
        if self.loai != 'boolop' or not self._duoc(node):
            return node
        if type(node.op).__name__ != self.khoa:
            return node
        moi = ast.Or if isinstance(node.op, ast.And) else ast.And
        self.dinh += 1
        return ast.BoolOp(op=moi(), values=node.values)

    def visit_UnaryOp(self, node):
        # BỎ một chữ not. Thêm vào thì dễ sinh mã vô nghĩa; bỏ đi mới là lỗi
        # người học thật sự mắc, và nó lật hẳn nghĩa của câu.
        self.generic_visit(node)
        if self.loai != 'bo_not' or not self._duoc(node):
            return node
        if isinstance(node.op, ast.Not):
            self.dinh += 1
            return node.operand
        return node

    def visit_Compare(self, node):
        self.generic_visit(node)
        if self.loai != 'sosanh' or not self._duoc(node) or len(node.ops) != 1:
            return node
        if type(node.ops[0]).__name__ != self.khoa:
            return node
        moi = _SS.get(self.khoa)
        if not moi:
            return node
        self.dinh += 1
        return ast.Compare(left=node.left, ops=[moi()], comparators=node.comparators)

_DUNG_SAN = {
    'print', 'len', 'int', 'str', 'float', 'bool', 'list', 'dict', 'set',
    'tuple', 'range', 'sum', 'min', 'max', 'sorted', 'abs', 'round', 'input',
    'open', 'enumerate', 'zip', 'any', 'all', 'type', 'isinstance',
}

_PHEP = {'Add': ast.Sub, 'Sub': ast.Add, 'Mult': ast.Add, 'Div': ast.Mult}
_SS = {'Eq': ast.NotEq, 'NotEq': ast.Eq, 'Lt': ast.LtE, 'LtE': ast.Lt,
       'Gt': ast.GtE, 'GtE': ast.Gt,
       'Is': ast.IsNot, 'IsNot': ast.Is, 'In': ast.NotIn, 'NotIn': ast.In}
_TEN = {'Add': 'cộng', 'Sub': 'trừ', 'Mult': 'nhân', 'Div': 'chia',
        'Eq': '==', 'NotEq': '!=', 'Lt': '<', 'LtE': '<=', 'Gt': '>', 'GtE': '>='}


def sinh_dot_bien(ma, dong_duoc_sua, tran=6):
    dong_duoc_sua = set(dong_duoc_sua)
    try:
        goc = ast.parse(ma)
    except SyntaxError:
        return json.dumps([])
    goc_txt = ast.unparse(goc).strip()

    # Gom các Ý sửa được, theo thứ tự gặp, không trùng.
    # Ba lớp chuỗi/tên/thuộc tính đổi lấy MỘT GIÁ TRỊ KHÁC ĐANG CÓ MẶT trong
    # chính lời giải, không bịa giá trị mới. Đó mới là lỗi thật: chọn nhầm một
    # trong mấy thứ đang bày ra trước mắt.
    chuoi, ten, thuoc_tinh = [], [], []
    for nut in ast.walk(goc):
        if getattr(nut, 'lineno', None) not in dong_duoc_sua:
            continue
        if isinstance(nut, ast.Constant) and isinstance(nut.value, str):
            if nut.value not in chuoi:
                chuoi.append(nut.value)
        elif isinstance(nut, ast.Name) and isinstance(nut.ctx, ast.Load):
            # Không gom hàm dựng sẵn vào rổ thay thế. Đổi một cái tên thành
            # 'print' hay 'len' không phải lỗi người học nào viết ra — nó chỉ
            # là nhiễu, và nhiễu trong một cổng thì tốn đúng thứ mà cổng sinh
            # ra để tiết kiệm: lòng tin rằng đỏ nghĩa là có chuyện.
            if nut.id not in ten and nut.id not in _DUNG_SAN:
                ten.append(nut.id)
        elif isinstance(nut, ast.Attribute):
            if nut.attr not in thuoc_tinh:
                thuoc_tinh.append(nut.attr)

    y = []
    for nut in ast.walk(goc):
        if getattr(nut, 'lineno', None) not in dong_duoc_sua:
            continue
        if isinstance(nut, ast.BinOp) and type(nut.op).__name__ in _PHEP:
            k = ('phep', type(nut.op).__name__, None)
        elif isinstance(nut, ast.Compare) and len(nut.ops) == 1 and type(nut.ops[0]).__name__ in _SS:
            k = ('sosanh', type(nut.ops[0]).__name__, None)
        elif isinstance(nut, ast.Constant) and isinstance(nut.value, int) and not isinstance(nut.value, bool):
            k = ('hang', nut.value, None)
        elif isinstance(nut, ast.Constant) and isinstance(nut.value, str) and len(chuoi) > 1:
            k = ('chuoi', nut.value, [c for c in chuoi if c != nut.value][0])
        elif isinstance(nut, ast.Name) and isinstance(nut.ctx, ast.Load) and len(ten) > 1:
            k = ('ten', nut.id, [c for c in ten if c != nut.id][0])
        elif isinstance(nut, ast.Attribute) and len(thuoc_tinh) > 1:
            k = ('thuoc_tinh', nut.attr, [c for c in thuoc_tinh if c != nut.attr][0])
        elif isinstance(nut, ast.BoolOp):
            k = ('boolop', type(nut.op).__name__, None)
        elif isinstance(nut, ast.UnaryOp) and isinstance(nut.op, ast.Not):
            k = ('bo_not', None, None)
        elif isinstance(nut, ast.Subscript) and isinstance(nut.slice, ast.Slice) \
                and nut.slice.lower is None and nut.slice.upper is None \
                and nut.slice.step is None:
            k = ('bo_lat_cat', None, None)
        else:
            continue
        if k not in y:
            y.append(k)

    ra = []
    for loai, khoa, thay in y[:tran]:
        t = _Doi(dong_duoc_sua, loai, khoa, thay)
        cay = t.visit(ast.parse(ma))
        if not t.dinh:
            continue
        ast.fix_missing_locations(cay)
        try:
            moi = ast.unparse(cay)
        except Exception:
            continue
        if moi.strip() == goc_txt:
            continue
        if loai == 'hang':
            mo_ta = f'đổi MỌI hằng số {khoa} thành {khoa + 1} ({t.dinh} chỗ)'
        elif loai == 'phep':
            mo_ta = f'đổi MỌI phép {_TEN[khoa]} thành {_TEN[_PHEP[khoa].__name__]} ({t.dinh} chỗ)'
        elif loai == 'sosanh':
            m1 = _TEN.get(khoa, khoa)
            m2 = _SS[khoa].__name__
            mo_ta = f'đổi MỌI dấu {m1} thành {_TEN.get(m2, m2)} ({t.dinh} chỗ)'
        elif loai == 'chuoi':
            mo_ta = f'đổi MỌI chuỗi {khoa!r} thành {thay!r} ({t.dinh} chỗ)'
        elif loai == 'ten':
            mo_ta = f'đổi MỌI chỗ đọc tên {khoa} thành {thay} ({t.dinh} chỗ)'
        elif loai == 'thuoc_tinh':
            mo_ta = f'đổi MỌI .{khoa} thành .{thay} ({t.dinh} chỗ)'
        elif loai == 'boolop':
            cu_, moi_ = ('and', 'or') if khoa == 'And' else ('or', 'and')
            mo_ta = f'đổi MỌI {cu_} thành {moi_} ({t.dinh} chỗ)'
        elif loai == 'bo_not':
            mo_ta = f'BỎ mọi chữ not ({t.dinh} chỗ)'
        else:
            mo_ta = f'bỏ lát cắt trọn vẹn [:] ({t.dinh} chỗ)'
        ra.append([mo_ta, moi])
    return json.dumps(ra, ensure_ascii=False)
`);
const sinh = py.globals.get('sinh_dot_bien');

// ── Bộ sinh đột biến TypeScript — mirror triết lý của bản Python ở trên ───
//
// Không cài lại bằng py.runPython: TypeScript compiler API là JS thật, gọi
// thẳng từ đây rẻ hơn và không phải đi vòng qua Pyodide.
const { factory: TS_F } = TS;

// Ba bảng toán tử — TÁCH RIÊNG số học/so sánh vì TypeScript biểu diễn CẢ HAI
// bằng cùng một loại nút (`BinaryExpression`), chỉ khác `operatorToken.kind`.
// Python không cần tách vì `ast.BinOp` (số học) và `ast.Compare` (so sánh)
// vốn đã là hai loại nút khác nhau.
const TOAN_TU_SO = new Map([
  [TS.SyntaxKind.PlusToken, TS.SyntaxKind.MinusToken],
  [TS.SyntaxKind.MinusToken, TS.SyntaxKind.PlusToken],
  [TS.SyntaxKind.AsteriskToken, TS.SyntaxKind.PlusToken],
  [TS.SyntaxKind.SlashToken, TS.SyntaxKind.AsteriskToken],
]);
const TOAN_TU_SS = new Map([
  [TS.SyntaxKind.EqualsEqualsToken, TS.SyntaxKind.ExclamationEqualsToken],
  [TS.SyntaxKind.ExclamationEqualsToken, TS.SyntaxKind.EqualsEqualsToken],
  [TS.SyntaxKind.EqualsEqualsEqualsToken, TS.SyntaxKind.ExclamationEqualsEqualsToken],
  [TS.SyntaxKind.ExclamationEqualsEqualsToken, TS.SyntaxKind.EqualsEqualsEqualsToken],
  [TS.SyntaxKind.LessThanToken, TS.SyntaxKind.LessThanEqualsToken],
  [TS.SyntaxKind.LessThanEqualsToken, TS.SyntaxKind.LessThanToken],
  [TS.SyntaxKind.GreaterThanToken, TS.SyntaxKind.GreaterThanEqualsToken],
  [TS.SyntaxKind.GreaterThanEqualsToken, TS.SyntaxKind.GreaterThanToken],
]);
const TEN_TOAN_TU = new Map([
  [TS.SyntaxKind.PlusToken, 'cộng'], [TS.SyntaxKind.MinusToken, 'trừ'],
  [TS.SyntaxKind.AsteriskToken, 'nhân'], [TS.SyntaxKind.SlashToken, 'chia'],
  [TS.SyntaxKind.EqualsEqualsToken, '=='], [TS.SyntaxKind.ExclamationEqualsToken, '!='],
  [TS.SyntaxKind.EqualsEqualsEqualsToken, '==='], [TS.SyntaxKind.ExclamationEqualsEqualsToken, '!=='],
  [TS.SyntaxKind.LessThanToken, '<'], [TS.SyntaxKind.LessThanEqualsToken, '<='],
  [TS.SyntaxKind.GreaterThanToken, '>'], [TS.SyntaxKind.GreaterThanEqualsToken, '>='],
]);

// Không gom vào rổ thay thế — cùng vai trò `_DUNG_SAN` bên Python: đổi một
// tên thành `console`/`Math` không phải lỗi người học nào viết ra, chỉ là
// nhiễu.
const DUNG_SAN_TS = new Set([
  'console', 'Math', 'JSON', 'Object', 'Array', 'String', 'Number', 'Boolean',
  'parseInt', 'parseFloat', 'isNaN', 'undefined',
]);

/** Một Identifier có đang ở VỊ TRÍ ĐỌC không — đối lập VỊ TRÍ KHAI BÁO/GÁN.
 *
 *  Python phân biệt việc này thẳng bằng `ast.Load` vs `ast.Store`. TypeScript
 *  không có cờ tương đương trên chính node — phải tự suy từ NODE CHA. Bỏ sót
 *  một trường hợp ở đây nghĩa là đột biến có thể sửa nhầm một TÊN ĐANG ĐƯỢC
 *  KHAI BÁO (`let x = ...`) thay vì một tên đang được ĐỌC — sinh ra mã không
 *  còn parse được, hoặc parse được nhưng không còn là "cùng lỗi, khác vị
 *  trí" như đột biến phải là.
 */
function la_vi_tri_doc_ts(node) {
  const p = node.parent;
  if (!p) return true;
  if (TS.isVariableDeclaration(p) && p.name === node) return false;
  if (TS.isParameter(p) && p.name === node) return false;
  if (TS.isBindingElement(p) && p.name === node) return false;
  if ((TS.isFunctionDeclaration(p) || TS.isFunctionExpression(p) || TS.isClassDeclaration(p)
    || TS.isInterfaceDeclaration(p) || TS.isTypeAliasDeclaration(p) || TS.isEnumDeclaration(p))
    && p.name === node) return false;
  if ((TS.isPropertyAssignment(p) || TS.isShorthandPropertyAssignment(p)) && p.name === node) return false;
  if ((TS.isPropertySignature(p) || TS.isMethodSignature(p) || TS.isPropertyDeclaration(p)
    || TS.isMethodDeclaration(p)) && p.name === node) return false;
  // `.thuoc_tinh` — đây là loại 'thuoc_tinh' riêng, không phải 'ten'.
  if (TS.isPropertyAccessExpression(p) && p.name === node) return false;
  if (TS.isBinaryExpression(p) && p.operatorToken.kind === TS.SyntaxKind.EqualsToken && p.left === node) return false;
  if (TS.isImportSpecifier(p) || TS.isExportSpecifier(p)) return false;
  if (TS.isLabeledStatement(p) && p.label === node) return false;
  return true;
}

/** Sinh mọi đột biến TypeScript có thể trên các DÒNG người học phải điền.
 *
 *  Trả về `[[mo_ta, ma_moi], ...]` — cùng hình dạng `JSON.parse(sinh(...))`
 *  bên Python, để vòng lặp chính dùng chung một đường xử lý.
 */
function sinh_dot_bien_ts(ma, dong_duoc_sua, tran = 6) {
  const dong = new Set(dong_duoc_sua);
  let goc;
  try {
    goc = TS.createSourceFile('loi_giai.ts', ma, TS.ScriptTarget.Latest, true);
  } catch {
    return [];
  }
  const printer = TS.createPrinter({ newLine: TS.NewLineKind.LineFeed });
  const goc_txt = printer.printFile(goc).trim();
  const line_cua = (node) => goc.getLineAndCharacterOfPosition(node.getStart(goc)).line + 1;

  // Gom rổ giá trị thay thế — CÙNG giá trị đang có mặt trong chính lời giải,
  // không bịa giá trị mới, đúng nguyên tắc bản Python.
  const chuoi = [];
  const ten = [];
  const thuoc_tinh = [];
  (function gom(node) {
    if (dong.has(line_cua(node))) {
      if (TS.isStringLiteral(node)) {
        if (!chuoi.includes(node.text)) chuoi.push(node.text);
      } else if (TS.isIdentifier(node) && la_vi_tri_doc_ts(node) && !DUNG_SAN_TS.has(node.text)) {
        if (!ten.includes(node.text)) ten.push(node.text);
      } else if (TS.isPropertyAccessExpression(node)) {
        if (!thuoc_tinh.includes(node.name.text)) thuoc_tinh.push(node.name.text);
      }
    }
    TS.forEachChild(node, gom);
  })(goc);

  // Gom các Ý sửa được, theo thứ tự gặp, không trùng.
  const y = [];
  const y_da_thay = new Set();
  (function gomY(node) {
    if (dong.has(line_cua(node))) {
      let k = null;
      if (TS.isBinaryExpression(node) && TOAN_TU_SO.has(node.operatorToken.kind)) {
        k = ['phep', node.operatorToken.kind, null];
      } else if (TS.isBinaryExpression(node) && TOAN_TU_SS.has(node.operatorToken.kind)) {
        k = ['sosanh', node.operatorToken.kind, null];
      } else if (TS.isNumericLiteral(node)) {
        k = ['hang', node.text, null];
      } else if (TS.isStringLiteral(node) && chuoi.length > 1) {
        const khac = chuoi.find((c) => c !== node.text);
        if (khac !== undefined) k = ['chuoi', node.text, khac];
      } else if (TS.isIdentifier(node) && la_vi_tri_doc_ts(node) && !DUNG_SAN_TS.has(node.text) && ten.length > 1) {
        const khac = ten.find((t) => t !== node.text);
        if (khac !== undefined) k = ['ten', node.text, khac];
      } else if (TS.isPropertyAccessExpression(node) && thuoc_tinh.length > 1) {
        const khac = thuoc_tinh.find((t) => t !== node.name.text);
        if (khac !== undefined) k = ['thuoc_tinh', node.name.text, khac];
      } else if (TS.isBinaryExpression(node) && (node.operatorToken.kind === TS.SyntaxKind.AmpersandAmpersandToken
        || node.operatorToken.kind === TS.SyntaxKind.BarBarToken)) {
        k = ['boolop', node.operatorToken.kind, null];
      } else if (TS.isPrefixUnaryExpression(node) && node.operator === TS.SyntaxKind.ExclamationToken) {
        k = ['bo_not', null, null];
      }
      if (k) {
        const kStr = JSON.stringify(k);
        if (!y_da_thay.has(kStr)) { y_da_thay.add(kStr); y.push(k); }
      }
    }
    TS.forEachChild(node, gomY);
  })(goc);

  const ra = [];
  for (const [loai, khoa, thay] of y.slice(0, tran)) {
    let dinh = 0;
    function tham(node) {
      const con_moi = TS.visitEachChild(node, tham, undefined);
      const n = con_moi || node;
      if (!dong.has(line_cua(node))) return n;
      if (loai === 'phep' && TS.isBinaryExpression(n) && n.operatorToken.kind === khoa) {
        dinh++;
        return TS_F.updateBinaryExpression(n, n.left, TS_F.createToken(TOAN_TU_SO.get(khoa)), n.right);
      }
      if (loai === 'sosanh' && TS.isBinaryExpression(n) && n.operatorToken.kind === khoa) {
        dinh++;
        return TS_F.updateBinaryExpression(n, n.left, TS_F.createToken(TOAN_TU_SS.get(khoa)), n.right);
      }
      if (loai === 'hang' && TS.isNumericLiteral(n) && n.text === khoa) {
        dinh++;
        return TS_F.createNumericLiteral(String(Number(khoa) + 1));
      }
      if (loai === 'chuoi' && TS.isStringLiteral(n) && n.text === khoa) {
        dinh++;
        return TS_F.createStringLiteral(thay);
      }
      if (loai === 'ten' && TS.isIdentifier(n) && la_vi_tri_doc_ts(n) && n.text === khoa) {
        dinh++;
        return TS_F.createIdentifier(thay);
      }
      if (loai === 'thuoc_tinh' && TS.isPropertyAccessExpression(n) && n.name.text === khoa) {
        dinh++;
        return TS_F.updatePropertyAccessExpression(n, n.expression, TS_F.createIdentifier(thay));
      }
      if (loai === 'boolop' && TS.isBinaryExpression(n) && n.operatorToken.kind === khoa) {
        dinh++;
        const moi_kind = khoa === TS.SyntaxKind.AmpersandAmpersandToken
          ? TS.SyntaxKind.BarBarToken : TS.SyntaxKind.AmpersandAmpersandToken;
        return TS_F.updateBinaryExpression(n, n.left, TS_F.createToken(moi_kind), n.right);
      }
      if (loai === 'bo_not' && TS.isPrefixUnaryExpression(n) && n.operator === TS.SyntaxKind.ExclamationToken) {
        dinh++;
        return n.operand;
      }
      return n;
    }
    const cay_moi = tham(goc);
    if (!dinh) continue;
    let moi;
    try {
      moi = printer.printFile(cay_moi).trim();
    } catch {
      continue;
    }
    if (moi === goc_txt) continue;

    let mo_ta;
    if (loai === 'hang') mo_ta = `đổi MỌI hằng số ${khoa} thành ${Number(khoa) + 1} (${dinh} chỗ)`;
    else if (loai === 'phep') mo_ta = `đổi MỌI phép ${TEN_TOAN_TU.get(khoa)} thành ${TEN_TOAN_TU.get(TOAN_TU_SO.get(khoa))} (${dinh} chỗ)`;
    else if (loai === 'sosanh') mo_ta = `đổi MỌI dấu ${TEN_TOAN_TU.get(khoa)} thành ${TEN_TOAN_TU.get(TOAN_TU_SS.get(khoa))} (${dinh} chỗ)`;
    else if (loai === 'chuoi') mo_ta = `đổi MỌI chuỗi ${JSON.stringify(khoa)} thành ${JSON.stringify(thay)} (${dinh} chỗ)`;
    else if (loai === 'ten') mo_ta = `đổi MỌI chỗ đọc tên ${khoa} thành ${thay} (${dinh} chỗ)`;
    else if (loai === 'thuoc_tinh') mo_ta = `đổi MỌI .${khoa} thành .${thay} (${dinh} chỗ)`;
    else if (loai === 'boolop') {
      const [cu, moi_] = khoa === TS.SyntaxKind.AmpersandAmpersandToken ? ['&&', '||'] : ['||', '&&'];
      mo_ta = `đổi MỌI ${cu} thành ${moi_} (${dinh} chỗ)`;
    } else mo_ta = `BỎ mọi dấu ! (${dinh} chỗ)`;

    ra.push([mo_ta, moi]);
  }
  return ra;
}

/** Dòng nào của lời giải ứng với một dòng có `___` ở mã khởi đầu. */
function dong_nguoi_hoc_dien(starter, solution) {
  const a = starter.split('\n');
  const b = solution.split('\n');
  // Chỉ tin phép gióng theo chỉ số dòng khi hai bên cùng số dòng — lời giải
  // là mã khởi đầu đã điền, nên bình thường chúng bằng nhau. Lệch thì thà
  // không kiểm còn hơn kiểm nhầm dòng rồi báo một lỗ không có thật.
  if (a.length !== b.length) return null;
  const ra = [];
  for (let i = 0; i < a.length; i++) if (a[i].includes('___')) ra.push(i + 1);
  return ra.length ? ra : null;
}

const mien_tru = new Set();
if (existsSync(MIEN_TRU)) {
  for (const d of readFileSync(MIEN_TRU, 'utf-8').split('\n')) {
    // Bóc đúng MỘT lớp nháy ngoài cùng, không dùng lớp ký tự loại trừ nháy.
    //
    // Bản cũ dùng `[^"'#]+?`, nên một khoá có nháy LỒNG bên trong — mà khoá mô
    // tả đột biến chuỗi thì luôn có, kiểu `đổi MỌI chuỗi 'a.txt' thành 'w'` —
    // bị cắt cụt ở dấu nháy đầu tiên. Miễn trừ khai đúng mà không bao giờ khớp,
    // và cổng cứ báo đỏ mãi một chỗ đã được xét.
    const m = /^\s*-\s*(.*?)\s*(?:#.*)?$/.exec(d);
    if (m) {
      let s = m[1].trim();
      if (s.length >= 2 && (s[0] === '"' || s[0] === "'") && s[s.length - 1] === s[0]) {
        s = s.slice(1, -1);
      }
      if (s) mien_tru.add(s.trim());
    }
  }
}

const tep = readdirSync(THU_MUC).filter((f) => f.endsWith('.json') && f !== 'index.json');
const ho = [];
let da_thu = 0;
let da_kiem = 0;
// Bước nào cổng KHÔNG sinh nổi một đột biến nào. Phải đếm và phải in ra.
//
// Người viết T1.5 báo: "cổng đột biến sinh 0 đột biến trên cả ba bài, nên nó
// không chứng minh được gì ở đây". Họ đúng, và im lặng về chuyện đó là đúng
// cái hình dạng lỗi mà cả tập cổng này dựng lên để chặn — một con số xanh
// không đo thứ nó nói là nó đo. Một cổng không với tới được thì phải NÓI RA,
// chứ không được lặng lẽ tính mình là đã kiểm.
const khong_voi_toi = [];
// Bước bị bỏ qua vì số dòng `starter` khác `solution`, nên không gióng được
// chỗ trống. Cũng là vùng mù, cũng phải đếm.
let lech_dong = 0;
// Rust CHƯA có hạ tầng sinh đột biến — `byte-rust` không lộ AST ra ngoài
// WASM (chỉ có `chay()`), khác TypeScript (npm package `typescript` là AST
// đầy đủ, gọi thẳng được từ Node). Đếm và NÓI RA, đúng kỷ luật minh bạch
// phạm vi của cổng này — không được lặng lẽ coi Rust "đã kiểm".
let rust_bo_qua = 0;

let da_doc = 0;
for (const f of tep) {
  const bai = JSON.parse(readFileSync(join(THU_MUC, f), 'utf-8'));
  if (++da_doc % 20 === 0) process.stderr.write(`  … ${da_doc}/${tep.length} bài, ${da_thu} đột biến, ${ho.length} lỗ\n`);
  for (const b of bai.steps) {
    const c = b.code;
    if (!c?.solution) continue;
    const la_py = c.lang === 'python';
    const la_ts = c.lang === 'typescript';
    if (!la_py && !la_ts) {
      if (c.lang === 'rust' && c.starter?.includes('___')) rust_bo_qua++;
      continue;
    }
    if (!c.starter?.includes('___')) continue;
    // MỌI luật output — xem chú thích cùng chỗ ở `kiem_ma_bai_hoc.mjs`.
    const luat_out_ds = (b.validation?.rules ?? []).filter((r) => r.tier === 'output');
    const luat_out = luat_out_ds[0];
    // `tier: static` cho TypeScript bị `kiem_ma_bai_hoc.mjs` chặn cứng từ
    // trước (TsAstKind chưa cài) — nếu nội dung đã qua được cổng đó thì
    // KHÔNG bài TS nào còn luật static để soi ở đây. Chỉ Python có static.
    const luat_static = la_py ? (b.validation?.rules ?? []).filter((r) => r.tier === 'static') : [];
    if (!luat_out && !c.test) continue;

    const dong = dong_nguoi_hoc_dien(c.starter, c.solution);
    if (!dong) { lech_dong++; continue; }
    da_kiem++;

    let ds;
    if (la_py) {
      try { ds = JSON.parse(sinh(c.solution, dong)); } catch { ds = []; }
    } else {
      try { ds = sinh_dot_bien_ts(c.solution, dong); } catch { ds = []; }
    }
    if (ds.length === 0) khong_voi_toi.push(`${bai.id} · ${b.id}`);

    for (const [mo_ta, ma] of ds) {
      da_thu++;
      // Chấm y hệt cách bài chấm người học: static → chạy → test → output.
      let qua = true;
      for (const r of luat_static) {
        const kq = kiemAst(py, ma, r.requireAst ?? [], r.forbidAst ?? []);
        if (!kq.dat) { qua = false; break; }
      }
      if (qua) {
        // Chạy mã trần một lần: lấy được CẢ output lẫn "có nổ không". Rồi chỉ
        // khi cần mới chạy thêm khối test. Bản đầu chạy hai lượt cho cùng một
        // câu trả lời và cổng chậm tới mức không ai chạy nổi.
        //
        // CỐ Ý không gộp py/ts qua một hàm async dùng chung — route Python
        // (`chay`) giữ nguyên đồng bộ, không một chữ `await` nào chạm vào,
        // đúng kỷ luật đã đặt ra khi vá lỗi tương tự ở `kiem_ma_bai_hoc.mjs`.
        const r = la_py ? chay(ma) : await chay_ts(ma);
        qua = r.ok && luat_out_ds.every((lo) => khop(r.xuat, lo));
        if (qua && c.test) {
          qua = la_py ? chay(`${ma}\n${c.test}`).ok : (await chay_ts(ma, c.test)).ok;
        }
      }
      if (qua) {
        const khoa = `${bai.id} · ${b.id} · ${mo_ta}`;
        if (mien_tru.has(khoa)) continue;
        // Báo HẾT lỗ của một bước, không dừng ở cái đầu.
        //
        // Bản cũ `break` sau lỗ đầu tiên. Nghe thì gọn, nhưng nó biến vòng khai
        // miễn trừ thành trò đập chuột: tha cái thứ nhất thì cái thứ hai mới ló
        // ra, phải chạy lại cả cổng cho mỗi cái. Tệ hơn, một bước hổng ba chỗ
        // trông y hệt một bước hổng một chỗ.
        ho.push({ khoa, bai: bai.id, buoc: b.id, mo_ta });
      }
    }
  }
}

console.log(`Đã thử ${da_thu} đột biến trên ${da_kiem} bước \`code\` có chỗ trống, trong ${tep.length} bài.`);

// PHẠM VI, in ra trước phán quyết. Xanh trên một phạm vi hẹp vẫn là xanh hẹp.
if (khong_voi_toi.length || lech_dong || rust_bo_qua) {
  const n = khong_voi_toi.length;
  console.log(
    `\n⚠️  PHẠM VI: ${n}/${da_kiem} bước không sinh nổi đột biến nào` +
      (lech_dong ? `, ${lech_dong} bước bị bỏ qua vì số dòng starter khác solution` : '') +
      (rust_bo_qua ? `, ${rust_bo_qua} bước Rust CHƯA có hạ tầng sinh đột biến (byte-rust không lộ AST ra WASM)` : '') +
      `.\n   Cổng KHÔNG nói gì về mấy bước ấy — đừng đọc màu xanh dưới đây như thể nó có.`,
  );
  for (const k of khong_voi_toi.slice(0, 12)) console.log(`     · ${k}`);
  if (n > 12) console.log(`     · … và ${n - 12} bước nữa`);
}

if (ho.length === 0) {
  console.log(
    khong_voi_toi.length || lech_dong || rust_bo_qua
      ? `\n✅ Trong ${da_kiem - khong_voi_toi.length} bước cổng VỚI TỚI ĐƯỢC: không cách chấm nào cho lọt đáp án sai.`
      : '\n✅ Không cách chấm nào cho lọt đáp án sai.',
  );
  process.exit(0);
}
console.log(`❌ ${ho.length} bước có LỖ CHẤM ĐIỂM — sửa lời giải một chỗ mà vẫn qua:\n`);
for (const h of ho) {
  console.log(`  ✗ ${h.bai} · bước ${h.buoc}`);
  console.log(`     ${h.mo_ta} — vẫn qua sạch mọi tầng chấm`);
}
console.log(`\nSửa bằng cách siết khối test hoặc luật static, ĐỪNG nới lời giải.`);
console.log(`Đột biến nào thật sự vô hại thì khai vào \`${MIEN_TRU}\` kèm lý do.`);
process.exit(1);
