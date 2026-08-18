<script lang="ts">
  import { onMount } from 'svelte';
  import { loadManifest, loadBook } from './state/db';
  import Lesson from './screens/Lesson.svelte';
  import type { Book } from './types';
  import './App.css';

  let currentBook: string | null = $state(null);
  let isLoaded = $state(false);
  let bookData: Book | null = $state(null);
  let books: any[] = $state([]);
  let selectedLanguage = $state('python');

  let filteredBooks = $derived(books.filter(b => b.language === selectedLanguage));

  onMount(async () => {
    try {
      const data = await loadManifest();
      books = data.books;
      isLoaded = true;
    } catch (e) {
      console.error(e);
    }
  });

  async function selectBook(id: string) {
    try {
      // Because manifest only has metadata, we must load the full book JSON
      const fullBook = await loadBook(id);
      bookData = fullBook;
      currentBook = id;
    } catch (e) {
      console.error("Failed to load book:", e);
    }
  }
</script>

{#if !isLoaded}
  <div>Đang tải dữ liệu...</div>
{:else if currentBook && bookData}
  <Lesson book={bookData} onBack={() => currentBook = null} />
{:else}
  <div class="menu-container">
    <h1>Code Odyssey - Svelte Edition</h1>
    <div class="nav-tabs-container">
      <button 
        class="nav-tab-button {selectedLanguage === 'python' ? 'active' : ''}" 
        onclick={() => selectedLanguage = 'python'}>
        Python
      </button>
      <button 
        class="nav-tab-button {selectedLanguage === 'typescript' ? 'active' : ''}" 
        onclick={() => selectedLanguage = 'typescript'}>
        TypeScript
      </button>
      <button 
        class="nav-tab-button {selectedLanguage === 'rust' ? 'active' : ''}" 
        onclick={() => selectedLanguage = 'rust'}>
        Rust
      </button>
    </div>

    <div class="book-grid">
      {#each filteredBooks as book}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="book-card" style="--card-color: {book.coverColor || '#ccc'}" onclick={() => selectBook(book.id)}>
          <div class="book-emoji">{book.coverEmoji || '📘'}</div>
          <h3>{book.title}</h3>
          <p>{book.description}</p>
        </div>
      {/each}
    </div>
  </div>
{/if}
