import setuptools
import pathlib

here = pathlib.Path(__file__).parent.resolve()
long_description = (here / "README.md").read_text(encoding="utf-8")

setuptools.setup(
    name="fingerprint_analyze",
    version="0.1.0",
    author="Fuhao",
    author_email="fangshiyu2@gmail.com",
    description="A package for detecting the session fingerprint",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/tangfuhao/WasmFingerprint/tree/main/backend/py/fingerprint_analyze",
    packages=setuptools.find_packages(),
    install_requires=[],
    python_requires='>=3.6',
    license="MIT",
    classifiers=[],
    keywords="wasm",
)