chcp 437
cls

@title determine whether the folder exists or not

@echo off

rem remove / from the date
set date=%date:/=%

rem remove : from the time

set time=%time::=%
rem set time=%time: =%
if "%time:~0,1%"==" " set time0=0%time:~1%

set yymmddss=%date:~3,4%_%date:~7,2%_%date:~9,2%_%time0:~0,6%

echo %yymmddss%

if not exist backups ( @md backups
)

@md backups\%yymmddss%

if not exist .git ( echo .git does not exist
) else ( @xcopy/s/e/y .git\ backups\%yymmddss%\.git\
)

if not exist .git_hub ( echo .git_hub does not exist
) else ( @xcopy/s/e/y .git_hub\ backups\%yymmddss%\.git_hub\
)

if not exist .git_ee ( echo .git_ee does not exist
) else ( @xcopy/s/e/y .git_ee\ backups\%yymmddss%\.git_ee\
)


@pause