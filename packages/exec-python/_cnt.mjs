import { py, S } from './_t2.mjs';
import { kiemAst } from './dist/kiem-ast.js';
const q = (t) => [{lang:'python',kind:'uses-operator',target:t,min:0}];
function count(code, target) {
  // dùng min rất lớn để đọc số thực tế? -> thay vào: đếm thủ công bằng python
  py.globals.set('_s', code);
  py.globals.set('_t', target);
  return py.runPython(`
import ast
_c = ast.parse(_s)
_m = {'*': ast.Mult, '%': ast.Mod, '+': ast.Add}
_n = 0
for _x in ast.walk(_c):
    if isinstance(_x, ast.BinOp) and isinstance(_x.op, _m[_t]):
        _n += 1
_n
`);
}
console.log('L23 solution  * =', count(S.L23.code.solution, '*'));
console.log('L23 khai trien * =', count(S.L23.code.solution.replace('return 2 * (p + q)', 'return 2 * p + 2 * q'), '*'));
console.log('L23 starter   * =', count(S.L23.code.starter.replace(/___/g, '0'), '*'));
console.log('L22 solution  * =', count(S.L22.code.solution, '*'), ' % =', count(S.L22.code.solution, '%'));
console.log('L24 solution  * =', count(S.L24.code.solution, '*'), ' % =', count(S.L24.code.solution, '%'));
console.log('L24 starter   % =', count(S.L24.code.starter.replace(/___/g, '0'), '%'));
process.exit(0);
