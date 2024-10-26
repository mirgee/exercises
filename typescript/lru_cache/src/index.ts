type CacheNode<T = number, U = number> = {
  prev?: CacheNode<T>,
  next?: CacheNode<T>,
  value: T,
  key: U
}

export class LRUCache {
  private capacity: number;
  private length: number;
  private map: Map<number, CacheNode>;

  private head?: CacheNode;
  private tail?: CacheNode;

  constructor(capacity: number) {
    this.capacity = capacity;
    this.length = 0;
    this.map = new Map();
  }

  get(key: number): number {
    const node = this.map.get(key);
    if (!node) {
      return -1;
    }
    this.detach(node);
    this.prepend(node);
    return node.value;
  }

  put(key: number, value: number): void {
    const node = this.map.get(key);
    if (!node) {
      const newNode = { value, key } as CacheNode;
      this.length += 1;
      this.prepend(newNode);
      this.trim();
      this.map.set(key, newNode);
    } else {
      this.detach(node);
      this.prepend(node);
      node.value = value;
    }
  }

  private prepend(node: CacheNode) {
    if (!this.head) {
      this.head = node;
      this.tail = node;
    } else {
      node.next = this.head;
      this.head.prev = node;
      this.head = node;
    }
  }

  private detach(node: CacheNode) {
    if (node.prev) {
      node.prev.next = node.next;
    }
    if (node.next) {
      node.next.prev = node.prev;
    }
    if (node === this.head) {
      this.head = node.next;
    }
    if (node === this.tail) {
      this.tail = node.prev;
    }
    node.next = undefined;
    node.prev = undefined;
  }

  private trim() {
    if (this.length > this.capacity && this.tail) {
      this.map.delete(this.tail.key)
      this.detach(this.tail);
      this.length -= 1;
    }
  }
}
