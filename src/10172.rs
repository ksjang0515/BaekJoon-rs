use std::io::{self, BufWriter, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());

    write!(
        writer,
        r##"|\_/|
|q p|   /}}
( 0 )"""\
|"^"`    |
||_/=\\__|"##
    );
}
