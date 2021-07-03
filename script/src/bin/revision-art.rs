use base16::decode;
use bishop::BishopArt;

fn main() {
    let sha_enc = std::env::args().nth(1).unwrap();
    let sha = decode(&sha_enc).unwrap();
    let mut ba = BishopArt::with_size(16, 8).unwrap();
    ba.input(sha);

    let content = format!(
        r"
## Revision
[`{0}`](https://github.com/rsk0315/library-rs/tree/{0})

```text
{1}```
",
        sha_enc,
        ba.draw()
    );

    for line in content.lines() {
        println!("//! {}", line);
    }
}
