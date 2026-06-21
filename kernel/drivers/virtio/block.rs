const VIRTIO_BASE: usize = 0x10001000;
const VIRTIO_MAGIC: usize = 0x000;
const VIRTIO_VERSION: usize = 0x004;
const VIRTIO_DEVICE_ID: usize = 0x008;
const VIRTIO_STATUS: usize = 0x070;
const STATUS_ACK: u8 = 1;
const STATUS_DRIVER: u8 = 2;
const STATUS_DRIVER_OK: u8 = 4;

pub struct VirtioBlock;

impl VirtioBlock {
    pub fn new() -> Self {
        VirtioBlock
    }

    pub fn init(&self) {
        let status = (VIRTIO_BASE + VIRTIO_STATUS) as *mut u8;
        unsafe {
            status.write_volatile(STATUS_ACK);
            status.write_volatile(STATUS_ACK | STATUS_DRIVER);
            status.write_volatile(STATUS_ACK | STATUS_DRIVER | STATUS_DRIVER_OK);
        }
    }
}

pub fn virtio_scan() {
    let magic = (VIRTIO_BASE + VIRTIO_MAGIC) as *const u32;
    unsafe {
        if magic.read_volatile() == 0x74726976 {
            let dev_id = (VIRTIO_BASE + VIRTIO_DEVICE_ID) as *const u32;
            let id = dev_id.read_volatile();
            if id == 2 {
                let block = VirtioBlock::new();
                block.init();
            }
        }
    }
}
