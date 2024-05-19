loghunter ninejalapenos@proton.me

# COLLECTION
collect all lines from all files matching file_name recursively in the scan_directory
pair each line with it's parent directory and write as a row in csv output

`cargo run scan_directory file_name_to_hunt output_file`

## examples using data directory with collected folder for output
```
cargo run ~/data .bash_history collected/bash_histories.csv
cargo run ~/data access.log collected/access_logs.csv
cargo run ~/data .zsh_history collected/zsh_histories.csv
cargo run ~/data .python_history collected/python_histories.csvcargo run ~/data/202405 known_hosts collected/known_hosts.csv
cargo run ~/data .mysql_history collected/mysql_histories.csv
cargo run ~/data authorized_keys collected/authorized_keys.csv
```

# ANALYSIS
searches all logs for keyword and reports directoryparent (ip) and matching lines
`bash lsearch.sh <keyword>`

## example searching logs for netcat usage

`bash ~/collector/collected/lsearch.sh netcat`
