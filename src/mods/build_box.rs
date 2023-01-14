use std::io::{self, Write};
use super::types::Lang;

pub fn build_selected_option_box(lang: &Lang, title: &str, options: &Vec<String>) -> u8 {
    match lang {
        Lang::ZhCn => {
            print!("\n[{}]\n", title);
            for option in options.into_iter().zip(1..) {
                print!("选项 {} : {}\n", option.1, option.0);
            }
            if options.len() == 0 {
                panic!("选项数量为 0\n");
            }else if options.len() == 1 {
                print!("选项数量为 1, 默认选择 1\n");
                return 1;
            }else {
                print!("输入选中的选项 (1-{}): ", options.len());
            }
            io::stdout().flush().unwrap_or_default();
            let mut selected_option_string = String::with_capacity(0);
            let mut selected_option: u8;
            let mut is_first = true;
            while {
                selected_option = selected_option_string.trim().parse::<u8>().unwrap_or(0);
                if !is_first {
                    print!("\x1b[\x01\x41\x1b[2K\x1b[1D请输入合法的选项 (1-{}): ", options.len());
                    io::stdout().flush().unwrap_or_default();
                }else{
                    is_first = false;
                }
                selected_option == 0 || selected_option > options.len() as u8
            } {
                selected_option_string = String::with_capacity(10);
                io::stdin()
                    .read_line(&mut selected_option_string)
                    .expect("");
                io::stdout().flush().unwrap_or_default();
            }
            print!("\x1b[2J\x1b[H");
            selected_option
        },
        Lang::EnUs => {
            print!("\n[{}]\n", title);
            for option in options.into_iter().zip(1..) {
                print!("Option {} : {}\n", option.1, option.0);
            }
            if options.len() == 0 {
                panic!("Option count is 0\n");
            }else if options.len() == 1 {
                print!("Option count is 1, default select 1\n");
                return 1;
            }else {
                print!("Input selected option (1-{}): ", options.len());
            }
            io::stdout().flush().unwrap_or_default();
            let mut selected_option_string = String::with_capacity(0);
            let mut selected_option: u8;
            let mut is_first = true;
            while {
                selected_option = selected_option_string.trim().parse::<u8>().unwrap_or(0);
                if !is_first {
                    print!("\x1b[\x01\x41\x1b[2K\x1b[1DPlease input a valid option (1-{}): ", options.len());
                    io::stdout().flush().unwrap_or_default();
                }else{
                    is_first = false;
                }
                selected_option == 0 || selected_option > options.len() as u8
            } {
                selected_option_string = String::with_capacity(10);
                io::stdin()
                    .read_line(&mut selected_option_string)
                    .expect("");
                io::stdout().flush().unwrap_or_default();
            }
            print!("\x1b[2J\x1b[H");
            selected_option
        }
    }
}

pub fn build_input_box(lang: &Lang, title: &str, default: &str) -> String {
    match lang {
        Lang::ZhCn => {
            print!("[{}]\n", title);
            print!("输入 (默认 {}): ", default);
            io::stdout().flush().unwrap_or_default();
            let mut input_string = String::with_capacity(100);
            io::stdin()
                .read_line(&mut input_string)
                .expect("");
            print!("\x1b[2J\x1b[H");
            io::stdout().flush().unwrap_or_default();
            if input_string.trim() == "" {
                default.to_string()
            }else {
                input_string.trim().to_string()
            }
        },
        Lang::EnUs => {
            print!("[{}]\n", title);
            print!("Input (default {}): ", default);
            io::stdout().flush().unwrap_or_default();
            let mut input_string = String::with_capacity(100);
            io::stdin()
                .read_line(&mut input_string)
                .expect("");
            print!("\x1b[2J\x1b[H");
            io::stdout().flush().unwrap_or_default();
            if input_string.trim() == "" {
                default.to_string()
            }else {
                input_string.trim().to_string()
            }
        }
    }
}