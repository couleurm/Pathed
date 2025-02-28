<# : batch portion, replace this whole block on *nix with !#/usr/bin/pwsh
@echo off
cd /D "%~dp0"
@echo %~0 | powershell.exe -command "$exepath = $input | ?{$_}; iex (${%~f0} | out-string)"
: end batch / begin powershell #>
Add-Type -AssemblyName Microsoft.VisualBasic

$pd_executable = "pathed2.exe"

$ext = if (!$IsLinux){
    '.exe'
} else {
    ''
}

$dir = $exepath | Split-Path

$old_links = Get-ChildItem -File $dir | Where-Object {$_.Attributes.HasFlag([IO.FileAttributes]::ReparsePoint)}

foreach ($link in $old_links){ [Microsoft.VisualBasic.FileIO.FileSystem]::DeleteFile($link,'OnlyErrorDialogs','SendToRecycleBin')}

$txt = Join-Path $dir pd.txt


$cfg = Get-Content $txt |
    # discards comments
    Where-Object {$_ -like "*;*"} |

    # for each line
    Foreach-Object { 
        # get the first alias
        ($_ -split ';' -split '\|' |
        # only retain the first word and remove spaces around it
        Select-Object -First 1).trim()
    } |
    # trim out empty strings
    Where-Object {$_}

foreach($filename in $cfg){
    & "$env:COMSPEC" /c mklink (Join-Path $dir "$filename$ext") (Join-Path $dir $pd_executable)
    # New-Item -ItemType SymbolicLink -Path (Join-Path $dir "$filename$ext") -Target (Join-Path $dir $pd_executable)
}

$folders = Get-Content $txt | Where-Object {$_ -notlike "*;*" -and $_ -like "*=*"} | ConvertFrom-StringData

foreach($folder in $folders.GetEnumerator()){

    if (Test-Path $folder.value -PathType Leaf){
        Write-Host "note: $($folder.value) is a file, not a folder"
    }
    elseif (-not(Test-Path $folder.value -PathType Container)){
        Write-Warning "$($folder.value) does not exist, skipping!"
        continue
    }

    $admin, $minimized, $maximized = $false
    
    switch($folder.key){
        {$_ -like "\$*"}{
            $admin = $true
            $name = $_ -replace "\^",""
            continue
        }
        {$_ -like "_*"}{
            $minimized = $true
            $name = $_ -replace '_',''
            continue
        }
        {$_ -like "^*"}{
            $maximized = $true
            $name = $_ -replace '_',''
            continue
        }
    }

    $lnkPath = Join-Path $dir "$name.lnk"
    $WScriptShell = New-Object -ComObject WScript.Shell
    $Shortcut = $WScriptShell.CreateShortcut($lnkPath)
    $Shortcut.TargetPath = $folder.value

    enum WindowStyle {
        Normal    = 1
        Maximized = 3
        Minimized = 7
    }
    if ($minimized){
        $Shortcut.WindowStyle = [WindowStyle]::Minimized
    }
    elseif ($maximized){
        $Shortcut.WindowStyle = [WindowStyle]::Maximized
    }
    if ($admin){
        $bytes = [System.IO.File]::ReadAllBytes($LnkPath)
        $bytes[0x15] = $bytes[0x15] -bor 0x20 #set byte 21 (0x15) bit 6 (0x20) ON
        [System.IO.File]::WriteAllBytes($LnkPath, $bytes)
    }

}