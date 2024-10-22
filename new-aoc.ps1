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

    # changed to wildcard members in workspace file, so no need to change it

    Push-Location $folder

    # download input (only once) (curl is easier than invoke-webrequest)
    # expect session code in environment variable aoc_session
    $uri = "https://adventofcode.com/$year/day/$day/input"
    curl $uri --cookie "session=$env:aoc_session" -o "src/input.txt" -A "gbegerow@gmail.com via curl"

    if ((Get-Content "src/input.txt") -eq "Puzzle inputs differ by user.  Please log in to get your puzzle input." ) {
        "Renew session code"
    }

    # modify cargo.toml
    $toml = switch -Regex -File ".\cargo.toml" {
        '^\s*name\s*=\s*' { 
            'name = "{0}"' -f $folder
        }
        Default { $_ }
    } 
    $toml | Set-Content ".\cargo.toml" -Force

    # modify lib.rs to have better names for test (maybe go the full way and simply replace all occurences?)
    $lib = switch -Regex -File ".\src\lib.rs" {
        '^(.*)fn (part|aoc_\d+_\d+)_(.+)$' { 
            '{2}fn {0}_{1}' -f $folder, $Matches.3, $Matches.1
        }
        
        '^(.*)assert_eq!\(super::(part|aoc_\d+_\d+)_(.+)$' {
            '{2}assert_eq!(super::{0}_{1}' -f $folder, $Matches.3, $Matches.1
        }
        
        '^(.*)(https://adventofcode.com/\d+/day/\d+)(.*)$' {
            '{2}https://adventofcode.com/{0}/day/{1:00}{3}' -f $year, $day, $Matches.1, $Matches.3
        }
     
        Default { $_ }
    } 
    $lib | Set-Content ".\src\lib.rs" -Force

    Pop-Location

    git add .
}

#open in editor
code . ".\$folder\src\lib.rs"

# start watching (better in code terminal?) (some conflicts with vscode )
#cargo watch -x "test -p $folder --release -- --nocapture"