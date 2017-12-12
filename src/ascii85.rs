use std::io::stdin;

enum Ascii85Mode {
    Encode,
    Decode,
}

pub fn ascii85_codec() {
    let (mode, input) = get_input();

    // Decode or encode input string
    let output = match mode {
        Ok(Ascii85Mode::Decode) => ascii85_decode(&input),
        Ok(Ascii85Mode::Encode) => ascii85_encode(&input),
        Err(e) => {
            println!("{}", e);
            String::from("")
        }
    };

    println!("{}", output);
}

fn get_input() -> (Result<Ascii85Mode, String>, String) {
    println!(
        "Please enter 'd' or 'e' followed by a string\
        of text to decode or encode respectively:"
    );

    // Read input
    let mut s = String::new();
    stdin().read_line(&mut s).expect(
        "Please enter something to decode/encode from/to ascii85!",
    );

    // Detect mode
    let mode = match s.split_whitespace().count() {
        0...1 => Err("Error! Too few arguments provided.".to_string()),
        _ => {
            match s.split_whitespace().next() {
                Some("D") | Some("d") => Ok(Ascii85Mode::Decode),
                Some("E") | Some("e") => Ok(Ascii85Mode::Encode),
                _ => Err("Error! Unknown ascii85 mode specified.".to_string()),
            }
        }
    };

    // Remove encode/decode command (first two chars) from
    // the beginning of input string
    let _cmd: String = s.drain(..2).collect();

    // Strip off newline or carriage return characters
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    return (mode, s);
}

fn ascii85_decode(s: &str) -> String {
    // Calculate pad amount, such that input is divisible by 5
    let pad_count = (5 - s.len() % 5) % 5;

    // Split input into bytes, subtract 33
    let mut rad85: Vec<u8> = s.as_bytes()
        .iter().map(|x| x - 33).collect();

    // Pad with max base85 value
    rad85.append(&mut vec![84; pad_count]);

    // Convert to 32b concatenated characters
    let mut concat: Vec<u32> = Vec::new();
    for w in rad85.chunks(5) {
        let mut temp: u32 = w[0] as u32;
        for i in 1..w.len() {
            temp *= 85;
            temp += w[i] as u32;
        }
        concat.push(temp);
    }

    // Split into bytes
    let mut bytes: Vec<u8> = Vec::new();
    for mut n in concat {
        // Calculate each of the 4 bytes
        for _ in 0..4 {
            bytes.push(((n & 0xFF000000) >> 24)  as u8);
            n = n << 8;
        }
    }

    // Remove padded characters
    let corrected_length = bytes.len() - pad_count;
    bytes.truncate(corrected_length);

    return String::from_utf8(bytes).unwrap_or("".to_string());
}

fn ascii85_encode(s: &str) -> String {
    // Calculate pad amount, such that input is divisible by 4
    let pad_count = (4 - s.len() % 4) % 4;

    // Concatenate bytes into 32 bit numbers (groups of 4)
    let mut nums: Vec<u32> = Vec::new();
    for w in s.as_bytes().chunks(4) {
        // Concatenate bytes in groups of 4
        let concat = w.iter()
            .fold(0, |acc: u32, &x| (acc << 8) + (x as u32));

        // Pad input if not a multiple of 4
        let concat = if w.len() < 4 {
            concat * (256 as u32).pow(pad_count as u32)
        } else {
            concat
        };

        // Push new 32b number
        nums.push(concat);
    }

    // Divide 32b numbers into radix 85 numbers
    let mut rad85: Vec<u8> = Vec::new();
    for mut x in nums {
        let mut temp: Vec<u8> = Vec::new();
        temp.push((x % 85 + 33) as u8);
        for _ in 0..4 {
            x /= 85;
            temp.push((x % 85 + 33) as u8);
        }

        // Fix endian-ness
        temp.reverse();

        // Append to rad85
        rad85.append(&mut temp);
    }

    // Remove padded characters
    let corrected_length = rad85.len() - pad_count;
    rad85.truncate(corrected_length);

    return String::from_utf8(rad85).unwrap_or("".to_string());
}
