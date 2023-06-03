#[derive(Debug)]
struct NumberWithLogs<T> {
    value: T,
    logs: String,
}

// the generic type T is wrapped with the NumberWithLogs struct
// so it can be used on any type
fn wrap_with_logs<T>(value: T) -> NumberWithLogs<T> {
    NumberWithLogs::<T> {
        value,
        logs: "".to_string(),
    }
}

fn square(number: f64) -> NumberWithLogs<f64> {
    let squared: f64 = number * number;
    let new_entry: String = format!("Squared {} to get {}\n", number, squared);
    NumberWithLogs {
        value: squared,
        logs: new_entry,
    }
}

fn get_successor(number: f64) -> NumberWithLogs<f64> {
    let successor: f64 = number + 1.0;
    let new_entry: String = format!("Added 1 to {} to get {}\n", number, successor);
    NumberWithLogs {
        value: successor,
        logs: new_entry,
    }
}

fn run_with_logs<T>(
    number: NumberWithLogs<T>,
    transform: impl Fn(T) -> NumberWithLogs<T>,
) -> NumberWithLogs<T> {
    let transformed: NumberWithLogs<T> = transform(number.value);
    let mut log: String = number.logs.clone();
    let new_entry: String = transformed.logs;
    log.push_str(&new_entry);
    NumberWithLogs {
        value: transformed.value,
        logs: log,
    }
}

// using a trait to do the same thing as the function run_with_logs:
trait WithLogs<T> {
    fn with_logs(&self, transform: impl Fn(T) -> NumberWithLogs<T>) -> NumberWithLogs<T>;
}

impl WithLogs<f64> for NumberWithLogs<f64> {
    fn with_logs(&self, transform: impl Fn(f64) -> NumberWithLogs<f64>) -> NumberWithLogs<f64> {
        let transformed: NumberWithLogs<f64> = transform(self.value);
        let mut log: String = self.logs.clone();
        let new_entry: String = transformed.logs;
        log.push_str(&new_entry);
        NumberWithLogs {
            value: transformed.value,
            logs: log,
        }
    }
}

fn main() {
    let a = wrap_with_logs(4.0);
    let b = run_with_logs(a, square);
    let c = run_with_logs(b, square);
    let d = run_with_logs(c, get_successor);
    println!("{:?}", d);

    println!(
        "{:?}",
        wrap_with_logs(5.0)
            .with_logs(square)
            .with_logs(get_successor)
    );
}
