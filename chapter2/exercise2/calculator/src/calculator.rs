pub(crate) enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub fn calculate<T>(op: Operation, num_a: T, num_b: T) -> Result<T, &'static str>
where
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + PartialEq
        + From<u8>,
{
    match op {
        Operation::Add => Ok(num_a + num_b),
        Operation::Subtract => Ok(num_a - num_b),
        Operation::Multiply => Ok(num_a * num_b),
        Operation::Divide => {
            if num_b == T::from(0) {
                Err("Zero division is illegal!")
            } else {
                Ok(num_a / num_b)
            }
        }
    }
}
