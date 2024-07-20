#![feature(try_trait_v2)]

use core::fmt;
use std::{
    fmt::{Display, Formatter},
    ops::{ControlFlow, FromResidual, Try},
};

#[derive(Debug)]
pub struct Hand(i32);

#[derive(Debug)]
pub enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Card {
    fn draw() -> Self {
        match rand::random::<u8>() % 13 {
            0 => Card::Ace,
            1 => Card::Two,
            2 => Card::Three,
            3 => Card::Four,
            4 => Card::Five,
            5 => Card::Six,
            6 => Card::Seven,
            7 => Card::Eight,
            8 => Card::Nine,
            9 => Card::Ten,
            10 => Card::Jack,
            11 => Card::Queen,
            12 => Card::King,
            _ => unreachable!(),
        }
    }
}

impl Hand {
    fn hit(mut self, x: Card) -> Self {
        let value = match x {
            Card::Ace => 1,
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Ten | Card::Jack | Card::Queen | Card::King => 10,
        };
        self.0 += value;
        self
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.0 {
            21 => write!(f, "Blackjack!"),
            x if x > 21 => write!(f, "Bust!"),
            x => write!(f, "{}", x),
        }
    }
}

impl Try for Hand {
    type Output = Self;
    type Residual = Self;

    fn from_output(output: Self::Output) -> Self {
        output
    }

    fn branch(self) -> ControlFlow<Self, Self::Output> {
        if self.0 >= 17 {
            ControlFlow::Break(self)
        } else {
            ControlFlow::Continue(self)
        }
    }
}

impl FromResidual for Hand {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        residual
    }
}

fn play() -> Hand {
    let mut hand = Hand(0);
    loop {
        let card = Card::draw();
        println!("{:?}", card);
        hand = hand.hit(card)?;
    }
}

fn main() {
    let score = play();
    println!("Game over. Dealer {}", score);
}
