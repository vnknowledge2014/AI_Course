import { loadPyodide } from '/Volumes/SEAGATE/Personal_Projects/AI_Course/packages/exec-python/node_modules/pyodide/pyodide.mjs';
const py = await loadPyodide({ indexURL: '/Volumes/SEAGATE/Personal_Projects/AI_Course/packages/exec-python/node_modules/pyodide/' });
py.runPython(`
import sys
print(sys.version)
from fractions import Fraction
print("49/25 :", float(Fraction(49,25)), "< 2 ?", Fraction(49,25) < 2)
print("9801/4900 :", float(Fraction(9801,4900)), "< 2 ?", Fraction(9801,4900) < 2, "> 2 ?", Fraction(9801,4900) > 2)
print("9801/4900 - 2 =", Fraction(9801,4900) - 2)
print("49/25 - 2 =", Fraction(49,25) - 2)
print("sum so_quy:", sum([40000,25000,60000,15000,30000,45000]))
`);
