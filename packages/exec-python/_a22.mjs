import { grade, S } from './_t2.mjs';
const base = (soban, chan, le) => `def so_ban(n):
${soban}

def so_nguoi_chan(k):
    return ${chan}

def so_nguoi_le(k):
    return ${le}

print(so_ban(6), so_ban(7))
print(so_nguoi_chan(3), so_nguoi_le(3))`;

const cands = {
  'SOLUTION': S.L22.code.solution,
  'B1 — so_nguoi_le = so_nguoi_chan(k) + 1 (ĐÚNG, cách khác)':
    base('    if n % 2 == 0:\n        return n // 2\n    return None', '2 * k', 'so_nguoi_chan(k) + 1'),
  'B2 — so_nguoi_le = k + k + 1 (ĐÚNG, cách khác)':
    base('    if n % 2 == 0:\n        return n // 2\n    return None', '2 * k', 'k + k + 1'),
  'B3 — so_nguoi_chan = k + k, le = k + k + 1 (ĐÚNG)':
    base('    if n % 2 == 0:\n        return n // 2\n    return None', 'k + k', 'k + k + 1'),
  'B4 — so_ban dùng divmod (ĐÚNG, không %)':
    base('    if n - 2 * (n // 2) == 0:\n        return n // 2\n    return None', '2 * k', '2 * k + 1'),
  'B5 — so_ban trả n/2 float (SAI kiểu)':
    base('    if n % 2 == 0:\n        return n / 2\n    return None', '2 * k', '2 * k + 1'),
  'B6 — so_ban điều kiện n % 2 != 1 (ĐÚNG với n>=0)':
    base('    if n % 2 != 1:\n        return n // 2\n    return None', '2 * k', '2 * k + 1'),
};
for (const [name, code] of Object.entries(cands)) {
  const r = grade('L22', code);
  console.log(name, '=>', JSON.stringify({PASS:r.PASS, run:r.run, static:r.static, thieu:r.staticThieu, cam:r.staticCam, tests:r.tests, testsErr:r.testsErr, output:r.output, out:r.out.replace(/\n/g,'|')}));
}
process.exit(0);
