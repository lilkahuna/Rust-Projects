Write-Host 'Make sure fsx.exe is in the directory that this is ran from';
Write-Host 'Press any key to continue...';
$null = $Host.UI.RawUI.ReadKey('NoEcho,IncludeKeyDown');

mkdir 'C:\Program Files\fsx'
$sourcepath = Join-Path -Path $PWD.Path -ChildPath 'fsx.exe'
Move-Item -Path $sourcepath -Destination 'C:\Program Files\fsx'

$path = $env:Path + 'C:\Program Files\fsx;'

[System.Environment]::SetEnvironmentVariable('Path', $path, 'Machine')
Write-Output 'Set system wide enviroment variable'
