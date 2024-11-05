use regex::Regex;
use std::{collections::VecDeque, str::FromStr, string::ParseError};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    #[default]
    None,
    Output(usize),
    Bot(usize),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Bot {
    low: Option<usize>,
    high: Option<usize>,
    low_target: Target,
    high_target: Target,
}

#[derive(Debug)]
pub struct Bots {
    bots: Vec<Bot>,
    outputs: Vec<Option<usize>>,

    process: VecDeque<usize>,

    // solution part a
    pub bot_17_61: usize,
}

impl Bots {
    /// return Ref to bot at index
    /// guard against overflow (will probably never needed)
    fn ensure(&mut self, bot_index: usize) -> &mut Bot {
        if self.bots.len() < bot_index {
            self.bots.resize(bot_index + 100, Bot::default());
        }

        &mut self.bots[bot_index]
    }

    /// Set a value on bot. Will be low if only one value, otherwise ordered.
    /// If bot has two values / chips, add bot index to processing queue
    fn set_value(&mut self, target: Target, value: Option<usize>) {
        match (target, value) {
            (Target::None, _) => panic!("Uninitialized target"),
            (_, None) => panic!("Uninitialized value"),
            (Target::Output(output_index), val) => self.set_output(output_index, val),
            (Target::Bot(bot_index), Some(val)) => self.set_bot_value(bot_index, val),
        }
    }

    fn set_bot_value(&mut self, bot_index: usize, value: usize) {
        let bot: &mut Bot = self.ensure(bot_index);

        if let Some(known) = bot.low {
            assert!(
                bot.high.is_none(),
                "bot {} got more than 2 values: {:?}, {:?}, {}",
                bot_index,
                bot.low,
                bot.high,
                value
            );

            // set values in correct buckets
            if known > value {
                bot.low = Some(value);
                bot.high = Some(known);
            } else {
                bot.high = Some(value);
            }

            // bot has 2 values, add it to queue
            self.process.push_back(bot_index);
        } else {
            bot.low = Some(value);
        }
    }

    fn set_output(&mut self, output_index: usize, val: Option<usize>) {
        if self.outputs.len() < output_index {
            self.outputs.resize(output_index + 100, None);
        }

        self.outputs[output_index] = val;
    }

    /// set low and high target
    fn set_targets(&mut self, bot_index: usize, low_target: Target, high_target: Target) {
        let bot = self.ensure(bot_index);
        assert!(
            bot.low_target == Target::None && bot.high_target == Target::None,
            "bot {} targets are reset",
            bot_index
        );

        bot.low_target = low_target;
        bot.high_target = high_target;
    }

    fn visit(&mut self) {
        while let Some(index) = self.process.pop_front() {
            let bot = self.bots[index];

            // part a
            if bot.low == Some(17) && bot.high == Some(61) {
                self.bot_17_61 = index;
            }

            self.set_value(bot.low_target, bot.low);
            self.set_value(bot.high_target, bot.high);
        }
    }

    pub fn get_part_b(&self) -> usize {
        // no easter egg, err, christmas ball hidden in output
        // println!(
        //     "outputs: '{}'",
        //     self.outputs
        //         .iter()
        //         .map(|o| match o {
        //             Some(v) => char::from_u32(*v as u32).unwrap(),
        //             None => '❌',
        //         })
        //         .collect::<String>()
        // );

        self.outputs.iter().take(3).flatten().product()
    }
}

impl Default for Bots {
    fn default() -> Self {
        Self {
            bots: vec![Bot::default(); 500],
            outputs: vec![None; 500],
            process: VecDeque::with_capacity(500),
            bot_17_61: 0,
        }
    }
}

impl FromStr for Bots {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut bots = Bots::default();
        let value_rx = Regex::new(r"\s*value (?P<val>\d+) goes to bot (?P<bot>\d+)").unwrap();
        let lowhigh_rx =
            Regex::new(r"\s*bot (?P<bot>\d+) gives low to (?P<low_to>bot|output) (?P<low_id>\d+) and high to (?P<high_to>bot|output) (?P<high_id>\d+)")
                .unwrap();

        for line in input.lines() {
            // println!("{line}");

            if let Some(caps) = value_rx.captures(line) {
                let val = caps["val"].parse().expect("val should be numeric");
                let bot = caps["bot"].parse().expect("target bot should be numeric");
                bots.set_bot_value(bot, val);
            }

            if let Some(caps) = lowhigh_rx.captures(line) {
                let bot = caps["bot"].parse().expect("target bot should be numeric");
                let low_to = parse_target(&caps["low_to"], &caps["low_id"]);
                let high_to = parse_target(&caps["high_to"], &caps["high_id"]);
                bots.set_targets(bot, low_to, high_to);
            }
        }
        Ok(bots)
    }
}

fn parse_target(to: &str, id_str: &str) -> Target {
    let id = id_str.parse().expect("id should be numeric");

    match to {
        "bot" => Target::Bot(id),
        "output" => Target::Output(id),
        _ => unreachable!("invalid target"),
    }
}

pub fn distribute_chips(input: &str) -> Bots {
    let mut bots = input.parse::<Bots>().expect("definition should be valid");

    bots.visit();

    bots
}
