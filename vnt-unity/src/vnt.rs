use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::ptr::null;
use std::str::FromStr;
use anyhow::anyhow;

use common::args_parse::{ips_parse, out_ips_parse};
use vnt::channel::punch::PunchModel;
use vnt::channel::UseChannelType;
use vnt::cipher::CipherModel;
use vnt::compression::Compressor;
use vnt::core::{Config, Vnt};
use getopts::Options;



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
pub fn parse_command_str(command_str: &str) -> Option<Vec<(&str, String)>> {
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
        let command_str = "-k 123456 -s shf1.pop.teledriving.com:29872 --tcp --use-channel relay -d GCA9056a";
        if let Some(parsed_command) = parse_command_str(command_str) {
            for (opt, arg) in parsed_command {
                println!("Option: {}, Argument: {}", opt, arg);
            }
        }
    }
}




// example one
#[no_mangle]
pub extern "C" fn my_add(x: i32, y: i32) -> i32 {
    x + y
}

// example one
pub struct  CommandResult{
    pub result_code: i16,
    pub message:String
}
impl CommandResult {
    pub fn new() -> Self {
        CommandResult {
            result_code: 0,
            message: String::new(),
        }
    }

    pub fn set_result_code(&mut self, code: i16) {
        self.result_code = code;
    }

