// Thân worker cho Node — dùng trong test.
import { parentPort } from 'node:worker_threads';
import { transform } from 'sucrase';
import { chayTrongWorker } from '../dist/index.js';

parentPort.on('message', (yeuCau) => {
  const phanHoi = chayTrongWorker(yeuCau, (ma) =>
    transform(ma, { transforms: ['typescript'] }).code,
  );
  parentPort.postMessage(phanHoi);
});
parentPort.postMessage({ loai: 'san_sang' });
