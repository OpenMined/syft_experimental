import importlib

# find and run the import fixer
package_name = __name__.split(".")[0]
import_fixer = importlib.import_module(".import_fixer", package=package_name)
import_fixer.fix_imports(locals(), __file__)
