#!/usr/bin/env powershell

#Requires -Version 5

param (
    # # The name of the component to be built. Defaults to none
    # [string]$Component,
    # The base hab version to run the build with. Defaults to latest
    [string]$BaseHabVersion="latest",
    # The builder channel to pull from. Defaults to stable
    [string]$SourceChannel="stable"
)

# Import shared functions
. "$PSScriptRoot\shared.ps1" -ErrorAction Stop

Write-Host "--- Setting source package channel to $SourceChannel"
$Env:HAB_BLDR_CHANNEL="$SourceChannel"

Write-Host "--- Installing base habitat binary version: $BaseHabVersion"
$baseHabExe = [HabShared]::install_base_habitat_binary($BaseHabVersion, $SourceChannel)
Write-Host "--- Using hab executable at $baseHabExe"

# install buildkite agent because we are in a container :(
$Env:buildkiteAgentToken = $Env:BUILDKITE_AGENT_ACCESS_TOKEN
Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/buildkite/agent/master/install.ps1'))


$BuildVersion = Invoke-Expression "buildkite-agent meta-data get version --job $Env:BUILDKITE_JOB_ID"
Write-Host "THING: $BuildVersion and job $Env:BUILDKITE_JOB_ID"

echo "--- :windows: Publishing Windows 'hab' ${version}-${release}"
$bintray_repository="unstable"
publish "https://api.bintray.com/content/habitat/${bintray_repository}/hab-x86_64-windows/${version}-${release}/publish"


# exit $LASTEXITCODE