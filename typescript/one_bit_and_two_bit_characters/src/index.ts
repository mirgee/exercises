// https://leetcode.com/problems/1-bit-and-2-bit-characters/description/

export function one_bit_and_two_bit_characters(bits: number[]) {
  let current_is_one_bit = false;
  for (let i = 0; i < bits.length; i++) {
    if (bits[i] === 1) {
      current_is_one_bit = false;
      i += 1;
    } else if (bits[i] === 0) {
      current_is_one_bit = true;
    } else {
      throw Error()
    }
  }
  return current_is_one_bit;
}
