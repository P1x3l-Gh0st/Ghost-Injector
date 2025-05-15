use std::thread;
use std::time::Duration;

fn main() {
    println!("[*] Initializing CYBER ATTACK ENGINE v0.0.1...");
    thread::sleep(Duration::from_secs(1));
    println!("[*] Establishing reverse shell to 127.0.0.1...");
    thread::sleep(Duration::from_secs(1));
    println!("[!] ERROR: Firewall detected. Rebooting router via... Clippy?");
    thread::sleep(Duration::from_secs(1));

    let advice = get_clippy_advice();
    println!("[*] Clippy says: {}", advice);

    println!("[+] Exploit failed successfully. Hacker status: ✅ Certified Script Kiddie");
    println!("💡 Pro Tip: Real hackers write tests. And documentation. And sleep 8 hours.");
}

fn get_clippy_advice() -> &'static str {
    let advices = [
        "You should avoid unwrap(). Just like you should avoid unprotected Wi-Fi.",
        "Did you just use unsafe? NSA might be watching.",
        "Consider renaming your function. 'pwn()' is not very descriptive.",
        "Your code compiles. But should it?",
        "Clippy is watching... always watching 👀"
    ];

    let index = rand_index();
    advices[index]
}

fn rand_index() -> usize {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    (timestamp % 5) as usize
}
