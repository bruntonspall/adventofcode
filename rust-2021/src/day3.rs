use bit_vec::BitVec;

fn bits_for_position(input: &Vec<String>, pos: usize) -> BitVec {
    BitVec::from_iter(input.iter().map(|line| match line.chars().nth(pos) {
        Some('0') => false,
        Some('1') => true,
        _ => panic!("This shouldn't happen"),
    }))
}

fn mcb_for_position(input: &Vec<String>, pos: usize) -> i32 {
    let bits = bits_for_position(input, pos);
    if bits.count_ones() > bits.count_zeros() {
        1
    } else {
        0
    }
}

fn mcb(input: &Vec<String>) -> BitVec {
    let mut b = BitVec::new();
    for i in 0..input[0].len() {
        b.push(mcb_for_position(input, i) == 1);
    }
    b
}

fn lcb(input: &Vec<String>) -> BitVec {
    let mut v: BitVec = mcb(input);
    v.negate();
    return v;
}

fn leftpadbytes(input: &mut BitVec) -> Vec<u8> {
    let size = if input.len() > 8 {
        16 - input.len()
    } else {
        8 - input.len()
    };
    println!(
        "leftpadbytes({:?}) size {} + {} = {}",
        input,
        size,
        input.len(),
        size + input.len()
    );

    let mut newvec: BitVec = BitVec::from_elem(size, false);
    newvec.append(input);
    newvec.to_bytes()
}

fn as_u32(input: &mut BitVec) -> u32 {
    let bytes = leftpadbytes(input);
    println!("as_u32({:?})", bytes);
    let mut total: u32 = 0;
    let mut mul: u32 = 1;
    for byte in bytes {
        total *= mul;
        total = total + u32::from(byte);
        mul <<= 8;
    }
    total
}

fn bitvec_from_string(s: String) -> BitVec {
    BitVec::new()
}

fn find_oxygen_rating(input: &Vec<String>) -> &String {
    let input = input;
    for i in 0..input[0].len() {
        let m = mcb(input);
        println!("find_oxygen_rating on {:?},i is {}, mcb is now {:?}", input, i, m);
        let input2:Vec<&String> = input.iter().filter(|line|line.chars().nth(i) == match m[i] {
            true => Some('1'),
            false => Some('0')
        }).collect();
        if input2.len() == 1 { return input2[0]; };
        input = input2;
    }
    panic!("Shouldn't reach here");
}

#[aoc_generator(day3)]
pub fn parse(input: &str) -> Vec<String> {
    // The to_string here is required to create new objects and ensure that lifetimes are handled right.
    // The vector returned here will own the String objects
    input.lines().map(|s| s.to_string()).collect()
}

#[aoc(day3, part1, u32)]
pub fn part1(input: &Vec<String>) -> u32 {
    as_u32(&mut mcb(&input)) * as_u32(&mut lcb(&input))
}

#[aoc(day3, part2, u32)]
pub fn part2(input: &Vec<String>) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leftpadbits() {
        let mut b = BitVec::new();
        b.push(true);
        b.push(false);
        b.push(true);
        assert_eq!(b.to_bytes(), &[0b10100000]);
        assert_eq!(leftpadbytes(&mut b), &[0b00000101]);

        let mut b = BitVec::new();
        b.push(false);
        b.push(false);
        b.push(true);
        b.push(true);
        b.push(true);
        b.push(false);
        b.push(true);
        b.push(true);
        b.push(true);
        println!("{:?}", b);
        assert_eq!(b.to_bytes(), &[0b00111011, 0b10000000]);
        assert_eq!(leftpadbytes(&mut b), &[0b0, 0b01110111]);
    }

    #[test]
    fn test_as_u32() {
        let mut b = BitVec::from_bytes(&[0x7F]);
        assert_eq!(127, as_u32(&mut b));
        let mut b = BitVec::from_bytes(&[0xFF]);
        assert_eq!(255, as_u32(&mut b));
        let mut b = BitVec::from_bytes(&[0x01, 0x02]);
        assert_eq!(258, as_u32(&mut b));
    }

    #[test]
    fn test_bits_for_position() {
        let bits = parse(
            "0000
0011
0101",
        );
        assert_eq!(leftpadbytes(&mut bits_for_position(&bits, 0)), &[0b000]);
        assert_eq!(leftpadbytes(&mut bits_for_position(&bits, 1)), &[0b001]);
        assert_eq!(leftpadbytes(&mut bits_for_position(&bits, 2)), &[0b010]);
        assert_eq!(leftpadbytes(&mut bits_for_position(&bits, 3)), &[0b011]);
    }

    #[test]
    fn test_mcb_for_position() {
        let bits = parse(
            "0000
0011
0101",
        );
        assert_eq!(mcb_for_position(&bits, 0), 0);
        assert_eq!(mcb_for_position(&bits, 1), 0);
        assert_eq!(mcb_for_position(&bits, 2), 0);
        assert_eq!(mcb_for_position(&bits, 3), 1);
    }

    #[test]
    fn test_mcb_and_lcb() {
        let bits = parse(
            "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
        );
        assert_eq!(leftpadbytes(&mut mcb(&bits)), &[0b10110]);
        assert_eq!(leftpadbytes(&mut lcb(&bits)), &[0b01001]);
    }

    #[test]
    fn test_find_oxygen_rating() {
        let bits = parse(
            "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
        );
        assert_eq!(find_oxygen_rating(&bits), "10111");
    }
    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
            )),
            198
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), 0);
    }
}
