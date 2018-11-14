#!/usr/bin/env powershell

#Requires -Version 5

param (
    # The name of the component to be built. Defaults to none
    [string]$Component,
    # The base hab version to run the build with. Defaults to latest
    [string]$BaseHabVersion="latest",
    # The builder channel to pull from. Defaults to stable
    [string]$SourceChannel="stable"
)

# Import shared functions
. "$PSScriptRoot\shared.ps1" -ErrorAction Stop

if($Component.Equals("")) {
    Write-Error "--- :error: Component to build not specified, please use the -Component flag" -ErrorAction Stop
}

# install buildkite agent because we are in a container :(
Set-Item Env:buildkiteAgentToken -Value "faketoken"
Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/buildkite/agent/master/install.ps1'))
Remove-Item Env:buildkiteAgentToken


$thingy = Invoke-Expression "buildkite-agent meta-data get version --job $Env:BUILDKITE_JOB_ID"
Write-Host "THING: $thingy and job $Env:BUILDKITE_JOB_ID"

# Write-Host "--- Setting source package channel to $SourceChannel"
# $Env:HAB_BLDR_CHANNEL="$SourceChannel"

# Write-Host "--- Installing base habitat binary version: $BaseHabVersion"
# $baseHabExe = [HabShared]::install_base_habitat_binary($BaseHabVersion, $SourceChannel)
# Write-Host "--- Using hab executable at $baseHabExe"

# Write-Host "--- Importing Keys"
# [HabShared]::import_keys($baseHabExe)

# Write-Host "--- Moving build folder to new location"
# New-Item -ItemType directory -Path C:\build
# Copy-Item -Path C:\workdir\* -Destination C:\build -Recurse

# Push-Location "C:\build"
#     Write-Host "--- Running hab pkg build for $Component"
#     Invoke-Expression "$baseHabExe pkg build components\$Component --keys core" -ErrorAction Stop
#     . "components\$Component\habitat\results\last_build.ps1"
#     Write-Host "Running hab pkg upload for $Component"
#     Invoke-Expression "$baseHabExe pkg upload components\$Component\habitat\results\$pkg_artifact" -ErrorAction Stop
# Pop-Location

exit $LASTEXITCODE