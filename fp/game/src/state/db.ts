import Dexie, { type Table } from 'dexie';
import type { BookProgress, PageCode, Manifest, Book } from '../types';

export class CodeOdysseyDB extends Dexie {
  progress!: Table<BookProgress, string>;
  code!: Table<PageCode, string>; // pageId là primary key

  constructor() {
    super('CodeOdysseyDB');
    this.version(1).stores({
      progress: 'bookId, lastAccessed',
      code: 'pageId, savedAt'
    });
  }
}

export const db = new CodeOdysseyDB();

// Cache manifest
export let manifest: Manifest | null = null;

export async function loadManifest() {
  if (manifest) return manifest;
  const res = await fetch('/src/data/manifest.json');
  manifest = await res.json();
  return manifest;
}

export async function loadBook(bookId: string): Promise<Book> {
  const res = await fetch(`/src/data/books/${bookId}.json`);
  return res.json();
}

// Gọi load manifest lúc khởi động
loadManifest().catch(console.error);
