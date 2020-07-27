import importlib

# find and run the import fixer
package_name = __name__.split(".")[0]
import_fixer = importlib.import_module(".import_fixer", package=package_name)
import_fixer.fix_imports(locals(), __file__)

# re-organise normal python imports
# this goes from:
# syft.message.syft_message.SyftMessage -> syft.message.SyftMessage

from .syft_message import SyftMessage, create_handler, execute_capability

import_fixer.reexport(locals(), SyftMessage)
import_fixer.reexport(locals(), create_handler)
import_fixer.reexport(locals(), execute_capability)
