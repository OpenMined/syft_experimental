from syft.protos.message_pb2 import SyftMessage as SyftMessageProto
from syft.message import run_class_method_message

from typing import Optional, Dict, Any, List, Callable
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
        path: Optional[str] = None,
        args: Optional[List[str]] = None,
        kwargs: Optional[Dict[str, str]] = None,
        obj: Optional[Any] = None,
    ):
        request = self.__create_request(
            id=id, id_remote=id_remote, path=path, args=args, kwargs=kwargs, obj=obj
        )
        self.response = self.__get_response(
            address="localhost", capability="message", request=request
        )

        if self.response is not None and self.response.Object is not None:
            try:
                self.obj = pickle.loads(self.response.Object)
            except Exception as e:
                print(f"Failed to pickle load the obj bytes: {e}")
                self.obj = None

    def __create_request(
        self,
        id: Optional[str] = None,
        id_remote: Optional[str] = None,
        path: Optional[str] = None,
        args: Optional[List[str]] = None,
        kwargs: Optional[Dict[str, str]] = None,
        obj: Optional[Any] = None,
    ) -> SyftMessageProto:
        request = SyftMessageProto()
        if id is not None:
            request.local_id = id
        if id_remote is not None:
            request.remote = id_remote

        if path is not None:
            request.path = path

        if args is not None:
            request.args.extend(args)

        if kwargs is not None:
            for key, value in kwargs.items():
                request.kwargs[key] = value

        if obj is not None:
            request.Object = pickle.dumps(obj)

        return request

    def __get_response(
        self, address: str, capability: str, request: SyftMessageProto
    ) -> Optional[SyftMessageProto]:
        request_bytes = request.SerializeToString()
        try:
            # this is where we are calling rust
            response_bytes = run_class_method_message(
                address, capability, request_bytes
            )
            response = SyftMessageProto()
            response.ParseFromString(bytes(response_bytes))

            return response
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
        "_self": "obj",
    }

    def __init__(
        self,
        id: Optional[str] = None,
        id_remote: Optional[str] = None,
        path: Optional[str] = None,
        args: Optional[List[str]] = None,
        kwargs: Optional[Dict[str, str]] = None,
        obj: Optional[Any] = None,
    ):
        super().__init__(
            id=id, id_remote=id_remote, path=path, args=args, kwargs=kwargs, obj=obj
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
