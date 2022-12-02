PARAM (
    [int] $day = ((Get-Date).Day),
    [int] $year = ((Get-Date).Year),
    [string] $template = (".\template"),
    [string] $workspaceFile = (".\cargo.toml")
)
$folder = "aoc_{0:0000}_{1:00}" -f $year, $day;

if (-not (Test-Path $folder)) {
    "Create $folder"
    
    # copy template folder
    Copy-Item -Recurse -path $template -dest $folder

    # add folder to rust workspace
    $workspace = switch -Regex -File $workspaceFile {
        '^\s*]$' { 
            '"{0}",' -f $folder
            $_
        }
        Default { $_ }
    } 
    $workspace | Set-Content $workspaceFile -Force

    Push-Location $folder

    # download input (curl is easier as invoke-webrequest)
    # expect session code in environment aoc_session
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

    # modify cargo.toml
    $toml = switch -Regex -File ".\cargo.toml" {
        '^\s*name=' { 
            'name="{0}"' -f $folder
        }
        Default { $_ }
    } 
    $toml | Set-Content ".\cargo.toml" -Force

    # modify lib.rs to have better names for test
    $lib = switch -Regex -File ".\src\lib.rs" {
        '^\s*fn (part|aoc_\d+_\d+)_(.+)\(\) {' { 
            'fn {0}{1}() {{' -f $folder, $Matches.2
        }
        Default { $_ }
    } 
    $lib | Set-Content ".\src\lib.rs" -Force


    Pop-Location

    git add .
}

#open in editor
code . ".\$folder\src\lib.rs"

# start watching (better in code terminal?)
#cargo watch -x "test -p $folder --release -- --nocapture"