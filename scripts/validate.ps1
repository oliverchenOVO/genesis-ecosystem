param([switch]$Package, [switch]$FullStress)

$ErrorActionPreference = 'Stop'
$taskRoot = Split-Path -Parent $PSScriptRoot
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    $env:PATH = (Join-Path $env:USERPROFILE '.cargo/bin') + ';' + $env:PATH
}
function Invoke-Checked {
    param([string]$Name, [scriptblock]$Command)
    Write-Host $Name
    & $Command
    if ($LASTEXITCODE -ne 0) { throw "$Name failed with exit code $LASTEXITCODE" }
}

Push-Location -LiteralPath $taskRoot
try {
    New-Item -ItemType Directory -Force artifacts | Out-Null
    Invoke-Checked 'Frontend formatting' { pnpm format:check }
    Invoke-Checked 'Frontend lint' { pnpm lint }
    Invoke-Checked 'TypeScript' { pnpm typecheck }
    Invoke-Checked 'Frontend tests' { pnpm test }
    Invoke-Checked 'Frontend production build' { pnpm build }
    Invoke-Checked 'Rust formatting' { cargo fmt --all -- --check }
    Invoke-Checked 'Rust lint' { cargo clippy --workspace --all-targets --locked -- -D warnings }
    Invoke-Checked 'Rust tests' { cargo test --release --workspace --locked }
    Invoke-Checked 'Headless and application release build' {
        cargo build --release --locked -p sim-core -p sim-app -p sim-benchmark -p sim-server
    }
    Invoke-Checked 'Small benchmark' {
        & ./target/release/sim-benchmark.exe --seed 42 --ticks 10000 --population 500 --limit 2000 |
            Set-Content -Encoding utf8 artifacts/benchmark-small.json
    }
    Invoke-Checked 'Medium benchmark' {
        & ./target/release/sim-benchmark.exe --seed 42 --ticks 50000 --population 2000 --limit 2000 |
            Set-Content -Encoding utf8 artifacts/benchmark-medium.json
    }
    Invoke-Checked 'Sustained ecosystem benchmark' {
        & ./target/release/sim-benchmark.exe --seed 11 --ticks 50000 --population 50 --limit 200 |
            Set-Content -Encoding utf8 artifacts/benchmark-sustained.json
    }
    if ($FullStress) {
        Invoke-Checked '100-seed stability gate' {
            & ./target/release/sim-benchmark.exe --stress --seeds 100 --ticks 50000 --population 50 --limit 200 |
                Set-Content -Encoding utf8 artifacts/stress-latest.json
        }
    }
    if ($Package) { Invoke-Checked 'Windows distribution' { pnpm tauri build } }
    Write-Host 'Validation passed. Benchmark outputs are in artifacts/.'
} finally {
    Pop-Location
}
