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
function chay(ma) {
  const dong = [];
  const loi_goc = console.error;
  console.error = () => {};
  try {
    py.setStdout({ batched: (s) => { if (dong.length < TRAN_DONG) dong.push(s); } });
    py.runPython('__x = dict(globals()); [globals().pop(k) for k in list(globals()) if k not in __x and not k.startswith("__")]');
    py.runPython(ma);
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

class _Doi(ast.NodeTransformer):
    """Sửa ĐÚNG MỘT nút, nút thứ \`chi_so\` trong số các nút hợp lệ."""
    def __init__(self, dong_duoc_sua, chi_so):
        self.dong = dong_duoc_sua
        self.muc_tieu = chi_so
        self.dem = 0
        self.mo_ta = None

    def _duoc(self, node):
        return getattr(node, 'lineno', None) in self.dong

    def _lay_luot(self, mo_ta):
        """True nếu đây đúng là nút thứ \`muc_tieu\`. Đếm mọi nút hợp lệ để
        chỉ số ổn định giữa các lần gọi."""
        n = self.dem
        self.dem += 1
        if n == self.muc_tieu:
            self.mo_ta = mo_ta
            return True
        return False

    def visit_BinOp(self, node):
        self.generic_visit(node)
        if not self._duoc(node):
            return node
        doi = {ast.Add: ast.Sub, ast.Sub: ast.Add, ast.Mult: ast.Add, ast.Div: ast.Mult}
        moi = doi.get(type(node.op))
        if moi and self._lay_luot(f'đổi phép {type(node.op).__name__} thành {moi.__name__}'):
            return ast.BinOp(left=node.left, op=moi(), right=node.right)
        return node

    def visit_Constant(self, node):
        if not self._duoc(node) or not isinstance(node.value, int) or isinstance(node.value, bool):
            return node
        if self._lay_luot(f'đổi hằng số {node.value} thành {node.value + 1}'):
            return ast.Constant(value=node.value + 1)
        return node

    def visit_Compare(self, node):
        self.generic_visit(node)
        if not self._duoc(node) or len(node.ops) != 1:
            return node
        doi = {ast.Eq: ast.NotEq, ast.NotEq: ast.Eq, ast.Lt: ast.LtE, ast.LtE: ast.Lt,
               ast.Gt: ast.GtE, ast.GtE: ast.Gt}
        moi = doi.get(type(node.ops[0]))
        if moi and self._lay_luot(f'đổi so sánh {type(node.ops[0]).__name__} thành {moi.__name__}'):
            return ast.Compare(left=node.left, ops=[moi()], comparators=node.comparators)
        return node

def sinh_dot_bien(ma, dong_duoc_sua, tran=5):
    """Trả về [(mô tả, mã đã sửa)] — mỗi cái sửa đúng một chỗ."""
    dong_duoc_sua = set(dong_duoc_sua)
    try:
        goc = ast.parse(ma)
    except SyntaxError:
        return json.dumps([])
    # Đếm trước xem có bao nhiêu nút sửa được.
    dem = _Doi(dong_duoc_sua, -1)
    dem.visit(ast.parse(ma))
    ra = []
    for i in range(min(dem.dem, tran)):
        t = _Doi(dong_duoc_sua, i)
        cay = t.visit(ast.parse(ma))
        if t.mo_ta is None:
            continue
        ast.fix_missing_locations(cay)
        try:
            moi = ast.unparse(cay)
        except Exception:
            continue
        if moi.strip() != ast.unparse(goc).strip():
            ra.append([t.mo_ta, moi])
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
    const luat_out = (b.validation?.rules ?? []).find((r) => r.tier === 'output');
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
        qua = r.ok && (luat_out ? khop(r.xuat, luat_out) : true);
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
