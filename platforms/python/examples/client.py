from syft import node
from syft.message import run_class_method_message as remote_execute
from syft.protos import SyftMessageProto
import pickle
from typing import Optional, Callable

import numpy as np


counter = 1


def execute_capability(remote_addr: str, capability: str, data: object) -> object:
    try:
        request = make_message(capability, data)
        request_bytes = request.SerializeToString()
        response_bytes = remote_execute(remote_addr, request_bytes)
        try:
            response = read_message(response_bytes)
            if response is not None:
                data = pickle.loads(response.obj)
                return data
                print(f"Python got response: {response}")
            else:
                print(f"Python failed to decode response: {repr(response_bytes)}")
            return None
        except Exception as e:
            print(f"Python failed to decode response: {e} {response_bytes}")
        return None
    except Exception as e:
        print(
            f"Python failed to execute request: {e} {remote_addr} {capability} {data}"
        )
        return None


def make_message(capability: str, obj: object) -> SyftMessageProto:
    request = SyftMessageProto()
    request.capability = capability
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


target_addr = "http://[::1]:50051"

remote_caps = node.request_capabilities(target_addr)
print(f"Node at: {target_addr} has capabilities: {remote_caps}")

execute_capability(target_addr, "hello", "Client 1")
# execute_capability(target_addr, "sum", [1, 2, 3])
# execute_capability(target_addr, "sum_np", [1, 2, 3])
