# Automated Netlify deployment script
$processInfo = New-Object System.Diagnostics.ProcessStartInfo
$processInfo.FileName = "npx"
$processInfo.Arguments = "netlify-cli deploy --prod --dir=dist"
$processInfo.WorkingDirectory = "C:\Users\prate\linera\linera-poker\frontend"
$processInfo.RedirectStandardInput = $true
$processInfo.RedirectStandardOutput = $true
$processInfo.RedirectStandardError = $true
$processInfo.UseShellExecute = $false
$processInfo.CreateNoWindow = $false

$process = New-Object System.Diagnostics.Process
$process.StartInfo = $processInfo
$process.Start() | Out-Null

# Wait a bit for the prompt
Start-Sleep -Seconds 3

# Send Down arrow to select "Create & configure a new project"
$process.StandardInput.WriteLine([char]40)  # Down arrow
Start-Sleep -Seconds 1

# Send Enter to confirm
$process.StandardInput.WriteLine("")
Start-Sleep -Seconds 2

# Send Enter again to confirm team selection (default)
$process.StandardInput.WriteLine("")
Start-Sleep -Seconds 2

# Type site name
$process.StandardInput.WriteLine("linera-poker-conway")
Start-Sleep -Seconds 5

$output = $process.StandardOutput.ReadToEnd()
$errors = $process.StandardError.ReadToEnd()
$process.WaitForExit()

Write-Host $output
Write-Host $errors
