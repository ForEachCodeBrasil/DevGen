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
let revalidateTimer: number | undefined;

const shouldRevalidate = (lastValidatedAt?: string | null) => {
  if (!lastValidatedAt) return true;
  const last = Date.parse(lastValidatedAt);
  if (Number.isNaN(last)) return true;
  return Date.now() - last >= 24 * 60 * 60 * 1000;
};

onMounted(async () => {
  unlistenFn = await setupQuickActionListener();

  const prefs = usePreferencesStore();
  await prefs.loadPreferences();
  const state = prefs.licenseState;
  if (state.license_key && state.instance_id && shouldRevalidate(state.last_validated_at)) {
    try {
      await prefs.validateLicense();
    } catch (e) {
      console.error('Initial license validation failed:', e);
    }
  }

  revalidateTimer = window.setInterval(async () => {
    const next = prefs.licenseState;
    if (!next.license_key || !next.instance_id) return;
    if (!shouldRevalidate(next.last_validated_at)) return;
    try {
      await prefs.validateLicense();
    } catch (e) {
      console.error('Scheduled license validation failed:', e);
    }
  }, 60 * 60 * 1000);
});

onUnmounted(() => {
  if (unlistenFn) {
    unlistenFn();
  }
  if (revalidateTimer) {
    window.clearInterval(revalidateTimer);
  }
});
</script>

<template>
  <MainLayout>
    <RouterView />
  </MainLayout>
</template>
