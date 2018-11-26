# This script regenerates the spec test data in this directory.
if (!(Get-Command wast2json -ErrorAction SilentlyContinue)) {
    throw "Missing required command 'wast2json'"
}

$RepoRoot = Convert-Path "$PSScriptRoot/.."
$SpecTestRoot = [IO.Path]::Combine($RepoRoot, "vendor", "webassembly", "test", "core")
$DestRoot = [IO.Path]::Combine($RepoRoot, "tests", "spec")

# Right now, manually specify the tests to generate Rust tests for
# This is because we don't implement everything yet.
$TestsToGenerate = @(
    "i32"
)

Get-ChildItem $SpecTestRoot -Filter *.wast | ForEach-Object {
    $name = [IO.Path]::GetFileNameWithoutExtension($_.FullName)
    if($TestsToGenerate -icontains $name) {
        Write-Host "Processing $($_.Name)"
        $DestDir = Join-Path $DestRoot $name
        if(Test-Path $DestDir) {
            Remove-Item -Recurse -Force $DestDir
        }
        mkdir $DestDir | Out-Null
        $Dest = Join-Path $DestDir "$name.json"
        wast2json $_.FullName -o $Dest

        # Now process the spec into a Rust test using the specgen command
        pushd $RepoRoot
        try {
            cargo run --bin specgen -- $Dest
        } finally {
            popd
        }
    }
}