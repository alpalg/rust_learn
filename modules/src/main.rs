use std::{cmp::Ordering, iter};
use std::{self, write};
use std::collections::*;

mod sound;
mod menu;
mod performance_group;
mod plant;

use crate::sound::sound::instrument::strings;
use self::sound::sound::instrument::woodwind::clarinet;

fn main() {
    // Sounds:
    sound::sound::instrument::woodwind::clarinet();
    sound::sound::instrument::strings::guitar();
    strings::guitar();
    clarinet();

    // Vegetables:
    let tomato: plant::plant::Vegetable = plant::plant::Vegetable::new("Big red tomato!");
    println!("{}", tomato.name);
    // println!("{}", tomato.id);

    // Menu:
    let order1 = menu::menu::Appetizer::Soup;
    let order2 = menu::menu::Appetizer::Salad;

    performance_group::performance_group::music_group();
    performance_group::performance_group::instrument::strings::guitar();
}
