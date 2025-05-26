use std::path::Path;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name="rscli", version, author, about, long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "show csv or convert csv to other format")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser=verify_input_file)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    header: bool,
}

/// 验证输入文件是否存在
///
/// # 参数
/// * `file_name` - 要验证的文件路径字符串
///
/// # 返回值
/// 返回Result类型:
/// * Ok(String) - 如果文件存在，返回文件路径字符串
/// * Err(String) - 如果文件不存在，返回错误信息
fn verify_input_file(file_name: &str) -> anyhow::Result<String> {
    if Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err(anyhow::anyhow!("File does not exist"))
    }
}
