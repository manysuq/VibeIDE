/**
 * Ultralight Code - Main Application JavaScript
 * Core UI logic and event handling
 */

// Application State
const AppState = {
    currentFile: null,
    openFiles: [],
    recentFiles: [],
    workspace: null,
    settings: {
        tabSize: 4,
        wordWrap: true,
        lineNumbers: true,
        minimap: true,
    }
};

// Initialize application when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    initializeApp();
});

function initializeApp() {
    console.log('Ultralight Code initializing...');
    
    // Setup event listeners
    setupActivityBarListeners();
    setupPanelListeners();
    setupEditorListeners();
    setupWindowControls();
    
    // Load saved state
    loadAppState();
    
    // Render initial UI
    renderFileTree();
    renderRecentFiles();
    
    console.log('Ultralight Code ready!');
}

// Activity Bar Listeners
function setupActivityBarListeners() {
    const activityItems = document.querySelectorAll('.activity-item');
    
    activityItems.forEach(item => {
        item.addEventListener('click', () => {
            // Remove active class from all items
            activityItems.forEach(i => i.classList.remove('active'));
            // Add active class to clicked item
            item.classList.add('active');
            
            // Handle view switching
            const view = item.dataset.view;
            switchView(view);
        });
    });
}

function switchView(viewName) {
    console.log('Switching to view:', viewName);
    
    // Hide all sidebar views
    // Show selected view
    // This will be expanded with actual view implementations
}

// Panel Listeners
function setupPanelListeners() {
    const panelTabs = document.querySelectorAll('.panel-tab');
    
    panelTabs.forEach(tab => {
        tab.addEventListener('click', () => {
            // Remove active class from all tabs
            panelTabs.forEach(t => t.classList.remove('active'));
            // Add active class to clicked tab
            tab.classList.add('active');
            
            // Hide all panes
            document.querySelectorAll('.panel-pane').forEach(pane => {
                pane.classList.remove('active');
            });
            
            // Show selected pane
            const panelName = tab.dataset.panel;
            document.getElementById(`${panelName}-pane`).classList.add('active');
        });
    });
}

// Editor Listeners
function setupEditorListeners() {
    const textarea = document.getElementById('editor-textarea');
    const lineNumbers = document.getElementById('line-numbers');
    const cursorPosition = document.getElementById('cursor-position');
    
    if (textarea) {
        textarea.addEventListener('input', handleEditorInput);
        textarea.addEventListener('scroll', handleEditorScroll);
        textarea.addEventListener('keydown', handleEditorKeydown);
        textarea.addEventListener('click', updateCursorPosition);
        textarea.addEventListener('keyup', updateCursorPosition);
    }
    
    function handleEditorInput(e) {
        updateLineNumbers();
        highlightSyntax();
        saveCurrentFile();
    }
    
    function handleEditorScroll(e) {
        // Sync scroll with highlights and line numbers
        const highlights = document.getElementById('editor-highlights');
        if (highlights) {
            highlights.scrollTop = e.target.scrollTop;
            highlights.scrollLeft = e.target.scrollLeft;
        }
        if (lineNumbers) {
            lineNumbers.scrollTop = e.target.scrollTop;
        }
    }
    
    function handleEditorKeydown(e) {
        // Handle Tab key
        if (e.key === 'Tab') {
            e.preventDefault();
            const start = e.target.selectionStart;
            const end = e.target.selectionEnd;
            const spaces = ' '.repeat(AppState.settings.tabSize);
            
            e.target.value = e.target.value.substring(0, start) + spaces + e.target.value.substring(end);
            e.target.selectionStart = e.target.selectionEnd = start + AppState.settings.tabSize;
        }
    }
    
    function updateCursorPosition() {
        const textarea = document.getElementById('editor-textarea');
        const text = textarea.value.substring(0, textarea.selectionStart);
        const lines = text.split('\n');
        const line = lines.length;
        const col = lines[lines.length - 1].length + 1;
        
        cursorPosition.textContent = `Ln ${line}, Col ${col}`;
    }
    
    function updateLineNumbers() {
        const textarea = document.getElementById('editor-textarea');
        const lines = textarea.value.split('\n').length;
        
        lineNumbers.innerHTML = '';
        for (let i = 1; i <= lines; i++) {
            const num = document.createElement('div');
            num.textContent = i;
            lineNumbers.appendChild(num);
        }
    }
}

// Window Controls
function setupWindowControls() {
    const controls = document.querySelectorAll('.window-control');
    
    controls.forEach(control => {
        control.addEventListener('click', () => {
            const action = control.dataset.action;
            handleWindowAction(action);
        });
    });
}

function handleWindowAction(action) {
    console.log('Window action:', action);
    // Send message to Rust backend via Ultralight
    // This will be implemented with the actual IPC mechanism
}

// File Operations
function openFile() {
    console.log('Opening file dialog...');
    // Trigger native file open dialog via Rust backend
    sendMessageToBackend('openFile', {});
}

function openFolder() {
    console.log('Opening folder dialog...');
    // Trigger native folder open dialog via Rust backend
    sendMessageToBackend('openFolder', {});
}

function newFile() {
    console.log('Creating new file...');
    const filename = `Untitled-${Date.now()}`;
    createNewTab(filename, '');
}