    pub fn set_message(&mut self, msg: String) {
        self.message = msg;
    }
}
#[no_mangle]
pub extern "C" fn parse_command_line(command_str: &str) -> CommandResult {
    let mut result=CommandResult::new ();
    let mut opts = Options::new();
    opts.optopt("k", "", "组网标识", "<token>");
    opts.optopt("n", "", "设备名称", "<name>");
    opts.optopt("d", "", "设备标识", "<id>");
    opts.optflag("c", "", "关闭交互式命令");
    opts.optopt("s", "", "注册和中继服务器地址", "<server>");
    opts.optmulti("e", "", "stun服务器", "<stun-server>");
    opts.optflag("a", "", "使用tap模式");
    opts.optopt("", "nic", "虚拟网卡名称,windows下使用tap则必填", "<tun0>");
    opts.optmulti("i", "", "配置点对网(IP代理)入站时使用", "<in-ip>");
    opts.optmulti("o", "", "配置点对网出站时使用", "<out-ip>");
    opts.optopt("w", "", "客户端加密", "<password>");
    opts.optflag("W", "", "服务端加密");
    opts.optopt("u", "", "自定义mtu(默认为1430)", "<mtu>");
    opts.optflag("", "tcp", "tcp");
    opts.optopt("", "ip", "指定虚拟ip", "<ip>");
    opts.optflag("", "relay", "仅使用服务器转发");
    opts.optopt("", "par", "任务并行度(必须为正整数)", "<parallel>");
    opts.optopt("", "model", "加密模式", "<model>");
    opts.optflag("", "finger", "指纹校验");
    opts.optopt("", "punch", "取值ipv4/ipv6", "<punch>");
    opts.optopt("", "ports", "监听的端口", "<port,port>");
    opts.optflag("", "cmd", "开启窗口输入");
    opts.optflag("", "no-proxy", "关闭内置代理");
    opts.optflag("", "first-latency", "优先延迟");
    opts.optopt("", "use-channel", "使用通道 relay/p2p", "<use-channel>");
    opts.optopt("", "packet-loss", "丢包率", "<packet-loss>");
    opts.optopt("", "packet-delay", "延迟", "<packet-delay>");
    opts.optmulti("", "dns", "dns", "<dns>");
    opts.optmulti("", "mapping", "mapping", "<mapping>");
    opts.optopt("f", "", "配置文件", "<conf>");
    opts.optopt("", "compressor", "压缩算法", "<lz4>");
    opts.optflag("", "list", "后台运行时,查看其他设备列表");
    opts.optflag("", "all", "后台运行时,查看其他设备完整信息");
    opts.optflag("", "info", "后台运行时,查看当前设备信息");
    opts.optflag("", "route", "后台运行时,查看数据转发路径");
    opts.optflag("", "stop", "停止后台运行");
    opts.optflag("h", "help", "帮助");

    let args: Vec<String> = command_str.split_whitespace().map(String::from).collect();


    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_) => {
            result.set_result_code(-1);
            result.set_message("参数为空或参数不足".to_string());
            return result;
        }
    };
    if matches.opt_present("h") || args.len() == 1 {
        result.set_result_code(-1);
        result.set_message("参数不需要有h".to_string());
        return result;
    }
    if !matches.opt_present("k") {
        result.set_result_code(-1);
        result.set_message("参数中没有-k".to_string());
        return result;
    }

    let tap = matches.opt_present("a");
    let device_name = matches.opt_str("nic");
    let token: String = matches.opt_get("k").unwrap().unwrap();
    let device_id = matches.opt_get_default("d", String::new()).unwrap();

    if device_id.is_empty() {
        result.set_result_code(-1);
        result.set_message("参数中没有device_id".to_string());
        return result;
    }


    let name = matches
        .opt_get_default("n", "Windows".to_string())
        .unwrap();
    let server_address_str = matches
        .opt_get_default("s", "nat1.wherewego.top:29872".to_string())
        .unwrap();
    let mut stun_server = matches.opt_strs("e");
    if stun_server.is_empty() {
        stun_server.push("stun1.l.google.com:19302".to_string());
        stun_server.push("stun2.l.google.com:19302".to_string());
        stun_server.push("stun.miwifi.com:3478".to_string());
    }
    let dns = matches.opt_strs("dns");
    let in_ip = matches.opt_strs("i");
    let in_ip = match ips_parse(&in_ip) {
        Ok(in_ip) => in_ip,
        Err(e) => {
            result.set_result_code(-1);
            result.set_message("参数-i设置错误，example: -i 192.168.0.0/24,10.26.0.3".to_string());
            return result;
        }
    };
    let out_ip = matches.opt_strs("o");
    let out_ip = match out_ips_parse(&out_ip) {
        Ok(out_ip) => out_ip,
        Err(e) => {
            result.set_result_code(-1);
            result.set_message("参数-o设置错误， example: -o 0.0.0.0/0".to_string());
            return result;
        }
    };
    let password: Option<String> = matches.opt_get("w").unwrap();
    let server_encrypt = matches.opt_present("W");
    #[cfg(not(feature = "server_encrypt"))]
    {
        if server_encrypt {
            result.set_result_code(-1);
            result.set_message("Server encryption not supported".to_string());

            return result;
        }
    }
    let mtu: Option<String> = matches.opt_get("u").unwrap();
    let mtu = if let Some(mtu) = mtu {
        match u32::from_str(&mtu) {
            Ok(mtu) => Some(mtu),
            Err(e) => {
                result.set_result_code(-1);
                result.set_message("参数mtu设置错误".to_string());

                return result;
            }
        }
    } else {
        None
    };
    let virtual_ip: Option<String> = matches.opt_get("ip").unwrap();
    let virtual_ip =
        virtual_ip.map(|v| Ipv4Addr::from_str(&v).expect(&format!("'--ip {}' error", v)));
    if let Some(virtual_ip) = virtual_ip {
        if virtual_ip.is_unspecified() || virtual_ip.is_broadcast() || virtual_ip.is_multicast()
        {
            result.set_result_code(-1);
            result.set_message(" 参数--ip 设置错误".to_string());
            return result;
        }
    }
    let tcp_channel = matches.opt_present("tcp");
    let relay = matches.opt_present("relay");

    let parallel = matches.opt_get::<usize>("par").unwrap().unwrap_or(1);
    if parallel == 0 {
        result.set_result_code(-1);
        result.set_message(" 参数--par 设置错误".to_string());
        return result;
    }
    let cipher_model = match matches.opt_get::<CipherModel>("model") {
        Ok(model) => {
            #[cfg(not(any(feature = "aes_gcm", feature = "server_encrypt")))]
            {
                if password.is_some() && model.is_none() {
                    result.set_result_code(-1);
                    result.set_message(" 参数--model 设置错误".to_string());
                    return result;
                }
                model.unwrap_or(CipherModel::None)
            }
            #[cfg(any(feature = "aes_gcm", feature = "server_encrypt"))]
            model.unwrap_or(CipherModel::AesGcm)
        }
        Err(e) => {
            result.set_result_code(-1);
            result.set_message(" 参数--model 设置错误".to_string());
            return result;
        }
    };

    let finger = matches.opt_present("finger");
    let punch_model = matches
        .opt_get::<PunchModel>("punch")
        .unwrap()
        .unwrap_or(PunchModel::All);
    let use_channel_type = matches
        .opt_get::<UseChannelType>("use-channel")
        .unwrap()
        .unwrap_or_else(|| {
            if relay {
                UseChannelType::Relay
            } else {
                UseChannelType::All
            }
        });

    let ports = matches
        .opt_get::<String>("ports")
        .unwrap_or(None)
        .map(|v| v.split(",").map(|x| x.parse().unwrap_or(0)).collect());

    let cmd = matches.opt_present("cmd");
    #[cfg(feature = "ip_proxy")]
    let no_proxy = matches.opt_present("no-proxy");
    let first_latency = matches.opt_present("first-latency");
    let packet_loss = matches
        .opt_get::<f64>("packet-loss")
        .expect("--packet-loss");
    let packet_delay = matches
        .opt_get::<u32>("packet-delay")
        .expect("--packet-delay")
        .unwrap_or(0);
    #[cfg(feature = "port_mapping")]
    let port_mapping_list = matches.opt_strs("mapping");
    let compressor = if let Some(compressor) = matches.opt_str("compressor").as_ref() {
        Compressor::from_str(compressor)
            .map_err(|e| anyhow!("{}", e))
            .unwrap()
    } else {
        Compressor::None
    };
    let config = match Config::new(
        #[cfg(target_os = "windows")]
        tap,
        token,
        device_id,
        name,
        server_address_str,
        dns,
        stun_server,
        in_ip,
        out_ip,
        password,
        mtu,
        tcp_channel,
        virtual_ip,
        #[cfg(feature = "ip_proxy")]
        no_proxy,
        server_encrypt,
        parallel,
        cipher_model,
        finger,
        punch_model,
        ports,
        first_latency,
        device_name,
        use_channel_type,
        packet_loss,
        packet_delay,
        #[cfg(feature = "port_mapping")]
        port_mapping_list,
        compressor,
    )
    {
        Ok(config) => config,
        Err(e) => {
            result.set_result_code(-1);
            result.set_message(" config.toml error".to_string());
            return result;
        }
    };

    #[cfg(feature = "port_mapping")]
    for (is_tcp, addr, dest) in config.port_mapping_list.iter() {
        if *is_tcp {
            println!("TCP port mapping {}->{}", addr, dest)
        } else {
            println!("UDP port mapping {}->{}", addr, dest)
        }
    }


    result

}
mod callback;


fn start(config:Config,_show_cmd:bool)
{
    let vnt_util = Vnt::new(config, callback::VntHandler {}).unwrap();


    vnt_util.wait()

}
#[test]
fn test_parse_command_line() {
    let command_str = "-k 123456 -s shf1.pop.teledriving.com:29872 --tcp --use-channel relay -d GCA9056a --cmd";
    let result = parse_command_line(command_str);
    println!("{}{}", result.result_code, result.message);

}