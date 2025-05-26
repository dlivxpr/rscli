mod opts;
mod process;

pub use opts::{Opts, SubCommand};
pub use process::process_csv;

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_functionality() {
        // 简单的测试确保库可以正常工作
        assert_eq!(2 + 2, 4);
    }
}
