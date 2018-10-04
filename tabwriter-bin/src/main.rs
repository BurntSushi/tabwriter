extern crate docopt;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tabwriter;

use std::io::{self, Write};
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

#[derive(Deserialize)]
struct Args {
    flag_pad: usize,
    flag_width: usize,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.version(Some(version())).deserialize())
                            .unwrap_or_else(|e| e.exit());
    let mut tw = TabWriter::new(io::stdout())
                           .minwidth(args.flag_width)
                           .padding(args.flag_pad);
    ordie(io::copy(&mut io::stdin(), &mut tw));
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
    r.unwrap_or_else(|e| {
        let _ = write!(&mut io::stderr(), "{}", e.to_string());
        ::std::process::exit(1);
    })
}
