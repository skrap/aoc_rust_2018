fn main() {
    let input = include_bytes!("5_input");
    let poly : Vec<u8> = input[..].into();

    let mut next : Vec<u8> = Vec::with_capacity(poly.len());
    for p1 in poly {
        if p1 == b'\n' {
            continue;
        }
        if let Some(&p0) = next.last() {
            if p0^p1 == 0x20 {
                // skip both
                next.pop().unwrap(); // it was there above, right?
            } else {
                next.push(p1);
            }
        } else {
            next.push(p1);
        }
    }

    println!("{}", next.len());
}