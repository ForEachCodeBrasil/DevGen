<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import { usePreferencesStore } from '../stores/preferences'
import { useI18n } from 'vue-i18n'

const store = usePreferencesStore()
const { t } = useI18n()

const locale = ref('')

onMounted(async () => {
  if (!store.preferences) {
    await store.loadPreferences()
  }
  locale.value = store.preferences?.locale || 'pt-BR'
})

watch(locale, (newVal) => {
  if (store.preferences) {
    const updated = { ...store.preferences, locale: newVal }
    store.savePreferences(updated)
  }
})
</script>

<template>
  <div class="flex flex-col h-full max-w-2xl mx-auto px-6 py-8 animate-in fade-in duration-300">
    <!-- Header -->
    <div class="flex items-center gap-4 mb-8">
      <h1 class="text-xl font-semibold text-zinc-100 flex items-center gap-2">
        <span class="bg-blue-500/10 text-blue-400 p-1.5 rounded-md border border-blue-500/20">
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
            class="lucide lucide-settings-2">
            <path d="M20 7h-9" />
            <path d="M14 17H5" />
            <circle cx="17" cy="17" r="3" />
            <circle cx="7" cy="7" r="3" />
          </svg>
        </span>
        {{ t('settings.title') }}
      </h1>
    </div>

    <!-- Settings Panel -->
    <div class="bg-zinc-900/40 backdrop-blur-md rounded-xl border border-white/5 p-6 space-y-8 shadow-sm">

      <!-- Language Section -->
      <div class="flex items-center justify-between group">
        <div class="space-y-1">
          <label class="block text-sm font-medium text-zinc-300 group-hover:text-zinc-200 transition-colors">
            {{ t('settings.language') }}
          </label>
          <p class="text-xs text-zinc-500">Choose the interface language.</p>
        </div>

        <div class="relative">
          <select v-model="locale"
            class="appearance-none bg-zinc-950/50 border border-white/10 text-zinc-300 text-sm rounded-md py-1.5 pl-3 pr-8 focus:outline-none focus:ring-1 focus:ring-blue-500/30 hover:border-white/20 transition-all w-48 text-right cursor-pointer">
            <option value="pt-BR">Português (Brasil)</option>
            <option value="en-US">English (US)</option>
          </select>
          <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-zinc-500">
            <svg class="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
            </svg>
          </div>
        </div>
      </div>

      <!-- Add more sections here (History, etc.) as implemented -->

    </div>

    <div class="mt-auto text-center pt-8">
      <p class="text-xs text-zinc-600 font-mono">DevGen v0.1.0 (BETA)</p>
    </div>
  </div>
</template>
