#shellcheck shell=bash

Describe 'handle_args'
    It "Prints a help page"
        When call ./hn_offline.sh --help
        The status should be successful
        The first line of stdout should start with "USAGE:"
        The second line of stdout should start with "OPTIONS:"
        The output should include "--date"
    End

    It "Rejects malformed dates"
        When call ./hn_offline.sh --date "1/9/2024"
        The status should be failure
        The output should include "Date must match YYYY-MM-DD format"
    End
End
