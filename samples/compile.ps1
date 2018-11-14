if (!(Get-Command wat2wasm -ErrorAction SilentlyContinue)) {
    throw "Missing required command 'wat2wasm'"
}

Get-ChildItem $PSScriptRoot -fil *.wat | ForEach-Object {
    $target = [IO.Path]::ChangeExtension($_.FullName, ".wasm")
    Write-Host "Compiling $($_.FullName) ..."

    if(Test-Path $target) {
        Remove-Item $target
    }

    wat2wasm $_.FullName -o $target --debug-names
}