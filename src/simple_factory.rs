//! 简单工厂模式
//!
//! 将对象创建封装在独立函数中，并用条件语句区分对象类型
//!
//! 创建方法+枚举判断，可让同一个函数创建不同的对象
//! 缺点是违背了开闭原则，因为每次需要新增加对象类型，都需要修改工厂方法
#![allow(unused)]

trait Employee {}

struct Programmer;
impl Employee for Programmer {}
struct Designer;
impl Employee for Designer {}

enum EmployeeType {
    Programmer,
    Designer,
}

struct Department;

impl Department {
    pub fn create_employee(employee_type: EmployeeType) -> Box<dyn Employee> {
        match employee_type {
            EmployeeType::Programmer => Box::new(Programmer),
            EmployeeType::Designer => Box::new(Designer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_factory() {
        let _programmer = Department::create_employee(EmployeeType::Programmer);
        let _designer = Department::create_employee(EmployeeType::Designer);
    }
}
