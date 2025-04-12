// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let mut x = 100;

    let y = &mut x;  // 创建 y
    *y += 100;

    // 现在可以安全地创建 z，因为 y 已经结束作用
    let z = &mut x;  // 创建 z
    *z += 1000;

    assert_eq!(x, 1200);
}

