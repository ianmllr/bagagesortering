use chrono::Local;

#[derive(Debug, Clone)]
pub struct Reservation {
    pub passenger_number: i32,
    pub passenger_name: String,
    pub timestamp: String,
}

#[derive(Debug, Clone)]
pub struct Flyveplan {
    pub flyafgang: String,
    pub terminalnummer: i32,
}

#[derive(Debug, Clone)]
pub struct CheckInSkranke {
    pub open: bool,
    pub passenger_number: i32,
    pub baggage_number: i32,
    pub timestamp: String,
}

#[derive(Debug, Clone)]
pub struct Sorteringsanlaeg {
    pub baggage_number: i32,
    pub check_in_time: String,
    pub check_out_time: String,
}

#[derive(Debug, Clone)]
pub struct Terminal {
    pub terminal_number: i32,
    pub number_of_gates: i32,
}

#[derive(Debug, Clone)]
pub struct Gate {
    pub gate_number: i32,
    pub gate_open_closed: bool,
    pub baggage_number: i32,
    pub timestamp: String,
}

// hardcodede eksempel-reservationer så vi har noget data at arbejde med
pub fn create_reservations() -> Vec<Reservation> {
    let timestamp = Local::now().format("%d-%m-%Y %H:%M:%S").to_string();

    vec![
        Reservation {
            passenger_number: 1,
            passenger_name: String::from("Ian Møller"),
            timestamp: timestamp.clone(),
        },
        Reservation {
            passenger_number: 2,
            passenger_name: String::from("Joachim Nielsen"),
            timestamp: timestamp.clone(),
        },
        Reservation {
            passenger_number: 3,
            passenger_name: String::from("Casper"),
            timestamp: timestamp.clone(),
        },
        Reservation {
            passenger_number: 4,
            passenger_name: String::from("Thomas"),
            timestamp: timestamp.clone(),
        },
        Reservation {
            passenger_number: 5,
            passenger_name: String::from("Jon Bernild"),
            timestamp: timestamp.clone(),
        },
        Reservation {
            passenger_number: 6,
            passenger_name: String::from("Emil Stegmann"),
            timestamp: timestamp.clone(),
        },
        Reservation {
            passenger_number: 7,
            passenger_name: String::from("Brasiliansk Funk Enjoyer"),
            timestamp: timestamp.clone(),
        },
    ]
}

pub fn create_flyveplan() -> Vec<Flyveplan> {
    vec![
        Flyveplan {
            flyafgang: String::from("27-02-2026 14:30:00"),
            terminalnummer: 1,
        },
        Flyveplan {
            flyafgang: String::from("27-02-2026 21:45:00"),
            terminalnummer: 2,
        },
        Flyveplan {
            flyafgang: String::from("02-03-2026 11:00:00"),
            terminalnummer: 3,
        },
        Flyveplan {
            flyafgang: String::from("04-03-2026 15:30:00"),
            terminalnummer: 1,
        },
        Flyveplan {
            flyafgang: String::from("26-05-2026 19:00:00"),
            terminalnummer: 3,
        },
    ]
}

pub fn create_checkinskranke() -> Vec<CheckInSkranke> {
    let timestamp = Local::now().format("%d-%m-%Y %H:%M:%S").to_string();

    vec![
        CheckInSkranke {
            open: true,
            passenger_number: 32,
            baggage_number: 12,
            timestamp: timestamp.clone(),
        },
        CheckInSkranke {
            open: true,
            passenger_number: 196,
            baggage_number: 1254,
            timestamp: timestamp.clone(),
        },
        CheckInSkranke {
            open: true,
            passenger_number: 124,
            baggage_number: 6187,
            timestamp: timestamp.clone(),
        },
        CheckInSkranke {
            open: true,
            passenger_number: 1241,
            baggage_number: 654,
            timestamp: timestamp.clone(),
        },
        CheckInSkranke {
            open: false,
            passenger_number: 1,
            baggage_number: 22,
            timestamp: timestamp.clone(),
        },
    ]
}

pub fn create_sorteringsanlaeg() -> Vec<Sorteringsanlaeg> {
    let timestamp = Local::now().format("%d-%m-%Y %H:%M:%S").to_string();

    vec![
        Sorteringsanlaeg {
            baggage_number: 12,
            check_in_time: timestamp.clone(),
            check_out_time: String::new(), // endnu ikke sorteret
        },
        Sorteringsanlaeg {
            baggage_number: 1254,
            check_in_time: timestamp.clone(),
            check_out_time: String::new(),
        },
        Sorteringsanlaeg {
            baggage_number: 6187,
            check_in_time: timestamp.clone(),
            check_out_time: String::new(),
        },
        Sorteringsanlaeg {
            baggage_number: 654,
            check_in_time: timestamp.clone(),
            check_out_time: String::new(),
        },
        Sorteringsanlaeg {
            baggage_number: 22,
            check_in_time: timestamp.clone(),
            check_out_time: String::new(),
        },
    ]
}

pub fn create_terminals() -> Vec<Terminal> {
    vec![
        Terminal {
            terminal_number: 1,
            number_of_gates: 3,
        },
        Terminal {
            terminal_number: 2,
            number_of_gates: 2,
        },
        Terminal {
            terminal_number: 3,
            number_of_gates: 2,
        },
    ]
}

pub fn create_gates() -> Vec<Gate> {
    let timestamp = Local::now().format("%d-%m-%Y %H:%M:%S").to_string();

    vec![
        Gate {
            gate_number: 1,
            gate_open_closed: true,
            baggage_number: 0,
            timestamp: timestamp.clone(),
        },
        Gate {
            gate_number: 2,
            gate_open_closed: true,
            baggage_number: 0,
            timestamp: timestamp.clone(),
        },
        Gate {
            gate_number: 3,
            gate_open_closed: true,
            baggage_number: 0,
            timestamp: timestamp.clone(),
        },
        Gate {
            gate_number: 4,
            gate_open_closed: false,
            baggage_number: 0,
            timestamp: timestamp.clone(),
        },
        Gate {
            gate_number: 5,
            gate_open_closed: true,
            baggage_number: 0,
            timestamp: timestamp.clone(),
        },
        Gate {
            gate_number: 6,
            gate_open_closed: false,
            baggage_number: 0,
            timestamp: timestamp.clone(),
        },
        Gate {
            gate_number: 7,
            gate_open_closed: true,
            baggage_number: 0,
            timestamp: timestamp.clone(),
        },
    ]
}