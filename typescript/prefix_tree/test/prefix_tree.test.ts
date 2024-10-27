import { Trie } from '../src'

test('t0', () => {
  const trie = new Trie();
  trie.insert("apple");
  expect(trie.search("apple")).toBeTruthy();   // return True
  expect(trie.search("app")).toBeFalsy();     // return False
  expect(trie.startsWith("app")).toBeTruthy(); // return True
  trie.insert("app");
  expect(trie.search("app")).toBeTruthy();     // return True
})
