use std::fs::File;
use std::io::{self, Read};
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 || args[1] == "--version" {
        println!("Oddscript version {}",env!("CARGO_PKG_VERSION"));
        println!("Run odd ? for help");
        return Ok(());
    }
    if args[1] == "?" {
        println!("Oddscript Command Syntax:");
        println!("odd {{filename}}");
        return Ok(());
    }
    let filename = &args[1];

    let mut containers: [u8; 256] = [0; 256];
    let mut pointer: usize = 0;
    let mut loopstart = 256;

    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut line_index = 0;
    let lines: Vec<&str> = contents.lines().collect();

    while line_index < lines.len() {
        let line = lines[line_index].trim();

        line_index += 1; // Default increment

        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line == "!" {
            break;
        }

        let clean: String = line.chars().filter(|c| !c.is_whitespace()).collect();
        if clean.is_empty() {
            continue;
        }

        let op = clean.chars().next().unwrap();
        let value = clean.get(1..).and_then(|v| v.parse::<u8>().ok());

        match op {
            '~' => {
                let mut input = String::new();
                loop {
                    input.clear();
                    io::stdin().read_line(&mut input).unwrap();
                    let input_trimmed = input.trim();
                    if let Ok(n) = input_trimmed.parse::<u8>() {
                        containers[pointer] = n;
                        break;
                    } else {
                        println!("Type a number < 256");
                    }
                }
            }

            '<' => println!("{}", containers[pointer]),

            '.' => {
                if loopstart != 256 {
                    line_index = loopstart - 1;
                }
            }

            '>' => {
                if let Some(v) = value {
                    pointer = v as usize;
                }
            }

            '=' => {
                if let Some(v) = value {
                    containers[pointer] = v;
                }
            }

            '+' => {
                if let Some(v) = value {
                    containers[pointer] += containers[v as usize];
                }
            }

            '-' => {
                if let Some(v) = value {
                    containers[pointer] -= containers[v as usize];
                }
            }

            '*' => {
                if let Some(v) = value {
                        containers[pointer] *= containers[v as usize];
                }
            }

            '/' => {
                if let Some(v) = value {
                    containers[pointer] /= containers[v as usize];
                }
            }

            '?' => {
                
                if let Some(v) = value {
                    if containers[pointer] != containers[v as usize] {
                        loopstart = line_index;
                    } else {
                        if loopstart != line_index {
                            let index_check = line_index + 1;
                            while index_check < lines.len() {
                                if lines[index_check].chars().filter(|c| !c.is_whitespace()).collect::<String>() == r"." {
                                    line_index = index_check;
                                    break;
                                }
                            }
                            
                        }
                        loopstart = 256;
                    }
                }
            }
            's' => {
                if let Some(c) = clean.get(1..) {
                    let indexes = c.split(",");
                    let mut output = String::new();
                    for i in indexes {
                        if let Ok(parsed) = i.parse::<u8>() {
                            // parsed is now a u8
                            output.push(containers[parsed as usize] as char);
                        } else {
                            // handle parse error if needed
                            
                        }
                    }
                    println!("{}",output);
                }
            }

            _ => {
                eprintln!("Error: Syntactically Incorrect Line at {}", line_index);
                break;
            }
        }
    }

    Ok(())
}
