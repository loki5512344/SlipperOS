const PLIC_BASE: usize = 0x0C000000;
const PLIC_PRIORITY: usize = 0x000000;
const PLIC_ENABLE: usize = 0x002000;
const PLIC_THRESHOLD: usize = 0x200000;
const PLIC_CLAIM: usize = 0x200004;

const UART_IRQ: u32 = 10;
const VIRTIO_IRQ_BASE: u32 = 1;

pub fn plic_init() {
    let priority = PLIC_BASE as *mut u32;
    unsafe {
        priority.add(UART_IRQ as usize).write_volatile(1);
        priority.add(VIRTIO_IRQ_BASE as usize).write_volatile(1);
    }

    let enable = (PLIC_BASE + PLIC_ENABLE) as *mut u32;
    unsafe {
        enable.write_volatile((1 << UART_IRQ) | (1 << VIRTIO_IRQ_BASE));
    }

    let threshold = (PLIC_BASE + PLIC_THRESHOLD) as *mut u32;
    unsafe {
        threshold.write_volatile(0);
    }
}

pub fn plic_claim() -> u32 {
    let claim = (PLIC_BASE + PLIC_CLAIM) as *const u32;
    unsafe { claim.read_volatile() }
}

pub fn plic_complete(irq: u32) {
    let claim = (PLIC_BASE + PLIC_CLAIM) as *mut u32;
    unsafe {
        claim.write_volatile(irq);
    }
}
