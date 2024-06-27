


use vnt::channel::punch::PunchModel;
use vnt::channel::UseChannelType;
use vnt::cipher::CipherModel;
use vnt::compression::Compressor;
use vnt::core::{Config, Vnt};

// 命令选项结构体，用于识别短选项和长选项
struct CommandOption {
    short_opt: &'static str,
    long_opt: &'static str,
}

// 命令选项表
const COMMAND_OPTIONS: [CommandOption; 21] = [
    CommandOption { short_opt: "-k", long_opt: "" },
    CommandOption { short_opt: "-n", long_opt: "" },
    CommandOption { short_opt: "-d", long_opt: "" },
    CommandOption { short_opt: "-c", long_opt: "" },
    CommandOption { short_opt: "-s", long_opt: "" },
    CommandOption { short_opt: "-e", long_opt: "" },
    CommandOption { short_opt: "-a", long_opt: "" },
    CommandOption { short_opt: "", long_opt: "--nic" },
    CommandOption { short_opt: "-i", long_opt: "" },
    CommandOption { short_opt: "-o", long_opt: "" },
    CommandOption { short_opt: "-w", long_opt: "" },
    CommandOption { short_opt: "-W", long_opt: "" },
    CommandOption { short_opt: "-u", long_opt: "" },
    CommandOption { short_opt: "", long_opt: "--tcp" },
    CommandOption { short_opt: "", long_opt: "--ip" },
    CommandOption { short_opt: "", long_opt: "--relay" },
    CommandOption { short_opt: "", long_opt: "--par" },
    CommandOption { short_opt: "", long_opt: "--model" },
    CommandOption { short_opt: "", long_opt: "--finger" },
    CommandOption { short_opt: "", long_opt: "--punch" },
    CommandOption { short_opt: "", long_opt: "--ports" },
];

// 解析命令字符串的函数
pub fn main_logic(command_str: &str) -> Option<Vec<(&str, String)>> {
    let mut result = Vec::new();
    let mut iter = command_str.split_whitespace().peekable();

    while let Some(part) = iter.next() {
        if let Some(option) = COMMAND_OPTIONS.iter().find(|opt| part.starts_with(opt.short_opt) || part.starts_with(opt.long_opt)) {
            let mut arg = String::new();

            // 将当前part作为选项的标识符
            let current_option = if part.starts_with('-') {
                part
            } else {
                continue; // 跳过参数部分，因为我们关心的是选项部分
            };

            // 如果下一个部分是参数，则将其添加到arg中
            while let Some(next) = iter.peek() {
                if next.starts_with('-') {
                    break;
                }
                arg.push_str(iter.next().unwrap());
                arg.push(' ');
            }
            // 去掉最后一个空格
            arg = arg.trim().to_string();

            result.push((current_option, arg));
        }
    }

    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_logic() {
        let command_str = "-k 123456 -s lvzhiwen1126.xyz:29872 --tcp --use-channel relay -d GCA9056a";
        if let Some(parsed_command) = main_logic(command_str) {
            for (opt, arg) in parsed_command {
                println!("Option: {}, Argument: {}", opt, arg);
            }
        }
    }
}


