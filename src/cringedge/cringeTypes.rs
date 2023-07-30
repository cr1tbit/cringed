

pub(crate) const CRINGED_TMP_PATH: &str = "/tmp/cringed";

//define enum for button event
#[derive(serde::Deserialize, serde::Serialize)]
pub(crate) enum EvtType {
    ButtonPress,
    ButtonRelease,
    Overcurrent,
    CriticalError,
    TransportError
}

#[derive(serde::Deserialize, serde::Serialize)]
pub(crate) struct CringeEvt {
    pub(crate) io_bank_num: u8,
    pub(crate) event_type: EvtType,
    pub(crate) timestamp_ms: u32
}

impl CringeEvt {
    pub(crate) fn from_serial(input: &str) -> Option<CringeEvt> {
        let io_bank_num = input.chars().nth(2).unwrap().to_digit(10).unwrap_or(0) as u8;
        let timestamp_ms = input[6..]
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse::<u32>()
            .unwrap_or(0);

        let event_type = match input.chars().nth(3).unwrap() {
            'H' => EvtType::ButtonPress,
            'L' => EvtType::ButtonRelease,
            'O' => EvtType::Overcurrent,
            'C' => EvtType::CriticalError,
            'T' => EvtType::TransportError,
            _ => return None
        };
        Some(CringeEvt {
            io_bank_num,
            event_type,
            timestamp_ms,
        })
    }
}