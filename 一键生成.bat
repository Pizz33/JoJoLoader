@echo off
setlocal EnableDelayedExpansion

python encrypt.py

cargo build --release

set "sourceFile=target\release\project.exe"
set "targetDir=output"

set "chars=ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
set "randomName="

for /l %%i in (1,1,6) do (
    set /a "randIndex=!random! %% 52"
    for %%j in (!randIndex!) do set "randomName=!randomName!!chars:~%%j,1!"
)

set "targetFile=%targetDir%\!randomName!.exe"

copy "%sourceFile%" "%targetFile%"

echo Copied and renamed to: "%targetFile%"

endlocal
