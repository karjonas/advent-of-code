extern crate common;

fn to_binary(c: char) -> String {
    let m = match c {
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
        _ => "",
    };
    return String::from(m);
}

fn parse_input(input: &String) -> String {
    return input
        .chars()
        .map(|v| to_binary(v))
        .collect::<Vec<_>>()
        .join("");
}

fn from_binary(s: &str) -> usize {
    return usize::from_str_radix(s, 2).unwrap();
}

fn parse_literal(bits: &String, mut pos: usize) -> (usize, usize) {
    let mut parsed_bits = String::new();
    loop {
        let group_continues = from_binary(&bits[pos..pos + 1]) == 1;
        pos += 1;
        parsed_bits.push_str(&bits[pos..pos + 4]);
        pos += 4;

        if !group_continues {
            break;
        }
    }

    return (from_binary(parsed_bits.as_str()), pos);
}

fn parse_recursive(
    bits: &String,
    mut pos: usize,
    end: usize,
    num_packets: usize,
) -> (usize, Vec<usize>, usize) {
    let mut version_sum = 0;
    let mut num_parsed = 0;
    let mut packets = Vec::new();
    while num_parsed < num_packets && pos < end {
        let packet_sum;
        let version = from_binary(&bits[pos..pos + 3]);
        let type_id = from_binary(&bits[pos + 3..pos + 6]);
        version_sum += version;
        pos += 6;
        if type_id == 4 {
            let (literal, pos_next) = parse_literal(&bits, pos);
            pos = pos_next;
            packet_sum = literal;
        } else {
            let length_id = from_binary(&bits[pos..pos + 1]);
            pos += 1;
            let mut pkts = Vec::new();
            if length_id == 0 {
                let length = from_binary(&bits[pos..pos + 15]);
                pos += 15;
                let (_pos_next, mut subpkts, version_sum_next) =
                    parse_recursive(bits, pos, pos + length, std::usize::MAX);
                version_sum += version_sum_next;
                pkts.append(&mut subpkts);
                pos += length;
            } else {
                assert_eq!(length_id, 1);
                let num_packets = from_binary(&bits[pos..pos + 11]);
                pos += 11;
                let (pos_next, mut subpackets, version_sum_next) =
                    parse_recursive(bits, pos, end, num_packets);
                version_sum += version_sum_next;
                pkts.append(&mut subpackets);
                pos = pos_next;
            }

            packet_sum = match type_id {
                0 => pkts.iter().fold(0, |acc, x| acc + x),
                1 => pkts.iter().fold(1, |acc, x| acc * x),
                2 => pkts
                    .iter()
                    .fold(std::usize::MAX, |acc, x| std::cmp::min(acc, *x)),
                3 => pkts.iter().fold(0, |acc, x| std::cmp::max(acc, *x)),
                5 => (pkts[0] > pkts[1]) as usize,
                6 => (pkts[0] < pkts[1]) as usize,
                7 => (pkts[0] == pkts[1]) as usize,
                _ => 0,
            };
        }
        packets.push(packet_sum);
        num_parsed += 1;
    }
    return (pos, packets, version_sum);
}

fn solve_internal_p1(input: &String) -> usize {
    let bits = parse_input(input);
    return parse_recursive(&bits, 0, bits.len(), 1).2;
}

fn solve_internal_p2(input: &String) -> usize {
    let bits = parse_input(input);
    return parse_recursive(&bits, 0, bits.len(), 1).1[0];
}

pub fn solve() {
    let input = common::read_file("2021/day16/input");
    println!("Part one: {}", solve_internal_p1(&input));
    println!("Part two: {}", solve_internal_p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        assert_eq!(solve_internal_p1(&String::from("8A004A801A8002F478")), 16);
        assert_eq!(
            solve_internal_p1(&String::from("620080001611562C8802118E34")),
            12
        );
        assert_eq!(
            solve_internal_p1(&String::from("C0015000016115A2E0802F182340")),
            23
        );
        assert_eq!(
            solve_internal_p1(&String::from("A0016C880162017C3686B18A3D4780")),
            31
        );

        assert_eq!(solve_internal_p2(&String::from("C200B40A82")), 3);
        assert_eq!(solve_internal_p2(&String::from("04005AC33890")), 54);
        assert_eq!(solve_internal_p2(&String::from("880086C3E88112")), 7);
        assert_eq!(solve_internal_p2(&String::from("CE00C43D881120")), 9);
        assert_eq!(solve_internal_p2(&String::from("D8005AC2A8F0")), 1);
        assert_eq!(solve_internal_p2(&String::from("F600BC2D8F")), 0);
        assert_eq!(solve_internal_p2(&String::from("9C005AC2F8F0")), 0);
        assert_eq!(
            solve_internal_p2(&String::from("9C0141080250320F1802104A08")),
            1
        );
    }
}
