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
import { join } from 'node:path';
import { createRequire } from 'node:module';
import { pathToFileURL } from 'node:url';

const THU_MUC = process.argv[2] ?? 'dist/content';
const MIEN_TRU = 'content/curriculum/dot-bien-bo-qua.yaml';

const { kiemAst } = await import(new URL('../packages/exec-python/dist/kiem-ast.js', import.meta.url).href);
const require = createRequire(new URL('../packages/exec-python/package.json', import.meta.url));
const { loadPyodide } = await import(pathToFileURL(require.resolve('pyodide/pyodide.mjs')).href);
const py = await loadPyodide();

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
    def __init__(self, dong_duoc_sua, loai, khoa):
        self.dong = dong_duoc_sua
        self.loai = loai
        self.khoa = khoa
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
        if self.loai != 'hang' or not self._duoc(node):
            return node
        if not isinstance(node.value, int) or isinstance(node.value, bool):
            return node
        if node.value != self.khoa:
            return node
        self.dinh += 1
        return ast.Constant(value=node.value + 1)

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

_PHEP = {'Add': ast.Sub, 'Sub': ast.Add, 'Mult': ast.Add, 'Div': ast.Mult}
_SS = {'Eq': ast.NotEq, 'NotEq': ast.Eq, 'Lt': ast.LtE, 'LtE': ast.Lt,
       'Gt': ast.GtE, 'GtE': ast.Gt}
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
    y = []
    for nut in ast.walk(goc):
        if getattr(nut, 'lineno', None) not in dong_duoc_sua:
            continue
        if isinstance(nut, ast.BinOp) and type(nut.op).__name__ in _PHEP:
            k = ('phep', type(nut.op).__name__)
        elif isinstance(nut, ast.Compare) and len(nut.ops) == 1 and type(nut.ops[0]).__name__ in _SS:
            k = ('sosanh', type(nut.ops[0]).__name__)
        elif isinstance(nut, ast.Constant) and isinstance(nut.value, int) and not isinstance(nut.value, bool):
            k = ('hang', nut.value)
        else:
            continue
        if k not in y:
            y.append(k)

    ra = []
    for loai, khoa in y[:tran]:
        t = _Doi(dong_duoc_sua, loai, khoa)
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
        else:
            mo_ta = f'đổi MỌI dấu {_TEN[khoa]} thành {_TEN[_SS[khoa].__name__]} ({t.dinh} chỗ)'
        ra.append([mo_ta, moi])
    return json.dumps(ra, ensure_ascii=False)
`);
const sinh = py.globals.get('sinh_dot_bien');

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
    const m = /^\s*-\s*(?:khoa:\s*)?["']?([^"'#]+?)["']?\s*(?:#.*)?$/.exec(d);
    if (m) mien_tru.add(m[1].trim());
  }
}

const tep = readdirSync(THU_MUC).filter((f) => f.endsWith('.json') && f !== 'index.json');
const ho = [];
let da_thu = 0;
let da_kiem = 0;

let da_doc = 0;
for (const f of tep) {
  const bai = JSON.parse(readFileSync(join(THU_MUC, f), 'utf-8'));
  if (++da_doc % 20 === 0) process.stderr.write(`  … ${da_doc}/${tep.length} bài, ${da_thu} đột biến, ${ho.length} lỗ\n`);
  for (const b of bai.steps) {
    const c = b.code;
    if (!c?.solution || c.lang !== 'python') continue;
    if (!c.starter?.includes('___')) continue;
    // MỌI luật output — xem chú thích cùng chỗ ở `kiem_ma_bai_hoc.mjs`.
    const luat_out_ds = (b.validation?.rules ?? []).filter((r) => r.tier === 'output');
    const luat_out = luat_out_ds[0];
    const luat_static = (b.validation?.rules ?? []).filter((r) => r.tier === 'static');
    if (!luat_out && !c.test) continue;

    const dong = dong_nguoi_hoc_dien(c.starter, c.solution);
    if (!dong) continue;
    da_kiem++;

    let ds;
    try { ds = JSON.parse(sinh(c.solution, dong)); } catch { continue; }

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
        const r = chay(ma);
        qua = r.ok && luat_out_ds.every((lo) => khop(r.xuat, lo));
        if (qua && c.test) qua = chay(`${ma}\n${c.test}`).ok;
      }
      if (qua) {
        const khoa = `${bai.id} · ${b.id} · ${mo_ta}`;
        if (mien_tru.has(khoa)) continue;
        ho.push({ khoa, bai: bai.id, buoc: b.id, mo_ta });
        break; // một lỗ mỗi bước là đủ để phải sửa; đừng dội chín bản sao.
      }
    }
  }
}

console.log(`Đã thử ${da_thu} đột biến trên ${da_kiem} bước \`code\` có chỗ trống, trong ${tep.length} bài.`);
if (ho.length === 0) {
  console.log('✅ Không cách chấm nào cho lọt đáp án sai.');
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
