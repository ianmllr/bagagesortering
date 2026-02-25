
pub struct Reservation {
    pub(crate) passenger_number: i32,
    pub(crate) passenger_name: String,
    pub(crate) timestamp: String,
}

pub struct Flyveplan {
    pub(crate) flyafgang: i32,
    pub(crate) terminalnummer: i32
}

pub struct Check_in_skrank {
    pub(crate) open: bool,
    pub(crate) passenger_number: i32,
    pub(crate) baggage_number: i32,
    pub(crate) timestamp: String,
}

pub struct Sorteringsanlaeg {
    pub(crate) baggage_number: i32,
    pub(crate) check_in_time: String,
    pub(crate) check_out_time: String,
}

pub struct Terminal {
    pub(crate) terminal_number: i32,
    pub(crate) number_of_gates: i32

}

pub struct Gate {
    pub(crate) gate_number: i32,
    pub(crate) gate_open_closed: bool,
    pub(crate) baggage_number: i32,
    pub(crate) timestamp: String,
}