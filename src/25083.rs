use std::io::{self, BufWriter, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());

    write!(
        writer,
        r##"         ,r'"7
r`-_   ,'  ,/
 \. ". L_r'
   `~\/
      |
      |"##
    );
}
