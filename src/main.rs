extern crate getopts;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;

#[derive(Debug, Default)]
struct Args
{
    pub from: Option<String>,
    pub to: Option<String>,
    pub input: Option<String>,
    pub output: Option<String>,
    pub help: bool
}
#[derive(PartialEq)]
enum NewlineChracter {
    CrLf,
    Lf,
    Cr,
}

const CR_VALUE: u8 = 0x0d;
const LF_VALUE: u8 = 0x0a;

fn create_options() -> getopts::Options {
    let mut opts = getopts::Options::new();
    opts.optopt("f", "from", "input newline type,possible values: 'crlf'(default), 'cr', 'lf'", "[newline type]");
    opts.optopt("t", "to", "output newline type,possible values: 'crlf'(default), 'cr', 'lf'", "[newline type]");
    opts.optopt("i", "input", "input file path", "default: stdin");
    opts.optopt("o", "output", "output file path", "default: stdout");
    opts.optflag("h", "help", "output usage");
    opts
}

fn parse_args() -> Result<Args, getopts::Fail> {
    let arg = env::args();
    let opts = create_options();
    let matches  = opts.parse(arg)?;
    return Ok(Args {
        help: matches.opt_present("h"),
        from: matches.opt_str("f"),
        to: matches.opt_str("t"),
        input: matches.opt_str("i"),
        output: matches.opt_str("o")
    });
}

fn parse_newline(data: &[u8]) -> Option<NewlineChracter> {
    if data[0] == CR_VALUE {
        // if CRLF
        if data.len() >= 2 && data[1] == LF_VALUE {
            return Some(NewlineChracter::CrLf);
        }else{
            return Some(NewlineChracter::Cr);
        }
    }
    else if data[0] == LF_VALUE {
        return Some(NewlineChracter::Lf);
    }
    return None
}

fn do_main(args: Args) -> std::io::Result<()> {
    let stdin;
    let mut file;
    let mut stdin_lock;
    let input = match args.input {
        Some(v) => {
            match std::fs::File::open(v) {
                Ok(f) => {
                    file = f;
                    Ok(&mut file as &mut std::io::Read)
                },
                Err(e) => {
                    Err(e)
                }
            }
        },
        None => {
            stdin = std::io::stdin();
            stdin_lock = stdin.lock();
            Ok(&mut stdin_lock as &mut std::io::Read)
        }
    }?;
    let stdout;
    let mut ofile;
    let mut stdout_lock;
    let output = match args.output {
        Some(v) => {
            match std::fs::File::create(v) {
                Ok(f) => {
                    ofile = f;
                    Ok(&mut ofile as &mut std::io::Write)
                },
                Err(e) => {
                    Err(e)
                }
            }
        },
        None => {
            stdout = std::io::stdout();
            stdout_lock = stdout.lock();
            Ok(&mut stdout_lock as &mut std::io::Write)
        }
    }?;
    let fromchar = get_crlf_chars(&args.from);
    let tochar = get_crlf_chars(&args.to);
    let tostring = String::from(tochar.get_crlf_string());
    let mut buf = [0u8;2048];
    let mut offset = 0;
    loop {
        let bytesread = input.read(&mut buf[offset..])?;
        if bytesread == 0 {
            if offset > 0 {
                // output remaining cr value
                output.write(&[CR_VALUE])?;
            }
            break;
        }
        offset = 0;
        let mut shouldskip = false;
        for i in 0usize..bytesread {
            if shouldskip {
                info!("skipping");
                shouldskip = false;
                continue;
            }
            match parse_newline(&buf[i..]) {
                Some(v) => {
                    if v == NewlineChracter::CrLf {
                        info!("crlf detected");
                        shouldskip = true;
                    }
                    if v == fromchar {
                        info!("replace");
                        output.write(tostring.as_bytes())?;
                    } else if v == NewlineChracter::Cr && fromchar == NewlineChracter::CrLf && i + 1 == bytesread {
                        offset = 1;
                        buf[0] = CR_VALUE;
                        info!("stay");
                        break;
                    } else {
                        info!("write value");
                        output.write(&buf[i..i + 1])?;
                    }
                },
                None => {
                    info!("passthrough");
                    output.write(&buf[i..i + 1])?;
                }
            };
        }
    }
    Ok(())
}

impl NewlineChracter {
    fn get_crlf_string(&self) -> &str {
        match self {
            &NewlineChracter::Cr => "\r",
            &NewlineChracter::CrLf => "\r\n",
            &NewlineChracter::Lf => "\n"
        }
    }
}

fn get_crlf_chars(optstr: &Option<String>) -> NewlineChracter {
    match optstr {
        Some(v) => match v.to_lowercase().as_str() {
            "crlf" => NewlineChracter::CrLf,
            "lf" => NewlineChracter::Lf,
            "cr" => NewlineChracter::Cr,
            _ => NewlineChracter::CrLf
        },
        None => NewlineChracter::CrLf
    }
}

fn main() {
    env_logger::init();
    match parse_args() {
        Ok(arg) => {
            if arg.help {
                let brief = format!("Usage: crlfconv [OPTIONS]");
                eprintln!("{}", create_options().usage(brief.as_str()))
            } else {
                do_main(arg).unwrap();
            }
        },
        Err(e) => {
            eprintln!("option parse failed:{}", e);
            let brief = format!("Usage: crlfconv [OPTIONS]");
            eprintln!("{}", create_options().usage(brief.as_str()));
        }
    }
}
