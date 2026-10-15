# Run against a launched English windows-winui Showcase. Requires Python and the Day CLI.
param([string]$DayCli = "$PSScriptRoot/../../day/target/debug/day.exe")
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName UIAutomationClient
Add-Type -AssemblyName UIAutomationTypes
Add-Type -AssemblyName System.Windows.Forms
Add-Type 'using System; using System.Runtime.InteropServices; public static class ShowcaseInput { [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr hwnd); }'
function Drive($Steps) {
    $json = ConvertTo-Json -InputObject @($Steps) -Depth 12 -Compress
    $output = $json | python -c "import subprocess,sys; r=subprocess.run([sys.argv[1],'drive','-t','windows-winui','--steps-json',sys.stdin.read()],capture_output=True,text=True,encoding='utf-8'); print(r.stdout); sys.stderr.write(r.stderr)" $DayCli
    $report = $output | ConvertFrom-Json
    if (!$report -or $report.failed -ne 0) { throw "Drive failed: $output" }
}
function Find-Field($Id) {
    $byId = New-Object System.Windows.Automation.PropertyCondition([System.Windows.Automation.AutomationElement]::AutomationIdProperty, $Id)
    $field = $script:window.FindFirst([System.Windows.Automation.TreeScope]::Descendants, $byId)
    if (!$field) { throw "No mounted native control with ID $Id" }
    if ($field.Current.IsOffscreen -or $field.Current.BoundingRectangle.Width -lt 50) { throw "$Id has invalid native bounds" }
    return $field
}
Drive @(@{op='navigate';route='textfields'}, @{op='wait_for';id='tf-password'}, @{op='pause';secs=0.2})
$app = Get-Process day-showcase | Select-Object -First 1
$byProcess = New-Object System.Windows.Automation.PropertyCondition([System.Windows.Automation.AutomationElement]::ProcessIdProperty, $app.Id)
$script:window = [System.Windows.Automation.AutomationElement]::RootElement.FindFirst([System.Windows.Automation.TreeScope]::Children, $byProcess)
[ShowcaseInput]::SetForegroundWindow([IntPtr]$window.Current.NativeWindowHandle) | Out-Null
foreach ($shown in @($false, $true, $false, $true, $false)) {
    Drive @(@{op='toggle';id='tf-show-password';value=$shown})
    $field = Find-Field 'tf-password'
    if ($field.Current.IsPassword -eq $shown) { throw "Incorrect native password masking (shown=$shown)" }
    $field.SetFocus()
    [System.Windows.Forms.SendKeys]::SendWait('^a')
    [System.Windows.Forms.SendKeys]::SendWait('Native123')
    Drive @(@{op='assert_text';id='tf-password-length';text='9'}, @{op='assert_focused';id='tf-password'})
}
Drive @(@{op='scroll_to';id='tf-pin'})
$pin = Find-Field 'tf-pin'
if (!$pin.Current.IsPassword) { throw 'PIN is not a native masked control' }
$pin.SetFocus()
[System.Windows.Forms.SendKeys]::SendWait('^a123456')
Drive @(@{op='assert_text';id='tf-pin-count';text='4 / 4'})
Write-Output 'PASS: native password/PIN mounting, masking, keyboard edits, focus and repeated reveal transitions.'

