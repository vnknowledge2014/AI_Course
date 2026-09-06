import { test } from 'node:test';
import assert from 'node:assert/strict';
import {
  MCP_METHODS,
  MO_TA_MOI_DE_DOA,
  laJsonRpcError,
  laJsonRpcThanhCong,
  laMcpToolCallLoi,
  laA2aTaskKetThuc,
  agentCoKyNang,
} from '../dist/index.js';

test('laJsonRpcError / laJsonRpcThanhCong phân biệt đúng hai hình dạng response', () => {
  const ok = { jsonrpc: '2.0', id: 1, result: { xong: true } };
  const loi = { jsonrpc: '2.0', id: 1, error: { code: -32601, message: 'method not found' } };

  assert.equal(laJsonRpcThanhCong(ok), true);
  assert.equal(laJsonRpcError(ok), false);
  assert.equal(laJsonRpcThanhCong(loi), false);
  assert.equal(laJsonRpcError(loi), true);
});

test('laJsonRpcError chấp nhận id null (lỗi trước khi đọc được id gốc)', () => {
  const loiParse = { jsonrpc: '2.0', id: null, error: { code: -32700, message: 'parse error' } };
  assert.equal(laJsonRpcError(loiParse), true);
});

test('MCP_METHODS đúng bốn tên method chuẩn', () => {
  assert.equal(MCP_METHODS.initialize, 'initialize');
  assert.equal(MCP_METHODS.toolsList, 'tools/list');
  assert.equal(MCP_METHODS.toolsCall, 'tools/call');
  assert.equal(MCP_METHODS.notificationsInitialized, 'notifications/initialized');
});

test('laMcpToolCallLoi chỉ true khi isError === true tường minh', () => {
  const thanhCong = { content: [{ type: 'text', text: 'ok' }] };
  const loi = { content: [{ type: 'text', text: 'that bai' }], isError: true };
  assert.equal(laMcpToolCallLoi(thanhCong), false);
  assert.equal(laMcpToolCallLoi(loi), true);
});

test('laA2aTaskKetThuc đúng ba trạng thái kết thúc, sai với ba trạng thái còn lại', () => {
  assert.equal(laA2aTaskKetThuc('completed'), true);
  assert.equal(laA2aTaskKetThuc('canceled'), true);
  assert.equal(laA2aTaskKetThuc('failed'), true);
  assert.equal(laA2aTaskKetThuc('submitted'), false);
  assert.equal(laA2aTaskKetThuc('working'), false);
  assert.equal(laA2aTaskKetThuc('input-required'), false);
});

test('agentCoKyNang kiểm TRƯỚC khi gửi task, khớp id hoặc name', () => {
  const card = {
    name: 'agent-b',
    description: 'agent phu',
    url: 'https://vi-du.test/agent-b',
    capabilities: {},
    skills: [{ id: 'tinh-tong', name: 'Tinh tong hai so', description: 'cong hai so' }],
  };
  assert.equal(agentCoKyNang(card, 'tinh-tong'), true);
  assert.equal(agentCoKyNang(card, 'Tinh tong hai so'), true);
  assert.equal(agentCoKyNang(card, 'ky-nang-khong-ton-tai'), false);
});

test('MO_TA_MOI_DE_DOA có đúng ba mô tả, mỗi mô tả không rỗng', () => {
  const khoa = Object.keys(MO_TA_MOI_DE_DOA);
  assert.equal(khoa.length, 3);
  for (const k of khoa) {
    assert.ok(MO_TA_MOI_DE_DOA[k].length > 0, `mô tả của ${k} không được rỗng`);
  }
  assert.ok('tool_poisoning' in MO_TA_MOI_DE_DOA);
  assert.ok('confused_deputy' in MO_TA_MOI_DE_DOA);
  assert.ok('token_passthrough' in MO_TA_MOI_DE_DOA);
});
