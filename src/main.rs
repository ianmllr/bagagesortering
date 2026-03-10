use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;

use bagagesortering::{
    create_flyveplan, create_reservations, create_checkinskranke,
    create_gates, create_terminals,
    CheckInSkranke, Sorteringsanlaeg, Gate, Reservation, Flyveplan, Terminal
};

pub fn time() -> String {
    Local::now().format("%d-%m-%Y %H:%M:%S").to_string()
}

fn log_to_file(message: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("bagagesortering.log")
        .unwrap();
    writeln!(file, "{}", message).unwrap();
}

struct CheckInCounter {
    id: i32,
    baggage_log: Arc<Mutex<Vec<String>>>,
    sortering_buffer: Arc<Mutex<Vec<Sorteringsanlaeg>>>,
    buffer_kapacitet: usize,
}

impl CheckInCounter {
    fn new(
        id: i32,
        baggage_log: Arc<Mutex<Vec<String>>>,
        sortering_buffer: Arc<Mutex<Vec<Sorteringsanlaeg>>>,
        buffer_kapacitet: usize,
    ) -> Self {
        Self { id, baggage_log, sortering_buffer, buffer_kapacitet }
    }

    fn process_baggage(&self, skranke: &CheckInSkranke) {
        if !skranke.open {
            let msg = format!("[{}] Skranke {} er lukket, springer over bagage {}", time(), self.id, skranke.baggage_number);
            println!("{}", msg);
            log_to_file(&msg);
            return;
        }

        {
            let mut log = self.baggage_log.lock().unwrap();
            let entry = format!(
                "[{}] Skranke {} registrerede bagage {} for passager {}",
                time(), self.id, skranke.baggage_number, skranke.passenger_number
            );
            log.push(entry.clone());
            println!("{}", entry);
            log_to_file(&entry);
        }

        {
            let mut buffer = self.sortering_buffer.lock().unwrap();
            if buffer.len() >= self.buffer_kapacitet {
                let msg = format!("[{}] Sorteringsbuffer er fuld! Bagage {} afventer.", time(), skranke.baggage_number);
                println!("{}", msg);
                log_to_file(&msg);
            } else {
                let ny_bagage = Sorteringsanlaeg {
                    baggage_number: skranke.baggage_number,
                    check_in_time: time(),
                    check_out_time: String::new(),
                };
                buffer.push(ny_bagage);
                let msg = format!("[{}] Bagage {} sendt til sorteringsanlæg.", time(), skranke.baggage_number);
                println!("{}", msg);
                log_to_file(&msg);
            }
        }

        thread::sleep(Duration::from_millis(200));
    }
}

struct Sortering {
    sortering_buffer: Arc<Mutex<Vec<Sorteringsanlaeg>>>,
    gate_buffer: Arc<Mutex<Vec<Gate>>>,
    buffer_kapacitet: usize,
}

fn print_reservationer(reservations: &Vec<Reservation>) {
    println!("Reservationer");
    for r in reservations {
        println!("  Passager {}: {} ({})", r.passenger_number, r.passenger_name, r.timestamp);
    }
    println!();
}

fn print_flyveplan(flyveplaner: &Vec<Flyveplan>) {
    println!("Flyveplan");
    for f in flyveplaner {
        println!("  Afgangstid: {} | Terminal: {}", f.flyafgang, f.terminalnummer);
    }
    println!();
}

fn print_terminaler(terminaler: &Vec<Terminal>) {
    println!("Terminaler");
    for t in terminaler {
        println!("  Terminal {}: {} gates", t.terminal_number, t.number_of_gates);
    }
    println!();
}

fn main() {
    let reservations = create_reservations();
    let flyveplaner = create_flyveplan();
    let skranker = create_checkinskranke();
    let terminaler = create_terminals();
    let gates = create_gates();

    print_reservationer(&reservations);
    print_flyveplan(&flyveplaner);
    print_terminaler(&terminaler);

    let baggage_log: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let sortering_buffer: Arc<Mutex<Vec<Sorteringsanlaeg>>> = Arc::new(Mutex::new(Vec::new()));
    let gate_buffer: Arc<Mutex<Vec<Gate>>> = Arc::new(Mutex::new(gates));

    let buffer_kapacitet = 10;

    let start = Instant::now();



}