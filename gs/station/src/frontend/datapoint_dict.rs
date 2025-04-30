use std::fmt::Formatter;
use gslib::{Datapoint, Datatype};

const DATATYPE_HEADER: &str = "Datatype";
const VALUE_HEADER: &str = "Value";
const TIMESTAMP_HEADER: &str = "Timestamp";

pub struct DatapointDict {
    datapoints: [Datapoint; 50],
    size: usize,
    capacity: usize,
    max_length_type: usize,
    max_length_value: usize,
}

impl DatapointDict {
    pub fn new(capacity: usize) -> Self {
        Self {
            datapoints: [Datapoint::new(Datatype::DefaultDatatype, 0, 0); 50],
            size: 0,
            capacity,
            max_length_type: DATATYPE_HEADER.len(),
            max_length_value: VALUE_HEADER.len(),
        }
    }

    pub fn as_string(&self) -> String {
        let mut result = format!(
            "{}{}|{}{}|{}\n{}",
            DATATYPE_HEADER,
            " ".repeat(self.max_length_type - DATATYPE_HEADER.len()),
            VALUE_HEADER,
            " ".repeat(self.max_length_value - VALUE_HEADER.len()),
            TIMESTAMP_HEADER,
            "-".repeat(self.max_length_value + self.max_length_type + TIMESTAMP_HEADER.len() + 2)
        );
        for i in 0..self.size {
            let dp = self.datapoints[i];
            result.push_str(&format!("\n{:?}{}|{}{}|{}",
                                     dp.datatype,
                                     " ".repeat(self.max_length_type - format!("{}", dp.datatype.to_id()).len()), //TODO: change to_id to to_str
                                     dp.value,
                                     " ".repeat(self.max_length_value - dp.value.to_string().len()),
                                     dp.timestamp,
            ));
        }
        result.push_str("\n");
        result
    }

    pub fn add_datapoint(&mut self, datapoint: Datapoint) {
        for i in 0..self.capacity {
            if self.datapoints[i].datatype == Datatype::DefaultDatatype {
                self.max_length_value = std::cmp::max(self.max_length_value, datapoint.value.to_string().len());
                self.max_length_type = std::cmp::max(self.max_length_type, format!("{}", datapoint.datatype.to_id()).len());  //TODO: change to_id to to_str
                self.datapoints[i] = datapoint;
                self.size += 1;
                return;
            } else if datapoint.datatype == self.datapoints[i].datatype {
                self.datapoints[i] = datapoint;
                return;
            }
        }
        // TODO: replace oldest?
    }
}

impl std::fmt::Display for DatapointDict {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = &mut format!("\x1B[{}A{}", self.capacity + 2, self.as_string());
        for _ in 0..(self.capacity - self.size) {
            result.push_str("\n");
        }
        write!(f, "{}", result)
    }
}
