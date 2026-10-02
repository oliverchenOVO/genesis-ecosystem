# Temporary local prerequisite fallback; normal installed MSVC requires no fallback.
$taskRoot = Split-Path $PSScriptRoot -Parent
$env:PATH = (Join-Path $env:USERPROFILE '.cargo\bin') + ';' + $env:PATH
$taskVc = Join-Path $taskRoot '.tools\msvc\Contents\VC\Tools\MSVC\14.44.35207'
$taskSdk = Join-Path $taskRoot '.tools\sdk\Windows Kits\10'
if (Test-Path -LiteralPath (Join-Path $taskVc 'bin\Hostx64\x64\link.exe')) {
    $env:PATH = (Join-Path $taskVc 'bin\Hostx64\x64') + ';' + (Join-Path $taskSdk 'bin\10.0.26100.0\x64') + ';' + $env:PATH
    $env:LIB = (Join-Path $taskVc 'lib\x64') + ';' + (Join-Path $taskSdk 'Lib\10.0.26100.0\ucrt\x64') + ';' + (Join-Path $taskSdk 'Lib\10.0.26100.0\um\x64')
    $env:INCLUDE = (Join-Path $taskVc 'include') + ';' + (Join-Path $taskSdk 'Include\10.0.26100.0\ucrt') + ';' + (Join-Path $taskSdk 'Include\10.0.26100.0\shared') + ';' + (Join-Path $taskSdk 'Include\10.0.26100.0\um')
}
