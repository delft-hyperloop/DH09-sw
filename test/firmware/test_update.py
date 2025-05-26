#!/usr/bin/env python3
import socket
import sys
import time
import hashlib

def send_firmware(filename, target_ip='192.168.1.100', target_port=4321):
    # Create UDP socket
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    
    try:
        with open(filename, 'rb') as f:
            data = f.read()
            
        # Calculate SHA256 hash
        sha256_hash = hashlib.sha256(data).hexdigest()
        print(f"Firmware SHA256: {sha256_hash}")
        
        # Send firmware in 512-byte chunks
        for i in range(0, len(data), 512):
            chunk = data[i:i+512]
            sock.sendto(chunk, (target_ip, target_port))
            print(f"Sent chunk {i//512 + 1} of {(len(data) + 511)//512}")
            time.sleep(0.1)  # Small delay between chunks
            
    except Exception as e:
        print(f"Error: {e}")
    finally:
        sock.close()

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python3 test_update.py <firmware_file>")
        sys.exit(1)
    
    send_firmware(sys.argv[1]) 