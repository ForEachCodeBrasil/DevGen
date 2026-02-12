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
  <div class="flex flex-col h-screen overflow-hidden panel-bg">
    <!-- Header -->
    <div
      class="flex items-center gap-2 px-4 pt-12 pb-2 border-b border-white/5 flex-shrink-0"
      data-tauri-drag-region
    >
      <button
        @click="router.push('/')"
        class="p-1.5 -ml-1 rounded-md text-zinc-500 hover:text-zinc-200 hover:bg-white/5 transition-all"
      >
        <ArrowLeft class="w-4 h-4" />
      </button>
      <span class="text-[13px] font-semibold text-zinc-100">{{ t('settings.title') }}</span>
    </div>

    <!-- Content -->
    <div class="flex-1 overflow-y-auto px-4 py-3 min-h-0">
      <div class="rounded-lg border border-white/5 bg-white/5 p-3 space-y-4">
        <div class="flex items-center justify-between gap-3">
          <div>
            <p class="text-[12px] font-medium text-zinc-300">{{ t('settings.language') }}</p>
            <p class="text-[10px] text-zinc-500 mt-0.5">Interface language</p>
          </div>
          <div class="relative">
            <select v-model="locale" class="native-select w-36 text-right text-[11px] py-1.5">
              <option value="pt-BR">Português</option>
              <option value="en-US">English</option>
            </select>
            <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-2 text-zinc-500">
              <svg class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7" />
              </svg>
            </div>
          </div>
        </div>
      </div>

      <p class="text-[10px] text-zinc-600 mt-4 text-center">DevGen v0.1.0 (BETA)</p>
    </div>
  </div>
</template>

<style scoped>
.panel-bg {
  background: rgba(0, 0, 0, 0.25);
  backdrop-filter: blur(40px);
  -webkit-backdrop-filter: blur(40px);
}
</style>
