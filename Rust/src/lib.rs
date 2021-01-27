pub struct Parrot<'a> {
    // 'a must be passed in and outlive Parrot
    parrot_type: &'a str,
    number_of_coconuts: usize,
    voltage: f32,
    nailed: bool,
}

impl<'a> Parrot<'a> {
    pub fn speed(&self) -> Result<f32, &'static str> {
        match self.parrot_type {
            "european_parrot" => Ok(base_speed()),
            "african_parrot" => {
                let african_speed = base_speed() - load_factor() * self.number_of_coconuts as f32;
                if african_speed > 0.0 {
                    Ok(african_speed)
                } else {
                    Ok(0.0)
                }
            }
            "norwegian_blue_parrot" => {
                if self.nailed == true {
                    Ok(0.0)
                } else {
                    Ok(compute_base_speed_for_voltage(self.voltage))
                }
            }
            _ => Err("Should be unreachable!"),
        }
    }

    pub fn create(t: &'a str, no: usize, v: f32, nailed: bool) -> Parrot {
        Parrot {
            parrot_type: t,
            number_of_coconuts: no,
            voltage: v,
            nailed: nailed,
        }
    }
}

fn compute_base_speed_for_voltage(voltage: f32) -> f32 {
    let fixed_base_speed = 24.0;
    let base_speed_for_voltage = voltage * base_speed();
    if base_speed_for_voltage < fixed_base_speed {
        base_speed_for_voltage
    } else {
        fixed_base_speed
    }
}

fn load_factor() -> f32 {
    9.0
}

fn base_speed() -> f32 {
    12.0
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct TestData<'a, 'b> {
        pub name: &'b str,
        input: Parrot<'a>,
        pub expected: f32,
    }

    impl<'a, 'b> TestData<'a, 'b> {
        pub fn new(name: &'b str, input: Parrot<'a>, expected: f32) -> TestData<'a, 'b> {
            TestData {
                name: name,
                input,
                expected,
            }
        }
    }

    fn run_tests(tests: Vec<TestData>) {
        for test in tests {
            println!("{:#?}", test.name);
            assert_eq!(test.input.speed().unwrap(), test.expected);
        }
    }

    #[test]
    pub fn main() {
        let test_cases = vec![
            TestData::new(
                "european_parrot_speed",
                Parrot::create("european_parrot", 0, 0.0, false),
                12.0,
            ),
            TestData::new(
                "african_parrot_speed_with_one_coconut",
                Parrot::create("african_parrot", 1, 0.0, false),
                3.0,
            ),
            TestData::new(
                "african_parrot_speed_with_two_coconut",
                Parrot::create("african_parrot", 2, 0.0, false),
                0.0,
            ),
            TestData::new(
                "african_parrot_speed_with_no_coconut",
                Parrot::create("african_parrot", 0, 0.0, false),
                12.0,
            ),
            TestData::new(
                "nailed_norwegian_blue_parrot",
                Parrot::create("norwegian_blue_parrot", 0, 1.5, true),
                0.0,
            ),
            TestData::new(
                "not_nailed_norwegian_blue_parrot",
                Parrot::create("norwegian_blue_parrot", 0, 1.5, false),
                18.0,
            ),
            TestData::new(
                "not_nailed_norwegian_blue_parrot_with_high_voltage",
                Parrot::create("norwegian_blue_parrot", 0, 4.0, false),
                24.0,
            ),
        ];

        run_tests(test_cases)
    }
}
