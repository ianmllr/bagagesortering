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

impl Sortering {
    fn new(
        sortering_buffer: Arc<Mutex<Vec<Sorteringsanlaeg>>>,
        gate_buffer: Arc<Mutex<Vec<Gate>>>,
        buffer_kapacitet: usize,
    ) -> Self {
        Self { sortering_buffer, gate_buffer, buffer_kapacitet }
    }

    fn sorter(&self) {
        loop {
            let naeste = {
                let mut buffer = self.sortering_buffer.lock().unwrap();
                if buffer.is_empty() {
                    None
                } else {
                    Some(buffer.remove(0))
                }
            };

            match naeste {
                None => {
                    thread::sleep(Duration::from_millis(100));
                }
                Some(mut bagage) => {
                    thread::sleep(Duration::from_millis(300));

                    bagage.check_out_time = time();

                    let msg = format!(
                        "[{}] Sorteringsanlæg: Bagage {} sorteret. Ind: {} - Ud: {}",
                        time(), bagage.baggage_number, bagage.check_in_time, bagage.check_out_time
                    );
                    println!("{}", msg);
                    log_to_file(&msg);

                    let mut gates = self.gate_buffer.lock().unwrap();
                    if gates.len() >= self.buffer_kapacitet {
                        let msg = format!("[{}] Gate-buffer er fuld! Bagage {} kan ikke leveres.", time(), bagage.baggage_number);
                        println!("{}", msg);
                        log_to_file(&msg);
                    } else {
                        let aaben_gate = gates.iter_mut().find(|g| g.gate_open_closed);
                        match aaben_gate {
                            Some(gate) => {
                                gate.baggage_number = bagage.baggage_number;
                                gate.timestamp = time();
                                let msg = format!(
                                    "[{}] Bagage {} leveret til gate {}.",
                                    time(), bagage.baggage_number, gate.gate_number
                                );
                                println!("{}", msg);
                                log_to_file(&msg);
                            }
                            None => {
                                let msg = format!("[{}] Ingen åbne gates til bagage {}!", time(), bagage.baggage_number);
                                println!("{}", msg);
                                log_to_file(&msg);
                            }
                        }
                    }
                }
            }

            let sortering_tom = self.sortering_buffer.lock().unwrap().is_empty();
            if sortering_tom {
                break;
            }
        }
    }
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

    let mut handles = vec![];

    for (i, skranke) in skranker.into_iter().enumerate() {
        let log_klon = Arc::clone(&baggage_log);
        let sortering_klon = Arc::clone(&sortering_buffer);

        let counter = CheckInCounter::new(
            (i + 1) as i32,
            log_klon,
            sortering_klon,
            buffer_kapacitet,
        );

        let handle = thread::spawn(move || {
            counter.process_baggage(&skranke);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let sortering_klon = Arc::clone(&sortering_buffer);
    let gate_klon = Arc::clone(&gate_buffer);

    let sortering_thread = thread::spawn(move || {
        let sortering = Sortering::new(sortering_klon, gate_klon, buffer_kapacitet);
        sortering.sorter();
    });

    sortering_thread.join().unwrap();

    let varighed = start.elapsed();
    let antal_behandlet = baggage_log.lock().unwrap().len();
    let throughput = antal_behandlet as f64 / varighed.as_secs_f64();

    println!();
    println!("Behandlet bagage: {}", antal_behandlet);
    println!("Tid brugt: {:?}", varighed);
    println!("Throughput: {:.2} enheder/sek", throughput);

    let maaling = format!(
        "Behandlet bagage: {}\nTid brugt: {:?}\nThroughput: {:.2} enheder/sek",
        antal_behandlet, varighed, throughput
    );
    log_to_file(&maaling);

    println!();
    println!("Bagagelog");
    let log = baggage_log.lock().unwrap();
    for entry in log.iter() {
        println!("{}", entry);
    }

    println!();
    println!("Gate status");
    let gates_final = gate_buffer.lock().unwrap();
    for gate in gates_final.iter() {
        let status = if gate.gate_open_closed { "åben" } else { "lukket" };
        println!("Gate {}: {} | Bagage: {} | Tidsstempel: {}", gate.gate_number, status, gate.baggage_number, gate.timestamp);
    }
}