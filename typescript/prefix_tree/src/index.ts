type TrieNode = {
  children: Map<string, TrieNode>,
  isLeaf: boolean;
}

export class Trie {
  private root: TrieNode;

  constructor() {
    this.root = { children: new Map(), isLeaf: false };
  }

  insert(word: string): void {
    let curr_node = this.root;
    for (const char of word) {
      if (!curr_node.children.has(char)) {
        const newNode = { children: new Map(), isLeaf: false};
        curr_node.children.set(char, newNode);
        curr_node = newNode;
      } else {
        curr_node = curr_node.children.get(char)!;
      }
    }
    curr_node.isLeaf = true;
  }

  search(word: string): boolean {
    let curr_node = this.root;
    for (const char of word) {
      if (curr_node.children.has(char)) {
        curr_node = curr_node.children.get(char)!;
      } else {
        return false;
      }
    }
    return curr_node.isLeaf;
  }

  startsWith(prefix: string): boolean {
    let curr_node = this.root;
    for (const char of prefix) {
      if (!curr_node.children.has(char)) {
        return false
      }
      curr_node = curr_node.children.get(char)!;
    }
    return true;
  }
}
