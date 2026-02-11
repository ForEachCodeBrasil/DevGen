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
  <div class="max-w-2xl mx-auto space-y-8">
    <h1 class="text-2xl font-bold text-white">{{ t('settings.title') }}</h1>
    
    <div class="bg-zinc-900 rounded-xl border border-white/5 p-6 space-y-6">
      <div class="space-y-2">
        <label class="block text-sm font-medium text-zinc-400">{{ t('settings.language') }}</label>
        <select 
          v-model="locale"
          class="w-full bg-zinc-950 border border-zinc-800 rounded-lg px-3 py-2 text-white focus:outline-none focus:ring-2 focus:ring-blue-600"
        >
          <option value="pt-BR">Português (Brasil)</option>
          <option value="en-US">English (US)</option>
        </select>
      </div>
    </div>
  </div>
</template>
