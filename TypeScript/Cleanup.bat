@echo off
echo This will delete all '*.js' and '*.js.map' files. Are you sure?
pause
del *.js.map /s
del *.js /s