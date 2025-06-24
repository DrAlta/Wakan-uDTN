trait Foo {
    fn to_fixed(&self, n: i8) -> Self;
}
impl Foo for f64 {
    fn to_fixed(&self, n: i8) -> Self {
        assert_eq!(n, 3);
        (self * 1000.0).round() / 1000.0
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Bandwidth {
    KHz7_8,
    KHz10_4,
    KHz15_6,
    KHz20_8,
    KHz31_2,
    KHz41_7,
    KHz62_5,
    KHz125,
    KHz250,
    KHz500,
}
impl Into<f64> for Bandwidth {
    fn into(self) -> f64 {
        match self {
            Bandwidth::KHz7_8 => 7.8,
            Bandwidth::KHz10_4 => 10.4,
            Bandwidth::KHz15_6 => 12.6,
            Bandwidth::KHz20_8 => 20.8,
            Bandwidth::KHz31_2 => 31.2,
            Bandwidth::KHz41_7 => 41.7,
            Bandwidth::KHz62_5 => 62.5,
            Bandwidth::KHz125 => 125.0,
            Bandwidth::KHz250 => 250.0,
            Bandwidth::KHz500 => 500.0,
        }
    }
}

pub struct LoRa {
    payload_len: f64,
    preamble_len: f64,
    spreading_factor: f64,
    bandwidth: f64,
    coding_rate: f64,
    pub crc: bool,
    pub explicit_header: bool,
    pub low_data_rate_opt: bool,
}
impl LoRa {
    pub fn new(
        payload_len: u8,
        preamble_len: u16,
        spreading_factor: u8,
        bandwidth: Bandwidth,
        coding_rate: u8,
    ) -> Result<Self, String> {
        if !Self::check_payload_len(payload_len) {
            return Err("Invalid payload length".into());
        }
        let payload_len = payload_len as f64;

        if !Self::check_preamble_len(preamble_len) {
            return Err("Invalid preamble length".into());
        }
        let preamble_len = preamble_len as f64;

        let spreading_factor = match spreading_factor {
            x @ (6 | 7 | 8 | 9 | 10 | 11 | 12) => x as f64,
            _ => return Err("invalid speading factor".into()),
        };
        let coding_rate = match coding_rate {
            x @ (5 | 6 | 7 | 8) => x as f64,
            _ => return Err("invalid coding rate".into()),
        };
        Ok(Self {
            payload_len,
            preamble_len,
            spreading_factor,
            bandwidth: bandwidth.into(),
            coding_rate,
            crc: false,
            explicit_header: false,
            low_data_rate_opt: false,
        })
    }
    pub fn check_payload_len(payload_len: u8) -> bool {
        !(payload_len < 1)
    }
    pub fn check_preamble_len(preamble_len: u16) -> bool {
        !(preamble_len < 6)
    }
    pub fn symbol_time(&self) -> f64 {
        return (2_f64.powf(self.spreading_factor) / self.bandwidth).to_fixed(3);
    }
    #[allow(dead_code)]
    pub fn symbol_rate(&self) -> f64 {
        return (1000.0 / self.symbol_time()).to_fixed(3);
    }
    #[allow(dead_code)]
    pub fn throughput(&self) -> Option<f64> {
        let t_total = self.t_total();
        Some((((8.0 * self.payload_len) / t_total) * 1000.0).to_fixed(3))
    }
    pub fn n_preamble(&self) -> f64 {
        self.preamble_len + 4.25
    }
    pub fn t_preamble(&self) -> f64 {
        (self.n_preamble() * self.symbol_time()).to_fixed(3)
    }
    pub fn n_payload(&self) -> f64 {
        let mut payload_bit = 8.0 * self.payload_len; // The lenght of payload in bits
        payload_bit -= 4.0 * self.spreading_factor; // ???
        payload_bit += 8.0; // Mistry magic overhead
        payload_bit += if self.crc { 16.0 } else { 0.0 }; // The length of CRC is 16 bits
        payload_bit += if self.explicit_header { 20.0 } else { 0.0 }; // The length of LoRa header is 20 bits
        payload_bit = payload_bit.max(0.0);
        let bits_per_symbol = if self.low_data_rate_opt {
            self.spreading_factor - 2.0
        } else {
            self.spreading_factor
        }; // If low data rate optimization is enabled, onlt SF-2 bits will be mapped to each symbol
        let mut payload_symbol = (payload_bit / 4.0 / bits_per_symbol).ceil() * self.coding_rate; // Perform coding and mapping bits to symbol
        payload_symbol += 8.0; // There's always a 8-symbol-long overhead. Probably is SyncWord.

        payload_symbol
    }
    pub fn t_payload(&self) -> f64 {
        (self.n_payload() * self.symbol_time()).to_fixed(3)
    }
    pub fn t_total(&self) -> f64 {
        let t_preamble = self.t_preamble();
        let t_payload = self.t_payload();

        (t_preamble + t_payload).to_fixed(3)
    }
}
