import { grade, S } from './_t2.mjs';

const cands = {
  'SOLUTION (nguyên bản)': S.L23.code.solution,

  'A1 — vế TRÁI 2p+2q (khai triển, KHÔNG gom)': `so_chan = [0, 2, 4, 6, 8, 10, 12]
so_le = [1, 3, 5, 7, 9, 11, 13]

def gop_hai_ve(p, q):
    return 2 * p + 2 * q

def cap_pha_cau(a, b):
    return (a + b) % 2 != 0

def san(danh_sach_a, danh_sach_b):
    for a in danh_sach_a:
        for b in danh_sach_b:
            if cap_pha_cau(a, b):
                return (a, b)
    return None

print(gop_hai_ve(3, 5))
print(san(so_chan, so_chan))
print(san(so_le, so_chan))`,

  'A2 — p+p+q+q (không nhân 2 vào tổng)': `so_chan = [0, 2, 4, 6, 8, 10, 12]
so_le = [1, 3, 5, 7, 9, 11, 13]

def gop_hai_ve(p, q):
    return p + p + q + q

def cap_pha_cau(a, b):
    return (a + b) % 2 != 0

def san(danh_sach_a, danh_sach_b):
    for a in danh_sach_a:
        for b in danh_sach_b:
            if cap_pha_cau(a, b):
                return (a, b)
    return None

print(gop_hai_ve(3, 5))
print(san(so_chan, so_chan))
print(san(so_le, so_chan))`,

  'A3 — cap_pha_cau dùng == 1 (đúng, cách khác)': `so_chan = [0, 2, 4, 6, 8, 10, 12]
so_le = [1, 3, 5, 7, 9, 11, 13]

def gop_hai_ve(p, q):
    return 2 * (p + q)

def cap_pha_cau(a, b):
    return (a + b) % 2 == 1

def san(danh_sach_a, danh_sach_b):
    for a in danh_sach_a:
        for b in danh_sach_b:
            if cap_pha_cau(a, b):
                return (a, b)
    return None

print(gop_hai_ve(3, 5))
print(san(so_chan, so_chan))
print(san(so_le, so_chan))`,

  'A4 — (q+p)*2 (đúng, đổi chỗ)': `so_chan = [0, 2, 4, 6, 8, 10, 12]
so_le = [1, 3, 5, 7, 9, 11, 13]

def gop_hai_ve(p, q):
    return (q + p) * 2

def cap_pha_cau(a, b):
    return (a + b) % 2 != 0

def san(danh_sach_a, danh_sach_b):
    for a in danh_sach_a:
        for b in danh_sach_b:
            if cap_pha_cau(a, b):
                return (a, b)
    return None

print(gop_hai_ve(3, 5))
print(san(so_chan, so_chan))
print(san(so_le, so_chan))`,
};

for (const [name, code] of Object.entries(cands)) {
  const r = grade('L23', code);
  console.log(name, '=>', JSON.stringify({PASS:r.PASS, run:r.run, static:r.static, thieu:r.staticThieu, cam:r.staticCam, tests:r.tests, testsErr:r.testsErr, output:r.output, out:r.out.replace(/\n/g,'|')}));
}
process.exit(0);
