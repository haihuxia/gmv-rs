@echo off

if not exist "C:\Users\%username%\.ssh\id_rsa.pub" (
  echo 下面的命令连续敲三个回车即可
  echo;
  ssh-keygen -t rsa
)

echo;
echo;
echo 请复制下面的内容
echo;
set /P OEM=<"C:\Users\%username%\.ssh\id_rsa.pub"
echo %OEM%

pause>nul