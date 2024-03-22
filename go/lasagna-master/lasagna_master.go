package lasagna

func PreparationTime(layers []string, avgPrepTime int) int {
	if avgPrepTime == 0 {
		avgPrepTime = 2
	}
	return avgPrepTime * len(layers)
}

func Quantities(layers []string) (noodles int, sauce float64) {
	noodles = 0
	sauce = 0
	for i := 0; i < len(layers); i++ {
		switch layers[i] {
		case "noodles":
			noodles += 50
		case "sauce":
			sauce += 0.2
		}
	}
	return
}

func AddSecretIngredient(friendsList, myList []string) {
	myList[len(myList)-1] = friendsList[len(friendsList)-1]
}

func ScaleRecipe(amountNeeded []float64, numPortions int) (scaledAmounts []float64) {
  for i := 0; i < len(amountNeeded); i++ {
    scaledAmounts = append(scaledAmounts, (float64(numPortions) / 2) * amountNeeded[i])
  }
  return
}
