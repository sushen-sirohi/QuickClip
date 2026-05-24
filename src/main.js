const { invoke } = window.__TAURI__.core;
const { register } = window.__TAURI__.globalShortcut;
const { getCurrentWindow } = window.__TAURI__.window;
const { listen } = window.__TAURI__.event;

let currentWindow = getCurrentWindow();
let isLoading = false;
let isToggling = false;
let activeTab = 'all';

// Helpers
function formatTime(timestamp) {
  const now = Date.now() / 1000;
  const diff = now - timestamp;
  if (diff < 60) return 'Just now';
  if (diff < 3600) return `${Math.floor(diff / 60)}m ago`;
  if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`;
  if (diff < 604800) return `${Math.floor(diff / 86400)}d ago`;
  return new Date(timestamp * 1000).toLocaleDateString();
}

function truncateText(text, maxLength = 300) {
  if (!text) return '';
  if (text.length <= maxLength) return text;
  return text.substring(0, maxLength) + '...';
}

function escapeHtml(text) {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}

function getIconForType(type, content) {
  if (type === 'image') return { icon: 'image', color: 'text-purple-400', bg: 'bg-purple-500/10', label: 'Image' };
  if (type === 'link' || content.startsWith('http')) return { icon: 'link', color: 'text-blue-400', bg: 'bg-blue-500/10', label: 'Link' };
  if (content && (content.includes('function') || content.includes('const ') || content.match(/[{};=()]/))) {
    return { icon: 'code', color: 'text-yellow-400', bg: 'bg-yellow-500/10', label: 'Code' };
  }
  return { icon: 'align-left', color: 'text-slate-400', bg: 'bg-slate-500/10', label: 'Text' };
}

// Render Logic (Simplified - No opacity/animations to ensure visibility)
function renderEntries(entries) {
  const listEl = document.getElementById('clipboard-list');
  console.log('Rendering items:', entries.length);

  if (entries.length === 0) {
    listEl.innerHTML = `
      <div class="flex flex-col items-center justify-center h-64 text-slate-500">
        <div class="w-12 h-12 bg-white/5 rounded-full flex items-center justify-center mb-4">
          <i data-lucide="clipboard-list" class="w-6 h-6 opacity-40"></i>
        </div>
        <p class="font-medium text-sm opacity-60">No history found</p>
      </div>
    `;
    if (window.lucide) window.lucide.createIcons();
    return;
  }

  const fragment = document.createDocumentFragment();

  entries.forEach((entry, index) => {
    const rawType = entry.type_ || 'text';
    const { icon, color, bg, label } = getIconForType(rawType, entry.content);
    const timeStr = formatTime(entry.timestamp);

    const item = document.createElement('div');

    // SAFE RENDER: No initial opacity/transform manipulations.
    item.className = 'clipboard-card group relative p-3 rounded-xl bg-[#252525] border border-white/5 hover:bg-[#2a2a2a] hover:border-white/10 transition-colors duration-200 cursor-pointer flex items-start gap-3 mb-2';

    item.dataset.id = entry.id;
    item.dataset.type = rawType;
    item.dataset.content = entry.content;

    let contentHtml = '';

    if (rawType === 'image') {
      contentHtml = `
            <div class="relative rounded-lg overflow-hidden border border-white/10 bg-black/50 h-32 w-full flex justify-center items-center">
                <img src="data:image/png;base64,${entry.content}" class="max-h-full max-w-full object-contain" loading="lazy" />
            </div>
        `;
    } else {
      const shortContent = truncateText(entry.content);
      contentHtml = `
            <p class="text-[13px] text-slate-300 font-light leading-relaxed whitespace-pre-wrap font-mono break-all opacity-90 group-hover:opacity-100 transition-opacity max-h-24 overflow-hidden mask-bottom">
              ${escapeHtml(shortContent)}
            </p>
        `;
    }

    item.innerHTML = `
        <div class="flex-shrink-0 mt-0.5">
          <div class="w-8 h-8 rounded-lg ${bg} flex items-center justify-center border border-white/5">
             <i data-lucide="${icon}" class="w-4 h-4 ${color}"></i>
          </div>
        </div>

        <div class="flex-1 min-w-0 space-y-1.5">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-2">
              <span class="text-[10px] uppercase tracking-wider font-bold text-slate-600">${label}</span>
              <span class="text-[10px] text-slate-600">• ${timeStr}</span>
            </div>
          </div>
          ${contentHtml}
        </div>

        <div class="absolute right-2 top-2 flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity duration-200 gpu-layer">
             <button class="copy-btn p-1.5 rounded-md bg-blue-600 hover:bg-blue-500 text-white shadow-sm transition-transform active:scale-95" title="Copy">
                <i data-lucide="copy" class="w-3.5 h-3.5 pointer-events-none"></i>
             </button>
             <button class="delete-btn p-1.5 rounded-md bg-zinc-800 hover:bg-red-500/20 text-slate-400 hover:text-red-400 transition-transform active:scale-95" title="Delete">
                <i data-lucide="trash-2" class="w-3.5 h-3.5 pointer-events-none"></i>
             </button>
        </div>
    `;

    fragment.appendChild(item);
  });

  listEl.innerHTML = '';
  listEl.appendChild(fragment);

  if (window.lucide) window.lucide.createIcons();
}

async function handleListClick(e) {
  const item = e.target.closest('.group');
  if (!item) return;

  const id = parseInt(item.dataset.id);
  const content = item.dataset.content;
  const type = item.dataset.type;

  if (e.target.closest('.delete-btn')) {
    e.stopPropagation();
    try {
      await invoke('delete_entry', { id });
      item.remove(); // Direct remove, no fancy animation for reliability
      if (document.getElementById('clipboard-list').children.length === 0) {
        renderEntries([]);
      }
    } catch (error) { console.error(error); }
    return;
  }

  const performCopy = async () => {
    try {
      await invoke('copy_to_clipboard', { content, entryType: type });
      // Pulse effect
      item.classList.add('border-blue-500', 'bg-blue-500/10');

      // Delay hide to allow feedback to be seen
      setTimeout(async () => {
        await currentWindow.hide();
        // Cleanup pulse
        setTimeout(() => {
          item.classList.remove('border-blue-500', 'bg-blue-500/10');
        }, 300);
      }, 150);
    } catch (error) { console.error(error); }
  };

  if (e.target.closest('.copy-btn')) {
    e.stopPropagation();
    await performCopy();
    return;
  }

  await performCopy();
}

async function loadClipboard(query = '') {
  if (isLoading) return;
  isLoading = true;

  try {
    let entries;
    if (query.trim()) {
      entries = await invoke('search_clipboard', { query });
    } else {
      entries = await invoke('get_recent_clipboard', {
        limit: 50,
        offset: 0,
        typeFilter: activeTab
      });
    }
    renderEntries(entries);
  } catch (error) {
    console.error('Failed to load:', error);
  } finally {
    isLoading = false;
  }
}

async function init() {
  const searchInput = document.getElementById('search-input');
  const tabsContainer = document.getElementById('tabs-container');

  document.getElementById('clipboard-list').addEventListener('click', handleListClick);

  // Clean initialization
  console.log('App Initializing...');

  tabsContainer.addEventListener('click', (e) => {
    if (e.target.classList.contains('tab-btn')) {
      document.querySelectorAll('.tab-btn').forEach(btn => btn.classList.remove('active'));
      e.target.classList.add('active');
      activeTab = e.target.dataset.tab;
      loadClipboard(searchInput.value);
    }
  });

  // Initial Load
  await loadClipboard();

  let searchTimeout;
  searchInput.addEventListener('input', (e) => {
    clearTimeout(searchTimeout);
    searchTimeout = setTimeout(() => {
      loadClipboard(e.target.value);
    }, 200);
  });

  document.addEventListener('keydown', async (e) => {
    if (e.key === 'Escape') {
      e.preventDefault();
      await currentWindow.hide();
    }
  });

  if (window.lucide) window.lucide.createIcons();

  await listen('clipboard-updated', async () => {
    // Safety check before reload
    try {
      if (await currentWindow.isVisible()) {
        await loadClipboard(searchInput.value);
      }
    } catch (e) { }
  });

  try {
    await register('CommandOrControl+Shift+V', async () => {
      if (isToggling) return;
      isToggling = true;

      try {
        const isVisible = await currentWindow.isVisible();

        if (isVisible) {
          await currentWindow.hide();
        } else {
          await currentWindow.show();
          await currentWindow.setFocus();

          searchInput.value = '';
          await loadClipboard(); // Force fresh load on showing

          setTimeout(() => searchInput.focus(), 50);
        }
      } catch (error) {
        console.error('Shortcut error:', error);
      } finally {
        setTimeout(() => { isToggling = false; }, 300);
      }
    });
  } catch (e) { console.log('Shortcut reg check', e); }

  await currentWindow.listen('tauri://focus', async () => {
    searchInput.value = '';
    await loadClipboard();
    setTimeout(() => searchInput.focus(), 50);
  });
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', init);
} else {
  init();
}
