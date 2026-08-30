import { loadPyodide } from '/Volumes/SEAGATE/Personal_Projects/AI_Course/packages/exec-python/node_modules/pyodide/pyodide.mjs';
import { kiemAst } from '/Volumes/SEAGATE/Personal_Projects/AI_Course/packages/exec-python/dist/kiem-ast.js';

const py = await loadPyodide({ indexURL: '/Volumes/SEAGATE/Personal_Projects/AI_Course/packages/exec-python/node_modules/pyodide/' });

function run(code) {
  py.runPython(`
import sys, io
_buf = io.StringIO()
_old = sys.stdout
sys.stdout = _buf
`);
  let out = '', err = null;
  try {
    py.globals.set('_code', code);
    py.runPython('exec(compile(_code, "<sub>", "exec"), {"__name__": "__main__"})');
  } catch (e) {
    err = String(e).split('\n').filter(Boolean).slice(-3).join(' | ');
  }
  try {
    py.runPython('sys.stdout = _old');
    out = String(py.runPython('_buf.getvalue()'));
  } catch {}
  return { out, err };
}

console.log('=== PREDICT L19');
console.log(JSON.stringify(run(`
thanh_vien = ["Nam", "Lan", "Minh", "Hoa", "Tú", "Khanh"]
deo_the = {"Nam": True, "Lan": True, "Minh": True,
           "Hoa": False, "Tú": True, "Khanh": True}
to_truc_nhat = []

def bang_dung_sai(danh_sach):
    ra = []
    for ten in danh_sach:
        ra.append(deo_the[ten])
    return ra

print(all(bang_dung_sai(thanh_vien)))
print(all(bang_dung_sai(to_truc_nhat)))
print(any(bang_dung_sai(to_truc_nhat)))
`)));

console.log('=== PREDICT L20');
console.log(JSON.stringify(run(`
thanh_vien = ["Nam", "Lan", "Minh", "Hoa", "Tú", "Khanh"]
da_nop_quy = {"Nam": True, "Lan": True, "Minh": True,
              "Hoa": True, "Tú": True, "Khanh": True}

da_nop = []
chua_nop = []
for ten in thanh_vien:
    da_nop.append(da_nop_quy[ten])
    chua_nop.append(not da_nop_quy[ten])

print(any(chua_nop))
print(not any(chua_nop))
print(all(da_nop))
`)));

console.log('=== PREDICT L21');
console.log(JSON.stringify(run(`
def la_nguyen_to(n):
    if n < 2:
        return False
    d = 2
    while d * d <= n:
        if n % d == 0:
            return False
        d = d + 1
    return True

def cong_thuc(n):
    return n * n + n + 41

bang = []
for n in range(40):
    bang.append(la_nguyen_to(cong_thuc(n)))

print(len(bang))
print(all(bang))
print(la_nguyen_to(cong_thuc(40)))
print("first counterexample:", [n for n in range(200) if not la_nguyen_to(cong_thuc(n))][:6])
print("n=39 ->", cong_thuc(39), la_nguyen_to(cong_thuc(39)))
print("n=41 ->", cong_thuc(41), la_nguyen_to(cong_thuc(41)))
print("table:", [(n, cong_thuc(n), la_nguyen_to(cong_thuc(n))) for n in range(6)])
`)));
