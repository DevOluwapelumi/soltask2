///-------------------------------------------------------------------------------
///
/// This is your calculator implementation task 
/// to practice enums, structs, and methods.
/// 
/// Complete the implementation of the Calculator struct and its methods.
/// 
/// The calculator should support basic arithmetic 
/// operations (addition, subtraction, multiplication)
/// with overflow protection and maintain a history 
/// of operations.
/// 
/// Tasks:
/// 1. Implement the OperationType enum methods
/// 2. Implement the Operation struct constructor
/// 3. Implement all Calculator methods
/// 
///-------------------------------------------------------------------------------



#[derive(Clone)]
pub enum OperationType {
    Addition,
    Subtraction,
    Multiplication,
}

impl OperationType {
    // Return the symbol for each operation
    pub fn get_sign(&self) -> &str {
        match self {
            OperationType::Addition => "+",
            OperationType::Subtraction => "-",
            OperationType::Multiplication => "*",
        }
    }

    // Perform the operation with overflow checking
    pub fn perform(&self, x: i64, y: i64) -> Option<i64> {
        match self {
            OperationType::Addition => x.checked_add(y),
            OperationType::Subtraction => x.checked_sub(y),
            OperationType::Multiplication => x.checked_mul(y),
        }
    }
}

#[derive(Clone)]
pub struct Operation {
    pub first_num: i64,
    pub second_num: i64,
    pub operation_type: OperationType,
}

impl Operation {
    pub fn new(first_num: i64, second_num: i64, operation_type: OperationType) -> Self {
        Operation {
            first_num,
            second_num,
            operation_type,
        }
    }
}

pub struct Calculator {
    pub history: Vec<Operation>,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            history: Vec::new(),
        }
    }

    pub fn addition(&mut self, x: i64, y: i64) -> Option<i64> {
        let operation_type = OperationType::Addition;
        let result = operation_type.perform(x, y)?;
        let op = Operation::new(x, y, operation_type);
        self.history.push(op);
        Some(result)
    }

    pub fn subtraction(&mut self, x: i64, y: i64) -> Option<i64> {
        let operation_type = OperationType::Subtraction;
        let result = operation_type.perform(x, y)?;
        let op = Operation::new(x, y, operation_type);
        self.history.push(op);
        Some(result)
    }

    pub fn multiplication(&mut self, x: i64, y: i64) -> Option<i64> {
        let operation_type = OperationType::Multiplication;
        let result = operation_type.perform(x, y)?;
        let op = Operation::new(x, y, operation_type);
        self.history.push(op);
        Some(result)
    }

    pub fn show_history(&self) -> String {
        let mut history_string = String::new();
        for (i, operation) in self.history.iter().enumerate() {
            if let Some(result) = operation.operation_type.perform(operation.first_num, operation.second_num) {
                let line = format!(
                    "{}: {} {} {} = {}\n",
                    i,
                    operation.first_num,
                    operation.operation_type.get_sign(),
                    operation.second_num,
                    result
                );
                history_string.push_str(&line);
            }
        }
        history_string
    }

    pub fn repeat(&mut self, operation_index: usize) -> Option<i64> {
        if let Some(operation) = self.history.get(operation_index).cloned() {
            let result = operation.operation_type.perform(operation.first_num, operation.second_num)?;
            self.history.push(operation); // Add repeated operation to history
            Some(result)
        } else {
            None
        }
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}
