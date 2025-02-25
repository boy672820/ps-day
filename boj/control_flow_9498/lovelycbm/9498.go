package main

import "fmt"

func main() {
    var a int
    fmt.Scanf("%d", &a)
    if a>= 90{
        fmt.Println("A")
    } else if a>= 80{
        fmt.Println("B")
    } else if a>= 70{
        fmt.Println("C")
    } else if a>= 60{
        fmt.Println("D")
    } else {
        fmt.Println("F")
    }   
}
