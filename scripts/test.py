#!/usr/bin/env python3
import socket
import time

device_reset_cmd = bytes([0x20, 0, 0, 1, 0])

def main():
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.connect(('127.0.0.1', 7000))
    s.send(device_reset_cmd)
    rsp = s.recv(1024)
    rsp = ', '.join([str(b) for b in rsp])
    print(f"Received: [{rsp}]")
    while True:
        time.sleep(2)
        s.send(device_reset_cmd)
        rsp = s.recv(1024)
        pass
    s.close()

if __name__ == '__main__':
    main()
