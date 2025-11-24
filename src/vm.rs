pub type Stack = Vec<i32>;

#[derive(Clone)]
pub enum OduOp {
    PushConst(i32),
    PopVoid,
    Dup,
    Swap,
    Add,
    Sub,
    HaltIfOne,
}

impl OduOp {
    pub fn execute(&self, stack: &mut Stack) {
        match self {
            OduOp::PushConst(v) => stack.push(*v),
            OduOp::PopVoid => { let _ = stack.pop(); },
            OduOp::Dup => if let Some(top) = stack.last() { stack.push(*top); },
            OduOp::Swap => if stack.len() >= 2 {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(b); stack.push(a);
            },
            OduOp::Add => if stack.len() >= 2 {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a + b);
            },
            OduOp::Sub => if stack.len() >= 2 {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a - b);
            },
            OduOp::HaltIfOne => if stack.last() == Some(&1) {
                println!("Àṣẹ");
                std::process::exit(0);
            },
        }
    }
}

pub struct IfaVM {
    pub stack: Stack,
}

impl IfaVM {
    pub fn new() -> Self { Self { stack: Vec::new() } }

    pub fn execute(&mut self, program: Vec<&str>) {
        use crate::odu::ODU_TABLE;
        for odu_name in program {
            if let Some(op) = ODU_TABLE.get(odu_name) {
                op.clone().execute(&mut self.stack);
            }
        }
    }
}
