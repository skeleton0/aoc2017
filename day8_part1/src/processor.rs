use std::collections::HashMap;
use instruction::*;

pub struct Processor {
    registers: HashMap<String, i32>,
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            registers: HashMap::new(),
        }
    }

    fn evaluate_condition(&mut self, instr: &Instruction) -> bool {
        let condition_register_val = self.registers.entry(instr.condition_register.clone()).or_insert(0);
        let condition_register_val = *condition_register_val;
        let condition_val = instr.condition_value;

        match instr.condition_type {
            ConditionOperator::Equal          => condition_register_val == condition_val,
            ConditionOperator::NotEqual       => condition_register_val != condition_val,
            ConditionOperator::Greater        => condition_register_val >  condition_val,
            ConditionOperator::Less           => condition_register_val <  condition_val,
            ConditionOperator::GreaterOrEqual => condition_register_val >= condition_val,
            ConditionOperator::LessOrEqual    => condition_register_val <= condition_val,
        }
    }

    pub fn execute_instruction(&mut self, instr: &Instruction) {
        if self.evaluate_condition(instr) {
            let operation_register_val = self.registers.entry(instr.operation_register.clone()).or_insert(0);
            let operation_val = instr.operation_value;

            match instr.operation_type {
                RegisterOperation::Increase => *operation_register_val += operation_val,
                RegisterOperation::Decrease => *operation_register_val -= operation_val,
            }
        }
    }

    pub fn largest_register_value(&self) -> Option<i32> {
        let mut largest_value = None;

        for (_, register_val) in &self.registers {
            let register_val = *register_val;
            
            match largest_value {
                Some(val) => { 
                    if register_val > val {
                        largest_value = Some(register_val);
                    }
                },
                None        => largest_value = Some(register_val),
            }
        }

        largest_value
    }
}