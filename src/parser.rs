// TODO: Actually use iterators instead of calling collect all the time.

#[derive(PartialEq)]
pub enum MorseToken {
    Short,
    Long,
    ShortPause,
    LongPause,
    Error,
}

use std::fmt;
impl fmt::Display for MorseToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MorseToken::Short => "Short",
                MorseToken::Long => "Long",
                MorseToken::ShortPause => "ShortPause",
                MorseToken::LongPause => "LongPause",
                MorseToken::Error => "Error",
            }
        )
    }
}

enum ParserState {
    Short,
    Long,
    ShortPause,
    LongPause,
}

struct ParserStateCounter {
    state: ParserState,
    c_true: usize,
    c_false: usize,
}

impl ParserStateCounter {
    fn reset_c(&mut self) {
        self.c_true = 0;
        self.c_false = 0;
    }
}

// TODO: Implement single unit length const.
pub fn translate(quantized_frames: Vec<bool>) {
    for hus in 1..20 {
        println!(
            "Half unit size '{}' yielded: {}",
            hus,
            foo(&quantized_frames, hus).unwrap()
        );
    }
}

fn foo(quantized_frames: &Vec<bool>, half_unit_size: usize) -> Option<String> {
    let mut tokens: Vec<MorseToken> = vec![];

    let mut c = ParserStateCounter {
        state: ParserState::ShortPause,
        c_true: 0,
        c_false: 0,
    };
    for &f in quantized_frames {
        if f {
            c.c_true += 1;
        } else {
            c.c_false += 1;
        }

        match c.state {
            ParserState::Short => {
                if c.c_true >= 3 * half_unit_size {
                    c.state = ParserState::Long;
                    c.c_false = 0;
                } else if c.c_false >= half_unit_size {
                    c.state = ParserState::ShortPause;
                    c.reset_c();
                    tokens.push(MorseToken::Short);
                }
            }
            ParserState::Long => {
                if c.c_false >= half_unit_size {
                    c.state = ParserState::ShortPause;
                    c.reset_c();
                    tokens.push(MorseToken::Long);
                }
            }
            ParserState::ShortPause => {
                if c.c_true >= half_unit_size {
                    c.state = ParserState::Short;
                    c.c_false = 0;
                    tokens.push(MorseToken::ShortPause);
                } else if c.c_false >= 8 * half_unit_size {
                    c.state = ParserState::LongPause;
                    c.c_true = 0;
                }
            }
            ParserState::LongPause => {
                if c.c_true >= half_unit_size {
                    c.state = ParserState::Short;
                    tokens.push(MorseToken::LongPause);
                    c.c_false = 0;
                }
            }
        }
    }
    to_ascii(tokens)
}

fn to_ascii(input: Vec<MorseToken>) -> Option<String> {
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
            MorseToken::LongPause => {
                if temp.len() != 0 {
                    result.push(temp);
                    temp = vec![];
                    result.push(vec![MorseToken::LongPause]);
                }
            }
            MorseToken::ShortPause => {}
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
        _ => MorseToken::Error,
    }
}
fn token_to_char(t: &MorseToken) -> char {
    match t {
        MorseToken::Short => '.',
        MorseToken::Long => '-',
        MorseToken::LongPause => ' ',
        _ => '?',
    }
}

fn decode_tokenstream(tokens: &Vec<MorseToken>) -> char {
    if tokens.len() <= 0 {
        return '_';
    }

    if tokens.len() == 1 && tokens[0] == MorseToken::LongPause {
        return ' ';
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
