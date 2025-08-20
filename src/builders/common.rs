macro_rules! exe_file {
    ("windows") => {
        "out.exe"
    };
    ($el:expr) => {
        "out"
    };
}

pub const BUILD_DIR: &str = "xbf_build";
pub const ASM_FILE: &str = "out.s";
pub const OBJ_FILE: &str = "out.o";
pub const EXE_FILE: &str = exe_file!(std::env::consts::OS);
