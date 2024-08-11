const prom1 = new Promise((resolve, reject) =>
  setTimeout(() => {
    console.log('First timer');
    resolve(null)
  }, 3000));

const prom2 = new Promise((resolve, reject) =>
  setTimeout(() => {
    console.log('Second timer');
  }, 2000));

const prom3 = new Promise((resolve, reject) =>
  setTimeout(() => {
    console.log('Third timer');
  }, 1000));

const prom4 = new Promise((resolve, reject) =>
  setTimeout(() => {
    console.log('Fourth timer');
  }, 0));


async function main() {
  await prom1;
  await prom2;
  await prom3;
  await prom4;
}

main()
