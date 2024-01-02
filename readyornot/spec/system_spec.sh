#shellcheck shell=sh

value() {
  test "${value:?}" "$1" "$2"
}

Describe 'Disks'
  Describe 'Space'
    Parameters
      $HOME
      /
    End

    It "has at least 25G free on the volume containing $1"
      When call df --output=avail $1
      The stdout lines should eq 2
      The second line of stdout should satisfy value -gt 2500000
    End
  End
End

Describe 'CPU'
  It "has at least 4 cores"
    When call nproc
    The stdout should satisfy value -gt 4
  End
End
