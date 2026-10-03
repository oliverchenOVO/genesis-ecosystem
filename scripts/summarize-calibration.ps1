param(
    [Parameter(Mandatory=$true)][string]$InputPath,
    [Parameter(Mandatory=$true)][string]$OutputPath
)
$ErrorActionPreference = 'Stop'
$report = Get-Content -LiteralPath $InputPath -Raw | ConvertFrom-Json
if ($report.results.Count -ne $report.seeds) { throw 'Incomplete seed report' }
if (($report.results.seed | Sort-Object -Unique).Count -ne $report.seeds) { throw 'Duplicate seeds' }
$failures = @($report.results | Where-Object { $_.PSObject.Properties.Name -contains 'error' })
$rows = @($report.results | Where-Object { $_.PSObject.Properties.Name -notcontains 'error' })
if (@($rows | Where-Object { $_.ticks -ne $report.ticks_per_seed }).Count) { throw 'Incomplete world ticks' }
function Get-Distribution([object[]]$Values) {
    $ordered = @($Values | Where-Object { $null -ne $_ } | ForEach-Object { [double]$_ } | Sort-Object)
    if (-not $ordered.Count) { return @{count=0;min=$null;median=$null;p90=$null;p95=$null;max=$null;mean=$null} }
    $last = $ordered.Count - 1
    return @{count=$ordered.Count; min=$ordered[0]; median=$ordered[[Math]::Floor($last*0.5+0.5)]; p90=$ordered[[Math]::Floor($last*0.9+0.5)]; p95=$ordered[[Math]::Floor($last*0.95+0.5)]; max=$ordered[$last]; mean=($ordered | Measure-Object -Average).Average}
}
$distributions = [ordered]@{}
foreach ($field in @('final_population','peak_population','minimum_population_after_warmup','species_formed','maximum_simultaneous_species','final_living_species','extinct_species','first_speciation_tick','first_extinction_tick','births','deaths','predations','mutations','final_mean_genome_distance_to_centroid','mean_completed_lifespan','population_coefficient_of_variation','maximum_persistent_lineages','elapsed_seconds','ticks_per_second','organism_updates_per_second')) {
    $distributions[$field] = Get-Distribution @($rows | ForEach-Object { $_.$field })
}
$classifications = [ordered]@{}
foreach ($category in @('Collapse','Healthy','Rich','Sterile','Explosion','Oscillating')) {
    $classifications[$category] = @($rows | Where-Object { $_.classification -eq $category }).Count
}
$summary = [ordered]@{
    source=$InputPath; scenario=$report.scenario; simulation_version=$report.simulation_version; analysis_version=$report.analysis_version;
    seeds=$report.seeds; ticks_per_seed=$report.ticks_per_seed; workers=$report.workers; batch_elapsed_seconds=$report.elapsed_seconds;
    failures=$failures.Count; classifications=$classifications;
    worlds_with_speciation=@($rows | Where-Object { $_.species_formed -gt 0 }).Count;
    rich_without_speciation=@($rows | Where-Object { $_.classification -eq 'Rich' -and $_.species_formed -eq 0 }).Count;
    worlds_surviving=@($rows | Where-Object { $_.final_population -gt 0 }).Count;
    distributions=$distributions
}
$summary | ConvertTo-Json -Depth 20 | Set-Content -Encoding utf8 -LiteralPath $OutputPath
$summary | ConvertTo-Json -Depth 20
