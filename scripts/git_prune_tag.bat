@echo off 
cd ../.git 
set tagFileName=local-tag.txt 
git tag -l >> %tagFileName%
for /f %%x in (%tagFileName%) do git tag -d %%x
git fetch origin --prune 
git fetch origin --tags 
del %tagFileName%
cd ..
