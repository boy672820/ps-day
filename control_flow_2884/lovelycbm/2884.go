package main

import "fmt"

func main() {
    var H,M int
    fmt.Scanf("%d %d", &H, &M)
    
    if M > 44 {
        M-=45
    } else if M <45 && H>0{
        H--
        M+=15
    } else {
        H=23
        M+=15
    }

    fmt.Println(H, M)

}
