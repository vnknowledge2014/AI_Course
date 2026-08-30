import { loadPyodide } from '/Volumes/SEAGATE/Personal_Projects/AI_Course/packages/exec-python/node_modules/pyodide/pyodide.mjs';
import { kiemAst } from '/Volumes/SEAGATE/Personal_Projects/AI_Course/packages/exec-python/dist/kiem-ast.js';

const py = await loadPyodide({ indexURL: '/Volumes/SEAGATE/Personal_Projects/AI_Course/packages/exec-python/node_modules/pyodide/' });
console.log('PYVER', py.runPython('import sys; sys.version'));

const REQ19 = [
  { lang: 'python', kind: 'uses-name', target: 'deo_the', min: 1 },
  { lang: 'python', kind: 'uses-name', target: 'da_xet', min: 3 },
  { lang: 'python', kind: 'uses-operator', target: '+', min: 1 },
];
const CAM19 = [
  { lang: 'python', kind: 'has-literal', target: 4 },
  { lang: 'python', kind: 'has-literal', target: 6 },
];

const head19 = `thanh_vien = ["Nam", "Lan", "Minh", "Hoa", "Tú", "Khanh"]
deo_the = {"Nam": True, "Lan": True, "Minh": True,
           "Hoa": False, "Tú": True, "Khanh": True}
to_truc_nhat = []

`;
const tail19 = `
ai, ton = san_phan_vi_du(thanh_vien)
ai_to, ton_to = san_phan_vi_du(to_truc_nhat)

print(ai, ton)
print(ai_to, ton_to)
`;

function body19(inc, cond) {
  return head19 + `def san_phan_vi_du(danh_sach):
    da_xet = 0
    for ten in danh_sach:
        ${inc}
        if ${cond}:
            return ten, da_xet
    return None, da_xet
` + tail19;
}

const TEST19 = `
assert san_phan_vi_du([])[0] is None
assert san_phan_vi_du([])[1] == 0
assert ai == "Hoa"
assert ton == 4
assert ai_to is None and ton_to == 0
assert san_phan_vi_du(["Hoa"]) == ("Hoa", 1)
assert san_phan_vi_du(["Nam", "Lan"]) == (None, 2)
assert san_phan_vi_du(["Lan", "Hoa", "Minh"]) == ("Hoa", 2)
print("TESTS OK")
`;

const cases = {
  'A canonical  da_xet = da_xet + 1 / not deo_the[ten]': body19('da_xet = da_xet + 1', 'not deo_the[ten]'),
  'B AUGASSIGN  da_xet += 1        / not deo_the[ten]': body19('da_xet += 1', 'not deo_the[ten]'),
  'C AUGASSIGN  da_xet += 1        / deo_the[ten] == False': body19('da_xet += 1', 'deo_the[ten] == False'),
  'D canonical  + 1                / deo_the[ten] == False': body19('da_xet = da_xet + 1', 'deo_the[ten] == False'),
  'E canonical  + 1                / deo_the[ten] is False': body19('da_xet = da_xet + 1', 'deo_the[ten] is False'),
  'F canonical  1 + da_xet         / not deo_the[ten]': body19('da_xet = 1 + da_xet', 'not deo_the[ten]'),
};

for (const [name, code] of Object.entries(cases)) {
  const r = kiemAst(py, code, REQ19, CAM19);
  // run + tests + output
  let runOut = '';
  let ok = 'RUN_OK';
  try {
    py.runPython(`
import sys, io
_buf = io.StringIO()
_old = sys.stdout
sys.stdout = _buf
`);
    py.globals.set('_code', code + TEST19);
    py.runPython('exec(compile(_code, "<sub>", "exec"), {"__name__": "__main__"})');
    py.runPython('sys.stdout = _old');
    runOut = String(py.runPython('_buf.getvalue()'));
  } catch (e) {
    try { py.runPython('import sys; sys.stdout = _old'); } catch {}
    ok = 'FAIL: ' + String(e).split('\n').slice(-4).join(' | ');
  }
  console.log('----', name);
  console.log('   static.dat =', r.dat, ' thieu =', JSON.stringify(r.thieu), ' cam =', JSON.stringify(r.cam), r.loi_cu_phap ?? '');
  console.log('   exec =', ok, ' out =', JSON.stringify(runOut));
}
