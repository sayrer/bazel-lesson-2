package nonbazel

import (
    "fmt"
    "rsc.io/quote"
)

func getQuote() string {
    return fmt.Sprintf("A Go library string: \"%s\"", quote.Hello())
}