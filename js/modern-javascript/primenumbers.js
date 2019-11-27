"use strict"

let n = prompt('Select n');

outer: for (let x = 2; x <= n; x++) {
  for (let d = 2; d < x; d++) {
    if (x % d == 0) continue outer; 
  }
  alert(x)
}
