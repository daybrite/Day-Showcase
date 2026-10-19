# Run against a launched English windows-winui Showcase. Only setup/assertions use
# dayscript: clicks and transfers go through Windows input and native hit testing.
param([string]$DayCli = "$PSScriptRoot/../../day/target/debug/day.exe")
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName UIAutomationClient
Add-Type -AssemblyName UIAutomationTypes
Add-Type @'
using System;
using System.Runtime.InteropServices;
public static class ShowcaseCanvasInput {
    [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr hwnd);
    [DllImport("user32.dll")] private static extern int GetSystemMetrics(int index);
    [DllImport("user32.dll")] private static extern void mouse_event(uint flags, uint x, uint y, uint data, UIntPtr extra);
    public static void Move(int x, int y) {
        mouse_event(0xC001, (uint)((x - GetSystemMetrics(76)) * 65535L / (GetSystemMetrics(78) - 1)),
            (uint)((y - GetSystemMetrics(77)) * 65535L / (GetSystemMetrics(79) - 1)), 0, UIntPtr.Zero);
    }
    public static void Down() { mouse_event(2, 0, 0, 0, UIntPtr.Zero); }
    public static void Up() { mouse_event(4, 0, 0, 0, UIntPtr.Zero); }
}
'@
function Drive($Steps) {
    $json = ConvertTo-Json -InputObject @($Steps) -Depth 12 -Compress
    $output = $json | python -c "import subprocess,sys; r=subprocess.run([sys.argv[1],'drive','--project',sys.argv[2],'-t','windows-winui','--steps-json',sys.stdin.read()],capture_output=True,text=True,encoding='utf-8'); print(r.stdout); sys.stderr.write(r.stderr)" $DayCli "$PSScriptRoot/.."
    $report = $output | ConvertFrom-Json
    if (!$report -or $report.failed -ne 0) { throw "Drive failed: $output" }
}
function Find-Id($Id) {
    $condition = New-Object System.Windows.Automation.PropertyCondition([System.Windows.Automation.AutomationElement]::AutomationIdProperty, $Id)
    $element = $script:window.FindFirst([System.Windows.Automation.TreeScope]::Descendants, $condition)
    if (!$element) { throw "Missing native element $Id" }
    return $element
}
function Bounds($Id) {
    $element = Find-Id $Id
    $bounds = $element.Current.BoundingRectangle
    if ($element.Current.IsOffscreen -or $bounds.Width -lt 50 -or $bounds.Height -lt 30) {
        throw "Invalid native bounds for ${Id}: $bounds"
    }
    return $bounds
}
function Foreground {
    [ShowcaseCanvasInput]::SetForegroundWindow([IntPtr]$window.Current.NativeWindowHandle) | Out-Null
    Start-Sleep -Milliseconds 100
}
function Tile-Bounds($Index) {
    # An unlabeled drawing has no UIA peer. Its MIME label is the next child in
    # the zero-spacing column: both are 170 points wide, the drawing is 112 high.
    # Derive physical pixels from that native label, including the display scale.
    $label = Find-Id "dnd-value-$Index"
    if ($label.Current.IsOffscreen) { throw "Tile $Index is offscreen" }
    $bounds = $label.Current.BoundingRectangle
    $height = $bounds.Width * 112.0 / 170
    $bounds.Y -= $height
    $bounds.Height = $height
    return $bounds
}
function Drag-Tile($From, $To, $Reject = $false) {
    Foreground
    $a = Tile-Bounds $From
    $b = Tile-Bounds $To
    $x = $a.Left + $a.Width / 2; $y = $a.Top + $a.Height / 2
    # Slot 3's rejection circle is centered at (40, 36) in its 170 x 112 canvas.
    $tx = $b.Left + $b.Width * $(if ($Reject) { 40.0 / 170 } else { 0.5 })
    $ty = $b.Top + $b.Height * $(if ($Reject) { 36.0 / 112 } else { 0.5 })
    [ShowcaseCanvasInput]::Move([int]$x, [int]$y)
    Start-Sleep -Milliseconds 100
    [ShowcaseCanvasInput]::Down()
    try {
        Start-Sleep -Milliseconds 100
        for ($i = 1; $i -le 30; $i++) {
            [ShowcaseCanvasInput]::Move([int]($x + ($tx - $x) * $i / 30), [int]($y + ($ty - $y) * $i / 30))
            Start-Sleep -Milliseconds 30
        }
    } finally { [ShowcaseCanvasInput]::Up() }
    Start-Sleep -Milliseconds 500
}
$app = Get-Process day-showcase | Select-Object -First 1
$byProcess = New-Object System.Windows.Automation.PropertyCondition([System.Windows.Automation.AutomationElement]::ProcessIdProperty, $app.Id)
$script:window = [System.Windows.Automation.AutomationElement]::RootElement.FindFirst([System.Windows.Automation.TreeScope]::Children, $byProcess)
Drive @(@{op='navigate';route='physics'}, @{op='wait_for';id='frame-ball'}, @{op='set_value';id='frame-ball-slider';value=40})
Foreground
for ($i = 0; $i -lt 5; $i++) {
    Drive @(@{op='tap';id='frame-reset'}, @{op='assert_text';id='frame-status';key='frame_resting'})
    $b = Bounds 'frame-ball'
    [ShowcaseCanvasInput]::Move([int]($b.Left + $b.Width * (0.2 + $i * 0.12)), [int]($b.Top + $b.Height / 2))
    Start-Sleep -Milliseconds 100
    [ShowcaseCanvasInput]::Down()
    Start-Sleep -Milliseconds 80
    [ShowcaseCanvasInput]::Up()
    Drive @(@{op='pause';secs=0.3}, @{op='assert_text';id='frame-status';key='frame_running'}, @{op='assert_focused';id='frame-ball'})
}
# Keep the mouse pressed across animated redraws and a reset. Only releasing the
# real press can restart the reset simulation; a replaced hit target loses this tap.
[ShowcaseCanvasInput]::Down()
try {
    Start-Sleep -Milliseconds 250
    Drive @(@{op='tap';id='frame-reset'}, @{op='assert_text';id='frame-status';key='frame_resting'})
} finally { [ShowcaseCanvasInput]::Up() }
Drive @(@{op='pause';secs=0.3}, @{op='assert_text';id='frame-status';key='frame_running'})
Write-Output 'PASS: native Physics clicks, canvas focus, and a press spanning animated redraws.'
Drive @(@{op='navigate';route='drag-drop'}, @{op='wait_for';id='dnd-value-2'}, @{op='toggle';id='dnd-disallowed';on=$true})
Drag-Tile 2 3 $true
Drive @(@{op='assert_text';id='dnd-value-3';key='dnd_empty'}, @{op='assert_text';id='dnd-value-2';text='application/vnd.day.showcase-card'})
Drag-Tile 2 3
Drive @(@{op='assert_text';id='dnd-value-3';text='application/vnd.day.showcase-card'}, @{op='assert_text';id='dnd-value-2';key='dnd_empty'})
Drag-Tile 0 1
Drive @(@{op='assert_text';id='dnd-value-1';text='image/png'}, @{op='assert_text';id='dnd-value-0';key='dnd_empty'})
Drive @(@{op='toggle';id='dnd-disallowed';on=$false})
Write-Output 'PASS: native canvas drag sources, rejected drop regions, and binary/PNG transfers.'
