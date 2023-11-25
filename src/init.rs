use crate::settings;
use crate::LoadKind;

extern "C" {
    /* These magic symbols are provided by the linker.  */
    #[link_name = "__preinit_array_start"]
    //#[linkage = "extern_weak"]
    static __preinit_array_start: extern "C" fn();

    #[link_name = "__preinit_array_end"]
    //#[linkage = "extern_weak"]
    static __preinit_array_end: extern "C" fn();

    #[link_name = "__init_array_start"]
    //#[linkage = "extern_weak"]
    static __init_array_start: extern "C" fn();

    #[link_name = "__init_array_end"]
    //#[linkage = "extern_weak"]
    static __init_array_end: extern "C" fn();

    /* Exported by program. */
    //#[link_name = "exl_main"]
    //fn exl_main(x0: *const u8, x1: *const u8) -> ();
}

extern "Rust" {
    fn main() -> ();
}

#[cfg(feature = "fake_heap")]
static __FAKE_HEAP: [u8; settings::FAKE_HEAP_SIZE] = [0; settings::FAKE_HEAP_SIZE];

#[cfg(feature = "fake_heap")]
extern "C" fn __init_heap() {
    extern "C" {
        #[link_name = "fake_heap_start"]
        static mut __fake_heap_start: u8;

        #[link_name = "fake_heap_end"]
        static mut __fake_heap_end: u8;
    }

    unsafe {
        __fake_heap_start = __FAKE_HEAP.as_ptr() as *const u8 as usize as u8;
        __fake_heap_end =
            __FAKE_HEAP.as_ptr().add(settings::FAKE_HEAP_SIZE) as *const u8 as usize as u8;
    }
}

extern "C" fn __init_array() {
    let count = unsafe {
        (&__preinit_array_end as *const _ as usize) - (&__preinit_array_start as *const _ as usize)
    };

    for i in 0..count {
        let f: extern "C" fn() = unsafe {
            core::mem::transmute(core::ptr::read_volatile(
                ((&__preinit_array_start as *const _ as usize) + i) as *const *const (),
            ))
        };

        unsafe {
            f();
        }
    }

    let count = unsafe {
        (&__init_array_end as *const _ as usize) - (&__init_array_start as *const _ as usize)
    };

    for i in 0..count {
        let f: extern "C" fn() = unsafe {
            core::mem::transmute(core::ptr::read_volatile(
                ((&__init_array_start as *const _ as usize) + i) as *const *const (),
            ))
        };
        unsafe {
            f();
        };
    }
}

/* Called when loaded as a module with RTLD. */
extern "C" fn exl_module_init() {
    #[cfg(feature = "fake_heap")]
    __init_heap();

    unsafe {
        exl_init();
    }

    __init_array();

    unsafe {
        main();
    }
}

/* Called when loaded as the entrypoint of the process, like RTLD. */
extern "C" fn exl_entrypoint_init(x0: *const u8, x1: *const u8) {
    #[cfg(feature = "fake_heap")]
    __init_heap();

    unsafe {
        exl_init();
    }

    __init_array();

    unsafe {
        main();
    }
}

extern "C" fn exl_module_finit() {}

extern "C" fn exl_init() {
    if settings::LOAD_KIND == LoadKind::Module {}
}
