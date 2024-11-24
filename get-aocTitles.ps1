# USE WITH CARE! Webscraping for titles and stars. 
# Will create a json file with the scraped data to never download this again

# if the current year is already online include it
$endYear = if ( (get-date).Month -gt 11 ) { (get-date).year } else { (get-date).year - 1 };
$years = 2015 .. $endYear;

$aocData =
foreach ($year in $years) {
    for ($day = 1; $day -lt 26; $day++) {
        $uri = "https://adventofcode.com/$year/day/$day";
        $h = curl $uri --cookie "session=$env:aoc_session"; #$h will be array 

        $title = if ( "$h" -match '<article class="day-desc"><h2>--- Day (\d+): ([^<]+) ---</h2>' ) { 
            $matches[2] 
        }
        else { "AoC $year $day" };

        $stars = if ( "$h" -match '<p class="day-success">[^<*]+(\*?\*?)</p>' ) { 
            $matches[1].length 
        }
        else { 0 }
            
        $data = [PSCustomObject]@{
            year  = $year;
            day   = $day;
            title = $title;
            a     = if ( $stars -ge 1 ) { "*" } else { "" };
            b     = if ( $stars -eq 2 ) { "*" } else { "" };
        };
        $data
    }
}


ConvertTo-Json -InputObject $aocData | Set-Content -Force "$PSScriptRoot\aoc_data.json" 