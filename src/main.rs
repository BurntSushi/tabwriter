extern crate libc;
extern crate serialize;
extern crate docopt;
extern crate tabwriter;

use std::io;
use docopt::Docopt;
use tabwriter::TabWriter;

static USAGE: &'static str = "
Usage:
    tabwriter [options]

Options:
    -w, --width <arg>   Minimum width of each column.
                        [default: 2]
    -p, --pad <arg>     Padding to separate each column.
                        [default: 2]
    -h, --help          Display this message
    --version           Print version info and exit
";

#[deriving(Decodable)]
struct Args {
    flag_pad: uint,
    flag_width: uint,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.version(Some(version())).decode())
                            .unwrap_or_else(|e| e.exit());
    let mut tw = TabWriter::new(io::stdout())
                           .minwidth(args.flag_width)
                           .padding(args.flag_pad);
    ordie(io::util::copy(&mut io::stdin(), &mut tw));
    ordie(tw.flush());
}

fn version() -> String {
    let (maj, min, pat) = (
        option_env!("CARGO_PKG_VERSION_MAJOR"),
        option_env!("CARGO_PKG_VERSION_MINOR"),
        option_env!("CARGO_PKG_VERSION_PATCH"),
    );
    match (maj, min, pat) {
        (Some(maj), Some(min), Some(pat)) => format!("{}.{}.{}", maj, min, pat),
        _ => "".to_string(),
    }
}

fn ordie<T, E: ToString>(r: Result<T, E>) -> T {
    match r {
        Ok(r) => r,
        Err(e) => {
            io::stderr().write_str(e.to_string().as_slice()).unwrap();
            unsafe { libc::exit(1 as libc::c_int) }
        }
    }
}
