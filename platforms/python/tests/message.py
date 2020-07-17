from typing import Optional, Dict, Any, List, Callable

from syft.core import message as syft_message
from syft.protos.message_pb2 import SyftMessage


def make_message(
    id: Optional[str],
    path: Optional[str],
    args: List[str],
    kwargs: Dict[str, str],
    object: Any,
) -> SyftMessage:
    message = SyftMessage()
    if id is not None:
        message.remote = id

    if path is not None:
        message.path = path

    if args is not None:
        message.args.extend(args)

    if kwargs is not None:
        for key, value in kwargs.items():
            message.kwargs[key] = value

    return message

    # serialize protobuf to bytes


if __name__ == "__main__":

    message = make_message(
        "id1", "method_path", ["arg1", "arg2"], {"kwarg1": "one", "kwarg2": "two"}, ""
    )
    request_bytes = message.SerializeToString()
    print(f"Python sending to rust: {message} {request_bytes}")
    response_bytes = syft_message.run_class_method_message(
        "localhost", "message", request_bytes
    )

    try:
        response = SyftMessage()
        response.ParseFromString(bytes(response_bytes))
        print(f"Python got response: {response}")
    except Exception as e:
        print(f"Python failed to decode response {response_bytes}")
