//! 工厂方法模式
//!
//! 设计模式，将对象创建解耦，分散到不同子类工厂，支持模板流程。
//!
//! 工厂方法模式使用子工厂类来创建对象，当需要新对象时，只需要新建子工厂类即可，无需修改现有函数。
//! 工厂方法可以在父工厂定义模板方法，模板方法就是把公共的流程提取出来，具体调用什么对象是由子工厂来决定的。
//! **工厂方法的价值主要体现在流程模板的抽象上**，下面例子中就是onboard方法
#![allow(unused)]

trait Employee {
    fn register_account(&self);
}

struct Programmer;
impl Employee for Programmer {
    fn register_account(&self) {
        println!("Registering a programmer account");
    }
}
struct Designer;
impl Employee for Designer {
    fn register_account(&self) {
        println!("Registering a designer account");
    }
}

trait Department {
    fn create_employee(&self) -> Box<dyn Employee>;

    /// 模板方法
    /// 业务的执行步骤是固定的，但每一个步骤所对应的接口的具体实现是由哪个子类来提供，完全由调用方来决定。
    fn onboard(&self) {
        let employee = self.create_employee();
        employee.register_account();
    }
}
struct ITDepartment;
impl Department for ITDepartment {
    fn create_employee(&self) -> Box<dyn Employee> {
        Box::new(Programmer)
    }
}
struct UIDepartment;
impl Department for UIDepartment {
    fn create_employee(&self) -> Box<dyn Employee> {
        Box::new(Designer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_method() {
        let it_department = ITDepartment;
        // ITDepartment 不需要实现onboard方法，只需要实现create_employee方法提供Employee即可。
        it_department.onboard();
    }
}
