use std::borrow::Borrow;
use std::fmt::{self, LowerHex};

pub struct Hexdump<'a, B: ?Sized>(pub &'a B);

impl<'a, B, U> LowerHex for Hexdump<'a, B>
where
    B: ?Sized,
    &'a B: IntoIterator<Item = U>,
    U: Borrow<u8>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let count = if f.alternate() { 8 } else { 16 };
        let hex_width = (3 * 8 + 1) * (count / 8);
        let mut hex = "".to_owned();
        let mut ch = vec![];
        let mut len = 0;
        for (i, b) in self.0.into_iter().enumerate() {
            let b = b.borrow().clone();
            if i % count == 0 {
                write!(f, "{i:08x}")?;
            }
            if i % 8 == 0 {
                hex.push(' ');
            }
            hex.push_str(&format!(" {b:02x}"));
            ch.push(b);
            if (i + 1) % count == 0 {
                write!(f, "{hex:hex_width$}  |")?;
                hex.clear();
                for c in ch.drain(..).map(char_display) {
                    write!(f, "{c}")?;
                }
                writeln!(f, "|")?;
            }
            len += 1;
        }
        if !hex.is_empty() {
            write!(f, "{hex:hex_width$}  |")?;
            hex.clear();
            for c in ch.drain(..).map(char_display) {
                write!(f, "{c}")?;
            }
            writeln!(f, "|")?;
        }
        write!(f, "{len:08x}")
    }
}

fn char_display(b: u8) -> char {
    match b {
        b' '..=b'~' => b as char,
        _ => '.',
    }
}

#[test]
fn test() {
    let a: Vec<_> = (0_u8..=50).collect();
    println!("{:x}", Hexdump(&a));
    println!("{:#x}", Hexdump(&a));

    let a: Vec<_> = (0_u8..32).collect();
    println!("{:x}", Hexdump(&a));
    println!("{:#x}", Hexdump(&a));

    println!("{:x}", Hexdump(AsRef::<[u8]>::as_ref("test")));
    println!("{:x}", Hexdump::<[u8]>("test".as_ref()));
    println!("{:x}", Hexdump::<[u8]>(b"test"));
    println!("{:x}", Hexdump(b"test"));

    let s = "hello";
    println!("{:x}", Hexdump::<[u8]>(s.as_ref()));

    panic!()
}
