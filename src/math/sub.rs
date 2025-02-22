pub fn sub(x: i32, y: i32) -> i32 {
    let add = "something else";
    let res = {
        // 在这个代码块中，`add` 是 `add` 模块导出的函数
        use super::add::*;
        add(x, -y)
    };
    // 现在我们离开代码块，`add` 又变为 "something else"
    res
}
