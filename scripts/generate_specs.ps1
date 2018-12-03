# This script regenerates the spec test data in this directory.
if (!(Get-Command wast2json -ErrorAction SilentlyContinue)) {
    throw "Missing required command 'wast2json'"
}

$RepoRoot = Convert-Path "$PSScriptRoot/.."
$SpecTestRoot = [IO.Path]::Combine($RepoRoot, "vendor", "webassembly", "test", "core")
$DestRoot = [IO.Path]::Combine($RepoRoot, "tests", "spec")
$SpecGenRoot = [IO.Path]::Combine($RepoRoot, "specgen")

# Right now, manually specify the tests to generate Rust tests for
# This is because we don't implement everything yet.
$TestsToGenerate = @(
    "i32",
    "i64",
    "f32",
    "f32_cmp",
    "f64",
    "f64_cmp",
    # "conversions",
    # "int_exprs",
    "names"
)

Push-Location $SpecGenRoot
try {
    # Build specgen
    Write-Host -ForegroundColor Green "Building specgen..."
    cargo build

    $SpecModulePath = Join-Path $DestRoot "mod.rs"
    $SpecModuleContent = "";
    Get-ChildItem $SpecTestRoot -Filter *.wast | ForEach-Object {
        $name = [IO.Path]::GetFileNameWithoutExtension($_.FullName)
        if ($TestsToGenerate -icontains $name) {
            Write-Host "Processing $($_.Name)"
            $DestDir = Join-Path $DestRoot $name
            if (Test-Path $DestDir) {
                Remove-Item -Recurse -Force $DestDir
            }
            mkdir $DestDir | Out-Null
            $Dest = Join-Path $DestDir "$name.json"
            wast2json $_.FullName -o $Dest

            # Copy the wast file as well
            Copy-Item $_.FullName $DestDir

            # Now process the spec into a Rust test using the specgen command
            & "$SpecGenRoot\target\debug\specgen.exe" $Dest
            $SpecModuleContent += "mod $name;" + [Environment]::NewLine;
        }
    }

    # Generate the spec module
    $SpecModuleContent | Set-Content $SpecModulePath -Encoding UTF8 
}
finally {
    Pop-Location
}