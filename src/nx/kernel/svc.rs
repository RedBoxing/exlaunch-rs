use core::arch::asm;

use crate::nx::result;
use crate::{check_result, nx::result::NxResult};

pub fn set_heap_size(size: usize) -> Result<usize, NxResult> {
    let res: NxResult;
    let out: usize;

    unsafe {
        asm!(
            "svc 0x1",
            in("x0") size,
            lateout("w0") res as u32,
            out("x1") out,
        )
    }

    check_result!(res, out);
}
