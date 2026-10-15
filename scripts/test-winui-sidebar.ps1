# Run against an already launched windows-winui Showcase (English, unfiltered sidebar).
# Native selection is essential: dayscript's tree_select suppresses the callback that
# caused the section-heading stack overflow.
param([int]$AppProcessId = 0, [int]$Iterations = 25, [int]$CollapseIterations = 10)
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName UIAutomationClient
Add-Type -AssemblyName UIAutomationTypes
if (!$AppProcessId) { $AppProcessId = (Get-Process day-showcase | Select-Object -First 1).Id }
if (!$AppProcessId) { throw 'Launch Day-Showcase on windows-winui first.' }

function Find-Sidebar {
    $process = New-Object System.Windows.Automation.PropertyCondition([System.Windows.Automation.AutomationElement]::ProcessIdProperty, $AppProcessId)
    $type = New-Object System.Windows.Automation.PropertyCondition([System.Windows.Automation.AutomationElement]::ControlTypeProperty, [System.Windows.Automation.ControlType]::Tree)
    $condition = New-Object System.Windows.Automation.AndCondition($process, $type)
    $tree = [System.Windows.Automation.AutomationElement]::RootElement.FindFirst([System.Windows.Automation.TreeScope]::Descendants, $condition)
    if ($null -eq $tree) { throw 'Showcase sidebar is missing or the app exited.' }
    return $tree
}

$tree = Find-Sidebar
$textType = New-Object System.Windows.Automation.PropertyCondition([System.Windows.Automation.AutomationElement]::ControlTypeProperty, [System.Windows.Automation.ControlType]::Text)
$labels = @($tree.FindAll([System.Windows.Automation.TreeScope]::Descendants, $textType) | ForEach-Object { $_.Current.Name })
if ($labels -match '^Microsoft\..*TreeViewNode') { throw 'Sidebar renders native class names.' }
if ('About' -notin $labels) { throw 'Sidebar has no rendered About label.' }

$itemType = New-Object System.Windows.Automation.PropertyCondition([System.Windows.Automation.AutomationElement]::ControlTypeProperty, [System.Windows.Automation.ControlType]::TreeItem)
for ($i = 0; $i -lt $Iterations; $i++) {
    $items = (Find-Sidebar).FindAll([System.Windows.Automation.TreeScope]::Descendants, $itemType)
    if ($items.Count -lt 2) { throw 'Expected an expanded section and its destination.' }
    if ($items[1].Current.Name -ne 'About') { throw 'The About destination has an incorrect accessibility name.' }
    $leaf = $items[1].GetCurrentPattern([System.Windows.Automation.SelectionItemPattern]::Pattern)
    $leaf.Select()
    $items[0].GetCurrentPattern([System.Windows.Automation.SelectionItemPattern]::Pattern).Select()
    Start-Sleep -Milliseconds 100
    Get-Process -Id $AppProcessId -ErrorAction Stop | Out-Null
    if (!$leaf.Current.IsSelected) { throw "Section selection lost the destination highlight (iteration $i)." }
}
Write-Output "PASS: sidebar labels render; $Iterations native destination/heading selections preserved the destination without crashing."

function Assert-RenderedRows {
    # TreeItem names survive when the template has lost its content. Inspect actual text
    # descendants, not just the navigation item's name or ability to receive selection.
    $deadline = (Get-Date).AddSeconds(2)
    do {
        $missing = @()
        $items = (Find-Sidebar).FindAll([System.Windows.Automation.TreeScope]::Descendants, $itemType)
        foreach ($item in $items) {
            if ($item.Current.IsOffscreen) { continue }
            $texts = @($item.FindAll([System.Windows.Automation.TreeScope]::Descendants, $textType) | ForEach-Object { $_.Current.Name })
            if ($item.Current.Name -notin $texts) { $missing += $item.Current.Name }
        }
        if (!$missing.Count) { return }
        Start-Sleep -Milliseconds 50
    } while ((Get-Date) -lt $deadline)
    throw "Visible sidebar rows lost their rendered content: $($missing -join ', ')"
}

foreach ($group in @('Overview', 'Controls', 'Layout')) {
    for ($i = 0; $i -lt $CollapseIterations; $i++) {
        foreach ($expand in @($false, $true)) {
            $items = (Find-Sidebar).FindAll([System.Windows.Automation.TreeScope]::Descendants, $itemType)
            $header = $items | Where-Object { $_.Current.Name -eq $group } | Select-Object -First 1
            if (!$header) { throw "Missing $group section" }
            $pattern = $header.GetCurrentPattern([System.Windows.Automation.ExpandCollapsePattern]::Pattern)
            if ($expand) { $pattern.Expand() } else { $pattern.Collapse() }
            Start-Sleep -Milliseconds 150
            Assert-RenderedRows
        }
    }
}
Write-Output "PASS: $($CollapseIterations * 3) native collapse/expand cycles kept the visible rows rendered."
