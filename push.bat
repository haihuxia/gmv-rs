@echo off

cd %1
git remote remove origin
git remote add origin %2
echo ""
git remote -v
git push -u origin master

