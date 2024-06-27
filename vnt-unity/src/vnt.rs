


use vnt::channel::punch::PunchModel;
use vnt::channel::UseChannelType;
use vnt::cipher::CipherModel;
use vnt::compression::Compressor;
use vnt::core::{Config, Vnt};




pub fn main_logic(command_str: &str) -> Option<Vec<(&str, &str, &str, Option<&str>)>> {
    let mut result = Vec::new();
    let mut current_option = None;

    let mut iter = command_str.split_whitespace().peekable();

    while let Some(option) = iter.next() {
        if !option.starts_with('-') {
            continue; // Skip non-option parts
        }

        let long_opt = iter.peek().unwrap_or(&"");
        let desc = iter.peek().unwrap_or(&"");
        let arg = iter.peek().unwrap_or(&"");

        current_option = Some((option, long_opt, desc, arg));
        iter.next(); // Consume the next element
    }

    if let Some((opt, long_opt, desc, arg)) = current_option {
        result.push((opt, long_opt, desc, arg));
    }

    if result.is_empty() {
        None
    } else {
        Some(result)
    }

}


fn main()
{
    let command_str = "-k 123456 -s lvzhiwen1126.xyz:29872 --tcp --use-channel relay -d GCA9056a";
    if let Some(parsed_command) = main_logic(command_str) {
        for (opt, long_opt, desc, arg) in parsed_command {
            println!("Option: {}, Long Opt: {}, Description: {}, Argument: {:?}", opt, long_opt, desc, arg);
        }
    }
}
