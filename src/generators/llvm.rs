use std::{collections::HashMap, error::Error};

use crate::{
    generators::common::match_brackets,
    lexer::{Token, TokenType},
};

use inkwell::{
    builder::{Builder, BuilderError},
    context::Context,
    types::ArrayType,
    values::{BasicValueEnum, PointerValue},
    AddressSpace,
};

/*
fn emit(&self) {
    let fn_type = self.context.void_type().fn_type(&[], false);
    let function = self.module.add_function("puts", fn_type, None);
    let basic_block = self.context.append_basic_block(function, "");

    self.builder.position_at_end(basic_block);

    let mem = self
        .builder
        .build_malloc(self.context.i32_type(), "")
        .unwrap();

    self.builder
        .build_store(mem, self.context.i32_type().const_int(20, false))
        .unwrap();

    self.builder.build_free(mem).unwrap();

    self.builder.build_return(None).unwrap();

    println!("{}", self.module.to_string());
}
*/

fn format_vars(vars: &mut usize) -> String {
    let str = format!("_var_{}", vars);
    *vars += 1;
    str
}

fn access_cell<'ctx>(
    context: &'ctx Context,
    builder: &'ctx Builder<'ctx>,
    mem: &PointerValue<'ctx>,
    mem_array_type: &ArrayType<'ctx>,
    idxp: &PointerValue<'ctx>,
    vars: &mut usize,
) -> Result<(PointerValue<'ctx>, BasicValueEnum<'ctx>), BuilderError> {
    let mut subvars = vars.clone();

    let idx = builder.build_load(context.i64_type(), *idxp, &format_vars(&mut subvars))?;

    let elem_ptr = unsafe {
        builder.build_in_bounds_gep(
            *mem_array_type,
            *mem,
            &[context.i32_type().const_zero(), idx.into_int_value()],
            &format_vars(&mut subvars),
        )
    }?;

    let cell = builder.build_load(context.i8_type(), elem_ptr, &format_vars(&mut subvars))?;

    *vars = subvars;

    Ok((elem_ptr, cell))
}

