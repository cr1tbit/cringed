

pub(crate) const CRINGED_TMP_PATH: &str = "/tmp/cringed";

//define enum for button event
#[derive(serde::Deserialize, serde::Serialize)]
pub(crate) enum EvtType {
    ButtonPress,
    ButtonRelease,
    Overcurrent,
    CriticalError,
    TransportError,
    ParseError,
    BoardBoot
}

#[derive(serde::Deserialize, serde::Serialize)]
pub(crate) struct CringeEvt {
    pub(crate) io_bank_num: u8,
    pub(crate) event_type: EvtType,
    pub(crate) timestamp_ms: u32
}

impl CringeEvt {
    pub(crate) fn from_serial(input: &str) -> Option<CringeEvt> {
        // example: <I11H T114374>
        
        let mut io_bank_num = 0;
        let mut timestamp_ms = 0;
        let mut event_type = EvtType::ParseError;
        for s in input.clone().replace(['<','>'],"").split_ascii_whitespace(){
            // println!("{}",s);
            match s.chars().nth(0).unwrap() {
                'I' => {
                    event_type = match s.chars().last().unwrap() {
                        'H' => EvtType::ButtonPress,
                        'L' => EvtType::ButtonRelease,
                        _ => continue
                    };
                    io_bank_num = s.chars().filter(|c| c.is_numeric())
                        .collect::<String>()
                        .parse::<u8>()
                        .unwrap_or(0);
                    },
                'T' => {
                    timestamp_ms = s.chars().filter(|c| c.is_numeric())
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap_or(0);
                }
            _ => continue
            }
        }

        Some(CringeEvt {
            io_bank_num,
            event_type,
            timestamp_ms,
        })
    }
}