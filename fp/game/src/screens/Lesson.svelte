<script lang="ts">
  import { onMount, onDestroy, tick } from 'svelte';
  import Markdown from '../components/Markdown.svelte';
  import CodeEditor from '../components/CodeEditor.svelte';
  import { db } from '../state/db';
  import { runCode, validatePatterns } from '../engine/runner';
  import { LiveFactory } from '../engine/factory';
  import type { Book } from '../types';

  let { book, onBack } = $props<{ book: Book, onBack: () => void }>();

  let pageIndex = $state(0);
  let userCode = $state('');
  let output = $state<{ text: string, type: 'normal'|'error'|'success' }[]>([]);
  let isRunning = $state(false);
  let showSuccess = $state(false);
  
  let canvasWrapper: HTMLDivElement | undefined = $state();
  let factory: LiveFactory | null = null;
  
  let currentPage = $derived(book.pages[pageIndex]);
  let hasEditable = $derived(currentPage?.blocks.some(b => b.type === 'code-editable'));

  onMount(async () => {
    // Resume from last page
    const prog = await db.progress.get(book.id);
    if (prog && prog.currentPage) {
      pageIndex = prog.currentPage;
    }
    
    // Init LiveFactory
    if (canvasWrapper) {
      factory = new LiveFactory(canvasWrapper);
    }
  });

  onDestroy(() => {
    if (factory) factory.destroy();
  });

  // Load code when page changes
  $effect(() => {
    if (!currentPage) return;
    
    output = [];
    showSuccess = false;

    db.code.get(currentPage.id).then(saved => {
      if (saved) {
        userCode = saved.code;
      } else {
        const codeBlock = currentPage.blocks.find(b => b.type === 'code-editable') as any;
        userCode = codeBlock ? codeBlock.starterCode : '';
      }
    });

    db.progress.put({
      bookId: book.id,
      currentPage: pageIndex,
      completedPages: [],
      stars: 0,
      lastAccessed: new Date()
    }).catch(console.error);
  });

  async function handleRun() {
    if (!currentPage || !book || isRunning) return;
    isRunning = true;
    output = [{ text: 'Đang chạy code...', type: 'normal' }];

    await db.code.put({
      pageId: currentPage.id,
      code: userCode,
      savedAt: new Date()
    });

    if (currentPage.validation) {
      const val = validatePatterns(userCode, currentPage.validation.requiredPatterns, currentPage.validation.forbiddenPatterns);
      if (!val.passed) {
        output = [{ text: val.message, type: 'error' }];
        factory?.playError();
        isRunning = false;
        return;
      }
    }

    const testCode = currentPage.validation?.testCode;
    const res = await runCode(book.language, userCode, testCode);

    isRunning = false;

    if (res.success) {
      output = [
        { text: res.output || 'Chạy thành công (Không có output)', type: 'normal' },
        { text: '✅ Bài giải chính xác!', type: 'success' }
      ];
      factory?.playSuccess();
      
      // TRIGGER ANIMATION WITH OUTPUT
      if (res.output) {
         // Naive extraction for demo: look for numbers in output like "add_one(5) = 6"
         const match = res.output.match(/\d+/g);
         if (match && match.length >= 2) {
             const machineMatch1 = userCode.match(/def (\w+)/);
             const machineMatch2 = userCode.match(/(\w+)\s*=\s*lambda/);
             const machineName = machineMatch1 ? machineMatch1[1] : (machineMatch2 ? machineMatch2[1] : 'function');
             factory?.animateProcess(machineName, match[0], match[1]);
         }
      }

      showSuccess = true;
      
      db.progress.get(book.id).then(p => {
        if (p && !p.completedPages.includes(currentPage.id)) {
          p.completedPages.push(currentPage.id);
          db.progress.put(p);
        }
      });
    } else {
      output = [
        { text: res.output, type: 'normal' },
        { text: res.error, type: 'error' }
      ];
      factory?.playError();
    }
  }
</script>

