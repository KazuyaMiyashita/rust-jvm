use crate::class_file::structure::attribute::*;
use crate::class_file::structure::attribute::stack_map_table::*;
use super::padding;
use std::fmt;

impl fmt::Display for StackMapTableAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attribute::Code(stack_map_table::StackMapTableAttribute {{\n")?;
        write!(f, "    attribute_name_index: {},\n", self.attribute_name_index)?;
        write!(f, "    attribute_length: {},\n", self.attribute_length)?;
        write!(f, "    number_of_entries: {},\n", self.number_of_entries)?;
        write!(f, "    entries: vec![\n")?;
        self.entries.iter().try_for_each(|entries| {
            write!(f, "{},\n", padding(entries.to_string(), 8))
        })?;
        write!(f, "    ]\n")?;
        write!(f, "}}")?;

        Ok(())
    }
}

impl fmt::Display for StackMapFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            StackMapFrame::SameFrame { frame_type } => {
                format!("stack_map_table::StackMapFrame::SameFrame {{ frame_type: {} }}", frame_type)
            }
            StackMapFrame::SameLocals1StackItemFrame { frame_type, stack } => {
                let stack_str: String = stack.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
                format!("stack_map_table::StackMapFrame::SameLocals1StackItemFrame {{ frame_type: {}, stack: vec![{}] }}", frame_type, stack_str)
            }
            StackMapFrame::SameLocals1StackItemFrameExtended { frame_type, offset_delta, stack } => {
                let stack_str: String = stack.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
                format!("stack_map_table::StackMapFrame::SameLocals1StackItemFrameExtended {{ frame_type: {}, offset_delta: {}, stack: vec![{}] }}", frame_type, offset_delta, stack_str)
            }
            StackMapFrame::ChopFrame { frame_type, offset_delta } => {
                format!("stack_map_table::StackMapFrame::ChopFrame {{ frame_type: {}, offset_delta: {} }}", frame_type, offset_delta)
            }
            StackMapFrame::SameFrameExtended { frame_type, offset_delta } => {
                format!("stack_map_table::StackMapFrame::SameFrameExtended {{ frame_type: {}, offset_delta: {} }}", frame_type, offset_delta)
            }
            StackMapFrame::AppendFrame { frame_type, offset_delta, locals } => {
                let locals_str: String = locals.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
                format!("stack_map_table::StackMapFrame::AppendFrame {{ frame_type: {}, offset_delta: {}, locals: vec![{}] }}", frame_type, offset_delta, locals_str)
            }
            StackMapFrame::FullFrame { frame_type, offset_delta, number_of_locals, locals, number_of_stack_items, stack } => {
                let locals_str: String = locals.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
                let stack_str: String = stack.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
                format!("stack_map_table::StackMapFrame::FullFrame {{ frame_type: {}, offset_delta: {}, number_of_locals: {}, locals: vec![{}], number_of_stack_items: {}, stack_str: vec![{}] }}", frame_type, offset_delta, number_of_locals, locals_str, number_of_stack_items, stack_str)
            }
        };
        write!(f, "{}", str)
    }
}

impl fmt::Display for VerificationTypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            VerificationTypeInfo::TopVariableInfo { tag } => {
                format!("stack_map_table::VerificationTypeInfo::TopVariableInfo {{ tag: {} }}", tag)
            }
            VerificationTypeInfo::IntegerVariableInfo { tag } => {
                format!("stack_map_table::VerificationTypeInfo::IntegerVariableInfo {{ tag: {} }}", tag)
            }
            VerificationTypeInfo::FloatVariableInfo { tag } => {
                format!("stack_map_table::VerificationTypeInfo::FloatVariableInfo {{ tag: {} }}", tag)
            }
            VerificationTypeInfo::DoubleVariableInfo { tag } => {
                format!("stack_map_table::VerificationTypeInfo::DoubleVariableInfo {{ tag: {} }}", tag)
            }
            VerificationTypeInfo::LongVariableInfo { tag } => {
                format!("stack_map_table::VerificationTypeInfo::LongVariableInfo {{ tag: {} }}", tag)
            }
            VerificationTypeInfo::NullVariableInfo { tag } => {
                format!("stack_map_table::VerificationTypeInfo::NullVariableInfo {{ tag: {} }}", tag)
            }
            VerificationTypeInfo::UninitializedThisVariableInfo { tag } => {
                format!("stack_map_table::VerificationTypeInfo::UninitializedThisVariableInfo {{ tag: {} }}", tag)
            }
            VerificationTypeInfo::ObjectVariableInfo { tag, cpool_index } => {
                format!("stack_map_table::VerificationTypeInfo::ObjectVariableInfo {{ tag: {}, cpool_index: {} }}", tag, cpool_index)
            }
            VerificationTypeInfo::UninitializedVariableInfo { tag, offset } => {
                format!("stack_map_table::VerificationTypeInfo::UninitializedVariableInfo {{ tag: {}, offset: {} }}", tag, offset)
            }
        };
        write!(f, "{}", str)
    }
}
