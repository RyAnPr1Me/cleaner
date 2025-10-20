@echo off
REM Build script for Cleaner - Rust-based performance improver (Windows)

echo Building Cleaner...

REM Build release version
cargo build --release

REM Create build directory
if not exist build mkdir build

REM Copy binary to build directory
copy target\release\cleaner.exe build\cleaner.exe

echo.
echo ✓ Build complete!
echo.
echo Binary location: build\cleaner.exe
dir build\cleaner.exe
echo.
echo To run: build\cleaner.exe --help
