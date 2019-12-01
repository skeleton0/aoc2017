pub enum RegisterOperation {
    Increase,
    Decrease,
}

pub enum ConditionOperator {
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterOrEqual,
    LessOrEqual,
}

pub struct Instruction {
    pub operation_register: String,
    pub operation_type: RegisterOperation,
    pub operation_value: i32,

    pub condition_register: String,
    pub condition_type: ConditionOperator, 
    pub condition_value: i32,
}

impl Instruction {
    pub fn from_msg(msg: &str) -> Instruction {
        let mut iter = msg.split_whitespace();

        let operation_register = iter.next().expect("Failed to extract operation_register from msg.");
        let operation_register = String::from(operation_register);
        
        let operation_type = iter.next().expect("Failed to extract operation_type from msg.");
        let operation_type = match operation_type {
            "inc" => RegisterOperation::Increase,
            "dec" => RegisterOperation::Decrease,
            _ => panic!("Failed to parse operation_type from msg."),
        };

        let operation_value = iter.next().expect("Failed to extract operation_value from msg.");
        let operation_value: i32 = operation_value.parse().expect("Failed to parse operation_value to i32."); 

        //skip 'if' keyword
        iter.next().expect("Failed to find 'if' keyword in msg.");

        let condition_register = iter.next().expect("Failed to extract condition_register from msg.");
        let condition_register = String::from(condition_register);
        
        let condition_type = iter.next().expect("Failed to extract condition_type from msg.");
        let condition_type = match condition_type {
            "==" => ConditionOperator::Equal,
            "!=" => ConditionOperator::NotEqual,
            ">"  => ConditionOperator::Greater,
            "<"  => ConditionOperator::Less,
            ">=" => ConditionOperator::GreaterOrEqual,
            "<=" => ConditionOperator::LessOrEqual,
            _    => panic!("Failed to parse condition_type from msg."), 
        };

        let condition_value = iter.next().expect("Failed to extract condition_value from msg.");
        let condition_value: i32 = condition_value.parse().expect("Failed to parse condition_value to i32");

        Instruction {
            operation_register,
            operation_type,
            operation_value,

            condition_register,
            condition_type,
            condition_value,
        }
    }
}