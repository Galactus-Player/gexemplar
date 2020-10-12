package openapi

import (
	"testing"
)

// TestCountAPIFirst tests the first call of the count API
func TestCountAPIFirst(t *testing.T) {
	defaultService := NewDefaultApiService()
	ct, err := defaultService.CountGet()
	count := ct.(int)
	if err != nil {
		t.Errorf("Count API returned an error when there should have been no errors: %s", err)
	}
	if count != 1 {
		t.Errorf("The first count should always be 1, it was not %d", ct)
	}
}

// TestDogAPI tests the dog API
func TestDogAPI(t *testing.T) {
	defaultService := NewDefaultApiService()
	wf, err := defaultService.DogGet()
	woof := wf.(string)
	if err != nil {
		t.Errorf("Dog API returned an error when there should have been no errors: %s", err)
	}
	if woof != "woof" {
		t.Errorf("The first dog should always say \"woof\", it was %s", woof)
	}
}
