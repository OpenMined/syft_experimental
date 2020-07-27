from syft.protos import SyftMessageProto
from syft.message import run_class_method_message

from typing import Optional, Dict, Any, List, Callable, Tuple
import pickle


class SyftMessageProxy:
    """
    This is a class which manages serialization and deserialization of protobuf messages
    and invokes the ffi functions in syft core rust
    """

    def __init__(
        self,
        id: Optional[str] = None,
        id_remote: Optional[str] = None,
        capability: Optional[str] = None,
        args: Optional[List[str]] = None,
        kwargs: Optional[Dict[str, str]] = None,
        obj: Optional[Any] = None,
    ):
        request = SyftMessageProto()
        if id is not None:
            request.local_id = id
        if id_remote is not None:
            request.remote = id_remote

        if capability is not None:
            request.capability = capability

        if args is not None:
            request.args.extend(args)

        if kwargs is not None:
            for key, value in kwargs.items():
                request.kwargs[key] = value

        if obj is not None:
            request.obj = pickle.dumps(obj)

        self.request = request

    @staticmethod
    def _deserialize(
        response_bytes: bytes,
    ) -> Tuple[SyftMessageProto, Optional[object]]:
        response = SyftMessageProto()
        response.ParseFromString(bytes(response_bytes))
        obj = None
        if response is not None and response.obj is not None:
            try:
                obj = pickle.loads(response.obj)
            except Exception as e:
                print(f"Failed to pickle load the obj bytes: {e}")

        return response, obj

    def send(self, address: str) -> Optional[object]:
        if self.request is not None:
            request_bytes = self.request.SerializeToString()
            try:
                # this is where we are calling rust
                response_bytes = run_class_method_message(address, request_bytes)
                _, obj = SyftMessageProxy._deserialize(response_bytes)
                if obj is not None:
                    return obj
            except Exception as e:
                print(f"Python failed to decode response {response_bytes}, error: {e}")
        return None


class SyftMessage(SyftMessageProxy):
    """
    This is a wrapper interface to SyftMessageProxy which allows a clean property
    accessor pattern using __getattr__ and some key mapping
    """

    _proxy_map: Dict[str, str] = {
        "id": "local_id",
        "id_remote": "remote",
    }

    def __init__(
        self,
        id: Optional[str] = None,
        id_remote: Optional[str] = None,
        capability: Optional[str] = None,
        args: Optional[List[str]] = None,
        kwargs: Optional[Dict[str, str]] = None,
        obj: Optional[Any] = None,
    ):
        super().__init__(
            id=id,
            id_remote=id_remote,
            capability=capability,
            args=args,
            kwargs=kwargs,
            obj=obj,
        )

    def __getattr__(self, attr: str) -> Any:
        property_key = attr if attr not in self._proxy_map else self._proxy_map[attr]

        try:
            if not property_key == "obj":
                return getattr(self.response, property_key)
            else:
                return getattr(self, property_key)

        except Exception as e:
            print(f"Unable to find attr: {property_key} on proxy object. {e}")
            return None


def _make_message(obj: object, capability: str) -> SyftMessageProto:
    request = SyftMessageProto()
    request.capability = capability
    request.obj = pickle.dumps(obj)
    return request


def _read_message(request_bytes: bytes) -> Optional[SyftMessageProto]:
    try:
        request = SyftMessageProto()
        request.ParseFromString(bytes(request_bytes))
        return request
    except Exception as e:
        print(f"Python failed to decode request {repr(request_bytes)}, error: {e}")
        return None


def create_handler(handler: Callable[[object], object]) -> Callable[[bytes], bytes]:
    def wrapped_handler(request_bytes: bytes) -> bytes:
        try:
            message = _read_message(request_bytes)
            if message is not None:
                data = pickle.loads(message.obj)
                result = handler(data)
                response = _make_message(result, "response")

                # serialize protobuf to bytes
                response_bytes = response.SerializeToString()
                return response_bytes
            else:
                return b""
        except Exception as e:
            print(f"Python failed to handle request {repr(request_bytes)}, error: {e}")
            return b""

    return wrapped_handler


def execute_capability(remote_addr: str, capability: str, data: object) -> object:
    try:
        request = _make_message(data, capability)
        request_bytes = request.SerializeToString()
        response_bytes = run_class_method_message(remote_addr, request_bytes)
        try:
            response = _read_message(response_bytes)
            if response is not None:
                data = pickle.loads(response.obj)
                return data
            else:
                print(f"Python failed to decode response: {repr(response_bytes)}")
        except Exception as e:
            print(f"Python failed to decode response: {e} {response_bytes}")
    except Exception as e:
        print(
            f"Python failed to execute request: {e} {remote_addr} {capability} {data}"
        )
    return None
