
async function main() {
  await new Promise((resolve, reject) =>
    setTimeout(() => {
      console.log('First timer');
      resolve(null)
    }, 3000));

  await new Promise((resolve, reject) =>
    setTimeout(() => {
      console.log('Second timer');
      resolve(null)
    }, 2000));

  await new Promise((resolve, reject) =>
    setTimeout(() => {
      console.log('Third timer');
      resolve(null)
    }, 1000));

  await new Promise((resolve, reject) =>
    setTimeout(() => {
      console.log('Fourth timer');
      resolve(null)
    }, 0));
}

main()
