use anyhow::Result;
use itertools::Itertools;

pub fn part1(input: &str) -> Result<String> {
    let packet = parse_input(input)?;
    Ok(version_sum(&packet).to_string())
}

fn version_sum(packet: &Packet) -> u64 {
    packet.version as u64
        + match &packet.content {
            PacketContent::Operator(_, children) => children.iter().map(version_sum).sum(),
            PacketContent::Literal(_) => 0,
        }
}

pub fn part2(input: &str) -> Result<String> {
    let packet = parse_input(input)?;
    Ok(eval_packet(packet).to_string())
}

fn eval_packet(packet: Packet) -> u64 {
    match packet.content {
        PacketContent::Literal(val) => val,
        PacketContent::Operator(type_id, subpackets) => {
            let mut subvals = subpackets.into_iter().map(eval_packet);
            match type_id {
                0 => subvals.sum(),
                1 => subvals.product(),
                2 => subvals.min().unwrap(),
                3 => subvals.max().unwrap(),
                5 => (subvals.next().unwrap() > subvals.next().unwrap()) as u64,
                6 => (subvals.next().unwrap() < subvals.next().unwrap()) as u64,
                7 => subvals.all_equal() as u64,
                _ => panic!("invalid operator packet type id {}", type_id),
            }
        }
    }
}

#[derive(Debug)]
struct Packet {
    version: u64,
    length: usize,
    content: PacketContent,
}

#[derive(Debug)]
enum PacketContent {
    Operator(u64, Vec<Packet>),
    Literal(u64),
}

fn parse_input(input: &str) -> Result<Packet> {
    parse_packet(
        &mut hex::decode(input.trim())?
            .iter()
            .flat_map(|&b| (0..8).rev().map(move |i| (b >> i) & 1)),
    )
}

fn parse_packet(bits: &mut impl Iterator<Item = u8>) -> Result<Packet> {
    let version = parse_num(bits, 3)?;
    let typ = parse_num(bits, 3)?;
    let mut length = 6;
    let packet = match typ {
        // 4 indicates a literal integer.
        4 => {
            let mut val: u64 = 0;
            loop {
                let part = parse_num(bits, 5)?;
                length += 5;
                val <<= 4;
                let mask = 0b10000;
                if part & mask > 0 {
                    val |= (part & 0b1111) as u64;
                } else {
                    val |= part as u64;
                    break;
                }
            }

            Packet {
                version,
                length,
                content: PacketContent::Literal(val),
            }
        }
        // Anything else indicates an operator.
        _ => {
            let length_type = parse_num(bits, 1)?;
            length += 1;
            let mut subpackets = vec![];
            if length_type == 0 {
                let subpackets_length = parse_num(bits, 15)? as usize;
                length += 15;
                let target_length = length + subpackets_length;
                while length < target_length {
                    let sp = parse_packet(bits)?;
                    length += sp.length;
                    subpackets.push(sp);
                }
            } else {
                let subpacket_count = parse_num(bits, 11)?;
                length += 11;
                for _ in 0..subpacket_count {
                    let sp = parse_packet(bits)?;
                    length += sp.length;
                    subpackets.push(sp);
                }
            }

            Packet {
                version,
                length,
                content: PacketContent::Operator(typ, subpackets),
            }
        }
    };
    Ok(packet)
}

fn parse_num(bits: &mut impl Iterator<Item = u8>, len: usize) -> Result<u64> {
    assert!(len <= 32, "can only parse up to 32 bits");
    let bits = bits.take(len).collect_vec();
    if bits.len() < len {
        return Err(anyhow::anyhow!("expected {} bits, got {}", len, bits.len()));
    }
    let mut ret = 0;
    bits.iter().for_each(|&b| {
        assert!(b <= 1, "bits must only be zero or one, got {}", b);
        ret <<= 1;
        ret += b as u64;
    });
    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1("D2FE28").unwrap(), "6");
        assert_eq!(part1("8A004A801A8002F478").unwrap(), "16");
        assert_eq!(part1("620080001611562C8802118E34").unwrap(), "12");
        assert_eq!(part1("C0015000016115A2E0802F182340").unwrap(), "23");
        assert_eq!(part1("A0016C880162017C3686B18A3D4780").unwrap(), "31");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("C200B40A82").unwrap(), "3");
        assert_eq!(part2("04005AC33890").unwrap(), "54");
        assert_eq!(part2("880086C3E88112").unwrap(), "7");
        assert_eq!(part2("F600BC2D8F").unwrap(), "0");
        assert_eq!(part2("9C005AC2F8F0").unwrap(), "0");
        assert_eq!(part2("9C0141080250320F1802104A08").unwrap(), "1");
    }
}
