pub struct Info {
    pub heap_start: usize,
    pub heap_end: usize,
}

impl Info {
    pub fn init() -> Self {
        unsafe extern "C" {
            static _heap_start: u8;
            static _heap_end: u8;
        }

        let hs = unsafe { &_heap_start as *const u8 as usize };
        let he = unsafe { &_heap_end as *const u8 as usize };

        Self {
            heap_start: hs,
            heap_end: he,
        }
    }
}
