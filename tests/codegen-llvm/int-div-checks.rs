// With -Cinteger-div-checks=off (not enabled by default) integer zero div/overflow is UB,
// and compiler should not emit any checks

//@ revisions: CHECKED UNCHECKED
//@ compile-flags: -Copt-level=3
//@ [CHECKED] compile-flags: -Cinteger-div-checks=on
//@ [UNCHECKED] compile-flags: -Cinteger-div-checks=off

#![crate_type = "lib"]

// CHECK-LABEL: @possible_check
#[no_mangle]
pub fn possible_check(a: i32, b: i32) -> i32 {
    // CHECKED: icmp eq i32 %b, 0
    // CHECKED: br i1 {{.*}}, label %panic, label %bb1

    // UNCHECKED-NOT: icmp eq i32 %b, 0
    // UNCHECKED-NOT: panic

    // both should have an sdiv somewhere
    // CHECK: sdiv i32 %a, %b
    a / b
}
