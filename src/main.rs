use std::env;
use std::fs;
use std::process::Command;

// Constants for color codes
const CYAN: &str = "\x1b[36m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

// Function to execute shell commands
fn execute_command(command: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

// Function to print formatted output
fn print_output(label: &str, value: &str) {
    println!(
        "{}{}{}{} {}{}{}",
        CYAN, BOLD, label, RESET, BOLD, value, RESET
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mem_info = fs::read_to_string("/proc/meminfo")?;
    let mut mem_total = 0;
    let mut mem_avail = 0;

    for line in mem_info.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "MemTotal:" => mem_total = parts[1].parse::<i32>()?,
            "MemAvailable:" => mem_avail = parts[1].parse::<i32>()?,
            _ => (),
        }
    }

    let mem_used = (mem_total - mem_avail) / 1024;
    let mem_total = mem_total / 1024;
    let memory = format!("{}MiB / {}MiB", mem_used, mem_total);

    let user = env::var("USER")?;
    let hostname = fs::read_to_string("/etc/hostname")?.trim().to_string();
    let shell = execute_command("basename $SHELL")?;
    let os = execute_command("hostnamectl | grep 'Operating System' | awk '{print $3,$4}'")?;
    let kernel = execute_command("uname -sr")?;
    let uptime = execute_command("uptime -p | sed 's/up //'")?;

    let name = execute_command("cat /sys/devices/virtual/dmi/id/product_name")?;
    let version = execute_command("cat /sys/devices/virtual/dmi/id/product_version")?;

    let host = format!("{} {}", name, version);
    let pkgs = execute_command("pacman -Qe | wc -l")?;

    // The output

    println!(
        "{}{}{}{}{}@{}{}{}{}",
        "                    ", CYAN, BOLD, user, RESET, CYAN, BOLD, hostname, RESET
    );
    println!(
        "{}{}{}{}{}",
        "          ", CYAN, BOLD, "-----------------------------", RESET
    );
    print_output(" OS ~~~~~~~~~~>", &os);
    print_output("󰌢 HOST: ~~~~~~~>", &host);
    print_output("󰅐 UPTIME: ~~~~~>", &uptime);
    print_output(" KERNEL: ~~~~~>", &kernel);
    print_output(" SHELL: ~~~~~~>", &shell);
    print_output(" PACKAGES: ~~~>", &pkgs);
    print_output("󰍛 MEMORY: ~~~~~>", &memory);
    println!();
    Ok(())
}
