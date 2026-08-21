/** Kiểu dùng lại trong các thành phần giao diện.
 *
 *  Gom vào một chỗ để component không phải import chéo từ hai gói khác nhau,
 *  và để đổi nguồn kiểu về sau chỉ phải sửa một file.
 */
export type { CodeStep, PredictStep, ReflectStep, Step, Lesson, RichText, ByteMood } from '@byte/content-schema';
export type { KetQuaChay, ChanDoan } from '@byte/exec-core';
export type { SuKien, CauHinhLuoi } from '@byte/exec-python';
