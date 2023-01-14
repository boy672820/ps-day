package main

import "fmt"

func main() {
	var a,b,c,d,e,f int
    var g,h,i,j,k,l int = 1,1,2,2,2,8
	
    fmt.Scanf("%d %d %d %d %d %d", &a,&b,&c,&d,&e,&f)
	fmt.Printf("%d %d %d %d %d %d \n", g-a,h-b,i-c,j-d,k-e,l-f)    
}
