use aoc2021::prelude::*;

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
            _ => {
                let length_type_id = s.chars().nth(0).unwrap();
                *s = &s[1..];
                *bits_read += 1;

                match length_type_id {
                    '0' => {
                        let subpackets_len = usize::from_str_radix(&s[0..15], 2).unwrap();
                        *s = &s[15..];
                        *bits_read += 15;
                        dbg!(subpackets_len);

                        let mut subpackets_buf = &s[..subpackets_len];
                        let mut subpackets_buf_ptr = &mut subpackets_buf;
                        let mut subpackets = vec![];

                        while !subpackets_buf_ptr.is_empty() {
                            let subpacket = Packet::from_binstr(&mut subpackets_buf_ptr, bits_read);

                            subpackets.push(subpacket)
                        }

                        *s = &s[subpackets_len..];

                        PacketType::Operator(subpackets)
                    }
                    '1' => {
                        let num_subpackets = usize::from_str_radix(&s[0..11], 2).unwrap();
                        *s = &s[11..];
                        *bits_read += 11;
                        dbg!(num_subpackets);

                        let mut subpackets = vec![];

                        for _ in 0..num_subpackets {
                            let subpacket = Packet::from_binstr(&mut s, bits_read);

                            subpackets.push(subpacket)
                        }

                        PacketType::Operator(subpackets)
                    }
                    _ => panic!("unknown length type id"),
                }
            }
        };

        Self {
            version,
            packet_type,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketType {
    LiteralValue(u64),
    Operator(Vec<Packet>),
}

fn sum_packet_versions(packet: &Packet) -> usize {
    match &packet.packet_type {
        PacketType::LiteralValue(_) => packet.version,
        PacketType::Operator(packets) => {
            let sum: usize = packets.iter().map(sum_packet_versions).sum();
            packet.version + sum
        }
    }
}

fn part1(input: &str) -> usize {
    sum_packet_versions(&Packet::parse(input))
}

// fn part2(input: &str) -> String {
//     todo!()
// }

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
            packet_type: PacketType::Operator(vec![
                Packet {
                    version: 6,
                    packet_type: PacketType::LiteralValue(10),
                },
                Packet {
                    version: 2,
                    packet_type: PacketType::LiteralValue(20),
                },
            ]),
        };

        assert_eq!(Packet::parse(input), expected);
    }

    #[test]
    fn test_parse_operator_packet_with_length_type_id_one() {
        let input = "EE00D40C823060";
        let expected = Packet {
            version: 7,
            packet_type: PacketType::Operator(vec![
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

    // #[test]
    // fn test_part2() {
    //     let expected = include_str!("../../outputs/day16_part2.txt");
    //     assert_eq!(part2(INPUT), expected);
    // }
}
