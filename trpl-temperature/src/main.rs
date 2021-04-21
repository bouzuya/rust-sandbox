#[derive(Debug, PartialEq)]
struct Celsius(f64);

impl std::convert::From<Fahrenheit> for Celsius {
    fn from(Fahrenheit(f): Fahrenheit) -> Self {
        Self((f - 32_f64) * 5_f64 / 9_f64)
    }
}

impl std::fmt::Display for Celsius {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq)]
struct Fahrenheit(f64);

impl std::convert::From<Celsius> for Fahrenheit {
    fn from(Celsius(c): Celsius) -> Self {
        Self(c * 9_f64 / 5_f64 + 32_f64)
    }
}

impl std::fmt::Display for Fahrenheit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn celsius_to_fahrenheit() {
        assert_eq!(Fahrenheit::from(Celsius(-40_f64)), Fahrenheit(-40_f64));
        assert_eq!(Fahrenheit::from(Celsius(0_f64)), Fahrenheit(32_f64));
        assert_eq!(Fahrenheit::from(Celsius(10_f64)), Fahrenheit(50_f64));
        assert_eq!(Fahrenheit::from(Celsius(40_f64)), Fahrenheit(104_f64));
    }

    #[test]
    fn fahrenheit_to_celsius() {
        assert_eq!(Celsius::from(Fahrenheit(-40_f64)), Celsius(-40_f64));
        assert_eq!(Celsius::from(Fahrenheit(32_f64)), Celsius(0_f64));
        assert_eq!(Celsius::from(Fahrenheit(50_f64)), Celsius(10_f64));
        assert_eq!(Celsius::from(Fahrenheit(104_f64)), Celsius(40_f64));
    }
}

fn main() {
    let mut args = std::env::args();
    args.next().expect("prgram name");
    let to_fahrenheit = match args.next() {
        Some(s) if s == "--to-fahrenheit" => true,
        Some(s) if s == "--to-celsius" => false,
        _ => {
            eprintln!(
                "Usage: trpl-temperature --to-fahrenheit <C> or trpl-temperature --to-celsius <F>"
            );
            std::process::exit(1);
        }
    };
    let v = match args.next() {
        Some(s) => match s.parse::<f64>() {
            Ok(f) => f,
            Err(_) => {
                eprintln!(
                "Usage: trpl-temperature --to-fahrenheit <C> or trpl-temperature --to-celsius <F>"
            );
                std::process::exit(1);
            }
        },
        None => {
            eprintln!(
                "Usage: trpl-temperature --to-fahrenheit <C> or trpl-temperature --to-celsius <F>"
            );
            std::process::exit(1);
        }
    };
    println!(
        "{}",
        if to_fahrenheit {
            Fahrenheit::from(Celsius(v)).to_string()
        } else {
            Celsius::from(Fahrenheit(v)).to_string()
        }
    );
}
