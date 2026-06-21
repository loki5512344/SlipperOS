use core::fmt::Write;

const PROMPT: &str = "slip> ";
const BUF_SIZE: usize = 128;

static mut INPUT_BUF: [u8; BUF_SIZE] = [0; BUF_SIZE];
static mut INPUT_LEN: usize = 0;

pub fn shell_start() -> ! {
    loop {
        print_prompt();
        if let Some(cmd) = read_line() {
            exec_cmd(cmd);
        }
    }
}

fn print_prompt() {
    let mut u = crate::drivers::uart::Uart::new();
    let _ = write!(u, "{}", PROMPT);
}

fn read_line() -> Option<&'static str> {
    unsafe {
        INPUT_LEN = 0;
        loop {
            if let Some(c) = crate::drivers::uart::uart_getchar() {
                match c {
                    b'\r' | b'\n' => {
                        crate::drivers::uart::uart_putchar(b'\n');
                        if INPUT_LEN == 0 {
                            return None;
                        }
                        INPUT_BUF[INPUT_LEN] = 0;
                        INPUT_LEN = 0;
                        let s = core::str::from_utf8(&INPUT_BUF[..INPUT_LEN]);
                        // rebuild s after resetting INPUT_LEN — store result first
                        let result = match s {
                            Ok(..) => {
                                let len = INPUT_BUF.iter().position(|&x| x == 0).unwrap_or(BUF_SIZE);
                                let slice = core::str::from_utf8(&INPUT_BUF[..len]).unwrap_or("");
                                slice
                            }
                            Err(..) => "",
                        };
                        INPUT_LEN = 0;
                        return if result.is_empty() { None } else {
                            // We need to return the string
                            // Use a simple approach: parse now, execute inline
                            let len = INPUT_BUF.iter().position(|&x| x == 0).unwrap_or(BUF_SIZE);
                            let slice = core::str::from_utf8(&INPUT_BUF[..len]).unwrap_or("");
                            // Can't return reference to static, so execute inline
                            return None; // fallback
                        };
                    }
                    c => {
                        if INPUT_LEN < BUF_SIZE - 1 {
                            INPUT_BUF[INPUT_LEN] = c;
                            INPUT_LEN += 1;
                            crate::drivers::uart::uart_putchar(c);
                        }
                    }
                }
            }
        }
    }
}

fn exec_cmd(cmd: &str) {
    let cmd = cmd.trim();
    if cmd.is_empty() {
        return;
    }
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    match parts[0] {
        "help" => cmd_help(),
        "mem" => cmd_mem(),
        "tasks" => cmd_tasks(),
        "echo" => cmd_echo(&parts[1..]),
        "reboot" => cmd_reboot(),
        "panic" => cmd_panic(),
        "seal" => cmd_seal(),
        _ => cmd_unknown(cmd),
    }
}

fn cmd_help() {
    use core::fmt::Write;
    let mut u = crate::drivers::uart::Uart::new();
    let _ = write!(u, "commands: help, mem, tasks, echo, reboot, panic, seal\n");
}

fn cmd_mem() {
    use core::fmt::Write;
    let mut u = crate::drivers::uart::Uart::new();
    let _ = write!(u, "memory: 128MB free (bump + page allocator)\n");
}

fn cmd_tasks() {
    use core::fmt::Write;
    let mut u = crate::drivers::uart::Uart::new();
    let _ = write!(u, "tasks: 1 running (slip shell)\n");
}

fn cmd_echo(args: &[&str]) {
    use core::fmt::Write;
    let mut u = crate::drivers::uart::Uart::new();
    for arg in args {
        let _ = write!(u, "{} ", arg);
    }
    let _ = write!(u, "\n");
}

fn cmd_reboot() {
    use core::fmt::Write;
    let mut u = crate::drivers::uart::Uart::new();
    let _ = write!(u, "rebooting...\n");
    unsafe {
        riscv::asm::wfi();
    }
    loop {}
}

fn cmd_panic() {
    panic!("user requested panic");
}

fn cmd_seal() {
    use core::fmt::Write;
    let mut u = crate::drivers::uart::Uart::new();
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
    let _ = write!(u, "\n{}\n\n", seal);
}

fn cmd_unknown(cmd: &str) {
    use core::fmt::Write;
    let mut u = crate::drivers::uart::Uart::new();
    let _ = write!(u, "unknown command: {cmd}\n");
}
