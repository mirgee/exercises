import { LRUCache } from '../src'

test('t0', () => {
  const lRUCache = new LRUCache(2);
  expect(lRUCache.put(1, 1)).toBe(undefined); // cache is {1=1}
  expect(lRUCache.put(2, 2)).toBe(undefined); // cache is {1=1, 2=2}
  expect(lRUCache.get(1)).toBe(1);    // return 1
  expect(lRUCache.put(3, 3)).toBe(undefined); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
  expect(lRUCache.get(2)).toBe(-1);    // returns -1 (not found)
  expect(lRUCache.put(4, 4)).toBe(undefined); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
  expect(lRUCache.get(1)).toBe(-1);    // return -1 (not found)
  expect(lRUCache.get(3)).toBe(3);    // return 3
  expect(lRUCache.get(4)).toBe(4);    // return 4
})
