use sscanf::sscanf;

fn main() {
    let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    lines.next();
    let mut nodes: Vec<Node> = Vec::new();
    for line in lines {
        let (key, left_key, right_key) = sscanf!(line, "{str} = ({str}, {str})").unwrap();
        nodes.push(Node {
            key,
            left_key,
            right_key,
        });
    }

    let instruction_chars: Vec<char> = instructions.chars().collect();
    let mut i = 0;
    loop {
        if instruction_chars[i] == 'R' {
            // TODO
        }
        else if instruction_chars[i] == 'L' {
            // TODO
        }
        else {
            panic!("Unknown instruction");
        }

        i += 1;
        if i == instruction_chars.len() {
            i = 0;
        }
    }
}

struct Node {
    key: &'static str,
    left_key: &'static str,
    right_key: &'static str,
}
