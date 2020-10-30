// TODO: Actually use iterators instead of calling collect all the time.

pub enum MorseToken {
    Short,
    Long,
    Nop,
}

pub fn to_ascii(input: Vec<MorseToken>) -> Option<String> {
    Some(
        split_tokens(input)
            .iter()
            .map(|t| decode_tokenstream(t))
            .collect(),
    )
}

fn split_tokens(input: Vec<MorseToken>) -> Vec<Vec<MorseToken>> {
    let mut result: Vec<Vec<MorseToken>> = vec![];
    let mut temp: Vec<MorseToken> = vec![];

    for t in input {
        match t {
            MorseToken::Nop => {
                if temp.len() != 0 {
                    result.push(temp);
                    temp = vec![];
                }
            }
            _ => temp.push(t),
        }
    }

    result.push(temp);
    result
}

// TODO: Implement traits for this?
fn char_to_token(s: char) -> MorseToken {
    match s {
        '.' => MorseToken::Short,
        '-' => MorseToken::Long,
        _ => MorseToken::Nop,
    }
}
fn token_to_char(t: &MorseToken) -> char {
    match t {
        MorseToken::Short => '.',
        MorseToken::Long => '-',
        _ => '?',
    }
}

fn decode_tokenstream(tokens: &Vec<MorseToken>) -> char {
    if tokens.len() <= 0 {
        return '_';
    }

    let key = tokens
        .iter()
        .map(|t| token_to_char(t))
        .fold("".to_string(), |acc, x| [acc, x.to_string()].concat())
        .to_owned();

    match &key[..] {
        ".-" => 'A',
        "-..." => 'B',
        "-.-." => 'C',
        "-.." => 'D',
        "." => 'E',
        "..-." => 'F',
        "--." => 'G',
        "...." => 'H',
        ".." => 'I',
        ".---" => 'J',
        "-.-" => 'K',
        ".-.." => 'L',
        "--" => 'M',
        "-." => 'N',
        "---" => 'O',
        ".--." => 'P',
        "--.-" => 'Q',
        ".-." => 'R',
        "..." => 'S',
        "-" => 'T',
        "..-" => 'U',
        "...-" => 'V',
        ".--" => 'W',
        "-..-" => 'X',
        "-.--" => 'Y',
        "--.." => 'Z',
        ".----" => '1',
        "..---" => '2',
        "...--" => '3',
        "....-" => '4',
        "....." => '5',
        "-...." => '6',
        "--..." => '7',
        "---.." => '8',
        "----." => '9',
        "-----" => '0',
        _ => '?',
    }
}
