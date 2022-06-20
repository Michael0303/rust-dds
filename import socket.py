import socket 
import msgpack 
import time 
import os
import array

d = {}
d["joint"] = {"a": 0.1, "b": 0.2, "c": 0.3}
d["touch"] = {"a": 0.7, "b": 0.5, "c": 0.4}
d["teset"] = {"a": 0.6, "b": "hi", "c": 0.4}

stream = msgpack.packb(d)
# s = msgpack.dumps(d)
# print(', '.join(map(lambda x: '{:#2x}'.format(x), array.array('B', s))))
server = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) 
try: 
    os.remove("/tmp/robocup")
except:
    print("rm unix socket no found")
server.bind("/tmp/robocup")
server.listen(1)
conn, addr = server.accept()
while True: 
    conn.send(stream)
    print("send one")
    time.sleep(1)