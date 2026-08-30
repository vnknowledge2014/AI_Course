import { loadPyodide } from 'pyodide';
import { kiemAst } from './dist/kiem-ast.js';
import fs from 'node:fs';

const py = await loadPyodide();
const CD = '/Volumes/SEAGATE/Personal_Projects/AI_Course/dist/content/';
const lessons = {
  L22: 'toan.logic-va-chung-minh.mo-dinh-nghia-ra',
  L23: 'toan.logic-va-chung-minh.xet-mot-so-bat-ky',
  L24: 'toan.logic-va-chung-minh.chung-minh-cau-phan-dao-thay',
};
const S = {};
for (const [k, id] of Object.entries(lessons)) {
  const d = JSON.parse(fs.readFileSync(CD + id + '.json', 'utf8'));
  const step = d.steps.find((s) => s.kind === 'code');
  S[k] = { code: step.code, rules: step.validation.rules };
}

function runCapture(src) {
  py.runPython(`
import io, sys
_buf = io.StringIO()
_old = sys.stdout
sys.stdout = _buf
_g = {}
`);
  let err = null;
  py.globals.set('_src', src);
  try {
    py.runPython('exec(_src, _g)');
  } catch (e) { err = String(e.message || e).trim().split('\n').slice(-3).join(' | '); }
  py.runPython('sys.stdout = _old');
  return { out: String(py.runPython('_buf.getvalue()')), err };
}

function grade(key, code) {
  const { rules } = S[key];
  const res = {};
  // run
  const r1 = runCapture(code);
  res.run = r1.err === null;
  res.runErr = r1.err;
  res.out = r1.out;
  // static
  const st = rules.find(r => r.tier === 'static');
  if (st) {
    const k = kiemAst(py, code, st.requireAst ?? [], st.forbidAst ?? []);
    res.static = k.dat;
    res.staticThieu = k.thieu.map(q => q.kind + ':' + q.target);
    res.staticCam = k.cam.map(q => q.kind + ':' + q.target);
    res.syn = k.loi_cu_phap;
  }
  // tests
  const r2 = runCapture(code + '\n' + S[key].code.test);
  res.tests = r2.err === null;
  res.testsErr = r2.err;
  // output
  const ot = rules.find(r => r.tier === 'output');
  if (ot) {
    const re = new RegExp(ot.expected);
    res.output = re.test(r1.out);
  }
  res.PASS = res.run && res.static !== false && res.tests && res.output !== false;
  return res;
}

globalThis.grade = grade;
globalThis.S = S;
export { grade, S, py };
