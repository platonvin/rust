// With -C fp-mode=fast/algebraic, the compiler can optimize some operation.

//@ revisions: STRICT FAST ALGEBRAIC
//@ compile-flags: -Copt-level=3
//@ [STRICT] compile-flags: -C fp-mode=strict
//@ [FAST] compile-flags: -C fp-mode=fast
//@ [ALGEBRAIC] compile-flags: -C fp-mode=algebraic

#![crate_type = "lib"]

// STRICT: fadd float %x, 0.0
// FAST-NOT: fadd
// ALGEBRAIC-NOT: fadd
#[no_mangle]
pub fn check_identity(x: f32) -> f32 {
    x + 0.0
}

// STRICT: fadd float %x, 1.0
// STRICT: fadd float %{{.*}}, -1.0
// FAST-NOT: fadd
// ALGEBRAIC-NOT: fadd
#[no_mangle]
pub fn check_reassociate(x: f32) -> f32 {
    (x + 1.0) - 1.0
}

// STRICT: call float @llvm.sqrt.f32
// STRICT: fmul float
// FAST-NOT: call float @llvm.sqrt.f32
// ALGEBRAIC: call float @llvm.sqrt.f32
#[no_mangle]
pub fn check_sqrt_simplify(x: f32) -> f32 {
    x.sqrt() * x.sqrt()
}

// STRICT: fmul float
// STRICT: fmul float
// STRICT: fadd float
// ALGEBRAIC: fadd reassoc
// ALGEBRAIC: fmul reassoc
// FAST: fadd fast float
// FAST: fmul fast float
#[no_mangle]
pub fn check_distributive(x: f32, y: f32, z: f32) -> f32 {
    (x * y) + (x * z)
}

// STRICT: fdiv float %x
// ALGEBRAIC-NOT: fdiv
// FAST: fmul fast float %x
// FAST-NOT: fdiv
#[no_mangle]
pub fn check_reciprocal(x: f32) -> f32 {
    x / 10.0
}
