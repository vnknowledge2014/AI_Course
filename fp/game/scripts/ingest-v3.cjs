#!/usr/bin/env node
/**
 * Ingest Script v3
 * Parses Markdown chapters -> Book JSON files
 * 
 * Usage: node scripts/ingest-v3.js
 * Output: src/data/books/*.json + src/data/manifest.json
 */

const fs = require('fs');
const path = require('path');

// ========== CONFIG ==========
const CURRICULUM_ROOT = path.join(__dirname, '..', '..'); // /fp/
const OUT_DIR = path.join(__dirname, '..', 'src', 'data', 'books');
const MANIFEST_OUT = path.join(__dirname, '..', 'src', 'data', 'manifest.json');

const LANGUAGE_DIRS = {
  python: path.join(CURRICULUM_ROOT, 'Python_Books'),
  typescript: path.join(CURRICULUM_ROOT, 'TypeScript_Books'),
  rust: path.join(CURRICULUM_ROOT, 'Rust_Books'),
};

const WORLD_COLORS = {
  0: '#06b6d4',
  1: '#22c55e',
  2: '#a855f7',
  3: '#f97316',
  4: '#3b82f6',
  5: '#ef4444',
  6: '#eab308',
  7: '#ec4899',
};

const WORLD_EMOJIS = {
  0: '🧮', 1: '🔤', 2: '🔮', 3: '🏗️', 4: '🏛️', 5: '⚗️', 6: '🚀', 7: '🤖',
};

const PART_TO_WORLD = {
  'part_0': 0, 'part_0_cs': 0,
  'part_1': 1,
  'part_2': 2,
  'part_3': 3,
  'part_4': 4,
  'part_5': 5,
  'part_6': 6, 'part_7': 6,
  'part_8': 7,
};

const WORLD_NAMES = [
  'World 0: CS Foundations',
  'World 1: Fundamentals',
  'World 2: Thinking Functionally',
  'World 3: Design Patterns',
  'World 4: Domain-Driven Design',
  'World 5: FP Patterns',
  'World 6: Testing & Production',
  'World 7: Agentic AI',
];

// ========== HELPERS ==========

function getWorldIndex(filePath) {
  for (const [key, idx] of Object.entries(PART_TO_WORLD)) {
    if (filePath.includes(key)) return idx;
  }
  return 1;
}

function getLangPrefix(language) {
  return { python: 'py', typescript: 'ts', rust: 'rs' }[language];
}

function extractChapterNumber(filename) {
  const m = filename.match(/chapter_(\d+[a-z]?)/);
  return m ? m[1] : '00';
}

function makeBookId(language, filename) {
  const ch = extractChapterNumber(filename);
  return `${getLangPrefix(language)}-ch${ch}`;
}