function createNewTab(filename, content = '') {
    const tabsContainer = document.getElementById('tabs-container');
    const editorContainer = document.getElementById('editor-container');
    const welcomeTab = document.getElementById('welcome-tab');
    const codeEditor = document.getElementById('code-editor');
    
    // Hide welcome tab
    if (welcomeTab) welcomeTab.style.display = 'none';
    if (codeEditor) codeEditor.style.display = 'flex';
    
    // Create tab
    const tab = document.createElement('div');
    tab.className = 'tab active';
    tab.dataset.filename = filename;
    tab.innerHTML = `
        <span class="tab-icon">📄</span>
        <span class="tab-name">${filename}</span>
        <span class="tab-close" onclick="closeTab('${filename}')">×</span>
    `;
    
    // Remove active from other tabs
    document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
    
    tabsContainer.appendChild(tab);
    
    // Add to open files
    AppState.openFiles.push({ filename, content, modified: false });
    AppState.currentFile = filename;
    
    // Update editor
    const textarea = document.getElementById('editor-textarea');
    if (textarea) {
        textarea.value = content;
        updateLineNumbers();
    }
}

function closeTab(filename) {
    console.log('Closing tab:', filename);
    
    // Find and remove tab element
    const tab = document.querySelector(`.tab[data-filename="${filename}"]`);
    if (tab) {
        tab.remove();
    }
    
    // Remove from open files
    AppState.openFiles = AppState.openFiles.filter(f => f.filename !== filename);
    
    // If no more tabs, show welcome
    if (AppState.openFiles.length === 0) {
        const welcomeTab = document.getElementById('welcome-tab');
        const codeEditor = document.getElementById('code-editor');
        if (welcomeTab) welcomeTab.style.display = 'flex';
        if (codeEditor) codeEditor.style.display = 'none';
    }
}

function saveCurrentFile() {
    const textarea = document.getElementById('editor-textarea');
    const currentFilename = AppState.currentFile;
    
    if (currentFilename && textarea) {
        const content = textarea.value;
        sendMessageToBackend('saveFile', { filename: currentFilename, content });
    }
}

// Syntax Highlighting (basic implementation)
function highlightSyntax() {
    const textarea = document.getElementById('editor-textarea');
    const highlights = document.getElementById('editor-highlights');
    
    if (!textarea || !highlights) return;
    
    let text = textarea.value;
    
    // Escape HTML
    text = text.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
    
    // Basic syntax highlighting rules (can be extended per language)
    // Keywords
    text = text.replace(/\b(const|let|var|function|return|if|else|for|while|class|import|export|from)\b/g, 
        '<span class="syntax-keyword">$1</span>');
    
    // Strings
    text = text.replace(/(["'`])(?:(?!\1)[^\\]|\\.)*?\1/g, 
        '<span class="syntax-string">$&</span>');
    
    // Comments
    text = text.replace(/(\/\/.*$)/gm, '<span class="syntax-comment">$1</span>');
    
    // Numbers
    text = text.replace(/\b(\d+)\b/g, '<span class="syntax-number">$1</span>');
    
    // Functions
    text = text.replace(/\b([a-zA-Z_]\w*)(?=\()/g, '<span class="syntax-function">$1</span>');
    
    highlights.innerHTML = text;
}

// File Tree Rendering
function renderFileTree() {
    const fileTree = document.getElementById('file-tree');
    if (!fileTree) return;
    
    // This will be populated from the backend
    // Example structure:
    /*
    const exampleTree = [
        { name: 'src', type: 'folder', children: [
            { name: 'main.rs', type: 'file' },
            { name: 'lib.rs', type: 'file' }
        ]},
        { name: 'Cargo.toml', type: 'file' },
        { name: 'README.md', type: 'file' }
    ];
    */
    
    fileTree.innerHTML = '';
}

// Recent Files
function renderRecentFiles() {
    const recentList = document.getElementById('recent-files-list');
    if (!recentList) return;
    
    recentList.innerHTML = '';
    
    AppState.recentFiles.forEach(file => {
        const li = document.createElement('li');
        li.textContent = file;
        li.addEventListener('click', () => openRecentFile(file));
        recentList.appendChild(li);
    });
}

function openRecentFile(filename) {
    console.log('Opening recent file:', filename);
    sendMessageToBackend('openFile', { filename });
}

// App State Persistence
function loadAppState() {
    try {
        const saved = localStorage.getItem('ultralight_code_state');
        if (saved) {
            const state = JSON.parse(saved);
            Object.assign(AppState, state);
        }
    } catch (e) {
        console.error('Failed to load app state:', e);
    }
}

function saveAppState() {
    try {
        localStorage.setItem('ultralight_code_state', JSON.stringify(AppState));
    } catch (e) {
        console.error('Failed to save app state:', e);
    }
}

// Backend Communication
function sendMessageToBackend(type, data) {
    console.log('Sending to backend:', type, data);
    
    // This will use Ultralight's JS bridge to communicate with Rust
    // Implementation depends on Ultralight's API
    
    if (window.ultralight) {
        window.ultralight.postMessage(JSON.stringify({ type, data }));
    }
}

// Utility functions
function debounce(func, wait) {
    let timeout;
    return function executedFunction(...args) {
        const later = () => {
            clearTimeout(timeout);
            func(...args);
        };
        clearTimeout(timeout);
        timeout = setTimeout(later, wait);
    };
}

// Export for module usage
if (typeof module !== 'undefined' && module.exports) {
    module.exports = { AppState, sendMessageToBackend };
}
