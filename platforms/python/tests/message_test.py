from syft.message import SyftMessage


def test_message_ffi() -> None:
    any_object = set([1, 2, 3])
    message = SyftMessage(
        capability="message",
        obj=any_object,
        args=["a", "b"],
        kwargs={"arg1": "val1", "arg2": "val2"},
        id="3",
    )

    assert message.capability == "message"
    assert message.obj == set([1, 2, 3])
    assert type(message._self) == set
    assert message.args == ["a", "b"]
    assert message.kwargs == {"arg1": "val1", "arg2": "val2"}
    assert message.id == "3"
