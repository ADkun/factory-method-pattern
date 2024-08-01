//! 创建方法
//! 分为创建方法和静态创建方法两种
//!
//! - `创建方法`：泛指可以创建对象的方法。
//! - `静态创建方法`：可返回重用对象 / 区分以不同参数调用的构造函数
//!
//! 一个方法创建一种对象，主要作用是抽象出不同对象的创建过程。
//! Rust中可以使用 impl Object { fn new() -> Self { ... } } 来实现这种创建方法。
//! 但在Go中一般是使用 func NewObject() Object { ... } 来实现。
#![allow(unused)]

struct Programmer;
struct Department;

impl Department {
    /// 创建方法
    pub fn create_employee(&self) -> Programmer {
        Programmer
    }

    /// 静态创建方法
    pub fn static_create_employee() -> Programmer {
        Programmer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_method() {
        let department = Department;
        let _programmer = department.create_employee();
    }

    #[test]
    fn test_static_creation_method() {
        let _programmer = Department::static_create_employee();
    }
}
