use std::io::stdin;

use ockamd::cli;

fn main() {
    let args = cli::Args::parse();

    match args.exec_mode() {
        cli::Mode::Server => {
            // read stdin for messages to encrypt and route
            let input = stdin();
            let mut buf = String::new();

            loop {
                match input.read_line(&mut buf) {
                    Ok(n) => {
                        if n > 0 {
                            print!("ockamd: {}", buf);
                            buf.clear();
                        }
                    }
                    Err(e) => {
                        println!("error: {:?}", e);
                    }
                }
            }
        }
        cli::Mode::Control => unimplemented!(),
    }
}
