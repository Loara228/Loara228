'''
Вывод на информации с Raspberry Pi 4 на LCD 1602 с I2C модулем.

    LCD1602                  Raspberry Pi 4
  +===============================================+
  | GND            ->        Pin 6 (GND)          |
  | VCC (5V)       ->        Pin 2/4 (5V)         |
  | SDA            ->        Pin 3 (GPIO 2 / SDA) |
  | SCL            ->        Pin 5 (GPIO 3 / SCL) |
  +===============================================+

raspi-config: I2C Interfacing Options -> I2C -> Yes.
sudo apt install python3-smbus python3-pip python3-gpiozero
sudo pip install RPLCD psutil gpiozero --break-system-packages
nano /etc/systemd/system/lcd_monitor.service
'''

'''
[Unit]
Description=LCD Monitor Service
After=network.target

[Service]
Type=simple
User=root
ExecStart=/usr/bin/python3 /home/USERNAME/LED_display.py
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
'''

'''
sudo systemctl daemon-reload
sudo systemctl enable lcd_monitor.service
sudo systemctl start lcd_monitor.service
'''

import time
import psutil
import subprocess
from RPLCD.i2c import CharLCD

lcd = CharLCD('PCF8574', 0x27)

def check_ping(host):
    try:
        subprocess.check_output(["ping", "-c", "1", "-W", "1", host], stderr=subprocess.STDOUT)
        return True
    except:
        return False

def get_cpu_temp():
    try:
        with open("/sys/class/thermal/thermal_zone0/temp", "r") as f:
            temp = f.read()
        return float(temp) / 1000
    except FileNotFoundError:
        return 0.0

def has_ssh_connection():
    try:
        connections = psutil.net_connections()
        for conn in connections:
            if conn.laddr.port == 22 and conn.status == 'ESTABLISHED':
                return True
    except psutil.AccessDenied:
        return False
    return False

try:
    while True:
        temp = get_cpu_temp()
        load = psutil.cpu_percent(interval=0.5)
        
        is_router = check_ping("192.168.0.1")
        is_inet = check_ping("8.8.8.8")
        is_ssh = has_ssh_connection()

        lcd.clear()
        lcd.write_string(f"T:{temp:.1f}C L:{load}%")
        lcd.cursor_pos = (1, 0)
        
        ssh_status = "SSH:OK" if is_ssh else "SSH:NO"
        net_info = f"I:{'OK' if is_inet else 'NO'}    {ssh_status}"
        lcd.write_string(net_info)
        
        time.sleep(2)

except KeyboardInterrupt:
    lcd.clear()
