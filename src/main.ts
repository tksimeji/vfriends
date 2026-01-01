import {createApp} from 'vue';
import {createVfm} from 'vue-final-modal';
import App from './App.vue';
import 'vue-final-modal/style.css';
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css';

const app = createApp(App);
const vfm = createVfm();
app.use(vfm);
app.mount('#app');

const setupTitlebarInset = () => {
  const controls = document.getElementById('tauri-window-controls');
  if (!controls) return;

  const updateInset = () => {
    const rect = controls.getBoundingClientRect();
    const width = Math.max(0, Math.round(rect.width));
    document.documentElement.style.setProperty('--tauri-frame-controls-width', `${width}px`);
  };

  updateInset();

  const resizeObserver = new ResizeObserver(updateInset);
  resizeObserver.observe(controls);

  const mutationObserver = new MutationObserver(updateInset);
  mutationObserver.observe(controls, {childList: true, subtree: true});

  window.addEventListener('resize', updateInset);
};

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', setupTitlebarInset, {once: true});
} else {
  setupTitlebarInset();
}
