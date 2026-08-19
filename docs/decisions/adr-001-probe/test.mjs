import { readFileSync } from 'node:fs';
const bytes = readFileSync(new URL('./target/wasm32-unknown-unknown/release/wasm_probe.wasm', import.meta.url));
const { instance } = await WebAssembly.instantiate(bytes, {});
const { memory, input_ptr, eval: ev } = instance.exports;

function run(src) {
  const enc = new TextEncoder().encode(src);
  const ptr = input_ptr();
  new Uint8Array(memory.buffer, ptr, enc.length).set(enc);
  const r = ev(enc.length);
  return r === -9223372036854775808n ? 'LỖI' : r;
}

const cases = [
  ['1 + 2 * 3', 7n],
  ['(1 + 2) * 3', 9n],
  ['100 / 7', 14n],
  ['-5 + 10', 5n],
  ['2 * (3 + 4) - 5', 9n],
  ['1 / 0', 'LỖI'],
  ['1 + @', 'LỖI'],
];
let pass = 0;
for (const [src, want] of cases) {
  const got = run(src);
  const ok = got === want;
  if (ok) pass++;
  console.log(`  ${ok ? '✅' : '❌'} ${src.padEnd(16)} => ${got}${ok ? '' : `  (mong đợi ${want})`}`);
}
console.log(`\n  ${pass}/${cases.length} pass`);
console.log(`  memory ban đầu: ${(memory.buffer.byteLength/1024).toFixed(0)} KB`);
