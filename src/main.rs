use std::sync::{Arc, Mutex};
use chrono::Local;
use bagagesortering::{Gate, Flyveplan, CheckInSkranke, Reservation, Sorteringsanlaeg, Terminal};
use bagagesortering::{create_flyveplan, create_reservations, create_checkinskranke, create_gates, create_sorteringsanlaeg, create_terminals};


pub fn time() -> String {
    Local::now().format("%d-%m-%Y %H:%M:%S").to_string()
}

fn main() {
    let reservations = create_reservations();
    let flyveplaner = create_flyveplan();
    let skranker = create_checkinskranke();
    let sortering = create_sorteringsanlaeg();
    let terminaler = create_terminals();
    let gates = create_gates();

    println!("{:?}", reservations);
}
