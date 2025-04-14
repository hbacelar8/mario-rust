#![no_std]

#[cfg(feature = "trace")]
use defmt::Format;

use rustfsm::{StateBehavior, rustfsm};

#[derive(Clone, Copy, PartialEq)]
pub enum MarioConsummables {
    Mushroom,
    Flower,
    Feather,
}

impl MarioConsummables {
    fn value(&self) -> u16 {
        match self {
            Self::Mushroom => 100,
            Self::Flower => 200,
            Self::Feather => 300,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "trace", derive(Format))]
pub enum AliveStates {
    SmallMario,
    BigMario(BigMarioStates),
}

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "trace", derive(Format))]
pub enum BigMarioStates {
    SuperMario,
    CapeMario,
    FireMario,
}

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "trace", derive(Format))]
pub enum States {
    AliveMario(AliveStates),
    DeadMario,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Events {
    GetConsummables(MarioConsummables),
    GetHit,
}

pub struct Context {
    number_of_lifes: u8,
}

impl Default for Context {
    fn default() -> Self {
        Self { number_of_lifes: 1 }
    }
}

// Generate the state machine
rustfsm!(
    Mario {
        number_of_coins: u16 = 0
    },
    States,
    Events,
    Context
);

// Implement state behavior
impl StateBehavior for States {
    type State = States;
    type Event = Events;
    type Context = Context;

    fn handle(&self, event: &Self::Event, context: &mut Self::Context) -> Option<Self::State> {
        use AliveStates::*;
        use BigMarioStates::*;
        use Events::*;
        use MarioConsummables::*;
        use States::*;

        match (self, event) {
            (AliveMario(SmallMario), GetConsummables(Mushroom)) => {
                Some(AliveMario(BigMario(SuperMario)))
            }

            (
                AliveMario(SmallMario)
                | AliveMario(BigMario(SuperMario))
                | AliveMario(BigMario(CapeMario)),
                GetConsummables(Flower),
            ) => Some(AliveMario(BigMario(FireMario))),

            (
                AliveMario(SmallMario)
                | AliveMario(BigMario(SuperMario))
                | AliveMario(BigMario(FireMario)),
                GetConsummables(Feather),
            ) => Some(AliveMario(BigMario(CapeMario))),

            (AliveMario(SmallMario), GetHit) => {
                context.number_of_lifes -= 1;

                if context.number_of_lifes == 0 {
                    Some(DeadMario)
                } else {
                    None
                }
            }

            (AliveMario(BigMario(_)), GetHit) => Some(AliveMario(SmallMario)),

            _ => None,
        }
    }
}

impl Mario {
    pub fn is_alive(&self) -> bool {
        self.current_state != States::DeadMario
    }

    pub fn number_of_lifes(&self) -> u8 {
        self.context.number_of_lifes
    }

    pub fn number_of_coins(&self) -> u16 {
        self.number_of_coins
    }

    pub fn get_consummable(&mut self, consummable: MarioConsummables) {
        self.number_of_coins += consummable.value();

        if self.number_of_coins >= 1000 {
            self.number_of_coins = 0;
            self.context.number_of_lifes += 1;
        }

        self.handle(Events::GetConsummables(consummable));
    }

    pub fn get_hit(&mut self) {
        self.number_of_coins = 0;
        self.handle(Events::GetHit);
    }
}
