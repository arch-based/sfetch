#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "config.h"

#define CYAN "\x1b[36m"
#define BOLD "\x1b[1m"
#define RESET "\x1b[0m"

char* execute_command(const char* command) {
    char* buffer = malloc(128);
    FILE* pipe = popen(command, "r");
    if (!pipe) return "ERROR";
    fgets(buffer, 128, pipe);
    pclose(pipe);
    buffer[strcspn(buffer, "\n")] = 0;  // This line removes the newline character
    return buffer;
}

void print_output(const char* label, const char* value) {
    printf("%s%s%s%s %s%s%s\n", CYAN, BOLD, label, RESET, BOLD, value, RESET);
}
char* get_package_manager() {
    if (!system("command -v apt > /dev/null")) {
        return "apt-cache pkgnames | wc -l";
    } else if (!system("command -v dpkg > /dev/null")) {
        return "dpkg --list | wc -l";
    } else if (!system("command -v pacman > /dev/null")) {
        return "pacman -Qe | wc -l";
    } else if (!system("command -v zypper > /dev/null")) {
        return "zypper se -i | wc -l";
    } else if (!system("command -v dnf > /dev/null")) {
        return "dnf list installed | wc -l";
    } else {
        return "unknown";
    }
}

int main() {
    char* user = USER;
    char* hostname = HOSTNAME;
    char* shell = SHELL;
    char* os = OS;
    char* kernel = execute_command("uname -sr");
    char* uptime = execute_command("uptime -p | sed 's/up //'");

    char* name = execute_command("cat /sys/devices/virtual/dmi/id/product_name");
    char* version = execute_command("cat /sys/devices/virtual/dmi/id/product_version");

    char host[256];
    sprintf(host, "%s %s", name, version);
    
    char* pkgs = execute_command(get_package_manager());
    char* memory = execute_command("free -m | awk 'NR==2{printf \"%sMiB / %sMiB (%.2f%%)\", $3,$2,$3*100/$2 }'");
    
    printf("%s%s%s%s%s@%s%s%s%s\n", "                    ", CYAN, BOLD, user, RESET, CYAN, BOLD, hostname, RESET);
    printf("%s%s%s%s%s\n", "          ", CYAN, BOLD, "-----------------------------", RESET);
    print_output(" OS ~~~~~~~~~~>", os);
    print_output("󰌢 HOST: ~~~~~~~>", host);
    print_output("󰅐 UPTIME: ~~~~~>", uptime);
    print_output(" KERNEL: ~~~~~>", kernel);
    print_output(" SHELL: ~~~~~~>", shell);
    print_output(" PACKAGES: ~~~>", pkgs);
    print_output("󰍛 MEMORY: ~~~~~>", memory);
    printf("\n")
    
    free(hostname);
    free(shell);
    free(os);
    free(kernel);
    free(uptime);
    free(name);
    free(version);
    free(pkgs);
    
    return 0;
}

