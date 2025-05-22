#![no_std]

use allocator::{AllocError, BaseAllocator, ByteAllocator, PageAllocator};
use core::alloc::Layout;
use core::ptr::NonNull;
/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const PAGE_SIZE: usize> {
    start: usize,
    byte_pos: usize,
    end: usize,
    page_pos: usize,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    /// Creates a new empty [`EarlyAllocator`].
    pub const fn new() -> Self {
        Self {
            start: 0,
            byte_pos: 0,
            end: 0,
            page_pos: 0,
        }
    }
}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, _start: usize, _size: usize) {
        // Initialize the allocator with start address and size
        let end = _start + _size;
        self.start = _start;
        self.end = end;
        self.byte_pos = _start;
        self.page_pos = end;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> allocator::AllocResult {
        Err(AllocError::NoMemory) // unsupported
    }
}

impl<const PAGE_SIZE: usize> ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    /// Allocate memory with the given size (in bytes) and alignment.
    fn alloc(&mut self, layout: Layout) -> allocator::AllocResult<NonNull<u8>> {
        let align = layout.align();
        let size = layout.size();
        let start = self.byte_pos;
        let real_start = (start + align - 1) & !(align - 1);
        let end = real_start + size;
        if end > self.page_pos {
            return Err(AllocError::NoMemory);
        }
        self.byte_pos = end;
        NonNull::new(real_start as *mut u8).ok_or(AllocError::NoMemory)
    }

    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        let ptr = pos.as_ptr() as usize;
        if ptr + layout.size() == self.byte_pos {
            self.byte_pos = ptr;
        }
    }

    /// Returns total memory size in bytes.
    fn total_bytes(&self) -> usize {
        self.end - self.start
    }

    /// Returns allocated memory size in bytes.
    fn used_bytes(&self) -> usize {
        self.byte_pos - self.start
    }

    /// Returns available memory size in bytes.
    fn available_bytes(&self) -> usize {
        self.page_pos - self.byte_pos
    }
}

impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    const PAGE_SIZE: usize = PAGE_SIZE;
    fn alloc_pages(
        &mut self,
        num_pages: usize,
        align_pow2: usize,
    ) -> allocator::AllocResult<usize> {
        if align_pow2 % Self::PAGE_SIZE != 0 {
            return Err(AllocError::InvalidParam);
        }
        let align_pages = align_pow2 / Self::PAGE_SIZE;
        if !align_pages.is_power_of_two() {
            return Err(AllocError::InvalidParam);
        }
        let align_log2 = align_pages.trailing_zeros() as usize;

        let current_page = self.page_pos / Self::PAGE_SIZE;
        let start_page = match current_page.checked_sub(num_pages) {
            Some(page) => page,
            None => return Err(AllocError::NoMemory),
        };
        let aligned_start_page = (start_page >> align_log2) << align_log2;

        let byte_page = self.byte_pos / Self::PAGE_SIZE;
        if aligned_start_page < byte_page {
            return Err(AllocError::NoMemory);
        }

        self.page_pos = aligned_start_page * Self::PAGE_SIZE;
        Ok(self.page_pos)
    }

    fn dealloc_pages(&mut self, _pos: usize, _num_pages: usize) {
        // Pages are never freed in the early allocator
    }

    fn total_pages(&self) -> usize {
        (self.end - self.start) / Self::PAGE_SIZE
    }

    fn used_pages(&self) -> usize {
        (self.end - self.page_pos) / Self::PAGE_SIZE
    }

    fn available_pages(&self) -> usize {
        (self.page_pos - self.byte_pos) / Self::PAGE_SIZE
    }
}
