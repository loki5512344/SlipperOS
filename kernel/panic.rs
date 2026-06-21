use core::fmt::Write;
use core::panic::PanicInfo;

use crate::drivers::uart::Uart;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
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
    let _ = write!(u, "\x1b[1;31m\n{}\n\n!! SLIPPED !!\x1b[0m\n", seal);
    if let Some(msg) = info.message() {
        let _ = write!(u, "cause: {}\n", msg);
    }
    if let Some(loc) = info.location() {
        let _ = write!(u, "at: {}:{}\n", loc.file(), loc.line());
    }
    let _ = write!(u, "\x1b[1;31mhalted.\x1b[0m\n");
    loop {
        unsafe {
            riscv::asm::wfi();
        }
    }
}