pub fn generator_llvm(
    tokens: Vec<Token>,
    memory_size: usize,
    file: &String,
) -> Result<String, Box<dyn Error>> {
    let context = Context::create();
    let module = context.create_module(file);
    let builder = context.create_builder();

    let putc_fmt = module.add_global(context.i8_type().array_type(2), None, "putc_fmt");

    {
        let i8_type = context.i8_type();
        let putc_fmt_a = i8_type.const_int(b'%' as u64, false);
        let putc_fmt_b = i8_type.const_int(b'c' as u64, false);

        putc_fmt.set_initializer(&i8_type.const_array(&[putc_fmt_a, putc_fmt_b]));
    }

    let putc_fn_type = context
        .i32_type()
        .fn_type(&[context.ptr_type(AddressSpace::default()).into()], true);
    let putc = module.add_function("printf", putc_fn_type, None);

    let getc_fn_type = context.i32_type().fn_type(&[], false);
    let getc = module.add_function("getchar", getc_fn_type, None);

    let main_fn_type = context.i32_type().fn_type(&[], false);
    let function_main = module.add_function("main", main_fn_type, None);
    let basic_block = context.append_basic_block(function_main, "start");

    builder.position_at_end(basic_block);

    let idxp = builder.build_alloca(context.i64_type(), "idx")?;
    builder.build_store(idxp, context.i64_type().const_zero())?;

    let mem_array_type = context.i8_type().array_type(memory_size.try_into()?);

    let mem = builder.build_alloca(mem_array_type, "mem")?;

    let brackets = match_brackets(&tokens)?;

    let mut blocks: HashMap<usize, inkwell::basic_block::BasicBlock> = HashMap::new();

    for (i, t) in tokens.iter().enumerate() {
        match t.get_typ() {
            TokenType::BracketOpen => {
                let block =
                    context.append_basic_block(function_main, format!("_br_{}", i).as_str());
                blocks.insert(i, block);
            }
            TokenType::BracketClose => {
                let block =
                    context.append_basic_block(function_main, format!("_br_{}", i).as_str());
                blocks.insert(i, block);
            }
            _ => (),
        }
    }

    let mut vars: usize = 0;

    for (i, t) in tokens.iter().enumerate() {
        match t.get_typ() {
            TokenType::Inc(n) => {
                let (elem_ptr, cell) =
                    access_cell(&context, &builder, &mem, &mem_array_type, &idxp, &mut vars)?;

                let result = builder.build_int_add(
                    cell.into_int_value(),
                    context.i8_type().const_int(*n as u64, false),
                    &format_vars(&mut vars),
                )?;

                builder.build_store(elem_ptr, result)?;
            }
            TokenType::Dec(n) => {
                let (elem_ptr, cell) =
                    access_cell(&context, &builder, &mem, &mem_array_type, &idxp, &mut vars)?;

                let result = builder.build_int_sub(
                    cell.into_int_value(),
                    context.i8_type().const_int(*n as u64, false),
                    &format_vars(&mut vars),
                )?;

                builder.build_store(elem_ptr, result)?;
            }
            TokenType::Left(n) => {
                let idx = builder.build_load(context.i64_type(), idxp, &format_vars(&mut vars))?;

                let result = builder.build_int_sub(
                    idx.into_int_value(),
                    context.i64_type().const_int(*n as u64, false),
                    &format_vars(&mut vars),
                )?;

                builder.build_store(idxp, result)?;
            }
            TokenType::Right(n) => {
                let idx = builder.build_load(context.i64_type(), idxp, &format_vars(&mut vars))?;

                let result = builder.build_int_add(
                    idx.into_int_value(),
                    context.i64_type().const_int(*n as u64, false),
                    &format_vars(&mut vars),
                )?;

                builder.build_store(idxp, result)?;
            }
            TokenType::BracketOpen | TokenType::BracketClose => {
                let (_, cell) =
                    access_cell(&context, &builder, &mem, &mem_array_type, &idxp, &mut vars)?;

                let zero = context.i8_type().const_zero();

                let cmp = builder.build_int_compare(
                    if *t.get_typ() == TokenType::BracketClose {
                        inkwell::IntPredicate::NE
                    } else {
                        inkwell::IntPredicate::EQ
                    },
                    cell.into_int_value(),
                    zero,
                    &format_vars(&mut vars),
                )?;

                let alt = brackets.get(&i).unwrap();

                let block_then = blocks.get(alt).unwrap();
                let block_else = blocks.get(&i).unwrap();

                builder.build_conditional_branch(cmp, *block_then, *block_else)?;

                builder.build_unconditional_branch(*block_else)?;

                builder.position_at_end(*block_else);
            }
            TokenType::PutChar => {
                let (_, cell) =
                    access_cell(&context, &builder, &mem, &mem_array_type, &idxp, &mut vars)?;

                let zero = context.i32_type().const_zero();

                let elem_ptr = unsafe {
                    builder.build_gep(
                        context.ptr_type(AddressSpace::default()),
                        putc_fmt.as_pointer_value(),
                        &[zero, zero],
                        &format_vars(&mut vars),
                    )
                }?;

                builder.build_call(
                    putc,
                    &[elem_ptr.into(), cell.into()],
                    &format_vars(&mut vars),
                )?;
            }
            TokenType::GetChar => {
                let ch = builder.build_call(getc, &[], &format_vars(&mut vars))?;
                builder.build_call(getc, &[], &format_vars(&mut vars))?; // Remove newline

                let trunc = builder.build_int_truncate(
                    ch.try_as_basic_value().left().unwrap().into_int_value(),
                    context.i8_type(),
                    &format_vars(&mut vars),
                )?;

                let idx = builder.build_load(context.i64_type(), idxp, &format_vars(&mut vars))?;

                let elem_ptr = unsafe {
                    builder.build_in_bounds_gep(
                        mem_array_type,
                        mem,
                        &[context.i32_type().const_zero(), idx.into_int_value()],
                        &format_vars(&mut vars),
                    )
                }?;

                builder.build_store(elem_ptr, trunc)?;
            }
        }
    }

    builder.position_at_end(function_main.get_last_basic_block().unwrap());
    builder.build_return(Some(&context.i32_type().const_zero()))?;

    Ok(module.to_string())
}
