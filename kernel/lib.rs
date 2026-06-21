pub mod drivers {
    pub mod uart;
    pub mod clint;
    pub mod plic;
    pub mod virtio {
        pub mod block;
        pub mod net;
    }
}

pub mod mm {
    pub mod bump;
    pub mod page;
    pub mod map;
}

pub mod proc {
    pub mod task;
    pub mod sched;
    pub mod context;
}

pub mod fs {
    pub mod slipfs;
}

pub mod shell {
    pub mod slip;
}

pub use drivers::uart::*;
pub use drivers::clint::*;
pub use drivers::plic::*;
pub use drivers::virtio::block::*;
pub use mm::bump::*;
pub use mm::page::*;
pub use mm::map::*;
pub use proc::sched::*;
pub use fs::slipfs::*;
pub use shell::slip::*;
