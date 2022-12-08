// Advent of Code 2021
// Day 16

use std::str::{FromStr, Chars};

#[cfg(test)]
mod test_puzzles {
    #[test]
    fn part1() {
        assert_eq!(984, super::part1());
    }
}

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn literal() {
        let input = "D2FE28";

        let packet = Packet::from_str(input).expect("parsing failed!");
        assert_eq!(Packet::Literal(6, 2021), packet);
    }

    #[test]
    fn op_1() {
        let input = "38006F45291200";

        let packet = Packet::from_str(input).expect("parsing failed!");
        assert_eq!(Packet::Operator(1, Operation::LessThan, vec![Packet::Literal(6, 10), Packet::Literal(2, 20)]), packet);
    }

    #[test]
    fn version_sum_1() {
        let input = "8A004A801A8002F478";

        let packet = Packet::from_str(input).expect("parsing failed!");
        assert_eq!(16, version_sum(&packet));
    }

    #[test]
    fn version_sum_2() {
        let input = "620080001611562C8802118E34";

        let packet = Packet::from_str(input).expect("parsing failed!");
        assert_eq!(12, version_sum(&packet));
    }

    #[test]
    fn version_sum_3() {
        let input = "C0015000016115A2E0802F182340";

        let packet = Packet::from_str(input).expect("parsing failed!");
        assert_eq!(23, version_sum(&packet));
    }

    #[test]
    fn version_sum_4() {
        let input = "A0016C880162017C3686B18A3D4780";

        let packet = Packet::from_str(input).expect("parsing failed!");
        assert_eq!(31, version_sum(&packet));
    }
}

#[cfg(test)]
mod test_examples_operators {
    use super::*;

    #[test]
    fn value_1() {
        let input = "C200B40A82";

        let packet = Packet::from_str(input).expect("parsing failed!");
        assert_eq!(3, value(&packet));
    }

    #[test]
    fn value_2() {
        let input = "04005AC33890";

        let packet = Packet::from_str(input).expect("parsing failed!");
        assert_eq!(54, value(&packet));
    }

    #[test]
    fn value_3() {
        let input = "880086C3E88112";

        let packet = Packet::from_str(input).expect("parsing failed!");
        assert_eq!(7, value(&packet));
    }

    #[test]
    fn value_4() {
        let input = "CE00C43D881120";

        let packet = Packet::from_str(input).expect("parsing failed!");
        assert_eq!(9, value(&packet));
    }

    #[test]
    fn value_5() {
        let input = "D8005AC2A8F0";

        let packet = Packet::from_str(input).expect("parsing failed!");
        assert_eq!(1, value(&packet));
    }

    #[test]
    fn value_6() {
        let input = "F600BC2D8F";

        let packet = Packet::from_str(input).expect("parsing failed!");
        assert_eq!(0, value(&packet));
    }

    #[test]
    fn value_7() {
        let input = "9C005AC2F8F0";

        let packet = Packet::from_str(input).expect("parsing failed!");
        assert_eq!(0, value(&packet));
    }

    #[test]
    fn value_8() {
        let input = "9C0141080250320F1802104A08";

        let packet = Packet::from_str(input).expect("parsing failed!");
        assert_eq!(1, value(&packet));
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo
}

#[derive(PartialEq, Eq, Debug)]
enum Packet {
    Literal(u64, u64),
    Operator(u64, Operation, Vec<Packet>)
}

#[derive(Debug)]
enum PacketParseError {
    InvalidLiteral,
    InvalidVersion,
    InvalidLengthId,
    InvalidLengthField,
    InvalidOperation
}

fn to_hex(s: &str) -> String {
    let b = u32::from_str_radix(s, 16).expect("not hexadecimal!");
    format!("{:04b}", b)
}

impl FromStr for Packet {
    type Err = PacketParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut packet = String::new();
        for c in s.chars() {
            packet.push_str(&to_hex(&String::from(c)));
        }

        let mut packet_iter = packet.chars();

        let (packet, _length) = Self::from_binary(&mut packet_iter)?;
        Ok(packet)
    }
}

