# `SimDisk` (bản Python) — song sinh ngữ nghĩa với `../src/index.ts`.
#
# Dùng cho các bài Python của Realm 6 (vd R6-1 q00 "Chiếc hộp giày của Byte",
# gắn nhãn `[PY, SimDisk]`). Bài học KHÔNG `import simdisk` — sandbox Pyodide
# chạy mỗi bài độc lập, không nạp được gói workspace ngoài. Mỗi bài tự chép
# phần class cần dùng vào khối `readonly`, đúng quy ước không-import-chéo đã
# dùng xuyên suốt dự án (vd `chia_co_du`/`dong_du` ở T2.6, `cong_vector` ở
# T2.7). File này là NGUỒN SỰ THẬT để đối chiếu ngữ nghĩa lúc viết bài.
#
# v1: read/write/fsync/crash cơ bản, không fault injection (thêm khi viết
# tới q02 "Khi điện mất" và q18 "Kẻ phá hoại có chủ đích").

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
        for sector, data in self._cache.items():
            self._platter[sector] = data
        self._cache.clear()

    def crash(self):
        """Mô phỏng mất điện: ghi CHƯA fsync biến mất, platter giữ nguyên."""
        self._cache.clear()

    def co_ghi_chua_fsync(self):
        """Còn ghi nào chưa fsync không."""
        return len(self._cache) > 0
