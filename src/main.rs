#[cfg(target_os = "hermit")]
use hermit as _;

fn main() {
    let now_utc = time::OffsetDateTime::now_utc();
    println!("{now_utc}");
}
