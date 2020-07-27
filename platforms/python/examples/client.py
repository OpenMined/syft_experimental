from syft import node
from syft.message import execute_capability

import numpy as np

port = 50051
iface = "0.0.0.0"
target_addr = f"http://{iface}:{port}"

remote_caps = node.request_capabilities(target_addr)
print(f"Node at: {target_addr} has capabilities: {remote_caps}")

message = execute_capability(target_addr, "hello", "Client 1")
print(message)
sum1 = execute_capability(target_addr, "sum", [1, 2, 3])
print(sum1)
sum2 = execute_capability(target_addr, "sum_np", [1, 2, 3])
print(sum2)