{#if !book || !currentPage}
  <div class="loading-screen">Loading...</div>
{:else}
  <div class="lesson-screen">
    <!-- TOPBAR -->
    <div class="lesson-topbar">
      <button class="topbar-back-btn" onclick={onBack}>
        ← Thư viện
      </button>
      <div class="topbar-title">
        {book.title} <span class="topbar-page-indicator">({pageIndex + 1}/{book.totalPages})</span>
      </div>
      {#if currentPage.hints?.length > 0}
        <button class="topbar-hint-btn">?</button>
      {/if}
    </div>

    <!-- MAIN SPLIT -->
    <div class="lesson-main">
      
      <!-- LEFT PANE -->
      <div class="instruction-pane">
        <h1 class="prose-block h2" style="margin-top: 0">{currentPage.title}</h1>
        
        {#each currentPage.blocks as b, i}
          {#if b.type === 'prose'}
            <div class="prose-block">
              <Markdown content={b.content} />
            </div>
          {:else if b.type === 'code-readonly'}
            <div class="code-readonly-block">
              <div class="code-header">
                <span class="code-lang-badge {b.language}">{b.language}</span>
              </div>
              <div style="height: 150px">
                <CodeEditor value={b.code} language={b.language} readonly={true} />
              </div>
            </div>
          {:else if b.type === 'callout'}
            <div class="callout-block {b.variant}">
              <div class="callout-icon">💡</div>
              <div class="callout-content">{b.content}</div>
            </div>
          {/if}
        {/each}

        <!-- PAGE NAVIGATION -->
        <div class="page-navigation">
          <button 
            class="nav-btn" 
            disabled={pageIndex === 0}
            onclick={() => pageIndex -= 1}
          >
            Trang trước
          </button>
          <div class="page-dots">
            {#each book.pages as p, i}
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <div 
                class="page-dot {i === pageIndex ? 'current' : ''}"
                onclick={() => pageIndex = i}
              ></div>
            {/each}
          </div>
          <button 
            class="nav-btn {!hasEditable ? 'primary' : ''}"
            disabled={pageIndex === book.totalPages - 1}
            onclick={() => pageIndex += 1}
          >
            Trang sau
          </button>
        </div>
      </div>

      <!-- RIGHT PANE -->
      <div class="live-view-pane">
        <!-- Live View -->
        <div bind:this={canvasWrapper} class="live-view-canvas-wrapper" style="flex: 0.4; position: relative">
          <div class="live-view-label">Data Factory</div>
        </div>

        <!-- Code Editor -->
        <div style="flex: 0.6; display: flex; flex-direction: column">
          <div class="code-editable-block" style="margin: 0; flex: 1; border-radius: 0; border: none; border-top: 1px solid var(--border)">
            <div class="code-editable-label">{hasEditable ? 'Code của bạn' : 'Scratchpad (Nháp)'}</div>
            <CodeEditor 
              value={userCode} 
              language={book.language} 
              onchange={(val) => userCode = val} 
            />
          </div>
          
          <!-- Output Panel -->
          <div class="output-panel {output.some(o => o.type === 'error') ? 'has-error' : output.some(o => o.type === 'success') ? 'has-success' : ''}">
            {#if output.length === 0}
              <span style="color: #666">Chưa có kết quả chạy code...</span>
            {/if}
            {#each output as o}
              <div class="output-line {o.type}">
                {#each o.text.split('\n') as line}
                  <div>{line}</div>
                {/each}
              </div>
            {/each}
          </div>

          <!-- Suggestion Bar -->
          <div class="suggestion-bar">
            {#if currentPage.suggestions}
              {#each currentPage.suggestions as s}
                <button 
                  class="suggestion-chip"
                  onclick={() => userCode += s}
                >
                  {s}
                </button>
              {/each}
            {/if}
            <div class="suggestion-spacer"></div>
            <button 
              class="run-btn" 
              onclick={handleRun}
              disabled={isRunning}
            >
              {isRunning ? 'Đang chạy...' : '▶ Chạy Code'}
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- SUCCESS OVERLAY -->
    {#if showSuccess}
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div class="success-overlay" onclick={() => showSuccess = false}>
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <div class="success-card" onclick={e => e.stopPropagation()}>
          <div class="success-emoji">🎉</div>
          <div class="success-title">Tuyệt vời!</div>
          <div class="success-subtitle">Bạn đã hoàn thành xuất sắc bài tập này.</div>
          
          <button 
            class="success-next-btn"
            onclick={() => {
              showSuccess = false;
              if (pageIndex < book.totalPages - 1) pageIndex += 1;
              else onBack(); 
            }}
          >
            Tiếp tục ➔
          </button>
        </div>
      </div>
    {/if}
  </div>
{/if}
