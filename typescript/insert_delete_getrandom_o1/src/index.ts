class RandomizedSet {
  private hm

  constructor() {
    this.hm = new Set()
  }

  insert(val: number): boolean {
    if (this.hm.has(val)) return false
    this.hm.add(val)
    return true
  }

  remove(val: number): boolean {
    if (this.hm.has(val)) {
      this.hm.delete(val)
      return true
    }
    return false

  }

  getRandom(): number {
    const idx = Math.floor(this.hm.size * Math.random())
    let i = 0
    for (let val of this.hm.values()) {
      if (i === idx) return val
      i++
    }
    return 0
  }
}
