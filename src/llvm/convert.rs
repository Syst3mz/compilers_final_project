use crate::llvm::counters::Counters;
use crate::llvm::LLVM;

pub trait LLVMConvert {
    fn convert(self, counters: &mut Counters) -> LLVM;
}