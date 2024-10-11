#![no_std]
#![no_main]

use core::borrow::BorrowMut;
use core::cell::RefCell;
use core::mem::MaybeUninit;
use core::num;
use core::assert;

use embassy_boot::BootLoaderConfig;
use embassy_executor::Spawner;
use embassy_stm32::lptim::pwm::Ch1;
use embassy_stm32::peripherals::{self, TIM1, TIM3};
use embassy_stm32::time::{Hertz};
use embassy_stm32::timer::{Channel};
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::gpio::{AnyPin, Input, Level, Output, Pin, Pull, Speed, OutputType};
use embassy_time::{Duration, Timer, Instant};
// pick a panicking behavior
use panic_halt as _;


use cortex_m_semihosting::{hprintln};

macro_rules! debug_hprintln {
    ($($e:expr),+) => {
        {
            #[cfg(debug_assertions)]
            {
                hprintln!($($e),+)
            }
        }
    };
}
    
// #[embassy_executor::task(pool_size=3)]
// async fn blink(pwm: RefCell<SimplePwm<'static, TIM3>>, channel: Channel) {
//     let mut intensity = 0;

//     pwm.borrow_mut().enable(channel);

//     loop {
//         intensity += 1;

//         let duty = (pwm.borrow().get_max_duty() * intensity) / 100;
//         pwm.borrow_mut().set_duty(channel, duty);

//         if intensity > 100 {
//             intensity = 0;
//         }

//         Timer::after_millis(10).await;
//     }
// }

// #[embassy_executor::main]
// async fn main(spawner: Spawner) {
//     let p = embassy_stm32::init(Default::default());
    
//     let pwm = RefCell::new(SimplePwm::new(
//         p.TIM3,  
//         None,
//         None,
//         Some(PwmPin::new_ch3(p.PB0, OutputType::PushPull)), 
//         None, 
//         Hertz::hz(100),
//         embassy_stm32::timer::low_level::CountingMode::CenterAlignedBothInterrupts
//     ));

//     // debug_hprintln!("Hello world!");

//     spawner.spawn(blink(pwm, Channel::Ch1)).unwrap(); // Green LED
//     // spawner.spawn(blink(p.PE1.degrade())).unwrap();
//     // spawner.spawn(blink(p.PB14.degrade())).unwrap(); 
// }

#[embassy_executor::task(pool_size=3)]
async fn blink(pwm: &'static mut SimplePwm<'static, TIM3>, channel: Channel) {
    let mut intensity = 0;

    pwm.enable(channel);

    let mut dir: i32 = 1;

    loop {
        intensity += dir;

        let duty = (pwm.get_max_duty() * (intensity as u32))/100;
        pwm.set_duty(channel, duty);


        if dir == 1 && intensity >= 50 {
            dir = -1;
        }

        if dir == -1 && intensity <= 1 {
            dir = 1;
        }

        Timer::after_millis(10).await;
    }
}   

type IdFilter = fn(u32) -> bool; 

static mut PWM: MaybeUninit<SimplePwm<'static, TIM3>> = MaybeUninit::uninit();
static mut i: u32 = 0;
static mut lookup: [[u32; 32]; 64] = [[0; 32]; 64];
static mut lookup_func: [IdFilter; 64] = [|_| { false }; 64];

static num_subs: usize = 3;


fn linear_lookup_enumerate(id: u32) {
    for (sub_num, component) in unsafe{ lookup.iter().enumerate() } {
        if sub_num == num_subs { break; }
        for subscribed in component.iter() {
            if (*subscribed == 0) { break; }
            if (*subscribed == id) {
                unsafe {i += 1;}
                break;
            }
        }
    }
}

fn linear_lookup_take(id: u32) {
    // assert!(unsafe{ lookup.len() > num_subs });
    
    for component in unsafe { &lookup[0..num_subs] } {
        for subscribed in component.iter() {
            if (*subscribed == 0) { break; }
            if (*subscribed == id) {
                unsafe {i += 1;}
                break;
            }
        }
    }
}

fn function_lookup(id: u32) {
    for component_pred in unsafe{ &lookup_func[0..num_subs] } {
        if component_pred(id) {
            unsafe {i += 1;}
        }
    }
}

fn pred_1(id: u32) -> bool {
    match id {
        1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 => true,
        _ => false
    }   
}

fn pred_2(id: u32) -> bool {
    id > 0 && id < 22
}

