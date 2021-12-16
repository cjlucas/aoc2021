use aoc2021::prelude::*;

use std::rc::Rc;

const INPUT: &'static str = include_str!("../../inputs/day16.txt");

fn hex_to_bin(hex_str: &str) -> String {
    hex_str
        .trim()
        .chars()
        .map(|c| match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => unreachable!(),
        })
        .join("")
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Packet {
    version: usize,
    packet_type: PacketType,
}

impl Packet {
    fn parse(mut s: &str) -> Self {
        let mut binstr = hex_to_bin(s);
        let mut ptr = &binstr[..];
        let packet = Self::from_binstr(&mut ptr, &mut 0);

        // assert!(ptr.is_empty());

        packet
    }

    fn evaluate(&self) -> u64 {
        use PacketType::*;

        dbg!(&self.packet_type);

        match &self.packet_type {
            LiteralValue(n) => *n,
            Sum(subpackets) => subpackets.iter().map(Self::evaluate).sum(),
            Product(subpackets) => subpackets
                .iter()
                .map(Self::evaluate)
                .fold(1, |acc, n| acc * n),
            Minimum(subpackets) => subpackets.iter().map(Self::evaluate).min().unwrap(),
            Maximum(subpackets) => subpackets.iter().map(Self::evaluate).max().unwrap(),
            LessThan(lhs, rhs) => {
                if lhs.evaluate() < rhs.evaluate() {
                    1
                } else {
                    0
                }
            }
            GreaterThan(lhs, rhs) => {
                if lhs.evaluate() > rhs.evaluate() {
                    1
                } else {
                    0
                }
            }
            Equal(lhs, rhs) => {
                if lhs.evaluate() == rhs.evaluate() {
                    1
                } else {
                    0
                }
            }
        }
    }

    fn from_binstr(mut s: &mut &str, bits_read: &mut usize) -> Self {
        println!("--------------");
        dbg!(&s);
        dbg!(&s.len());

        let version = usize::from_str_radix(&s[0..3], 2).unwrap();
        let type_id = usize::from_str_radix(&s[3..6], 2).unwrap();
        *s = &s[6..];
        *bits_read += 6;

        dbg!(version);
        dbg!(type_id);

        let packet_type = match type_id {
            0 => {
                let subpackets = Self::parse_operator_packet(&mut s);
                PacketType::Sum(subpackets)
            }
            1 => {
                let subpackets = Self::parse_operator_packet(&mut s);
                PacketType::Product(subpackets)
            }
            2 => {
                let subpackets = Self::parse_operator_packet(&mut s);
                PacketType::Minimum(subpackets)
            }
            3 => {
                let subpackets = Self::parse_operator_packet(&mut s);
                PacketType::Maximum(subpackets)
            }
            4 => {
                let mut foo = String::new();

                loop {
                    foo.push_str(&s[1..5]);

                    let end = s.chars().nth(0).unwrap() == '0';
                    *s = &s[5..];
                    *bits_read += 5;

                    if end {
                        break;
                    }
                }

                dbg!(&bits_read);

                // strip trailing zeros
                let bits_to_skip = (4 - (*bits_read % 4)) % 4;
                // *s = &s[bits_to_skip..];
                *bits_read += bits_to_skip;

                let n = u64::from_str_radix(&foo, 2).unwrap();
                dbg!(n);
                PacketType::LiteralValue(n)
            }
            5 => {
                let subpackets = Self::parse_operator_packet(&mut s);
                let mut iter = subpackets.into_iter();
                let lhs = iter.next().unwrap();
                let rhs = iter.next().unwrap();

                PacketType::GreaterThan(Rc::new(lhs), Rc::new(rhs))
            }
            6 => {
                let subpackets = Self::parse_operator_packet(&mut s);
                let mut iter = subpackets.into_iter();
                let lhs = iter.next().unwrap();
                let rhs = iter.next().unwrap();

                PacketType::LessThan(Rc::new(lhs), Rc::new(rhs))
            }
            7 => {
                let subpackets = Self::parse_operator_packet(&mut s);
                let mut iter = subpackets.into_iter();
                let lhs = iter.next().unwrap();
                let rhs = iter.next().unwrap();

                PacketType::Equal(Rc::new(lhs), Rc::new(rhs))
            }
            _ => unimplemented!(),
        };

        Self {
            version,
            packet_type,
        }
    }

