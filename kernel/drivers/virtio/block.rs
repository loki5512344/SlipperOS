use core::sync::atomic::{fence, Ordering};

const REG_MAGIC: usize = 0x000;
const REG_VERSION: usize = 0x004;
const REG_DEVICE_ID: usize = 0x008;
const REG_DEVICE_FEATURES: usize = 0x010;
const REG_DEVICE_FEATURES_SEL: usize = 0x014;
const REG_DRIVER_FEATURES: usize = 0x020;
const REG_DRIVER_FEATURES_SEL: usize = 0x024;
const REG_QUEUE_SEL: usize = 0x030;
const REG_QUEUE_NUM_MAX: usize = 0x034;
const REG_QUEUE_NUM: usize = 0x038;
const REG_QUEUE_READY: usize = 0x044;
const REG_QUEUE_NOTIFY: usize = 0x050;
const REG_STATUS: usize = 0x070;
const REG_QUEUE_DESC_LOW: usize = 0x080;
const REG_QUEUE_DESC_HIGH: usize = 0x084;
const REG_QUEUE_DRIVER_LOW: usize = 0x088;
const REG_QUEUE_DRIVER_HIGH: usize = 0x08C;
const REG_QUEUE_DEVICE_LOW: usize = 0x090;
const REG_QUEUE_DEVICE_HIGH: usize = 0x094;

const STATUS_ACK: u8 = 1;
const STATUS_DRIVER: u8 = 2;
const STATUS_FEATURES_OK: u8 = 8;
const STATUS_DRIVER_OK: u8 = 16;

const DESC_NEXT: u16 = 1;
const DESC_WRITE: u16 = 2;

const QUEUE_NUM: usize = 16;

#[derive(Clone, Copy)]
#[repr(C)]
struct Descriptor {
    addr: u64,
    len: u32,
    flags: u16,
    next: u16,
}

#[repr(C, align(16))]
struct DescTable([Descriptor; QUEUE_NUM]);

#[repr(C, align(2))]
struct AvailRing {
    flags: u16,
    idx: u16,
    ring: [u16; QUEUE_NUM],
}

#[derive(Clone, Copy)]
#[repr(C)]
struct UsedElem {
    id: u32,
    len: u32,
}

#[repr(C, align(4))]
struct UsedRing {
    flags: u16,
    idx: u16,
    ring: [UsedElem; QUEUE_NUM],
}

#[repr(C)]
struct BlockHeader {
    type_: u32,
    reserved: u32,
    sector: u64,
}

const DESC_INIT: Descriptor = Descriptor { addr: 0, len: 0, flags: 0, next: 0 };
const USED_ELEM_INIT: UsedElem = UsedElem { id: 0, len: 0 };

static mut DESC: DescTable = DescTable([DESC_INIT; QUEUE_NUM]);
static mut AVAIL: AvailRing = AvailRing { flags: 0, idx: 0, ring: [0; QUEUE_NUM] };
static mut USED: UsedRing = UsedRing { flags: 0, idx: 0, ring: [USED_ELEM_INIT; QUEUE_NUM] };
static mut LAST_USED_IDX: u16 = 0;
static mut HEADER: BlockHeader = BlockHeader { type_: 0, reserved: 0, sector: 0 };
static mut STATUS: u8 = 0;

pub struct VirtioBlock {
    base: usize,
}

impl VirtioBlock {
    pub fn new(base: usize, _irq: u32) -> Self {
        VirtioBlock { base }
    }

    fn read_reg(&self, reg: usize) -> u32 {
        unsafe { ((self.base + reg) as *const u32).read_volatile() }
    }

    fn write_reg(&self, reg: usize, val: u32) {
        unsafe { ((self.base + reg) as *mut u32).write_volatile(val) }
    }

    fn write_status(&self, val: u8) {
        unsafe { ((self.base + REG_STATUS) as *mut u8).write_volatile(val) }
    }

    fn read_status(&self) -> u8 {
        unsafe { ((self.base + REG_STATUS) as *const u8).read_volatile() }
    }

