
// https://leetcode.com/problems/letter-combinations-of-a-phone-number

export function letter_combinations_of_a_phone_number(digits: string): string[] {
  if (digits.length === 0) return [];

  const lettersMap = new Map();
  lettersMap.set("2", ["a", "b", "c"]);
  lettersMap.set("3", ["d", "e", "f"]);
  lettersMap.set("4", ["g", "h", "i"]);
  lettersMap.set("5", ["j", "k", "l"]);
  lettersMap.set("6", ["m", "n", "o"]);
  lettersMap.set("7", ["p", "q", "r", "s"]);
  lettersMap.set("8", ["t", "u", "v"]);
  lettersMap.set("9", ["w", "x", "y", "z"]);

  const result: string[] = [];

  function combine(combination: string, remDigits: string) {
    if (remDigits.length === 0) {
      result.push(combination);
    } else {
      const letters = lettersMap.get(remDigits[0]);
      console.log(letters)
      for (const letter of letters) {
        combine(combination + letter, remDigits.slice(1));
      }
    }
  }

  combine("", digits);

  return result;
}