    fn parse_operator_packet(s: &mut &str) -> Vec<Packet> {
        let length_type_id = s.chars().nth(0).unwrap();
        *s = &s[1..];

        match length_type_id {
            '0' => {
                let subpackets_len = usize::from_str_radix(&s[0..15], 2).unwrap();
                *s = &s[15..];
                dbg!(subpackets_len);

                let mut subpackets_buf = &s[..subpackets_len];
                let mut subpackets_buf_ptr = &mut subpackets_buf;
                let mut subpackets = vec![];

                while !subpackets_buf_ptr.is_empty() {
                    let subpacket = Packet::from_binstr(&mut subpackets_buf_ptr, &mut 0);

                    subpackets.push(subpacket)
                }

                *s = &s[subpackets_len..];

                subpackets
            }
            '1' => {
                let num_subpackets = usize::from_str_radix(&s[0..11], 2).unwrap();
                *s = &s[11..];
                dbg!(num_subpackets);

                let mut subpackets = vec![];

                for _ in 0..num_subpackets {
                    let subpacket = Packet::from_binstr(s, &mut 0);

                    subpackets.push(subpacket)
                }

                subpackets
            }
            _ => panic!("unknown length type id"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketType {
    LiteralValue(u64),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    GreaterThan(Rc<Packet>, Rc<Packet>),
    LessThan(Rc<Packet>, Rc<Packet>),
    Equal(Rc<Packet>, Rc<Packet>),
}

fn sum_packet_versions(packet: &Packet) -> usize {
    match &packet.packet_type {
        PacketType::LiteralValue(_) => packet.version,
        PacketType::Sum(packets) => {
            let sum: usize = packets.iter().map(sum_packet_versions).sum();
            packet.version + sum
        }
        PacketType::Product(packets) => {
            let sum: usize = packets.iter().map(sum_packet_versions).sum();
            packet.version + sum
        }
        PacketType::Minimum(packets) => {
            let sum: usize = packets.iter().map(sum_packet_versions).sum();
            packet.version + sum
        }
        PacketType::Maximum(packets) => {
            let sum: usize = packets.iter().map(sum_packet_versions).sum();
            packet.version + sum
        }
        PacketType::GreaterThan(lhs, rhs) => {
            packet.version + sum_packet_versions(lhs) + sum_packet_versions(rhs)
        }
        PacketType::LessThan(lhs, rhs) => {
            packet.version + sum_packet_versions(lhs) + sum_packet_versions(rhs)
        }
        PacketType::Equal(lhs, rhs) => {
            packet.version + sum_packet_versions(lhs) + sum_packet_versions(rhs)
        }
    }
}

fn part1(input: &str) -> usize {
    sum_packet_versions(&Packet::parse(input))
}

fn part2(input: &str) -> u64 {
    Packet::parse(input).evaluate()
}

fn main() {
    println!("part1: {}", part1(INPUT));
    // println!("part2:");
    // println!("{}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_literal_value_packet() {
        let input = "D2FE28";
        let expected = Packet {
            version: 6,
            packet_type: PacketType::LiteralValue(2021),
        };

        assert_eq!(Packet::parse(input), expected);
    }

    #[test]
    fn test_parse_operator_packet_with_length_type_id_zero() {
        let input = "38006F45291200";
        let expected = Packet {
            version: 1,
            packet_type: PacketType::LessThan(
                Rc::new(Packet {
                    version: 6,
                    packet_type: PacketType::LiteralValue(10),
                }),
                Rc::new(Packet {
                    version: 2,
                    packet_type: PacketType::LiteralValue(20),
                }),
            ),
        };

        assert_eq!(Packet::parse(input), expected);
    }

    #[test]
    fn test_parse_operator_packet_with_length_type_id_one() {
        let input = "EE00D40C823060";
        let expected = Packet {
            version: 7,
            packet_type: PacketType::Maximum(vec![
                Packet {
                    version: 2,
                    packet_type: PacketType::LiteralValue(1),
                },
                Packet {
                    version: 4,
                    packet_type: PacketType::LiteralValue(2),
                },
                Packet {
                    version: 1,
                    packet_type: PacketType::LiteralValue(3),
                },
            ]),
        };

        assert_eq!(Packet::parse(input), expected);
    }

    #[test]
    fn test_part1_sample1() {
        assert_eq!(part1("8A004A801A8002F478"), 16);
    }

    #[test]
    fn test_part1_sample2() {
        assert_eq!(part1("620080001611562C8802118E34"), 12);
    }

    #[test]
    fn test_part1_sample3() {
        assert_eq!(part1("C0015000016115A2E0802F182340"), 23);
    }

    #[test]
    fn test_part1_sample4() {
        assert_eq!(part1("A0016C880162017C3686B18A3D4780"), 31);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 886);
    }

    #[test]
    fn test_part2_sample1() {
        let input = "C200B40A82";
        assert_eq!(Packet::parse(input).evaluate(), 3);
    }

    #[test]
    fn test_part2_sample2() {
        let input = "04005AC33890";
        assert_eq!(Packet::parse(input).evaluate(), 54);
    }

    #[test]
    fn test_part2_sample3() {
        let input = "880086C3E88112";
        assert_eq!(Packet::parse(input).evaluate(), 7);
    }

    #[test]
    fn test_part2_sample4() {
        let input = "CE00C43D881120";
        assert_eq!(Packet::parse(input).evaluate(), 9);
    }

    #[test]
    fn test_part2_sample5() {
        let input = "D8005AC2A8F0";
        assert_eq!(Packet::parse(input).evaluate(), 1);
    }

    #[test]
    fn test_part2_sample6() {
        let input = "F600BC2D8F";
        assert_eq!(Packet::parse(input).evaluate(), 0);
    }

    #[test]
    fn test_part2_sample7() {
        let input = "9C005AC2F8F0";
        assert_eq!(Packet::parse(input).evaluate(), 0);
    }

    #[test]
    fn test_part2_sample8() {
        let input = "9C0141080250320F1802104A08";
        assert_eq!(Packet::parse(input).evaluate(), 1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Packet::parse(INPUT).evaluate(), 184487454837);
    }
}
