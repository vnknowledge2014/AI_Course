# `SimDisk` (bản Python) — song sinh ngữ nghĩa với `../src/index.ts`.
#
# Dùng cho các bài Python của Realm 6 (vd R6-1 q00 "Chiếc hộp giày của Byte",
# gắn nhãn `[PY, SimDisk]`). Bài học KHÔNG `import simdisk` — sandbox Pyodide
# chạy mỗi bài độc lập, không nạp được gói workspace ngoài. Mỗi bài tự chép
# phần class cần dùng vào khối `readonly`, đúng quy ước không-import-chéo đã
# dùng xuyên suốt dự án (vd `chia_co_du`/`dong_du` ở T2.6, `cong_vector` ở
# T2.7). File này là NGUỒN SỰ THẬT để đối chiếu ngữ nghĩa lúc viết bài.
#
# v1 (q00): read/write/fsync/crash cơ bản.
#
# v2 (q02 "Khi điện mất"): hai kiểu lỗi, CHỦ ĐỘNG kích hoạt (không random —
# bài học cần tái lập được y hệt mỗi lần chạy):
# - bo_qua_fsync_ke_tiep() — "lost fsync": fsync() TIẾP theo là no-op hoàn
#   toàn, không báo lỗi. read() vẫn đúng (cache còn nguyên); crash() mới lộ.
# - danh_dau_torn_ghi(sector, so_byte_thanh_cong) — "torn write": fsync()
#   TIẾP theo chỉ ghi thành công so_byte_thanh_cong byte ĐẦU của sector đó,
#   phần còn lại giữ dữ liệu CŨ (hoặc số 0 nếu sector chưa từng ghi).
#
# latent sector error / misdirected write / crash tất định thứ N (q18) CHƯA
# xây — thêm khi viết tới quest đó.

KICH_THUOC_SECTOR_MAC_DINH = 512
SO_LUONG_SECTOR_MAC_DINH = 16


class SimDisk:
    """Đĩa mô phỏng: so_luong_sector sector, mỗi sector kich_thuoc_sector byte.

    Hai tầng, đúng cách một đĩa thật hoạt động:
    - platter — lưu trữ BỀN, chỉ cập nhật khi fsync(), sống sót qua crash().
    - cache — ghi CHƯA fsync(); read() vẫn thấy được, nhưng crash() xoá sạch.
    """

    def __init__(self, so_luong_sector=SO_LUONG_SECTOR_MAC_DINH,
                 kich_thuoc_sector=KICH_THUOC_SECTOR_MAC_DINH):
        self.so_luong_sector = so_luong_sector
        self.kich_thuoc_sector = kich_thuoc_sector
        self._platter = {}
        self._cache = {}
        self._mat_fsync_ke_tiep = False
        self._torn_sector = {}

    def _kiem_tra_sector(self, sector):
        if not isinstance(sector, int) or sector < 0 or sector >= self.so_luong_sector:
            raise IndexError(
                f"sector {sector} ngoai pham vi [0, {self.so_luong_sector})"
            )

    def read(self, sector):
        """Đọc một sector. Sector chưa từng ghi trả về toàn số 0."""
        self._kiem_tra_sector(sector)
        du_lieu = self._cache.get(sector, self._platter.get(sector))
        if du_lieu is None:
            return bytes(self.kich_thuoc_sector)
        return bytes(du_lieu)

    def write(self, sector, data):
        """Ghi một sector VÀO CACHE — chưa bền cho tới khi fsync()."""
        self._kiem_tra_sector(sector)
        if len(data) != self.kich_thuoc_sector:
            raise ValueError(
                f"write() can dung {self.kich_thuoc_sector} byte, nhan {len(data)}"
            )
        self._cache[sector] = bytes(data)

    def fsync(self):
        """Đẩy TOÀN BỘ cache xuống platter — sau lệnh này, mọi ghi đã bền."""
        if self._mat_fsync_ke_tiep:
            self._mat_fsync_ke_tiep = False
            return  # "noi doi": khong day gi ca, cache van con nguyen -- crash() se lo.
        for sector, data in self._cache.items():
            so_byte_thanh_cong = self._torn_sector.pop(sector, None)
            if so_byte_thanh_cong is None:
                self._platter[sector] = data
                continue
            cu = self._platter.get(sector, bytes(self.kich_thuoc_sector))
            ghi_duoc = bytearray(self.kich_thuoc_sector)
            ghi_duoc[0:so_byte_thanh_cong] = data[0:so_byte_thanh_cong]
            ghi_duoc[so_byte_thanh_cong:] = cu[so_byte_thanh_cong:]
            self._platter[sector] = bytes(ghi_duoc)
        self._cache.clear()

    def crash(self):
        """Mô phỏng mất điện: ghi CHƯA fsync biến mất, platter giữ nguyên."""
        self._cache.clear()

    def co_ghi_chua_fsync(self):
        """Còn ghi nào chưa fsync không."""
        return len(self._cache) > 0

    def bo_qua_fsync_ke_tiep(self):
        """Lần fsync() TIẾP theo là no-op hoàn toàn -- mô phỏng "lost fsync"."""
        self._mat_fsync_ke_tiep = True

    def danh_dau_torn_ghi(self, sector, so_byte_thanh_cong):
        """Lần fsync() TIẾP theo đụng sector này chỉ ghi thành công
        so_byte_thanh_cong byte đầu, phần còn lại giữ dữ liệu CŨ."""
        self._kiem_tra_sector(sector)
        if not isinstance(so_byte_thanh_cong, int) or not (0 <= so_byte_thanh_cong <= self.kich_thuoc_sector):
            raise ValueError(
                f"so_byte_thanh_cong {so_byte_thanh_cong} phai trong [0, {self.kich_thuoc_sector}]"
            )
        self._torn_sector[sector] = so_byte_thanh_cong
