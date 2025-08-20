use std::{error::Error, fmt::write};

use crate::{
    generators::common::match_brackets,
    lexer::{Token, TokenType},
};

pub fn generator_linux_arm64(
    tokens: Vec<Token>,
    memory_size: usize,
) -> Result<String, Box<dyn Error>> {
    let mut result = format!(
        ".global _main

.bss
.comm mem, {}

.text
_main:
sub sp, sp, 16
adrp x1, mem@PAGE
add x1, x1, mem@PAGEOFF
mov w0, 0\n",
        memory_size
    );

    let brackets = match_brackets(&tokens)?;

    for (i, t) in tokens.iter().enumerate() {
        match t.get_typ() {
            TokenType::Inc(size) => write(&mut result, format_args!("add w0, w0, {}\n", size)),
            TokenType::Dec(size) => write(&mut result, format_args!("sub w0, w0, {}\n", size)),
            TokenType::Left(size) => write(
                &mut result,
                format_args!("strb w0, [x1]\nsub x1, x1, {}\nldrb w0, [x1]\n", size),
            ),
            TokenType::Right(size) => write(
                &mut result,
                format_args!("strb w0, [x1]\nadd x1, x1, {}\nldrb w0, [x1]\n", size),
            ),
            TokenType::BracketOpen => write(
                &mut result,
                format_args!("cbz w0, _{}\n_{}:\n", brackets.get(&i).unwrap(), i),
            ),
            TokenType::BracketClose => write(
                &mut result,
                format_args!("cbnz w0, _{}\n_{}:\n", brackets.get(&i).unwrap(), i),
            ),
            TokenType::PutChar => write(
                &mut result,
                format_args!(
                    "strb w0, [x1]\nstr x1, [sp, 8]\nstr x16, [sp, 16]\nmov w16, 4\nmov w0, 1\nmov w2, 1\nsvc 0x80\nldr x1, [sp, 8]\nldr x16, [sp, 16]\nldrb w0, [x1]\n"
                ),
            ),
            TokenType::GetChar => write(
                &mut result,
                format_args!(
                    "strb w0, [x1]\nstr x1, [sp, 8]\nstr x16, [sp, 16]\nmov w16, 3\nmov w0, 0\nmov w2, 1\nsvc 0x80\nldr x1, [sp, 8]\nldr x16, [sp, 16]\nldrb w0, [x1]\n"
                ),
            ),
        }?;
    }

    write(
        &mut result,
        format_args!("add sp, sp, 16\nmov w16, 1\nmov w0, 0\nsvc 0x80"),
    )?;

    Ok(result)
}
