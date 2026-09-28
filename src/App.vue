<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { usePreferencesStore } from './stores/preferences';
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

  const prefs = usePreferencesStore();
  await prefs.loadPreferences();
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
