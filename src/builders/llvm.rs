use std::{error::Error, fs, path::Path, process::Command};

use crate::builders::common::{BUILD_DIR, EXE_FILE};

pub fn builder_llvm(
    asm: &String,
    _linker: &String,
    _assembler: &String,
    _is_macos: bool,
) -> Result<String, Box<dyn Error>> {
    if fs::exists(BUILD_DIR)? {
        fs::remove_dir_all(BUILD_DIR)?;
    }

    fs::create_dir(BUILD_DIR)?;

    let build_dir = Path::new(BUILD_DIR);

    let asm_file = build_dir.join("out.ll");

    fs::write(&asm_file, asm)?;

    let exe_file = build_dir.join(EXE_FILE);

    let mut link_cmd = Command::new("clang");
    link_cmd.arg(asm_file).arg("-o").arg(&exe_file);

    let _linker_output = link_cmd.spawn()?.wait_with_output()?;

    Ok(exe_file.to_str().unwrap().to_string())
}
