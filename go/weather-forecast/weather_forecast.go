// Package weather provides weather forecast functionality.
package weather

// CurrentCondition current weather condition.
var CurrentCondition string
// CurrentLocation current location.
var CurrentLocation string

// Forecast returns weather forecast for given city and condition.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
