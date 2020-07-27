from syft import node
from syft.message import create_handler

import numpy as np


port = 50051
iface = "0.0.0.0"
node.start(iface, port)


def hello_handler(input: str) -> str:
    print(f"Handling a hello message {type(input)} {input}")
    return f"Hello: {input}"


def sum_handler(input: list) -> int:
    print(f"Handling a sum message {type(input)} {input}")
    return sum(input)


def sum_np_handler(input: list) -> int:
    print(f"Handling an np sum message {type(input)} {input}")
    return int(np.sum(input))


node.register("hello", create_handler(hello_handler))
node.register("sum", create_handler(sum_handler))
node.register("sum_np", create_handler(sum_np_handler))
