PARAM (
    [int] $day = ((Get-Date).Day),
    [int] $year = ((Get-Date).Year),
    [string] $template = (".\template"),
    [string] $workspaceFile = (".\cargo.toml")
)
$folder = "aoc_{0:0000}_{1:00}" -f $year, $day;

# copy template folder
if (-not (Test-Path $folder)) {
    "Create $folder"

    Copy-Item -Recurse -path $template -dest $folder

    # add folder to workspace
    $workspace = switch -Regex -File $workspaceFile {
        '^\s*]$' { 
            '"{0}",' -f $folder
            $_
        }
        Default { $_ }
    } 
    $workspace | Set-Content $workspaceFile -Force

    Push-Location $folder

    #get input (curl is easier as invoke-webrequest)
    $uri = "https://adventofcode.com/$year/day/$day/input"
    curl $uri --cookie "session=$env:aoc_session" -o "src/input.txt"

    # modify cargo.toml
    $toml = switch -Regex -File ".\cargo.toml" {
        '^\s*name=' { 
            'name="{0}"' -f $folder
        }
        Default { $_ }
    } 
    $toml | Set-Content ".\cargo.toml" -Force

    Pop-Location

    git add .
}

#open in editor
code . ".\$folder\src\lib.rs"

# start watching (better in code terminal?)
#cargo watch -x "test -p $folder --release -- --nocapture"