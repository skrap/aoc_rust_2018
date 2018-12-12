fn calc_cell(x: i32, y: i32, serial: i32) -> i32 {
    /*
    Find the fuel cell's rack ID, which is its X coordinate plus 10.
    Begin with a power level of the rack ID times the Y coordinate.
    Increase the power level by the value of the grid serial number (your puzzle input).
    Set the power level to itself multiplied by the rack ID.
    Keep only the hundreds digit of the power level (so 12345 becomes 3; numbers with no hundreds digit become 0).
    Subtract 5 from the power level.
    */
    let rack_id = x+10;
    let mut power = rack_id*y+serial;
    power *= rack_id;
    power /= 100;
    power %= 10;
    power -= 5;
    power
}

fn calc_box(left: i32, top: i32, length: i32, serial: i32) -> i32 {
    // calculates the total power for a 3x3 square with the top-left given
    let mut sum = 0i32;
    for x in left..left+length {
        for y in top..top+length {
            sum += calc_cell(x, y, serial);
        }
    }
    sum
}

fn main() {
    const SERIAL : i32 = 7672;

    assert!(calc_cell(3,5,8) == 4);

    let mut max_xy = (0,0);
    let mut max_power = i32::min_value();
    for x in 1..=(300-3) {
        for y in 1..=(300-3) {
            let power = calc_box(x, y, 3, SERIAL);
            if power > max_power {
                max_xy = (x, y);
                max_power = power;
            }
        }
    }
    println!("3x3 max power is {} at {},{}",max_power, max_xy.0, max_xy.1);

    max_xy = (0,0);
    max_power = i32::min_value();
    let mut max_len = 0;
    for len in 1..300 {
        for x in 1..=(300-len) {
            for y in 1..=(300-len) {
                let power = calc_box(x, y, len, SERIAL);
                if power > max_power {
                    max_xy = (x, y);
                    max_power = power;
                    max_len = len;
                }
            }
        }
    }
    println!("max power is {} at {},{} size {}",max_power, max_xy.0, max_xy.1, max_len);
}