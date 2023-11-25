#[derive(PartialEq, Eq, Debug, Copy, Clone)]
#[repr(transparent)]
pub struct NxResult(u32);

impl NxResult {
    const MODULE_BITS: u32 = 9;
    const DESCRIPTION_BITS: u32 = 13;

    const MODULE_MASK: u32 = (1 << Self::MODULE_BITS) - 1;
    const DESCRIPTION_MASK: u32 = (1 << Self::DESCRIPTION_BITS) - 1;

    pub const fn new(module: u32, description: u32) -> Self {
        Self(
            (module & Self::MODULE_MASK) << Self::DESCRIPTION_BITS
                | (description & Self::DESCRIPTION_MASK),
        )
    }

    pub const fn module(&self) -> u32 {
        (self.0 >> Self::DESCRIPTION_BITS) & Self::MODULE_MASK
    }

    pub const fn description(&self) -> u32 {
        self.0 & Self::DESCRIPTION_MASK
    }
}

#[macro_export]
macro_rules! make_result {
    ($name:ident, $module:expr, $description:expr) => {
        #[allow(dead_code)]
        pub const $name: NxResult = NxResult::new($module, $description);
    };
}

#[macro_export]
macro_rules! mod_result {
    ($name:ident, $description:expr) => {
        make_result!($name, MODULE, $description);
    };
}

make_result!(SUCCESS, 0, 0);

#[macro_export]
macro_rules! check_result {
    ($result:expr, $out: expr) => {
        if $result == crate::nx::result::SUCCESS {
            return Ok($out);
        } else {
            return Err($result);
        }
    };
}

pub mod fs;
pub mod kern;
pub mod libnx;
