from syft import node
from syft.protos import SyftMessageProto
import pickle
from typing import Optional, Callable

import numpy as np


counter = 1

node.start()


def make_message(obj: object) -> SyftMessageProto:
    request = SyftMessageProto()
    request.capability = "response"
    request.obj = pickle.dumps(obj)
    return request


def read_message(request_bytes: bytes) -> Optional[SyftMessageProto]:
    try:
        request = SyftMessageProto()
        request.ParseFromString(bytes(request_bytes))
        return request
    except Exception as e:
        print(f"Python failed to decode request {repr(request_bytes)}, error: {e}")
        return None


def create_handler(handler: Callable[[object], object]) -> Callable[[bytes], bytes]:
    def wrapped_handler(request_bytes: bytes) -> bytes:
        global counter
        try:
            counter += 1
            message = read_message(request_bytes)
            if message is not None:
                data = pickle.loads(message.obj)
                result = handler(data)
                response = make_message(result)

                # serialize protobuf to bytes
                response_bytes = response.SerializeToString()
                print(
                    f"Python counter: {counter} responding with Protobuf Message: {response}"
                )
                return response_bytes
            else:
                print(f"Python Callback failed")
                return b""
        except Exception as e:
            print(f"Python failed handle request {repr(request_bytes)}, error: {e}")
            return b""

    return wrapped_handler


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
