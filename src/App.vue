<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import MainLayout from './layouts/MainLayout.vue';
import { RouterView } from 'vue-router';
// import { writeText } from '@tauri-apps/plugin-clipboard-manager'; // Need to check if plugin is installed/available or use navigator.clipboard

const setupQuickActionListener = async () => {
  const unlisten = await listen<string>('quick-action', async (event) => {
    const actionId = event.payload;
    console.log('Quick action triggered:', actionId);
    
    try {
      const response = await invoke<{ text: string }>('quick_generate', { action: actionId });
      if (response && response.text) {
        // Use navigator clipboard API which is supported in Tauri webview
        await navigator.clipboard.writeText(response.text);
        
        // Optional: Show notification toast?
        console.log('Copied to clipboard:', response.text);
        // We could implement a global toast here if we had a Toast component
      }
    } catch (error) {
      console.error('Quick generate failed:', error);
    }
  });
  
  return unlisten;
};

let unlistenFn: (() => void) | undefined;

onMounted(async () => {
  unlistenFn = await setupQuickActionListener();
});

onUnmounted(() => {
  if (unlistenFn) {
    unlistenFn();
  }
});
</script>

<template>
  <MainLayout>
    <RouterView />
  </MainLayout>
</template>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

body {
  margin: 0;
  padding: 0;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }
}
</style>