impl Packet {
    fn from_binary(b: &mut Chars) -> Result<(Self, u64), PacketParseError> {
        let mut bits = 0;

        // Get the version - 3 bits.
        let version = get_bits(b, 3).ok_or(PacketParseError::InvalidVersion)?;
        bits += 3;

        // Get ID - 3 bits.
        let id = get_bits(b, 3).ok_or(PacketParseError::InvalidVersion)?;
        bits += 3;

        let packet = if id == 4 {
            // Parse literal value.
            let mut literal_value = 0;
            loop {
                literal_value <<= 4;
                let literal = get_bits(b, 5).ok_or(PacketParseError::InvalidLiteral)?;
                bits += 5;

                literal_value |= literal & 0b01111;

                // First bit being 0 indicates this is the last set.
                if literal & 0b10000 == 0 {
                    break;
                }
            }

            Self::Literal(version, literal_value)
        }
        else {
            let length_id = get_bits(b, 1).ok_or(PacketParseError::InvalidLengthId)?;
            bits += 1;

            let length_field = if length_id == 1 { 11 } else { 15 };
            
            let sub_packets_length = get_bits(b, length_field).ok_or(PacketParseError::InvalidLengthField)?;
            bits += length_field;

            let mut sub_packets = Vec::new();

            if length_id == 0 {
                let mut i = 0;
                while i < sub_packets_length {
                    let (sub_packet, sub_packet_length) = Self::from_binary(b)?;
                    i += sub_packet_length;
                    bits += sub_packet_length;

                    sub_packets.push(sub_packet);
                }
            }
            else {
                for _ in 0..sub_packets_length {
                    let (sub_packet, sub_packet_length) = Self::from_binary(b)?;
                    bits += sub_packet_length;
                    sub_packets.push(sub_packet);
                }
            }
            
            let op = match id {
                0 => Operation::Sum,
                1 => Operation::Product,
                2 => Operation::Minimum,
                3 => Operation::Maximum,
                5 => Operation::GreaterThan,
                6 => Operation::LessThan,
                7 => Operation::EqualTo,
                _ => return Err(PacketParseError::InvalidOperation)
            };

            Self::Operator(version, op, sub_packets)
        };

        Ok((packet, bits))
    }
}

fn get_bits(s: &mut Chars, n: u64) -> Option<u64> {
    let mut result = 0;
    for _ in 0..n {
        result <<= 1;
        let c = s.next()?;
        result |= match c {
            '0' => 0,
            '1' => 1,
            _ => return None
        };
    }

    Some(result)
}

fn version_sum(p: &Packet) -> u64 {
    match p {
        Packet::Literal(v, _) => *v,
        Packet::Operator(v, _, sub_packets) => {
            let mut sum = *v;

            for sp in sub_packets {
                sum += version_sum(&sp);
            }

            sum
        }
    }
}

fn value(p: &Packet) -> u64 {
    match p {
        Packet::Literal(_, val) => *val,
        Packet::Operator(_, op, subpackets) => value_operator(op, subpackets)
    }
}

fn value_operator(op: &Operation, packets: &Vec<Packet>) -> u64 {
    match *op {
        Operation::Sum => {
            packets.iter().map(|p| value(p)).sum()
        },
        Operation::Product => {
            let mut product = 1;
            for v in packets.iter().map(|p| value(p)) {
                product *= v;
            }
            product
        }
        Operation::Minimum => {
            match packets.iter().map(|p| value(p)).min() {
                Some(v) => v,
                None => 0
            }
        },
        Operation::Maximum => {
            match packets.iter().map(|p| value(p)).max() {
                Some(v) => v,
                None => 0
            }
        },
        Operation::GreaterThan => {
            let lhs = &packets[0];
            let rhs = &packets[1];

            if value(lhs) > value(rhs) {
                1
            }
            else {
                0
            }
        },
        Operation::LessThan => {
            let lhs = &packets[0];
            let rhs = &packets[1];

            if value(lhs) < value(rhs) {
                1
            }
            else {
                0
            }
        },
        Operation::EqualTo => {
            let lhs = &packets[0];
            let rhs = &packets[1];

            if value(lhs) == value(rhs) {
                1
            }
            else {
                0
            }
        }
    }
}

fn part1() -> u64 {
    let packet: String = aoc::data::get::<String>("data/day16.txt")[0].clone();
    let packet = Packet::from_str(&packet).expect("parsing packet failed!");
    return version_sum(&packet);
}

fn part2() -> u64 {
    let packet: String = aoc::data::get::<String>("data/day16.txt")[0].clone();
    let packet = Packet::from_str(&packet).expect("parsing packet failed!");
    return value(&packet);
}

fn main() {
    aoc::solution!(16, "version sum", "value");
}
