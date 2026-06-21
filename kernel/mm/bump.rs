use core::alloc::{GlobalAlloc, Layout};

const HEAP_SIZE: usize = 0x100000; // 1MB
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
static mut NEXT: usize = 0;

pub struct BumpAllocator;

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let align = layout.align();
        let size = layout.size();
        let next = align_up(NEXT, align);
        if next + size > HEAP_SIZE {
            return core::ptr::null_mut();
        }
        NEXT = next + size;
        HEAP.as_mut_ptr().add(next)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // bump allocator never frees
    }
}

fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator;
