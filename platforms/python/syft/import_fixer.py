# -*- coding: utf-8 -*-
# Author: github.com/madhavajay
"""Fixes pyo3 mixed modules for import in python"""

import importlib
import os
from typing import Dict, Any, List, Optional

# gets the name of the top level module / package
package_name = __name__.split(".")[0]

# convert the subdirs from "package_name" into a list of sub module names
def get_module_name_from_init_path(path: str) -> List[str]:
    _, path_and_file = os.path.splitdrive(os.path.dirname(path))
    module_path = path_and_file.split(package_name)[-1]
    parts = module_path.split(os.path.sep)[1:]
    return parts


# step through the main base module from rust at myproj.myproj and unpack each level
def unpack_module_from_parts(module: Any, module_parts: List[str]) -> Any:
    for part in module_parts:
        module = getattr(module, part)
    return module


# take the local scope of the caller and populate it with the correct properties
def fix_imports(lcl: Dict[str, Any], init_file_path: str, debug: bool = False) -> None:
    # rust library is available as package_name.package_name
    import_string = f".{package_name}"
    base_module = importlib.import_module(import_string, package=package_name)
    module_parts = get_module_name_from_init_path(init_file_path)
    submodule = unpack_module_from_parts(base_module, module_parts)
    if debug:
        module_path = ".".join(module_parts)
        print(f"Parsed module_name: {module_path} from: {init_file_path}")

    # re-export functions
    keys = ["builtin_function_or_method", "module"]
    for k in dir(submodule):
        if type(getattr(submodule, k)).__name__ in keys:
            if debug:
                print(f"Loading: {submodule}.{k}")
            lcl[k] = getattr(submodule, k)


# re-export a python module, class or function onto the current module level
def reexport(lcl: Dict[str, Any], obj: Any, alt_name: Optional[str] = None) -> None:
    key = obj.__name__
    if alt_name is not None:
        key = alt_name
    lcl[key] = obj