    pub fn init(&self) -> bool {
        if self.base == 0 {
            return false;
        }

        let magic = self.read_reg(REG_MAGIC);
        if magic != 0x74726976 {
            return false;
        }

        let version = self.read_reg(REG_VERSION);
        if version != 2 {
            return false;
        }

        let dev_id = self.read_reg(REG_DEVICE_ID);
        if dev_id != 2 {
            return false;
        }

        self.write_status(0);
        self.write_status(STATUS_ACK);
        self.write_status(STATUS_ACK | STATUS_DRIVER);

        self.write_reg(REG_DEVICE_FEATURES_SEL, 1);
        let dev_features1 = self.read_reg(REG_DEVICE_FEATURES);
        if dev_features1 & 1 == 0 {
            return false;
        }

        self.write_reg(REG_DRIVER_FEATURES_SEL, 1);
        self.write_reg(REG_DRIVER_FEATURES, 1);
        self.write_reg(REG_DRIVER_FEATURES_SEL, 0);
        self.write_reg(REG_DRIVER_FEATURES, 0);

        self.write_status(STATUS_ACK | STATUS_DRIVER | STATUS_FEATURES_OK);

        let st = self.read_status();
        if st & STATUS_FEATURES_OK == 0 {
            return false;
        }

        self.write_reg(REG_QUEUE_SEL, 0);
        let num_max = self.read_reg(REG_QUEUE_NUM_MAX);
        let num = if num_max > QUEUE_NUM as u32 { QUEUE_NUM as u32 } else { num_max };
        if num < 3 {
            return false;
        }
        self.write_reg(REG_QUEUE_NUM, num);

        let desc_addr = core::ptr::addr_of!(DESC) as u64;
        self.write_reg(REG_QUEUE_DESC_LOW, desc_addr as u32);
        self.write_reg(REG_QUEUE_DESC_HIGH, (desc_addr >> 32) as u32);

        let avail_addr = core::ptr::addr_of!(AVAIL) as u64;
        self.write_reg(REG_QUEUE_DRIVER_LOW, avail_addr as u32);
        self.write_reg(REG_QUEUE_DRIVER_HIGH, (avail_addr >> 32) as u32);

        let used_addr = core::ptr::addr_of!(USED) as u64;
        self.write_reg(REG_QUEUE_DEVICE_LOW, used_addr as u32);
        self.write_reg(REG_QUEUE_DEVICE_HIGH, (used_addr >> 32) as u32);

        self.write_reg(REG_QUEUE_READY, 1);
        if self.read_reg(REG_QUEUE_READY) == 0 {
            return false;
        }

        self.write_status(STATUS_ACK | STATUS_DRIVER | STATUS_FEATURES_OK | STATUS_DRIVER_OK);

        if self.read_status() & STATUS_DRIVER_OK == 0 {
            return false;
        }

        unsafe {
            LAST_USED_IDX = 0;
        }

        true
    }

    pub fn read_sector(&self, lba: u64, buf: &mut [u8]) -> bool {
        if buf.len() < 512 {
            return false;
        }

        unsafe {
            HEADER = BlockHeader {
                type_: 0,
                reserved: 0,
                sector: lba,
            };
            STATUS = 0xFF;

            let desc = core::ptr::addr_of_mut!(DESC.0) as *mut Descriptor;

            (*desc.add(0)).addr = core::ptr::addr_of!(HEADER) as u64;
            (*desc.add(0)).len = 16;
            (*desc.add(0)).flags = DESC_NEXT;
            (*desc.add(0)).next = 1;

            (*desc.add(1)).addr = buf.as_mut_ptr() as u64;
            (*desc.add(1)).len = 512;
            (*desc.add(1)).flags = DESC_NEXT | DESC_WRITE;
            (*desc.add(1)).next = 2;

            (*desc.add(2)).addr = core::ptr::addr_of!(STATUS) as u64;
            (*desc.add(2)).len = 1;
            (*desc.add(2)).flags = DESC_WRITE;
            (*desc.add(2)).next = 0;

            fence(Ordering::SeqCst);

            let hd = core::ptr::addr_of_mut!(AVAIL);
            let idx = (*hd).idx;
            (*hd).ring[(idx as usize) % QUEUE_NUM] = 0;
            (*hd).idx = idx.wrapping_add(1);

            fence(Ordering::SeqCst);

            self.write_reg(REG_QUEUE_NOTIFY, 0);

            loop {
                fence(Ordering::SeqCst);
                let used = core::ptr::addr_of!(USED);
                if (*used).idx != LAST_USED_IDX {
                    break;
                }
            }

            fence(Ordering::SeqCst);

            LAST_USED_IDX = LAST_USED_IDX.wrapping_add(1);
        }

        unsafe { STATUS == 0 }
    }
}
