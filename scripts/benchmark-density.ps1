param([int]$Ticks=2000,[string]$OutputDirectory='benchmarks',[string]$Label='isolated')
$ErrorActionPreference='Stop'
$taskRoot=Split-Path -Parent $PSScriptRoot
$executable=Join-Path $taskRoot 'target/release/sim-benchmark.exe'
$outputRoot=Join-Path $taskRoot $OutputDirectory
New-Item -ItemType Directory -Force -Path $outputRoot | Out-Null
foreach ($population in @(1000,2500,5000)) {
    $capacity=if($population -eq 1000){2000}else{5000}
    $prefix=Join-Path $outputRoot "high-density-$Label-$population"
    $process=Start-Process -FilePath $executable -ArgumentList @('--calibrate','--seeds','1','--ticks',$Ticks,'--workers','1','--seed','42','--population',$population,'--limit',$capacity,'--size','1024','--regeneration','100','--label',"legal-density-$population") -RedirectStandardOutput "$prefix.json" -RedirectStandardError "$prefix.log" -WindowStyle Hidden -PassThru
    $peak=0L
    while(-not $process.HasExited){$process.Refresh();$peak=[Math]::Max($peak,$process.PeakWorkingSet64);Start-Sleep -Milliseconds 250}
    $process.WaitForExit()
    if($process.ExitCode -ne 0){throw "Density $population failed: exit $($process.ExitCode)"}
    @{scenario=$population;peak_working_set_bytes=$peak;measurement='Windows process peak working set, sampled every 250ms';timing_context=$Label} | ConvertTo-Json | Set-Content -Encoding utf8 "$prefix-memory.json"
    $report=Get-Content -LiteralPath "$prefix.json" -Raw | ConvertFrom-Json
    if($report.failures -ne 0){throw 'Density world validation failed'}
    $report.results | Select-Object seed,ticks,initial_population,final_population,peak_population,elapsed_seconds,ticks_per_second,organism_updates_per_second,world_hash
}
