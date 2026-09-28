<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import { useRouter } from 'vue-router'
import { usePreferencesStore } from '../stores/preferences'
import { useI18n } from 'vue-i18n'
import { ArrowLeft } from 'lucide-vue-next'

const router = useRouter()
const store = usePreferencesStore()
const { t } = useI18n()
const locale = ref('')

onMounted(async () => {
  if (!store.preferences) await store.loadPreferences()
  locale.value = store.preferences?.locale || 'pt-BR'
})

watch(locale, (newVal) => {
  if (store.preferences) {
    store.savePreferences({ ...store.preferences, locale: newVal })
  }
})
</script>


<template>
  <div class="flex flex-col h-screen overflow-hidden bg-deep-space font-mono text-gray-100">
    <!-- Header -->
    <div
      class="flex items-center gap-3 px-4 pt-5 pb-3 border-b border-border-dark bg-dark-surface/50 shrink-0"
      data-tauri-drag-region
    >
      <button
        @click="router.push('/')"
        class="p-1.5 -ml-1.5 rounded-sm text-gray-400 hover:text-neon-green hover:bg-white/5 transition-all"
      >
        <ArrowLeft class="w-4 h-4" />
      </button>
      <span class="text-sm font-bold tracking-tight text-gray-100 uppercase">{{ t('settings.title') }}</span>
    </div>

    <!-- Content -->
    <div class="flex-1 overflow-y-auto px-4 py-4 min-h-0 space-y-4">
      <!-- Language Section -->
      <div class="card space-y-3">
        <div class="flex items-center justify-between gap-4">
          <div>
            <p class="text-sm font-medium text-gray-200 uppercase tracking-wide">{{ t('settings.language') }}</p>
            <p class="text-xs text-gray-500 mt-0.5">Interface localization</p>
          </div>
          <div class="relative w-40">
            <select
              v-model="locale"
              class="appearance-none w-full bg-deep-space border border-gray-700 rounded-sm px-3 py-1.5 text-xs text-gray-200 focus:border-neon-green/50 focus:ring-1 focus:ring-neon-green/50 outline-none transition-all"
            >
              <option value="pt-BR">Português (BR)</option>
              <option value="en-US">English (US)</option>
            </select>
            <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-2 text-gray-500">
              <svg class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7" />
              </svg>
            </div>
          </div>
        </div>
      </div>

      <p class="text-[10px] text-gray-600 font-mono text-center uppercase tracking-widest pt-4">
        DevGen Sys v1.0.0
      </p>
    </div>
  </div>
</template>

<style scoped>
/* Scoped styles can be minimal now as we rely on global utility classes */
</style>
