use crate::calculator::CalculatorError;

pub fn error_message_text(error: &CalculatorError) -> &'static str {
    match error {
        CalculatorError::NegativeFuel => "燃料量不能为负数",
        CalculatorError::MissingOxidizer => "缺少氧化剂",
        CalculatorError::NegativeOxidizer => "氧化剂量不能为负数",
        CalculatorError::FuelExceedsCapacity => "燃料超过容量",
        CalculatorError::OxidizerExceedsCapacity => "氧化剂超过容量",
        CalculatorError::IncompatibleOxidizerTank => "氧化剂舱类型不兼容",
        CalculatorError::RocketExceedsMaxHeight => "火箭超过最大高度",
        CalculatorError::CommandModuleTooLess => "指挥舱数量不能为0",
        CalculatorError::CommandModuleTooMuch => "指挥舱数量不能多于1个",
        CalculatorError::MissingNosecone => "前锥数量不能为0",
        CalculatorError::MultipleNosecones => "前锥数量不能多于1个",
    }
}