function extractTitle(content) {
  const m = content.match(/^#\s+(.+)$/m);
  return m ? m[1].replace(/⭐/g, '').trim() : 'Untitled';
}

function extractDescription(content) {
  // Extract from blockquote "Bạn sẽ học được" first item
  const m = content.match(/>\s*-\s*(.+?)(?:\n|$)/);
  if (m) return m[1].replace(/`/g, '').trim().slice(0, 120);
  // Fallback: first prose paragraph
  const lines = content.split('\n').filter(l => l.trim() && !l.startsWith('#') && !l.startsWith('>') && !l.startsWith('```') && !l.startsWith('---'));
  return (lines[0] || '').slice(0, 120);
}

// ========== MARKDOWN PARSER ==========

/**
 * Split a chapter file into pages.
 * Page boundaries: ## headings + "## 🏋️ Bài tập" sections
 */
function parseChapterToPages(content, language, bookId) {
  const pages = [];

  // Split into sections by ## headings
  const sections = content.split(/^## /m).map((s, i) => i === 0 ? s : '## ' + s);

  let pageIndex = 0;

  for (const section of sections) {
    if (!section.trim()) continue;

    const isExerciseSection = section.match(/^## 🏋️|^## Bài tập|^## Exercise/m);

    if (isExerciseSection) {
      // Parse exercises into separate pages
      const exercisePages = parseExerciseSection(section, language, bookId, pageIndex);
      pages.push(...exercisePages);
      pageIndex += exercisePages.length;
    } else {
      // Parse regular section into a theory page
      const page = parseTheorySection(section, language, bookId, pageIndex);
      if (page) {
        pages.push(page);
        pageIndex++;
      }
    }
  }

  // Always add a summary page at the end if there's a checkpoint section
  const checkpointMatch = content.match(/## ✅ Checkpoint[\s\S]+?(?=\n---|\n## |$)/);
  if (checkpointMatch && !pages.some(p => p.title.includes('Checkpoint'))) {
    pages.push({
      id: `${bookId}-p${pageIndex}`,
      title: '✅ Checkpoint',
      blocks: [{ type: 'prose', content: markdownToSimple(checkpointMatch[0].replace('## ✅ Checkpoint', '').trim()) }],
      suggestions: [],
      hints: [],
      liveView: { scene: 'factory', onSuccess: 'cube-moves', onFailure: 'error-shake' },
    });
  }

  return pages;
}

function parseTheorySection(section, language, bookId, index) {
  const lines = section.split('\n');
  const titleLine = lines[0];
  const title = titleLine.replace(/^##\s*/, '').replace(/^\s*#\s*/, '').trim() || `Phần ${index + 1}`;

  // Skip if it's a summary/tóm tắt/tiếp theo section (no editable code)
  const blocks = [];
  let currentProse = [];
  let inCodeBlock = false;
  let currentCode = [];
  let currentCodeLang = language;
  let isEditable = false;

  const flushProse = () => {
    const text = currentProse.join('\n').trim();
    if (text) blocks.push({ type: 'prose', content: text });
    currentProse = [];
  };

  for (let i = 1; i < lines.length; i++) {
    const line = lines[i];

    if (line.startsWith('```')) {
      if (!inCodeBlock) {
        flushProse();
        inCodeBlock = true;
        currentCode = [];
        const langMatch = line.match(/```(\w+)/);
        currentCodeLang = langMatch ? langMatch[1] : language;
        isEditable = false; // Theory sections: always readonly
      } else {
        // End of code block
        const codeStr = currentCode.join('\n');
        blocks.push({
          type: 'code-readonly',
          code: codeStr,
          language: currentCodeLang,
        });
        inCodeBlock = false;
        currentCode = [];
      }
    } else if (inCodeBlock) {
      currentCode.push(line);
    } else if (line.match(/^<details>/)) {
      // Skip solution details in theory sections
    } else {
      currentProse.push(line);
    }
  }
  flushProse();

  if (blocks.length === 0) return null;

  return {
    id: `${bookId}-p${index}`,
    title,
    blocks,
    suggestions: [],
    hints: [],
    liveView: { scene: 'factory', onSuccess: 'cube-moves', onFailure: 'error-shake' },
  };
}

function parseExerciseSection(section, language, bookId, startIndex) {
  const pages = [];
  // Split by "**Bài N**:" pattern
  const exerciseBlocks = section.split(/\*\*Bài \d+\*\*:/g);

  exerciseBlocks.forEach((block, i) => {
    if (i === 0 || !block.trim()) return; // skip header

    const lines = block.split('\n');
    const instructionLines = [];
    let starterCode = '';
    let testCode = '';
    let hints = [];
    let inCode = false;
    let inDetails = false;
    let codeLang = language;

    for (const line of lines) {
      if (line.startsWith('<details>')) { inDetails = true; continue; }
      if (line.startsWith('</details>')) { inDetails = false; continue; }

      if (line.startsWith('```')) {
        if (!inCode) {
          inCode = true;
          const m = line.match(/```(\w+)/);
          codeLang = m ? m[1] : language;
        } else {
          inCode = false;
        }
        continue;
      }

      if (inDetails && inCode) {
        // This is the solution — make it the last hint
        hints.push(line);
      } else if (!inDetails && inCode) {
        // This is the starter code (first code block) or test code (has assert)
        if (line.includes('assert ') || line.includes('assert.')) {
          testCode += line + '\n';
        } else {
          starterCode += line + '\n';
        }
      } else if (!inDetails) {
        instructionLines.push(line);
      }
    }

    // Extract assert statements from starterCode if they're mixed in
    const starterLines = starterCode.split('\n');
    const cleanStarter = starterLines.filter(l => !l.trim().startsWith('assert')).join('\n').trim();
    const assertLines = starterLines.filter(l => l.trim().startsWith('assert')).join('\n');
    if (assertLines) testCode = assertLines + '\n' + testCode;

    // Generate suggestions from code
    const suggestions = generateSuggestions(cleanStarter + testCode, language);

    const pageHints = [
      ...extractInlineHints(instructionLines.join('\n')),
      hints.join('\n').trim(), // Full solution as last hint
    ].filter(Boolean);

    const validation = (testCode || starterCode.includes('assert')) ? {
      testCode: testCode.trim(),
      forbiddenPatterns: extractForbiddenPatterns(instructionLines.join('\n')),
      requiredPatterns: extractRequiredPatterns(instructionLines.join('\n')),
    } : undefined;

    pages.push({
      id: `${bookId}-p${startIndex + i - 1}`,
      title: `Bài Tập ${i}`,
      blocks: [
        { type: 'prose', content: instructionLines.join('\n').trim() },
        {
          type: 'code-editable',
          starterCode: cleanStarter || generateStarterCode(language),
          language: codeLang,
        },
      ],
      suggestions,
      validation,
      hints: pageHints,
      liveView: { scene: 'factory', onSuccess: 'cube-moves', onFailure: 'explosion' },
    });
  });

  return pages;
}

function generateSuggestions(code, language) {
  const suggestions = new Set();

  if (language === 'python') {
    if (code.includes('for') && code.includes('in')) suggestions.add('[x for x in');
    if (code.includes('def ')) {
      const m = code.match(/def (\w+)/);
      if (m) suggestions.add(`def ${m[1]}(`);
    }
    if (code.includes('return')) suggestions.add('return');
    if (code.includes('@dataclass')) suggestions.add('@dataclass(frozen=True)');
    if (code.includes('match')) suggestions.add('match ');
    if (code.includes('tuple')) suggestions.add('tuple[');
  } else if (language === 'typescript') {
    if (code.includes('=>')) suggestions.add('=> ');
    if (code.includes('const')) suggestions.add('const ');
    if (code.includes('readonly')) suggestions.add('readonly ');
    if (code.includes('match') || code.includes('switch')) suggestions.add('switch (');
    if (code.includes('type ')) suggestions.add('type ');
  } else if (language === 'rust') {
    if (code.includes('fn ')) {
      const m = code.match(/fn (\w+)/);
      if (m) suggestions.add(`fn ${m[1]}(`);
    }
    if (code.includes('match')) suggestions.add('match ');
    if (code.includes('impl')) suggestions.add('impl ');
    if (code.includes('->')) suggestions.add('-> ');
    if (code.includes('let mut')) suggestions.add('let mut ');
    suggestions.add('let ');
  }

  return [...suggestions].slice(0, 5);
}

function extractForbiddenPatterns(text) {
  const patterns = [];
  if (text.includes('không được dùng') && text.includes('.push(')) patterns.push('\\.push\\(');
  if (text.includes('không mutate') || text.includes('immutable')) {
    if (text.includes('Python')) patterns.push('\\[\\d\\]\\s*=');
  }
  if (text.includes('pure function') || text.includes('pure')) patterns.push('global\\s+\\w+');
  return patterns;
}

function extractRequiredPatterns(text) {
  const patterns = [];
  if (text.includes('comprehension')) patterns.push('for\\s+\\w+\\s+in');
  if (text.includes('match/case') || text.includes('pattern matching')) patterns.push('match\\s+');
  if (text.includes('frozen=True')) patterns.push('frozen=True');
  return patterns;
}

function extractInlineHints(text) {
  const hints = [];
  const m = text.match(/gợi ý[:\s]+(.+?)(?:\n|$)/gi);
  if (m) hints.push(...m.map(h => h.replace(/gợi ý[:\s]+/i, '').trim()));
  return hints.slice(0, 2);
}

function generateStarterCode(language) {
  if (language === 'python') return '# Viết code của bạn ở đây\ndef solution():\n    pass\n';
  if (language === 'typescript') return '// Viết code của bạn ở đây\nfunction solution() {\n  \n}\n';
  return '// Viết code của bạn ở đây\nfn solution() {\n    \n}\n';
}

function markdownToSimple(md) {
  return md
    .replace(/^>\s*/gm, '')
    .replace(/\*\*(.+?)\*\*/g, '**$1**')
    .trim();
}

// ========== MAIN PROCESSING ==========

function processLanguage(language, langDir) {
  if (!fs.existsSync(langDir)) {
    console.warn(`⚠️  Directory not found: ${langDir}`);
    return [];
  }

  const books = [];
  const files = getAllMdFiles(langDir);

  for (const filePath of files) {
    const filename = path.basename(filePath);
    // Chỉ nạp chương thật. Loại outline/preface/appendix/SKILL.md và mọi
    // file .md nằm trong thư mục phụ trợ (projects/, scripts/, ...).
    if (!/^chapter_\d+[a-z]?_/i.test(filename)) continue;
    if (filePath.includes('appendices')) continue;

    const content = fs.readFileSync(filePath, 'utf-8');
    const bookId = makeBookId(language, filename);
    const worldIndex = getWorldIndex(filePath);
    const title = extractTitle(content);
    const description = extractDescription(content);
    const chapterNum = extractChapterNumber(filename);

    const pages = parseChapterToPages(content, language, bookId);
    if (pages.length === 0) continue;

    const book = {
      id: bookId,
      language,
      world: WORLD_NAMES[worldIndex] || 'World 1: Fundamentals',
      worldIndex,
      chapter: `Chapter ${chapterNum}`,
      title,
      description,
      coverColor: WORLD_COLORS[worldIndex] || WORLD_COLORS[1],
      coverEmoji: WORLD_EMOJIS[worldIndex] || '📚',
      pages,
      prerequisites: [],
      totalPages: pages.length,
      sourceFile: path.relative(CURRICULUM_ROOT, filePath),
    };

    books.push(book);
  }

  return books;
}

function getAllMdFiles(dir, files = []) {
  if (!fs.existsSync(dir)) return files;
  const entries = fs.readdirSync(dir, { withFileTypes: true });
  for (const entry of entries) {
    if (entry.isDirectory() && !entry.name.startsWith('.')) {
      getAllMdFiles(path.join(dir, entry.name), files);
    } else if (entry.name.endsWith('.md')) {
      files.push(path.join(dir, entry.name));
    }
  }
  return files.sort();
}

function main() {
  console.log('🚀 Code Odyssey Curriculum Ingest v3');
  console.log('=====================================');

  fs.mkdirSync(OUT_DIR, { recursive: true });
  fs.mkdirSync(path.dirname(MANIFEST_OUT), { recursive: true });

  const allBooks = [];

  for (const [language, langDir] of Object.entries(LANGUAGE_DIRS)) {
    console.log(`\n📚 Processing ${language}...`);
    const books = processLanguage(language, langDir);
    console.log(`   Found ${books.length} books with ${books.reduce((s, b) => s + b.totalPages, 0)} total pages`);

    for (const book of books) {
      const outFile = path.join(OUT_DIR, `${book.id}.json`);
      fs.writeFileSync(outFile, JSON.stringify(book, null, 2));
      allBooks.push(book);
    }
  }

  // Generate manifest
  const manifest = {
    version: '3.0',
    generated: new Date().toISOString(),
    books: allBooks.map(b => ({
      id: b.id,
      language: b.language,
      world: b.world,
      worldIndex: b.worldIndex,
      chapter: b.chapter,
      title: b.title,
      description: b.description,
      coverColor: b.coverColor,
      coverEmoji: b.coverEmoji,
      totalPages: b.totalPages,
      prerequisites: b.prerequisites,
    })),
  };

  fs.writeFileSync(MANIFEST_OUT, JSON.stringify(manifest, null, 2));

  console.log('\n✅ Done!');
  console.log(`   Total books: ${allBooks.length}`);
  console.log(`   Total pages: ${allBooks.reduce((s, b) => s + b.totalPages, 0)}`);
  console.log(`   Manifest: ${MANIFEST_OUT}`);
}

main();
