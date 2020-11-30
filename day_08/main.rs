use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn read(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut rv = Vec::new();

    for line in br.lines() {
        let line = line?;
        rv.push(line);
    }
    Ok(rv)
}

#[derive(Debug,PartialEq,Eq)]
enum Op {
    Nop,
    Inc,
    Dec,
    Gt,
    Ge,
    Lt,
    Le,
    Eq,
    Ne,
}

impl Op {
    fn from_string(value: &str) -> Op {
        use Op::*;
        match value {
            "nop" => Nop,
            "inc" => Inc,
            "dec" => Dec,
            ">" => Gt,
            ">=" => Ge,
            "<" => Lt,
            "<=" => Le,
            "==" => Eq,
            "!=" => Ne,
            &_ => Nop,
        }
    }
}

// impl fmt::Debug for Op {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         use Self::*;
//         let v = match self {
//             Inc => "inc",
//             Dec => "dec",
//             Gt => ">",
//             Ge => ">=",
//             Lt => "<",
//             Le => "<=",
//             Eq => "==",
//             Ne => "!=",
//         };
//         write!(f, v)
//     }
// }

#[derive(Debug)]
struct Operation {
    register: String,
    operation: Op,
    value: i32,
}

impl Operation {
    fn new_nop() -> Operation {
        Operation{
            register: "ignored".to_string(),
            operation: Op::Nop,
            value: 0,
        }
    }

    fn test(&self, registers: &HashMap<String, i32>) -> Option<bool> {
        // eprintln!("Testing {:?}", self);
        let op_a = *registers.get(&self.register).unwrap_or(&0);
        let op_b = self.value;
        match &self.operation {
            Op::Gt => Some(op_a > op_b),
            Op::Ge => Some(op_a >= op_b),
            Op::Lt => Some(op_a < op_b),
            Op::Le => Some(op_a <= op_b),
            Op::Eq => Some(op_a == op_b),
            Op::Ne => Some(op_a != op_b),
            &_ => None
        }
    }
}

trait OperationTrait {
    fn execute(&self, registers: &mut HashMap<String, i32>, code_ptr: usize) -> usize;
}

impl OperationTrait for Operation {
    fn execute(&self, registers: &mut HashMap<String, i32>, code_ptr: usize) -> usize {
        // eprintln!("Exec {:?}", self);
        let mut regv = *registers.get(&self.register).unwrap_or(&0);

        match &self.operation {
            Op::Inc => regv += self.value,
            Op::Dec => regv -= self.value,
            &_ => {
                return code_ptr + 1;
            }
        };

        registers.insert(self.register.clone(), regv);
        code_ptr + 1
    }
}

#[derive(Debug)]
struct CompOperation {
    operation: Operation,
    comparison: Operation,
}

impl CompOperation {
    fn new(register: &str, op: Op, value: i32) -> CompOperation {
        CompOperation{
            operation: Operation{register: register.to_string(), operation: op, value: value},
            comparison: Operation::new_nop(),
        }
    }
}

impl OperationTrait for CompOperation {
    fn execute(&self, registers: &mut HashMap<String, i32>, code_ptr: usize) -> usize {
        // eprintln!("Exc {:?}", self);
        if self.comparison.test(registers).unwrap_or(false) {
            self.operation.execute(registers, code_ptr);
        }
        code_ptr + 1
    }
}

#[derive(Debug)]
struct Program {
    registers: HashMap<String, i32>,
    code: Vec<CompOperation>,
    code_ptr: usize,
}

impl Program {
    fn new() -> Program {
        Program{
            registers: HashMap::new(),
            code: Vec::new(),
            code_ptr: 0,
        }
    }

    fn load(lines: &[String]) -> Option<Program> {
        let mut rv = Program::new();

        for l in lines {
            let entries: Vec<&str> = l.split_whitespace().collect();

            let register = entries.get(0)?;
            let op = Op::from_string(entries.get(1)?);
            let value = entries.get(2)?.parse().unwrap_or(0);

            let mut operation = CompOperation::new(register, op, value);

            if *entries.get(3).unwrap_or(&"nop") == "if" {
                // TODO could alphanum to know if it's value or reg
                let op_a = entries.get(4)?;
                let op = Op::from_string(entries.get(5)?);
                let op_b = entries.get(6)?.parse().unwrap_or(0);
                operation.comparison = Operation{
                    register: op_a.to_string(),
                    operation: op,
                    value: op_b,
                };
            } else {
                eprintln!("Straight op: {:?}", operation);
            }

            // No need to initialise, initialised as it's running
            // rv.registers.entry(register.to_string()).or_insert(0);
            rv.code.push(operation);
        }

        Some(rv)
    }

    fn execute(&mut self) -> (usize, i32) {
        let mut code_ptr: usize = 0;
        let mut i = 0;
        let mut highest_ever = 0;
        self.registers.clear();

        while code_ptr < self.code.len() {
            eprintln!("i={} code_ptr={} highest_ever={}", i, code_ptr, highest_ever);
            let op = &self.code[code_ptr];
            code_ptr = op.execute(&mut self.registers, code_ptr);

            highest_ever = highest_ever.max(*self.registers.values().max().unwrap_or(&0));
            i += 1;
        }
        (i, highest_ever)
    }
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let contents = read(&path).expect("no content");
    let mut program = Program::load(&contents).expect("failed to load");

    println!("Loaded {} lines", program.code.len());
    let (i, highest_ever) = program.execute();

    println!("Max register is {:?} (max ever {}) in {} execution",
        program.registers.values().max().unwrap_or(&0),
        highest_ever,
        i,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
}
