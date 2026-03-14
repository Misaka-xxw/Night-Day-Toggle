// script.js - 优化版（使用requestAnimationFrame）

// 获取DOM元素
const BUTTON = document.querySelector("button.toggle");

// 状态变量
let isDragging = false;
let dragStartX = 0;
let dragStartY = 0;
let windowStartX = 0;
let windowStartY = 0;
let animationFrameId = null;
let lastUpdateTime = 0;
const UPDATE_INTERVAL = 16; // 约60fps

// 获取当前系统主题
async function getSystemTheme() {
  if (window.__TAURI__ && window.__TAURI__.core) {
    try {
      return await window.__TAURI__.core.invoke('get_system_theme');
    } catch (error) {
      console.error('获取系统主题失败:', error);
      return 'light';
    }
  }
  return 'light';
}

// 切换系统主题
async function toggleSystemTheme() {
  if (window.__TAURI__ && window.__TAURI__.core) {
    try {
      const newTheme = await window.__TAURI__.core.invoke('toggle_system_theme');

      if (window.__TAURI__?.notification) {
        window.__TAURI__.notification.sendNotification({
          title: '主题切换',
          body: `已切换到${newTheme === 'dark' ? '夜间' : '日间'}模式`
        });
      }

      return newTheme;
    } catch (error) {
      console.error('切换系统主题失败:', error);
      return null;
    }
  }
  return null;
}

// 主切换函数
const TOGGLE = async () => {
  if (isDragging) return; // 如果正在拖动，不切换主题
  
  const IS_PRESSED = BUTTON.matches("[aria-pressed=true]");
  const systemTheme = await toggleSystemTheme();

  if (systemTheme) {
    BUTTON.setAttribute("aria-pressed", (systemTheme === 'dark').toString());
  } else {
    BUTTON.setAttribute("aria-pressed", !IS_PRESSED);
  }
};

// 初始化主题
async function initTheme() {
  if (!BUTTON) return;

  const systemTheme = await getSystemTheme();
  const shouldBePressed = systemTheme === 'dark';
  const currentPressed = BUTTON.matches("[aria-pressed=true]");

  if (currentPressed !== shouldBePressed) {
    BUTTON.setAttribute("aria-pressed", shouldBePressed.toString());
  }
}

// 获取窗口位置
async function getWindowPosition() {
  if (window.__TAURI__ && window.__TAURI__.core) {
    try {
      const position = await window.__TAURI__.core.invoke('get_window_position');
      return { x: position[0], y: position[1] };
    } catch (err) {
      console.error('获取窗口位置失败:', err);
    }
  }
  return { x: 0, y: 0 };
}

// 设置窗口位置
async function setWindowPosition(x, y) {
  if (window.__TAURI__ && window.__TAURI__.core) {
    try {
      await window.__TAURI__.core.invoke('set_window_position', { x, y });
      return true;
    } catch (err) {
      console.error('设置窗口位置失败:', err);
    }
  }
  return false;
}

// 使用requestAnimationFrame优化拖动
function startAnimationLoop() {
  if (!animationFrameId) {
    animationFrameId = requestAnimationFrame(animateDrag);
  }
}

function stopAnimationLoop() {
  if (animationFrameId) {
    cancelAnimationFrame(animationFrameId);
    animationFrameId = null;
  }
}

let targetX = 0;
let targetY = 0;
let currentX = 0;
let currentY = 0;

async function animateDrag(timestamp) {
  if (!isDragging) return;
  if (timestamp - lastUpdateTime >= UPDATE_INTERVAL) {
    const roundedX = Math.round(targetX);
    const roundedY = Math.round(targetY);
    
    await setWindowPosition(roundedX, roundedY);
    lastUpdateTime = timestamp;
  }
  
  animationFrameId = requestAnimationFrame(animateDrag);
}

async function startManualDrag(e) {
  isDragging = true;
  dragStartX = e.screenX;
  dragStartY = e.screenY;
  const pos = await getWindowPosition();
  windowStartX = pos.x;
  windowStartY = pos.y;
  currentX = windowStartX;
  currentY = windowStartY;
  targetX = windowStartX;
  targetY = windowStartY;
  startAnimationLoop();
  
  document.addEventListener('mousemove', handleDragMove);
  document.addEventListener('mouseup', handleDragEnd);
  e.preventDefault();
  e.stopPropagation();
}

function handleDragMove(e) {
  if (!isDragging) return;
  const deltaX = e.screenX - dragStartX;
  const deltaY = e.screenY - dragStartY;
  targetX = windowStartX + deltaX;
  targetY = windowStartY + deltaY;
  e.preventDefault();
}

function handleDragEnd(e) {
  if (!isDragging) return;
  isDragging = false;
  document.removeEventListener('mousemove', handleDragMove);
  document.removeEventListener('mouseup', handleDragEnd);
  stopAnimationLoop();
  e.preventDefault();
  e.stopPropagation();
}

document.addEventListener("DOMContentLoaded", () => {
  document.addEventListener('contextmenu', (e) => e.preventDefault());
  document.addEventListener('mousedown', (e) => {
    const isOnButton = e.target.closest('.toggle') ||
      e.target.closest('.toggle__content') ||
      e.target.closest('.toggle__indicator') ||
      e.target.closest('.toggle__star') ||
      e.target.classList.contains('toggle');

    if (e.button === 2) { // 右键
      console.log(`右键按下${isOnButton ? '在按钮上' : '在透明区域'}`);
      startManualDrag(e);
      return;
    }

    if (e.button === 0) { // 左键
      if (!isOnButton) {
        console.log('左键在透明区域按下，开始手动拖动');
        startManualDrag(e);
      } else {
        console.log('左键在按钮上按下，准备切换主题');
      }
    }
  });

  if (BUTTON) {
    BUTTON.addEventListener("click", TOGGLE);
  }
  initTheme();
});