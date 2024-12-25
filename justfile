# use PowerShell instead of sh, but do not load the profile
set shell := ["powershell.exe", "-noprofile", "-c"]

# Use `just work day-01 a` to work on the specific binary for a specific day's problems
work year_day part:
    cargo watch -w aoc_{{year_day}} -x "check -p aoc_{{year_day}}" -s "just test {{year_day}} {{part}}" -s "just lint {{year_day}}" -s "just bench {{year_day}} {{part}}" 
# # www-watch:
# #    RUST_LOG=info cargo +nightly leptos watch --project www
# www-build:
#    cargo +nightly leptos build --project www --release
lint year_day:
    cargo clippy --lib -p aoc_{{year_day}}
fix year_day:
    cargo clippy --fix --lib -p aoc_{{year_day}}
test year_day part:
    cargo nextest run -p aoc_{{year_day}} aoc_{{year_day}}_{{part}}
bench-all:
    cargo bench -q > benchmarks.txt
bench year_day:
    cargo bench --bench aoc_{{year_day}}_bench >> aoc_{{year_day}}.bench.txt
flamegraph year_day part:
    # needs DTrace on Windows: https://github.com/microsoft/DTrace-on-Windows
    cargo flamegraph --profile flamegraph --root --package aoc_{{year_day}} --bin aoc_{{year_day}}_{{part}}  -o flamegraphs/aoc_{{year_day}}_{{part}}.svg
dhat year_day part:
    cargo run --profile dhat --features dhat-heap --package aoc_{{year_day}} --bin {{part}}
# create the directory for a new day's puzzle and fetch the inputaoc_{{year_day}}
# create day:
#    cargo generate --path ./daily-template --name {{day}}
#    just get-input {{day}}

# You can find SESSION by using Chrome tools:
# 1) Go to https://adventofcode.com/2022/day/1/input
# 2) right-click -> inspect -> click the "Application" tab.
# 3) Refresh
# 5) Click https://adventofcode.com under "Cookies"
# 6) Grab the value for session. Fill it into your .env file
# 
# example .env:
#
# ```
# SESSION=PASTE_COOKIE_VALUE_HERE
# ```
#
# get the input for a day's puzzle
# get-input day:
#     ./scripts/get-aoc-input.rs --day {{day}} --current-working-directory {{justfile_directory()}}