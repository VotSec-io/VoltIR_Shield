# VoltIR_Shield
VoltIR Shield is an advanced incident response automation tool crafted in Rust to fortify cybersecurity defenses. In the ever-evolving landscape of digital threats, VoltIR Shield empowers organizations to swiftly and intelligently respond to incidents, securing their systems from potential harm.



## Features

* **Isolate affected systems:** VoltIR Shield can automatically isolate affected systems by executing the shutdown command on them.
* **Block malicious IPs:** VoltIR Shield can automatically block malicious IPs by adding them to the iptables firewall.
* **Generate incident reports:** VoltIR Shield can generate detailed incident reports that include information about the affected systems, malicious IPs, and incident details.

## Installation

To install VoltIR Shield, simply clone the repository and run the following command:

```bash
cargo run

## Usage
To use VoltIR Shield, simply run the following command:

cargo run


VoltIR Shield will then prompt you for the following information:

* **Affected systems:** A list of the affected systems.
* **Malicious IPs:** A list of the malicious IPs.

Once you have entered this information, VoltIR Shield will isolate the affected systems, block the malicious IPs, and generate an incident report.

## Configuration

VoltIR Shield can be configured using a configuration file. The configuration file is a JSON file that contains the following fields:

* `affected_systems`: A list of the affected systems.
* `malicious_ips`: A list of the malicious IPs.

To use a configuration file, simply specify the path to the configuration file when you run VoltIR Shield:

bash
cargo run --config <path-to-config-file>

