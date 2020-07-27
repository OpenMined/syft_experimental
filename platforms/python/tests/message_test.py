from syft.message import SyftMessage
from syft.protos import SyftMessageProto

from syft.message.syft_message import SyftMessageProxy

import pickle


def test_message_creation() -> None:
    any_object = set([1, 2, 3])
    message = SyftMessage(
        capability="message",
        obj=any_object,
        args=["a", "b"],
        kwargs={"arg1": "val1", "arg2": "val2"},
        id="3",
    )

    obj_bytes = pickle.dumps(any_object)

    assert message.request.capability == "message"
    assert message.request.obj == obj_bytes
    assert message.request.local_id == "3"
    assert type(message.request) == SyftMessageProto


def test_message_decoding() -> None:
    any_object = int(6)
    message = SyftMessage(capability="message", obj=any_object, id="3",)
    message_bytes = message.request.SerializeToString()
    print(dir(SyftMessageProxy))
    response, data = SyftMessageProxy._deserialize(message_bytes)

    assert any_object == data
    assert response.capability == "message"
    assert response.local_id == "3"
