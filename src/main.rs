use std::io::BufRead;
use std::net::{IpAddr, SocketAddr};
use std::process::Command;
use std::sync::Mutex;
use std::thread;

fn main() {
    // Implement user configuration options, such as allowing the user to specify the path to a configuration file or providing a menu of options
    // If a configuration file is specified, load the configuration and apply the settings

    let incident = Mutex::new(Incident::new());

    let mut threads = Vec::new();

    let thread1 = thread::spawn(move || {
        let incident = incident.lock().unwrap();
        incident.isolate_all_affected_systems();
    });

    let thread2 = thread::spawn(move || {
        let incident = incident.lock().unwrap();
        incident.block_malicious_ips();
    });

    let thread3 = thread::spawn(move || {
        let incident = incident.lock().unwrap();
        incident.generate_incident_report();
    });

    threads.push(thread1);
    threads.push(thread2);
    threads.push(thread3);

    for thread in threads {
        thread.join().unwrap();
    }

    let incident = incident.lock().unwrap();
    println!("{}", incident.incident_report);
}
