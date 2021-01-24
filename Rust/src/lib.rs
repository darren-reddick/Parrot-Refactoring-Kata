const LOAD_FACTOR: f32 = 9.0;
const BASE_SPEED: f32 = 12.0;
const FIXED_BASE_SPEED: f32 = 24.0;

struct ParrotConfig {
    number_of_coconuts: usize,
    voltage: f32,
    nailed: bool,
}

impl ParrotConfig {
    #[allow(dead_code)]
    fn new(number_of_coconuts: usize, voltage: f32, nailed: bool) -> Self {
        ParrotConfig {
            number_of_coconuts: number_of_coconuts,
            voltage: voltage,
            nailed: nailed,
        }
    }
}

trait Parrot {
    fn speed(&self) -> Result<f32, &str>;
    fn new(conf: ParrotConfig) -> Self;
}

struct EuropeanParrot {}

impl Parrot for EuropeanParrot {
    fn speed(&self) -> Result<f32, &str> {
        Ok(BASE_SPEED)
    }
    fn new(_conf: ParrotConfig) -> EuropeanParrot {
        EuropeanParrot {}
    }
}

struct AfricanParrot {
    config: ParrotConfig,
}

impl Parrot for AfricanParrot {
    fn speed(&self) -> Result<f32, &str> {
        Ok(f32::max(
            BASE_SPEED - LOAD_FACTOR * self.config.number_of_coconuts as f32,
            0.0,
        ))
    }
    fn new(conf: ParrotConfig) -> AfricanParrot {
        AfricanParrot { config: conf }
    }
}

struct NorwegianBlueParrot {
    config: ParrotConfig,
}

impl Parrot for NorwegianBlueParrot {
    fn speed(&self) -> Result<f32, &str> {
        match self.config.nailed {
            true => Ok(0.0),
            false => Ok(f32::min(self.config.voltage * BASE_SPEED, FIXED_BASE_SPEED)),
        }
    }
    fn new(conf: ParrotConfig) -> NorwegianBlueParrot {
        NorwegianBlueParrot { config: conf }
    }
}

#[allow(dead_code)]
struct ParrotTest<'a> {
    expected: f32,
    config: ParrotConfig,
    description: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_european_parrot_speed() {
        let tests = vec![ParrotTest {
            expected: 12.0,
            config: ParrotConfig::new(0, 0.0, false),
            description: "Test european parrot speed",
        }];

        for test in tests {
            let parrot = EuropeanParrot::new(test.config);
            println!("{}", test.description);
            assert_eq!(parrot.speed().unwrap(), test.expected);
        }
    }

    #[test]
    fn test_african_parrot_speed() {
        let tests = vec![
            ParrotTest {
                expected: 3.0,
                config: ParrotConfig::new(1, 0.0, false),
                description: "Test african parrot with one coconut",
            },
            ParrotTest {
                expected: 0.0,
                config: ParrotConfig::new(2, 0.0, false),
                description: "Test african parrot with two coconuts",
            },
            ParrotTest {
                expected: 12.0,
                config: ParrotConfig::new(0, 0.0, false),
                description: "Test african parrot with no coconuts",
            },
        ];

        for test in tests {
            let parrot = AfricanParrot::new(test.config);
            println!("{}", test.description);
            assert_eq!(parrot.speed().unwrap(), test.expected);
        }
    }

    #[test]
    fn test_norwegian_blue_parrot_speed() {
        let tests = vec![
            ParrotTest {
                expected: 0.0,
                config: ParrotConfig::new(0, 1.5, true),
                description: "Test nailed norwegian blue parrot",
            },
            ParrotTest {
                expected: 18.0,
                config: ParrotConfig::new(0, 1.5, false),
                description: "Test not nailed norwegian blue parrot",
            },
            ParrotTest {
                expected: 24.0,
                config: ParrotConfig::new(0, 4.0, false),
                description: "Test not nailed norwegian blue parrot with high voltage",
            },
        ];

        for test in tests {
            let parrot = NorwegianBlueParrot::new(test.config);
            println!("{}", test.description);
            assert_eq!(parrot.speed().unwrap(), test.expected);
        }
    }
}
