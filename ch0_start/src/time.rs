use fugit::{Duration, Instant};
use microbit::{
    pac::interrupt,
    {hal::{rtc::RtcInterrupt, Rtc}, pac::{NVIC, RTC0}}
};


// Type aliasing because he didnt want to type out so much at every call
type TickInstant = Instant<u64, 1, 32768>;
type TickDuration = Duration<u64, 1, 32768>;

// He does the lifetimes here because he is trying to let the borrow
// checker know that the reference to the ticker will live
// as long as the timer at least. I still dont get it :/

pub struct Timer<'a> {
    end_time: TickInstant,
    ticker: &'a Ticker,
}
impl<'a> Timer <'a> {
    
    pub fn new(duration: TickDuration, ticker: &'a Ticker) -> Self {
        Self {
            end_time: ticker.now() + duration,
            ticker,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.ticker.now() >= self.end_time
    }
}


pub struct Ticker {
    rtc: Rtc<RTC0>,
}

impl Ticker {

    pub fn new(rtc0: RTC0, nvic: &mut NVIC) -> Self {
        let  mut rtc = Rtc::new(rtc0,0).unwrap();
        rtc.enable_counter();
        rtc.enable_event(RtcInterrupt::Overflow);
        rtc.enable_interrupt(RtcInterrupt::Overflow, Some(nvic));
        Self {rtc}
    }

    pub fn now(&self) -> TickInstant {
        TickInstant::from_ticks(self.rtc.get_counter() as u64)    }

}

static OVF_COUNT: AtomicU32 = AtomicU32::new(0);

#[interrupt]
// the interrupt handler functions are named after the peripherals
// they interrupt and require the pac::interrupt to give it the right location 
// in the nested vector interrupt counter
fn RTC0() {
    OVF_COUNT.fetch_add(1, Ordering::Relaxed);
}
