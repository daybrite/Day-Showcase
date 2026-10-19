# Run against an English windows-winui Showcase. Exercises native input, not injected list events.
param([string]$DayCli = "$PSScriptRoot/../../day/target/debug/day.exe")
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName UIAutomationClient
Add-Type -AssemblyName UIAutomationTypes
Add-Type -AssemblyName System.Windows.Forms
Add-Type @'
using System;
using System.Runtime.InteropServices;
public static class ShowcaseListInput {
    [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr hwnd);
    [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
    [DllImport("user32.dll")] public static extern void mouse_event(uint flags, uint x, uint y, uint data, UIntPtr extra);
    [DllImport("user32.dll")] private static extern int GetSystemMetrics(int index);
    public static void Move(int x, int y) {
        // Inject MOVE | ABSOLUTE | VIRTUALDESK. SetCursorPos moves the cursor but
        // does not feed WinUI's drag recognizer with the required input packets.
        mouse_event(0xC001, (uint)((x - GetSystemMetrics(76)) * 65535L / (GetSystemMetrics(78) - 1)),
            (uint)((y - GetSystemMetrics(77)) * 65535L / (GetSystemMetrics(79) - 1)), 0, UIntPtr.Zero);
    }
}
'@
function Drive($Steps) {
    $json = ConvertTo-Json -InputObject @($Steps) -Depth 12 -Compress
    $output = $json | python -c "import subprocess,sys; r=subprocess.run([sys.argv[1],'drive','--project',sys.argv[2],'-t','windows-winui','--steps-json',sys.stdin.read()],capture_output=True,text=True,encoding='utf-8'); print(r.stdout); sys.stderr.write(r.stderr)" $DayCli "$PSScriptRoot/.."
    $report = $output | ConvertFrom-Json
    if (!$report -or $report.failed -ne 0) { throw "Drive failed: $output" }
}
function Find-Id($Id) {
    $byId = New-Object System.Windows.Automation.PropertyCondition([System.Windows.Automation.AutomationElement]::AutomationIdProperty, $Id)
    $element = $script:window.FindFirst([System.Windows.Automation.TreeScope]::Descendants, $byId)
    if (!$element) { throw "Missing native element $Id" }
    return $element
}
function Row-Container($Number) {
    $element = Find-Id "list-row:$Number"
    while ($element -and $element.Current.ClassName -notmatch '(^|\.)ItemContainer$') {
        $element = [System.Windows.Automation.TreeWalker]::RawViewWalker.GetParent($element)
    }
    if (!$element) { throw "Row $Number is not hosted in a native ItemContainer" }
    $bounds = $element.Current.BoundingRectangle
    if ($element.Current.IsOffscreen -or $bounds.Width -lt 100 -or $bounds.Height -lt 20) {
        throw "Row $Number has invalid native bounds: $bounds"
    }
    return $element
}
function Drag-Row($From, $To) {
    [ShowcaseListInput]::SetForegroundWindow([IntPtr]$window.Current.NativeWindowHandle) | Out-Null
    Start-Sleep -Milliseconds 200
    $a = (Row-Container $From).Current.BoundingRectangle
    $b = (Row-Container $To).Current.BoundingRectangle
    $x = [int]($a.Left + 60)
    $startY = $a.Top + $a.Height / 2
    $endY = $b.Top + $b.Height / 2
    [ShowcaseListInput]::SetCursorPos($x, [int]$startY) | Out-Null
    Start-Sleep -Milliseconds 100
    [ShowcaseListInput]::mouse_event(2, 0, 0, 0, [UIntPtr]::Zero)
    Start-Sleep -Milliseconds 200
    for ($step = 1; $step -le 20; $step++) {
        [ShowcaseListInput]::Move($x, [int]($startY + ($endY - $startY) * $step / 20))
        Start-Sleep -Milliseconds 40
    }
    [ShowcaseListInput]::mouse_event(4, 0, 0, 0, [UIntPtr]::Zero)
    Start-Sleep -Milliseconds 300
}
Drive @(@{op='navigate';route='list'}, @{op='wait_for';id='list-row:1'})
$app = Get-Process day-showcase | Select-Object -First 1
$byProcess = New-Object System.Windows.Automation.PropertyCondition([System.Windows.Automation.AutomationElement]::ProcessIdProperty, $app.Id)
$script:window = [System.Windows.Automation.AutomationElement]::RootElement.FindFirst([System.Windows.Automation.TreeScope]::Children, $byProcess)
[ShowcaseListInput]::SetForegroundWindow([IntPtr]$window.Current.NativeWindowHandle) | Out-Null
$lastRow = [int]([regex]::Match((Find-Id 'list-caption').Current.Name, '\d+').Value)
for ($i = 0; $i -lt 5; $i++) {
    Drive @(@{op='tap';id='list-scroll-bottom'}, @{op='wait_for';id="list-row:$lastRow"}, @{op='pause';secs=0.15})
    Row-Container $lastRow | Out-Null
    Drive @(@{op='tap';id='list-scroll-top'}, @{op='wait_for';id='list-row:1'}, @{op='pause';secs=0.15})
    Row-Container 1 | Out-Null
}
# Real mouse selection and keyboard activation of a recycled row.
Drive @(@{op='tap';id='list-clear'})
$automationRow = Row-Container 4
$automationRow.GetCurrentPattern([System.Windows.Automation.SelectionItemPattern]::Pattern).Select()
Drive @(@{op='assert_text';id='list-selection';key='list_selection';args=@{count=1;rows='4'}}, @{op='tap';id='list-clear'})
$row = Row-Container 2
$bounds = $row.Current.BoundingRectangle
[ShowcaseListInput]::SetCursorPos([int]($bounds.Left + 30), [int]($bounds.Top + $bounds.Height / 2)) | Out-Null
[ShowcaseListInput]::mouse_event(2, 0, 0, 0, [UIntPtr]::Zero)
[ShowcaseListInput]::mouse_event(4, 0, 0, 0, [UIntPtr]::Zero)
Drive @(@{op='assert_text';id='list-selection';key='list_selection';args=@{count=1;rows='2'}})
[System.Windows.Forms.SendKeys]::SendWait('{ENTER}')
Drive @(@{op='assert_text';id='list-activated';key='list_activated';args=@{row=2}})
[System.Windows.Forms.SendKeys]::SendWait('{DOWN}')
Drive @(@{op='assert_text';id='list-selection';key='list_selection';args=@{count=1;rows='3'}})
# Keyboard context menu invokes the same guarded delete offer as native swipe.
[System.Windows.Forms.SendKeys]::SendWait('+{F10}')
Start-Sleep -Milliseconds 200
$byName = New-Object System.Windows.Automation.PropertyCondition([System.Windows.Automation.AutomationElement]::NameProperty, 'Delete')
$byType = New-Object System.Windows.Automation.PropertyCondition([System.Windows.Automation.AutomationElement]::ControlTypeProperty, [System.Windows.Automation.ControlType]::MenuItem)
$condition = New-Object System.Windows.Automation.AndCondition($byName, $byType)
$delete = $window.FindFirst([System.Windows.Automation.TreeScope]::Descendants, $condition)
if (!$delete) { throw 'The native row context menu did not offer Delete' }
$delete.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern).Invoke()
Drive @(@{op='assert_text';id='list-order';key='list_order';args=@{rows='1,2,4,5,6'}}, @{op='tap';id='list-reset'})
Drag-Row 2 5
Drive @(@{op='assert_text';id='list-order';key='list_order';args=@{rows='1,3,4,5,2'}}, @{op='tap';id='list-reset'})
Drag-Row 1 5
Drive @(@{op='assert_text';id='list-order';key='list_order';args=@{rows='1,2,3,4,5'}})
Drag-Row 4 1
Drive @(@{op='assert_text';id='list-order';key='list_order';args=@{rows='1,4,2,3,5'}}, @{op='tap';id='list-reset'})
Write-Output 'PASS: native rows/recycling, mouse and accessibility selection, keyboard navigation/activation, context-menu deletion, and guarded drag-to-reorder.'
