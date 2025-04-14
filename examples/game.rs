#![no_std]
#![no_main]

#[cfg(feature = "trace")]
use defmt::info;

#[cfg(feature = "trace")]
use defmt_rtt as _;

use embassy_stm32::{
    gpio::{Level, Output, Speed},
    rcc::{
        AHBPrescaler, APBPrescaler, Pll, PllMul, PllPDiv, PllPreDiv, PllRDiv, PllSource, Sysclk,
        mux::Adcsel,
    },
};
use mario_rust::{AliveStates, BigMarioStates, Mario, MarioConsummables, States};
use panic_halt as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut mario = Mario::new(States::AliveMario(AliveStates::SmallMario));

    // Prepare HAL configuration
    let mut config = embassy_stm32::Config::default();
    config.rcc.sys = Sysclk::PLL1_R;
    config.rcc.pll = Some(Pll {
        source: PllSource::HSI,
        prediv: PllPreDiv::DIV1,
        mul: PllMul::MUL9,
        divp: Some(PllPDiv::DIV24),
        divq: None,
        divr: Some(PllRDiv::DIV3),
    });
    config.rcc.ahb_pre = AHBPrescaler::DIV1;
    config.rcc.apb1_pre = APBPrescaler::DIV1;
    config.rcc.mux.adcsel = Adcsel::PLL1_P;

    // Init the HAL getting all the peripherals
    let p = embassy_stm32::init(config);

    // Init debug pin
    let mut debug_pin = Output::new(p.PA2, Level::Low, Speed::VeryHigh);

    #[cfg(feature = "trace")]
    info!("Start");

    // Initial state
    assert!(mario.current_state() == States::AliveMario(AliveStates::SmallMario));
    assert!(mario.number_of_lifes() == 1);
    assert!(mario.number_of_coins() == 0);
    assert!(mario.is_alive());
    #[cfg(feature = "trace")]
    info!(
        "\n\tState: {}\n\tAlive: {}\n\tLifes: {}\n\tCoins: {}",
        mario.current_state(),
        mario.is_alive(),
        mario.number_of_lifes(),
        mario.number_of_coins()
    );

    // Get a mushroom
    mario.get_consummable(MarioConsummables::Mushroom);
    assert!(
        mario.current_state()
            == States::AliveMario(AliveStates::BigMario(BigMarioStates::SuperMario))
    );
    assert!(mario.number_of_lifes() == 1);
    assert!(mario.number_of_coins() == 100);
    assert!(mario.is_alive());
    #[cfg(feature = "trace")]
    info!(
        "\n\tState: {}\n\tAlive: {}\n\tLifes: {}\n\tCoins: {}",
        mario.current_state(),
        mario.is_alive(),
        mario.number_of_lifes(),
        mario.number_of_coins()
    );

    // Get a hit
    mario.get_hit();
    assert!(mario.current_state() == States::AliveMario(AliveStates::SmallMario));
    assert!(mario.number_of_lifes() == 1);
    assert!(mario.number_of_coins() == 0);
    assert!(mario.is_alive());
    #[cfg(feature = "trace")]
    info!(
        "\n\tState: {}\n\tAlive: {}\n\tLifes: {}\n\tCoins: {}",
        mario.current_state(),
        mario.is_alive(),
        mario.number_of_lifes(),
        mario.number_of_coins()
    );

    // Get a flower
    mario.get_consummable(MarioConsummables::Flower);
    assert!(
        mario.current_state()
            == States::AliveMario(AliveStates::BigMario(BigMarioStates::FireMario))
    );
    assert!(mario.number_of_lifes() == 1);
    assert!(mario.number_of_coins() == 200);
    assert!(mario.is_alive());
    #[cfg(feature = "trace")]
    info!(
        "\n\tState: {}\n\tAlive: {}\n\tLifes: {}\n\tCoins: {}",
        mario.current_state(),
        mario.is_alive(),
        mario.number_of_lifes(),
        mario.number_of_coins()
    );

    // Get a feather
    mario.get_consummable(MarioConsummables::Feather);
    assert!(
        mario.current_state()
            == States::AliveMario(AliveStates::BigMario(BigMarioStates::CapeMario))
    );
    assert!(mario.number_of_lifes() == 1);
    assert!(mario.number_of_coins() == 500);
    assert!(mario.is_alive());
    #[cfg(feature = "trace")]
    info!(
        "\n\tState: {}\n\tAlive: {}\n\tLifes: {}\n\tCoins: {}",
        mario.current_state(),
        mario.is_alive(),
        mario.number_of_lifes(),
        mario.number_of_coins()
    );

    // Some more feathers until +1 life
    mario.get_consummable(MarioConsummables::Feather);
    mario.get_consummable(MarioConsummables::Feather);
    assert!(
        mario.current_state()
            == States::AliveMario(AliveStates::BigMario(BigMarioStates::CapeMario))
    );
    assert!(mario.number_of_lifes() == 2);
    assert!(mario.number_of_coins() == 0);
    assert!(mario.is_alive());
    #[cfg(feature = "trace")]
    info!(
        "\n\tState: {}\n\tAlive: {}\n\tLifes: {}\n\tCoins: {}",
        mario.current_state(),
        mario.is_alive(),
        mario.number_of_lifes(),
        mario.number_of_coins()
    );

    // Get a hit
    mario.get_hit();
    assert!(mario.current_state() == States::AliveMario(AliveStates::SmallMario));
    assert!(mario.number_of_lifes() == 2);
    assert!(mario.number_of_coins() == 0);
    assert!(mario.is_alive());
    #[cfg(feature = "trace")]
    info!(
        "\n\tState: {}\n\tAlive: {}\n\tLifes: {}\n\tCoins: {}",
        mario.current_state(),
        mario.is_alive(),
        mario.number_of_lifes(),
        mario.number_of_coins()
    );

    // Another one
    mario.get_hit();
    assert!(mario.current_state() == States::AliveMario(AliveStates::SmallMario));
    assert!(mario.number_of_lifes() == 1);
    assert!(mario.number_of_coins() == 0);
    assert!(mario.is_alive());
    #[cfg(feature = "trace")]
    info!(
        "\n\tState: {}\n\tAlive: {}\n\tLifes: {}\n\tCoins: {}",
        mario.current_state(),
        mario.is_alive(),
        mario.number_of_lifes(),
        mario.number_of_coins()
    );

    // Oh no
    mario.get_hit();
    assert!(mario.current_state() == States::DeadMario);
    assert!(mario.number_of_lifes() == 0);
    assert!(mario.number_of_coins() == 0);
    assert!(!mario.is_alive());
    #[cfg(feature = "trace")]
    info!(
        "\n\tState: {}\n\tAlive: {}\n\tLifes: {}\n\tCoins: {}",
        mario.current_state(),
        mario.is_alive(),
        mario.number_of_lifes(),
        mario.number_of_coins()
    );

    #[cfg(feature = "trace")]
    info!("End");

    if mario.current_state() == States::DeadMario
        && mario.number_of_lifes() == 0
        && mario.number_of_coins() == 0
        && !mario.is_alive()
    {
        debug_pin.set_high();
    }

    loop {
        cortex_m::asm::nop();
    }
}
