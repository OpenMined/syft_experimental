import importlib

# find and run the import fixer
package_name = __name__.split(".")[0]
import_fixer = importlib.import_module(".import_fixer", package=package_name)

# re-organise normal python imports
# this goes from:
# syft.protos.message_pb2.SyftMessage -> syft.protos.SyftMessageProto
from .message_pb2 import SyftMessage

import_fixer.reexport(locals(), SyftMessage, "SyftMessageProto")
