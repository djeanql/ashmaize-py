from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="ashmaize-py",
    version="0.1.0",
    rust_extensions=[
        RustExtension(
            "ashmaize_py",
            binding=Binding.PyO3,
            debug=False,
        )
    ],
    zip_safe=False,
)
