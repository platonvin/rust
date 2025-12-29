// With -Coverflow-checks=unchecked (not enabled by default) integer overflow is UB,
// and compiler can use associative property to remove multiply & delete by two

//@ revisions: WRAPPING UNCHECKED
//@ compile-flags: -Copt-level=3
//@ [WRAPPING] compile-flags: -Coverflow-checks=wrapping
//@ [UNCHECKED] compile-flags: -Coverflow-checks=unchecked

#![crate_type = "lib"]

// CHECK-LABEL: @possible_nop
#[no_mangle]
pub fn possible_nop(num: i32) -> i32 {
    // WRAPPING: shl i32 %num, 1
    // WRAPPING: ashr exact i32 {{.*}}, 1
    // We expect this to be turned into no-op
    // UNCHECKED: ret i32 %num
    (num * 2) / 2
}
