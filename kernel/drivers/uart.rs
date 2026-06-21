use core::fmt::{self, Write};

const UART_BASE: usize = 0x10000000;
const UART_THR: usize = 0;
const UART_RBR: usize = 0;
const UART_LSR: usize = 5;
const UART_LSR_THRE: u8 = 1 << 5;
const UART_LSR_DR: u8 = 1 << 0;

pub struct Uart;

impl Uart {
    pub fn new() -> Self {
        Uart
    }

    pub fn init(&self) {
        let base = UART_BASE as *mut u8;
        unsafe {
            base.add(3).write_volatile(0x03); // 8N1
            base.add(2).write_volatile(0x07); // FIFO enable
            base.add(1).write_volatile(0x01); // enable interrupts
        }
    }

    pub fn putchar(&self, c: u8) {
        let base = UART_BASE as *mut u8;
        unsafe {
            while (base.add(UART_LSR).read_volatile() & UART_LSR_THRE) == 0 {}
            base.add(UART_THR).write_volatile(c);
        }
    }

    pub fn getchar(&self) -> Option<u8> {
        let base = UART_BASE as *mut u8;
        unsafe {
            if (base.add(UART_LSR).read_volatile() & UART_LSR_DR) != 0 {
                Some(base.add(UART_RBR).read_volatile())
            } else {
                None
            }
        }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for &b in s.as_bytes() {
            if b == b'\n' {
                self.putchar(b'\r');
            }
            self.putchar(b);
        }
        Ok(())
    }
}

static UART: Uart = Uart::new();

pub fn uart_init() {
    UART.init();
}

pub fn uart_putchar(c: u8) {
    UART.putchar(c);
}

pub fn uart_getchar() -> Option<u8> {
    UART.getchar()
}

pub fn print_log(tag: &str, msg: &str) {
    let color = match tag {
        "BOOT" => "\x1b[1;34m",
        "MM" => "\x1b[1;32m",
        "PROC" => "\x1b[1;33m",
        "SLIP" => "\x1b[1;36m",
        "VIRTIO" => "\x1b[1;35m",
        _ => "\x1b[0m",
    };
    let reset = "\x1b[0m";
    let mut u = Uart::new();
    let _ = write!(u, "{color}[{tag}]{reset} {msg}\n");
}

pub fn print_seal() {
    let mut u = Uart::new();
    let seal = "\
        ⠴⠋⠉⠙⠦
       ⠾     ⠷
       ⣿⠷⠷⠷⠾⣿
       ⠙⠦   ⠴⠋
         ⠴⠷
        ⠴⠁ ⠳
       ⠰⠁   ⠁⠦
       ⠾⠳  ⠻⠟ ⠈⠦
       ⣿ ⠳      ⠳
       ⠻        ⣿
       ⠘⠴       ⣿
        ⠻       ⣿
        ⠾       ⣿
       ⠰⠋       ⠙⠦
       ⠾          ⠈⠙⠓⠦
      ⠰⠋              ⠙⠓⠦
      ⠸                  ⠙⠓⠦
      ⠸                    ⠙⠦
       ⠻                    ⠙⠷
        ⠻                    ⣿⠔⠈⠈⠈
        ⠌⠻  ⠸     ⠎          ⠟
   ⠄⠂⠁⠈⠁ ⠘     ⠅⠋⠉⠉⠉⠙⠉⠉⠁  ⠈⠑⠠
 ⠴⠮--⠄⠠⠄⠈⠁   ⠣    ⠣          ⠈⠠⠄⠁
              ⠑⠄   ⠜
                ⠈⠠⠄⠁";
    let _ = write!(u, "\n\x1b[1;36m{}\x1b[0m\n\n", seal);
}
