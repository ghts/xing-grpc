@echo off

REM 즌비믈
REM Visual Studio BuildTools 에서 clang 설치.
REM >cargo install bindgen                    # bindgen 설치
REM >rustup target add i686-pc-windows-msvc   # Xing API는 32비트 윈도우 DLL

cls

SET LIBCLANG_PATH=C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Tools\Llvm\x64\bin

cd %GOPATH%\src\github.com\ghts\xing-grpc\src

"%HOMEPATH%\.cargo\bin\bindgen.exe" --no-layout-tests type_c.h -o type_c.rs -- --target=i686-pc-windows-msvc
