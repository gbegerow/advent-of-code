### Create a new solution folder for a certain year and day

PARAM (
    [int] $day = '25', # ((Get-Date).Day),
    [int] $year = '2016', #((Get-Date).Year),
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

    "modify local cargo.toml"
    $toml = switch -Regex -File ".\cargo.toml" {
        '^\s*name\s*=\s*"aoc_{year}_{day}_(.)"' { 
            'name = "aoc_{0}_{1}_{2}"' -f $year, $day, $Matches.1
        }

        '^\s*name = "advent-of-code"' { 
            'name = "{0}"' -f $folder
        }

        Default { $_ }
    } 
    $toml | Set-Content ".\cargo.toml" -Force

    # modify lib.rs to have better names for test (maybe go the full way and simply replace all occurences?)
    "modify lib.rs"
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

    # rename binaries if needed
    if (Test-Path bin) {
        Push-Location bin

        foreach ( $p in ("aoc_yyyy_dd_a.rs", "aoc_yyyy_dd_b.rs")) { 
            if (Test-Path $p) {
                $f = switch -Regex -File $p {
                    '^(.*)(aoc_\d+_\d+)(.*)$' { 
                        '{0}{1}{2}' -f $Matches.1, $folder, $Matches.3
                    }
                    
                    Default { $_ }
                } 
                $f | Set-Content $p -Force
                
                $newname = $p.Replace("yyyy", $year).Replace("dd", $day);
                Rename-Item $p $newname;        
            }
        }

        Pop-Location
    }

    if (Test-Path benches) {
        "modify benchmarks"
        $lib = switch -Regex -File ".\benches\benchmarks.rs" {
            '^(.*)(aoc_\d+_\d+)\b' { 
                '{0}' -f $folder
            }
            
            Default { $_ }
        } 
        $lib | Set-Content ".\benches\benchmark.rs" -Force
    }
    
    Pop-Location
}

# second pass actions
if (Test-Path $folder) {
    Push-Location $folder

    # download input (only once) (curl is easier than invoke-webrequest)
    # expect session code in environment variable aoc_session
    if (-not (Test-Path "src/input.txt")) {
        "download input"
        $uri = "https://adventofcode.com/$year/day/$day/input"
        curl $uri --cookie "session=$env:aoc_session" -o "src/input.txt" -A "gbegerow@gmail.com via curl"

        if ((Get-Content "src/input.txt") -eq "Puzzle inputs differ by user.  Please log in to get your puzzle input." ) {
            "Renew session code in env aoc_session"
        }
    }

    Pop-Location
}

# todo: get title from webpage, get stars for year
# todo: add title and stars to aoc_data.json
# todo: regen table at end of Readme from aoc_data.json

# todo: add term_viz
# todo: add viz

# do not: submit result, I want to get it first hand 


# all preparation done, add it to 
git add .

Pop-Location

#open in editor
code . ".\$folder\src\lib.rs"

# start watching (better in code terminal?) (some conflicts with vscode )
#cargo watch -x "test -p $folder --release -- --nocapture"