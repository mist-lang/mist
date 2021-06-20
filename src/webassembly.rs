use either::Either;

mod module;

/// basically just represents the `$` before the given symbol.
pub enum Index {
	Name(&'static str),
	Number(u32),
}

pub struct Module {
	imports: Vec<Import>,
	exports: Vec<Export>,
	mems: Vec<Memory>,
	funcs: Vec<Func>,
	start: Index,
}

pub struct Import {
	module_name: &'static str,
	name: &'static str,
	desc: ImportDesc,
}

pub enum ImportDesc {
	Func(Index, FuncType),
	Table(Index, /* TODO */),
	Mem(Index, Memory),
	Global(Index, /* TODO */),
}

pub struct Export {
	name: &'static str,
	desc: ExportDesc,
}

pub enum ExportDesc {
	Func(Either<Index, Func>),
	Table(Index),
	Memory(Either<Index, Memory>),
	Global(Index),
}

pub struct Memory(Limits);

pub struct Limits(u32, Option<u32>);

pub struct Func {
	name: Index,
	typeuse: FuncType,
	locals: Vec<Local>,
	instrs: Vec<Instr>,
}

pub struct FuncType {
	params: Vec<Param>,
	ret_ty: Vec<Result>,
}

pub struct Param(Index, ValType);

pub struct Result(ValType);

pub struct Local {
	name: Index,
	valtype: ValType,
}

pub enum ValType {
	Num(NumType),
}

pub enum NumType {
	I32,
	I64,
	F32,
	F64,
}

pub enum HeapType {
	Func,
	Extern,
}

pub enum Instr {
	Plain(PlainInstr),
	Block(BlockInstr),
}

#[allow(non_camel_case_types)]
pub enum PlainInstr {
	unreachable,
	nop,
	br(Index),
	br_if(Index),
	br_table(Vec<Index>, Index),
	r#return,
	call(Index),
	call_indirect(Index, FuncType),
	Ref(RefInstr),
	drop,
	select(Vec<Result>),
	Local(LocalInstr),
	Global(GlobalInstr),
	I32(I32Instr),
	I64(I64Instr),
	F32(F32Instr),
	F64(F64Instr),
	// TODO: data.*
	// TODO: elem.*
	// TODO: memory.*
	// TODO: table.*
}

#[allow(non_camel_case_types)]
pub enum RefInstr {
	null(HeapType),
	is_null,
	func(Index),
}

#[allow(non_camel_case_types)]
pub enum LocalInstr {
	get(Index),
	set(Index),
	tee(Index),
}

#[allow(non_camel_case_types)]
pub enum GlobalInstr {
	get(Index),
	set(Index),
}

#[allow(non_camel_case_types)]
pub enum I32Instr {
	r#const(i32),
	clz,
	ctz,
	popcnt,
	add,
	sub,
	mul,
	div_s,
	div_u,
	rem_s,
	rem_u,
	and,
	or,
	xor,
	shl,
	shr_s,
	shr_u,
	rotl,
	rotr,
	eqz,
	eq,
	ne,
	lt_s,
	lt_u,
	gt_s,
	gt_u,
	le_s,
	le_u,
	ge_s,
	ge_u,
	wrap_i64,
	trunc_f32_s,
	trunc_f32_u,
	trunc_f64_s,
	trunc_f64_u,
	trunc_sat_f32_s,
	trunc_sat_f32_u,
	trunc_sat_f64_s,
	trunc_sat_f64_u,
	reinterpret_f32,
	extend8_s,
	extend16_s,
	// TODO: loads
	// TODO: stores
}

#[allow(non_camel_case_types)]
pub enum I64Instr {
	r#const(i64),
	clz,
	ctz,
	popcnt,
	add,
	sub,
	mul,
	div_s,
	div_u,
	rem_s,
	rem_u,
	and,
	or,
	xor,
	shl,
	shr_s,
	shr_u,
	rotl,
	rotr,
	eqz,
	eq,
	ne,
	lt_s,
	lt_u,
	gt_s,
	gt_u,
	le_s,
	le_u,
	ge_s,
	ge_u,
	extend_i32_s,
	extend_i32_u,
	trunc_f32_s,
	trunc_f32_u,
	trunc_f64_s,
	trunc_f64_u,
	trunc_sat_f32_s,
	trunc_sat_f32_u,
	trunc_sat_f64_s,
	trunc_sat_f64_u,
	reinterpret_f64,
	extend8_s,
	extend16_s,
	extend32_s,
	// TODO: loads
	// TODO: stores
}

#[allow(non_camel_case_types)]
pub enum F32Instr {
	r#const(f32),
	abs,
	neg,
	ceil,
	floor,
	trunc,
	nearest,
	sqrt,
	add,
	sub,
	mul,
	div,
	min,
	max,
	copysign,
	convert_i32_s,
	convert_i32_u,
	convert_i64_s,
	convert_i64_u,
	demote_f64,
	reinterpret_i32,
	// TODO: loads
	// TODO: stores
}

#[allow(non_camel_case_types)]
pub enum F64Instr {
	r#const(f64),
	abs,
	neg,
	ceil,
	floor,
	trunc,
	nearest,
	sqrt,
	add,
	sub,
	mul,
	div,
	min,
	max,
	copysign,
	convert_i32_s,
	convert_i32_u,
	convert_i64_s,
	convert_i64_u,
	promote_f32,
	reinterpret_i64,
	// TODO: loads
	// TODO: stores
}

pub enum BlockType {
	None,
	Result(Result),
	TypeUse(FuncType),
}

#[allow(non_camel_case_types)]
pub enum BlockInstr {
	block(BlockType, Vec<Instr>),
	r#loop(BlockType, Vec<Instr>),
	r#if(BlockType, Vec<Instr>, Vec<Instr>),
}
