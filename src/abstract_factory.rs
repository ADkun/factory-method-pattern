//! 抽象工厂模式
//!
//! 设计模式，用于生产属于同一产品族的对象
//!
//! 抽象工厂模式通常被作为一种对象生成器来使用，并且它通常用来生成属于同一产品族，但不同系列的产品。
//! 如下面例子中，ITProject与Programmer属于一个产品族，而ITProject与UIProject属于同一个产品的不同系列。
#![allow(unused)]

use rand::Rng;
use std::collections::HashMap;
use std::ops::Range;
use std::rc::Rc;

const ACCOUNT_ID_RANGE: Range<usize> = 1000..99999;

trait Employee {
    fn register_account(&self) -> usize;
}

struct Programmer;
impl Programmer {
    fn new() -> Self {
        Self
    }
}

impl Employee for Programmer {
    fn register_account(&self) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(ACCOUNT_ID_RANGE)
    }
}
struct Designer;
impl Designer {
    fn new() -> Self {
        Self
    }
}
impl Employee for Designer {
    fn register_account(&self) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(ACCOUNT_ID_RANGE)
    }
}

trait Project {
    /// id: Employee ID
    fn assign_to_employee(&self, id: usize);
}

struct ITProject;
impl Project for ITProject {
    fn assign_to_employee(&self, id: usize) {
        println!("Assigning IT project to employee with ID: {}", id);
    }
}

struct UIProject;
impl Project for UIProject {
    fn assign_to_employee(&self, id: usize) {
        println!("Assigning UI project to employee with ID: {}", id);
    }
}

trait Department {
    fn create_employee(&self) -> Rc<dyn Employee>;
    fn create_project(&self) -> Rc<dyn Project>;
}
struct ITDepartment;
impl Department for ITDepartment {
    fn create_employee(&self) -> Rc<dyn Employee> {
        Rc::new(Programmer::new())
    }

    fn create_project(&self) -> Rc<dyn Project> {
        Rc::new(ITProject)
    }
}
struct UIDepartment;
impl Department for UIDepartment {
    fn create_employee(&self) -> Rc<dyn Employee> {
        Rc::new(Designer::new())
    }

    fn create_project(&self) -> Rc<dyn Project> {
        Rc::new(UIProject)
    }
}

struct DeparmentManager {
    department: Box<dyn Department>,
    projects: Vec<Rc<dyn Project>>,
    employees: HashMap<usize, Rc<dyn Employee>>,
}

impl DeparmentManager {
    fn new(department: Box<dyn Department>) -> Self {
        Self {
            department,
            projects: vec![],
            employees: HashMap::new(),
        }
    }

    /// 创建一个项目，并将其添加到项目列表中
    fn create_project(&mut self) -> Rc<dyn Project> {
        let project = self.department.create_project();
        self.projects.push(Rc::clone(&project));
        project
    }

    /// 创建一个员工，并返回其账号ID
    fn create_employee(&mut self) -> usize {
        let employee = self.department.create_employee();
        let account_id = employee.register_account();
        self.employees.insert(account_id, Rc::clone(&employee));
        account_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abstract_factory() {
        let it_department = ITDepartment;
    }

    #[test]
    fn test_department_manager() {
        let mut manager = DeparmentManager::new(Box::new(ITDepartment));
        let project = manager.create_project();
        let account_id = manager.create_employee();
        project.assign_to_employee(account_id);
    }
}
