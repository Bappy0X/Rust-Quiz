#![allow(non_snake_case)]

use std::io::{stdin};

pub struct Game;

impl Game {
    pub fn new() -> Self {
        Game {}
    }

    pub fn take_input(&self, incentive: String) -> String {
        println!("{}", incentive);
        let mut input = String::new();

        stdin().read_line(&mut input).expect("Did not enter a correct string");

        let mut index = 0;
        input.retain(|i| {
            index += 1;
            //println!("{}", i as u32);
            i != '\n' && i != '\r' && index <= 4
        });

        return input as String;
    }
}

pub struct Player {
    name: String,
    score: i32
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            score: 32,
            name: name.to_string()
        }
    }

    pub fn print_score(&self) {
        println!("{} has a score of {}!", self.name, self.score);
    }
    
    pub fn add_score(&self, score: i32) {
        self.score += score
    }

    pub fn ask_question(&self, game: &Game) -> bool {
        println!("{}, this is your question!", self.name);
        let answer = game.take_input("What is my name?".to_string());
        let correct = answer.to_ascii_lowercase() == "josh";
        match correct {
            true => {println!("Correct!"); *self.add_score(5)},
            false => println!("Sorry, incorrect!")
        };
        return true
    }
}