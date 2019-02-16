#!/usr/bin/env python3
import socket
import sys

HOST = sys.argv[1]
PORT = 6667
s = socket.socket(socket.AF_INET6, socket.SOCK_STREAM)
s.bind((HOST, PORT))
s.listen(1)
while 1:
	conn, addr = s.accept()
	#print('Connection accept: %s' % (addr[0]))
	conn.sendall(':irc.example.org ERROR Please use TLS (SSL) to connect to this IRC network on port 6697\r\n'.encode('utf-8'))
	conn.close()
	#print('Connection close:  %s' % (addr[0]))
