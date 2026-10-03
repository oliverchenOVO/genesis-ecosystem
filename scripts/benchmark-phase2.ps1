param(
    [string]$Label='final',
    [string]$Executable='',
    [string]$Context='Sequential scenarios. External host load is not controlled; concurrent task processes must be recorded by the caller.'
)
$ErrorActionPreference='Stop'
$taskRoot=Split-Path -Parent $PSScriptRoot
$executable=if($Executable){$Executable}else{Join-Path $taskRoot 'target/release/sim-benchmark.exe'}
$scenarios=@(
    @{name='reference-200';seed=11;population=50;limit=200;size=512;regen=12;ticks=50000},
    @{name='medium-1000';seed=42;population=1000;limit=1000;size=1024;regen=100;ticks=10000},
    @{name='density-5000';seed=42;population=5000;limit=5000;size=1024;regen=100;ticks=2000}
)
foreach($scenario in $scenarios){
    $prefix=Join-Path $taskRoot "benchmarks/phase2-$Label-$($scenario.name)"
    $arguments=@('--calibrate','--seeds','1','--workers','1','--seed',$scenario.seed,'--population',$scenario.population,'--limit',$scenario.limit,'--size',$scenario.size,'--regeneration',$scenario.regen,'--ticks',$scenario.ticks,'--label',"phase2-$Label-$($scenario.name)")
    $process=Start-Process -FilePath $executable -ArgumentList $arguments -RedirectStandardOutput "$prefix.json" -RedirectStandardError "$prefix.log" -WindowStyle Hidden -PassThru
    $peak=0L
    while(-not $process.HasExited){$process.Refresh();$peak=[Math]::Max($peak,$process.PeakWorkingSet64);Start-Sleep -Milliseconds 250}
    $process.WaitForExit()
    if($process.ExitCode -ne 0){throw "Benchmark failed: $($scenario.name), exit $($process.ExitCode)"}
    @{peak_working_set_bytes=$peak;measurement='Windows process peak working set sampled every 250ms; includes replay verification';context=$Context;executable_sha256=(Get-FileHash -LiteralPath $executable -Algorithm SHA256).Hash} | ConvertTo-Json | Set-Content -Encoding utf8 "$prefix-memory.json"
    $report=Get-Content -LiteralPath "$prefix.json" -Raw | ConvertFrom-Json
    if($report.failures -ne 0){throw 'World validation failed'}
    $report.results | Select-Object seed,ticks,initial_population,final_population,peak_population,elapsed_seconds,ticks_per_second,organism_updates_per_second,world_hash
}
