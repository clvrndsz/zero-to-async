use crate::button::ButtonDirection;
use crate::channel::Receiver;
use crate::time::{Ticker, Timer};
use embedded_hal::digital::{OutputPin, StatefulOutputPin};
use fugit::ExtU64;
use microbit::{
    gpio::NUM_COLS,
    hal::gpio::{Output, Pin, PushPull},
};
use rtt_target::rprintln;

enum LedState<'a> {
    Toggle,
    Wait(Timer<'a>),
}

pub struct LedTask<'a> {
    col: [Pin<Output<PushPull>>; NUM_COLS],
    active_col: usize,
    ticker: &'a Ticker,
    state: LedState<'a>,
    receiver: Receiver<'a, ButtonDirection>,
}

impl<'a> LedTask<'a> {
    pub fn new(
        col: [Pin<Output<PushPull>>; NUM_COLS],
        ticker: &'a Ticker,
        receiver: Receiver<'a, ButtonDirection>,
    ) -> Self {
        Self {
            col,
            active_col: 0,
            ticker,
            state: LedState::Toggle,
            receiver,
        }
    }

    fn shift(&mut self, direction: ButtonDirection) {
        rprintln!("Button press detected...");
        self.col[self.active_col].set_high().ok();
        self.active_col = match direction {
            ButtonDirection::Left => match self.active_col {
                0 => 4, //wrap around to the last column when going left from 0
                _ => self.active_col - 1,
            },
            ButtonDirection::Right => match self.active_col {
                4 => 0, //reverse of the previous case
                _ => self.active_col + 1,
            },
        };
        self.col[self.active_col].set_high().ok();
    }

    pub fn poll(&mut self) {
        match self.state {
            LedState::Toggle => {
                rprintln!("Blinking LED {}", self.active_col);
                self.col[self.active_col].toggle().ok();
                //start timer...
                self.state = LedState::Wait(Timer::new(500.millis(), &self.ticker));
            }
            LedState::Wait(ref timer) => {
                if timer.is_ready() {
                    self.state = LedState::Toggle;
                }
                if let Some(direction) = self.receiver.receive() {
                    self.shift(direction);
                    self.state = LedState::Toggle;
                }
            }
        }
    }
}
