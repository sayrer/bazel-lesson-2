package basic

import "testing"

func TestBasic(t *testing.T) {
	expected := "Hello from Go"
	s := GetString()
	if s != expected {
		t.Errorf("Expected: " + expected, s)
	}
}