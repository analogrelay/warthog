# This script regenerates the spec test data in this directory.
if (!(Get-Command wast2json -ErrorAction SilentlyContinue)) {
    throw "Missing required command 'wast2json'"
}

$RepoRoot = Convert-Path "$PSScriptRoot/.."
$SpecTestRoot = [IO.Path]::Combine($RepoRoot, "vendor", "webassembly", "test", "core")
$DestRoot = [IO.Path]::Combine($RepoRoot, "tests", "spec")

Get-ChildItem $SpecTestRoot -Filter *.wast | ForEach-Object {
    Write-Host "Processing $($_.Name)"
    $name = [IO.Path]::GetFileNameWithoutExtension($_.FullName)
    $DestDir = Join-Path $DestRoot $name
    mkdir $Dest | Out-Null
    $Dest = Join-Path $DestDir "$name.json"
    wast2json $_.FullName -o $Dest
}