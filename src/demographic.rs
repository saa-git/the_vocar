use std::fmt::Display;

use {
    Class::{Incarcerated, Quintile},
    Race::{Black, White}
};

use rand::{
    distributions::{
        Distribution,
        Standard,
        WeightedIndex
    },
    random, Rng, thread_rng
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Class {
    Quintile(i8),
    Incarcerated,
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Class::Quintile(x) => match x {
                1 => write!(f, "1st Quintile"),
                2 => write!(f, "2nd Quintile"),
                3 => write!(f, "3rd Quintile"),
                4..=5 => write!(f, "{x}th Quintile"),
                other_int => panic!("Err: Impossible Quintile [{other_int}] Found.",),
            },
            Class::Incarcerated => write!(f, "Incarcerated"),

        }
    }
}

impl Distribution<Class> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Class {
        match rng.gen_range(1..=5) {
            1 => Class::Quintile(1),
            2 => Class::Quintile(2),
            3 => Class::Quintile(3),
            4 => Class::Quintile(4),
            _ => Class::Quintile(5),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum Race{
    Black,
    White,
}

impl Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Race::Black => write!(f, "Black"),
            Race::White => write!(f, "White"),
        }
    }
}

impl Distribution<Race> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Race {
        match rng.gen_range(0..=1) {
            0 => Race::Black,
            _ => Race::White,
        }
    }
}

/// Create a demographic from a [Class], history [Vec<Class>] & a [Race].
///
/// # Example
/// ```
/// use vocar::Demo;
/// 
/// let rand_demo = Demo::new();
/// ```
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Demo {
    /// Holds the initial [Class] used to transition.
    pub class_zero: Option<Class>,
    /// Holds the current generation's [Class].
    pub class_n: Option<Class>,
    /// Holds the 5th and final [Class] that was transitioned to.
    pub class_five: Option<Class>,
    /// Holds the [Class]es of previous generations.
    pub history: Vec<Class>,
    /// Holds [Race] which controls the likelihoods of transition to any [Class].
    pub race: Option<Race>,
}

impl Display for Demo {
    /// Prints the current demographic.
    /// Formatted to "`Race`: `Class`"
    /// # Example
    /// ```
    /// use vocar::{
    ///     Demo
    /// };
    /// 
    /// use rand::random();
    /// 
    /// let rand_class_black = Demo { random(), Vec::<Class>::new(), Race::Black };
    ///
    /// println!("{rand_class_black}");
    /// 
    /// // [Output]: "Black: 3rd Quintile"
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{}: {}", self.race, self.class_n);
        match self.race {
            Some(race) => {
                match self.class_n {
                    Some(class) => {
                        write!(f, "{race}: {class}")
                    },
                    None => {
                        write!(f, "{race}: ?")
                    }
                }
            },
            None => {
                write!(f, "Empty Demographic")
            }
        }
    }
}

impl Demo {
    #[must_use]
    pub fn new() -> Self {
        Demo { class_zero: None, class_n: None, class_five: None, history: Vec::<Class>::new(), race: None }
    }

    const BLACK_WEIGHTS: [[f64; 6]; 5] = [
        [32.30, 26.35, 14.45, 7.65, 4.25, 15.00],
        [24.30, 30.60, 22.50, 7.20, 5.40, 10.00],
        [13.02, 22.32, 34.41, 21.39, 1.86, 7.00],
        [10.45, 17.10, 40.85, 21.85, 4.75, 5.00],
        [4.85, 0.97, 15.52, 56.26, 19.40, 3.00],
    ];

    const WHITE_WEIGHTS: [[f64; 6]; 5] = [
        [36.10, 29.45, 16.15, 8.55, 4.75, 5.00],
        [21.34, 32.01, 25.22, 15.52, 2.91, 3.00],
        [12.74, 18.62, 33.32, 22.54, 10.78, 2.00],
        [3.96, 7.92, 19.80, 43.56, 23.76, 1.00],
        [2.97, 0.99, 1.98, 28.75, 64.45, 0.85],
    ];

    /// Change `Demo.class` containing `Class` variant `Incarcerated` to last `Class` in `Demo.2`.
    /// `Demo.2` is a `Vec<Class>`.
    fn leave_incarceration(&mut self) {
        let previous_class = self.history[self.history.len() - 1];

        match previous_class {
            Quintile(mut x) => {
                if self.race == Some(Black) {
                    x -= 2;
                } else {
                    x -= 1;
                }

                let x = x.clamp(1, 5);

                self.history.push(self.class_n.unwrap());
                self.class_n = Some(Quintile(x));
            }
            Incarcerated => panic!("Err:[{self}] Previous State is Incarceration"),
        }
    }

    /// Use Weighted Index Distribution to get a random new Class from the current [Class] and [Race].
    fn new_class(&mut self) {
        if self.race.is_none() {
            self.race = random();
            self.new_class();
        } else {
            let weights = match self.class_n.unwrap() {
                Quintile(x) => match self.race.unwrap() {
                    Black => {
                        let x: usize = (x - 1).try_into().unwrap();
                        Self::BLACK_WEIGHTS[x]
                    }
                    White => {
                        let x: usize = (x - 1).try_into().unwrap();
                        Self::WHITE_WEIGHTS[x]
                    }
                },
                Incarcerated => {
                    panic!("Err:[{self} Incarcerated Demos Should Never Reach This Function")
                }
            };

            let items = [
                (Quintile(1), weights[0]),
                (Quintile(2), weights[1]),
                (Quintile(3), weights[2]),
                (Quintile(4), weights[3]),
                (Quintile(5), weights[4]),
                (Incarcerated, weights[5]),
            ];

            let dist = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();

            let new_class: Class = items[dist.sample(&mut thread_rng())].0;

            self.history.push(new_class);
            self.class_n = Some(new_class);
        }
    }

    pub fn next_gen(&mut self) {
        match self.class_n {
            Some(Quintile(_)) => self.new_class(),
            Some(Incarcerated) => self.leave_incarceration(),
            None => {}
        }
    }

    pub fn reset(&mut self) {
        self.class_zero = None;
        self.class_n = None;
        self.class_five = None;
        self.history.clear();
    }
}