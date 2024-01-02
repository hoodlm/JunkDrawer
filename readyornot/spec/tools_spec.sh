#shellcheck shell=sh
Describe 'Tools installed and on PATH'
  Describe 'terminal/shell/tui tools:'
    Parameters
      shellspec
      shellcheck
      kitty
      fish
      gron
      jq
      ssh
      sftp
      vim
      git
    End

    It "$1"
      When call command -v $1
      The status should be success
      The stdout should include $1
    End
  End

  Describe 'Rust:'
    Parameters
      rustc
      rustup
      rustfmt
      cargo
    End

    It "$1"
      When call command -v $1
      The status should be success
      The stdout should include $1
    End
  End

  Describe 'Ruby:'
    It "rvm"
      When call command -v rvm
      The status should be success
      The stdout should include rvm
    End
  End

  Describe 'Other:'
    Parameters
      yt-dlp
      obs
      firefox
      neofetch
      gimp
      texworks
      smartctl
    End

    It "$1"
      When call command -v $1
      The status should be success
      The stdout should include $1
    End
  End
End