static lookup_set: phf::Set<u32> = phf::phf_set!{ 
    1u32, 
    2u32, 
    3u32, 
    4u32, 
    5u32, 
    6u32, 
    7u32, 
    8u32, 
    9u32, 
    10u32, 
    11u32, 
    12u32, 
    13u32, 
    14u32, 
    15u32, 
    16u32, 
    17u32, 
    18u32, 
    19u32, 
    20u32, 
    21u32 
};

fn set_lookup(id: u32) {
    for j in 0..num_subs {
        if lookup_set.contains(&id) {
            unsafe { i += 1;}
        }
    }
    
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {

    unsafe {
        lookup[0][0] = 21;
        lookup[0][1] = 1;
        lookup[0][2] = 2;
        lookup[0][3] = 3;
        lookup[0][4] = 4;
        lookup[0][5] = 5;
        lookup[0][6] = 6;
        lookup[0][7] = 7;
        lookup[0][8] = 8;
        lookup[0][9] = 9;
        lookup[0][10] = 10;
        lookup[0][11] = 11;
        lookup[0][12] = 12;
        lookup[0][13] = 13;
        lookup[0][14] = 14;
        lookup[0][15] = 15;
        lookup[0][16] = 16;
        lookup[0][17] = 17;
        lookup[0][18] = 18;
        lookup[0][19] = 19;
        lookup[0][20] = 20;

        lookup[1][0] = 21;
        lookup[1][1] = 1;
        lookup[1][2] = 2;
        lookup[1][3] = 3;
        lookup[1][4] = 4;
        lookup[1][5] = 5;
        lookup[1][6] = 6;
        lookup[1][7] = 7;
        lookup[1][8] = 8;
        lookup[1][9] = 9;
        lookup[1][10] = 10;
        lookup[1][11] = 11;
        lookup[1][12] = 12;
        lookup[1][13] = 13;
        lookup[1][14] = 14;
        lookup[1][15] = 15;
        lookup[1][16] = 16;
        lookup[1][17] = 17;
        lookup[1][18] = 18;
        lookup[1][19] = 19;
        lookup[1][20] = 20;

        lookup[2][0] = 21;
        lookup[2][1] = 1;
        lookup[2][2] = 2;
        lookup[2][3] = 3;
        lookup[2][4] = 4;
        lookup[2][5] = 5;
        lookup[2][6] = 6;
        lookup[2][7] = 7;
        lookup[2][8] = 8;
        lookup[2][9] = 9;
        lookup[2][10] = 10;
        lookup[2][11] = 11;
        lookup[2][12] = 12;
        lookup[2][13] = 13;
        lookup[2][14] = 14;
        lookup[2][15] = 15;
        lookup[2][16] = 16;
        lookup[2][17] = 17;
        lookup[2][18] = 18;
        lookup[2][19] = 19;
        lookup[2][20] = 20;
    }


    let p = embassy_stm32::init(Default::default());

    let init_start = Instant::now();

    for id in 0..27000 {
        linear_lookup_enumerate(id);
    }

    let init_finish = Instant::now();

    hprintln!("Linear lookup enumerate: {}us", (init_finish-init_start).as_micros());
    hprintln!("i value: {}", unsafe{i});

    unsafe {i = 0;}

    let init_start = Instant::now();

    for id in 0..27000 {
        linear_lookup_take(id);
    }

    let init_finish = Instant::now();

    hprintln!("Linear lookup take: {}us", (init_finish-init_start).as_micros());
    hprintln!("i value: {}", unsafe{i});

    unsafe {i = 0;}

    unsafe {
        lookup_func[0] = pred_1;
        lookup_func[1] = pred_1;
        lookup_func[2] = pred_1;
    }


    let init_start = Instant::now();

    for id in 0..27000 {
        function_lookup(id);
    }

    let init_finish = Instant::now();

    hprintln!("Function 1 lookup: {}us", (init_finish-init_start).as_micros());
    hprintln!("i value: {}", unsafe{i});

    unsafe {i = 0;}

    unsafe {
        lookup_func[0] = pred_2;
        lookup_func[1] = pred_2;
        lookup_func[2] = pred_2;
    }


    let init_start = Instant::now();

    for id in 0..27000 {
        function_lookup(id);
    }

    let init_finish = Instant::now();

    hprintln!("Function 2 lookup: {}us", (init_finish-init_start).as_micros());
    hprintln!("i value: {}", unsafe{i});

    unsafe {i = 0;}

    let init_start = Instant::now();

    for id in 0..27000 {
        set_lookup(id);
    }

    let init_finish = Instant::now();

    hprintln!("Set lookup: {}us", (init_finish-init_start).as_micros());
    hprintln!("i value: {}", unsafe{i});



}
