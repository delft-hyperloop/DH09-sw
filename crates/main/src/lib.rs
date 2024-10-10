#![no_std]

#![feature(type_alias_impl_trait)]

mod can;

/// Main task responsible for communicating with the ground station.
#[embassy_executor::task]
async fn gs_main() -> ! {
    loop {
        
    }
}
