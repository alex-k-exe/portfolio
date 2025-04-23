/** Map that requires both keys and values to be unique
User can find a value with a key, but also find a key with a value */
export class BidirectionalMap<K, V> {
  private forwardMap: Map<K, V>;
  private reverseMap: Map<V, K>;

  constructor(iterable?: Iterable<[K, V]>) {
    this.forwardMap = new Map<K, V>();
    this.reverseMap = new Map<V, K>();

    if (!iterable) return;
    for (const [key, value] of iterable) {
      this.set(key, value);
    }
  }

  /** Add a key-value pair */
  set(key: K, value: V) {
    if (this.forwardMap.has(key) || this.reverseMap.has(value)) {
      throw new Error("Key or Value already exists");
    }
    this.forwardMap.set(key, value);
    this.reverseMap.set(value, key);
  }

  /** Get value by key */
  getValue(key: K) {
    return this.forwardMap.get(key);
  }

  /** Get key by value */
  getKey(value: V): K | undefined {
    return this.reverseMap.get(value);
  }

  /** Delete a key-value pair by key */
  deleteWithKey(key: K) {
    const value = this.forwardMap.get(key);
    if (value !== undefined) {
      this.forwardMap.delete(key);
      this.reverseMap.delete(value);
    }
  }

  /** Delete a key-value pair by value */
  deleteWithValue(value: V) {
    const key = this.reverseMap.get(value);
    if (key !== undefined) {
      this.reverseMap.delete(value);
      this.forwardMap.delete(key);
    }
  }

  /** Check if a key exists */
  hasKey(key: K) {
    return this.forwardMap.has(key);
  }

  /** Check if a value exists */
  hasValue(value: V) {
    return this.reverseMap.has(value);
  }

  /** Get the size of the map */
  get size() {
    return this.forwardMap.size;
  }

  /** Get entries of the map */
  entries() {
    return this.forwardMap.entries();
  }
}

export const PROJECTS = new BidirectionalMap([
  ["Portfolio", "portfolio"],
  ["PT App", "pt-app"],
]);
export const SKETCHES = new BidirectionalMap([
  ["Three squares", "three_squares"],
  ["Epicyclogons", "epicyclogons"],
  ["Game of Life", "game_of_life"],
]);

export const SITE_TITLE = "Alex Kammin's portfolio";
export const SITE_DESCRIPTION =
  "See my projects, social media profiles, and an about me";

// export const PROJECTS: BiMap<string, string> = BiMap.from({
//   Portfolio: "portfolio",
//   "PT App": "pt-app",
// });
// export const SKETCHES: BiMap<string, string> = BiMap.from({
//   "Three squares": "three_squares",
//   Epicyclogons: "epicyclogons",
//   "Game of Life": "game_of_life",
// });
