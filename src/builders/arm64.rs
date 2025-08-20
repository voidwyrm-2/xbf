use std::{error::Error, fs, path::Path, process::Command};

use crate::builders::common::{ASM_FILE, BUILD_DIR, EXE_FILE, OBJ_FILE};

pub fn builder_linux_arm64(
    asm: &String,
    linker: &String,
    assembler: &String,
    is_macos: bool,
) -> Result<String, Box<dyn Error>> {
    if fs::exists(BUILD_DIR)? {
        fs::remove_dir_all(BUILD_DIR)?;
    }

    fs::create_dir(BUILD_DIR)?;

    let build_dir = Path::new(BUILD_DIR);

    let asm_file = build_dir.join(ASM_FILE);

    fs::write(&asm_file, asm)?;

    let obj_file = build_dir.join(OBJ_FILE);

    let mut asm_cmd = Command::new(assembler);
    asm_cmd.arg(asm_file).arg("-o").arg(&obj_file);

    let _asm_output = asm_cmd.spawn()?.wait_with_output()?;

    let exe_file = build_dir.join(EXE_FILE);

    let mut link_cmd = Command::new(linker);
    link_cmd.arg(obj_file).arg("-o").arg(&exe_file);

    if is_macos && linker == "ld" {
        link_cmd.args([
            "-macos_version_min",
            "15.0",
            "-lSystem",
            "-L/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/lib",
        ]);
    }

    let _linker_output = link_cmd.spawn()?.wait_with_output()?;

    Ok(exe_file.to_str().unwrap().to_string())
}
