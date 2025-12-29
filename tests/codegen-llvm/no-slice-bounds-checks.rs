// Test for "remove all slice indexing bounds checks" option.

//@ revisions: REGULAR NO_BOUNDS_CHECKS
//@ compile-flags: -Copt-level=3
//@ [NO_BOUNDS_CHECKS] compile-flags: -C bounds-checks=off

#![crate_type = "lib"]

// CHECK-LABEL: @removed_bounds_checks
#[no_mangle]
pub fn removed_bounds_checks(buf: &[u8], index: usize) -> u8 {
    // REGULAR: br i1 {{.*}}, label %{{.*}}, label %panic

    // NO_BOUNDS_CHECKS: getelementptr inbounds {{.*}}i8, ptr {{.*}}, i64 {{.*}}
    // NO_BOUNDS_CHECKS-NEXT: load i8, ptr {{.*}}
    // NO_BOUNDS_CHECKS-NEXT: ret i8 {{.*}}
    buf[index]
}
