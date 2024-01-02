#shellcheck shell=sh
Describe 'Network health'
  Describe 'can ping:'
    Parameters:matrix
      4 6 
      "example.org" "cloudflare.com"
    End

    It "$2 via ipv$1"
      When call ping -$1 -c 1 $2
      The output should include "1 packets transmitted, 1 received, 0% packet loss"
    End
  End

  Describe 'https connectivity:'
    Parameters:matrix
      4 6
      "example.org"
    End
  
    It "$2 via ipv$1"
      When call curl -$1 --silent -I https://$2
      The output should include "HTTP/2 200"
    End
  End
End
