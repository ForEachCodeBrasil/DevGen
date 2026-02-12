<script setup lang="ts">
import { onMounted, ref, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useGeneratorsStore } from '../stores/generators'
import { ArrowLeft, Play, Copy, Check, Loader2 } from 'lucide-vue-next'
import { useClipboard } from '@vueuse/core'

const route = useRoute()
const router = useRouter()
const store = useGeneratorsStore()
const { copy, copied } = useClipboard()

const generatorId = route.params.id as string
const generator = computed(() => store.getGeneratorById(generatorId))
const loading = ref(false)
const result = ref<string | null>(null)
const error = ref<string | null>(null)
const options = ref<Record<string, unknown>>({})

onMounted(async () => {
  if (store.generators.length === 0) await store.fetchGenerators()
  if (!generator.value) {
    router.replace('/')
    return
  }
  if (generator.value?.options?.fields) {
    const defaults: Record<string, unknown> = {}
    for (const field of generator.value.options.fields) {
      if (field.default !== undefined) defaults[field.name] = field.default
    }
    options.value = defaults
  } else {
    options.value = {}
  }
})

async function handleGenerate() {
  loading.value = true
  result.value = null
  error.value = null
  try {
    const response = await store.generate({ generator_id: generatorId, options: options.value })
    if (response.text) result.value = response.text
  } catch (e: unknown) {
    error.value = (e as Error).message
  } finally {
    loading.value = false
  }
}

function handleCopy() {
  if (result.value) copy(result.value)
}
</script>

<template>
  <div v-if="generator" class="flex flex-col h-screen overflow-hidden panel-bg">
    <!-- Header with back -->
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
      <span class="text-[13px] font-semibold text-zinc-100 truncate flex-1">
        {{ generator.name }}
      </span>
    </div>

    <!-- Content -->
    <div class="flex-1 overflow-y-auto px-4 py-3 space-y-3 min-h-0">
      <!-- Options -->
      <div
        v-if="generator.options?.fields?.length"
        class="rounded-lg border border-white/5 bg-white/5 p-3 space-y-3"
      >
        <div
          v-for="field in generator.options.fields"
          :key="field.name"
          class="flex items-center justify-between gap-3"
        >
          <label
            :for="field.name"
            class="text-[11px] font-medium text-zinc-400 shrink-0 cursor-pointer"
            @click="field.type === 'boolean' && (options[field.name] = !options[field.name])"
          >
            {{ field.label }}
          </label>
          <div v-if="field.type === 'select'" class="relative min-w-0">
            <select
              :id="field.name"
              v-model="options[field.name]"
              class="native-select w-32 text-right text-[11px] py-1.5"
            >
              <option v-for="opt in field.options" :key="opt" :value="opt">{{ opt }}</option>
            </select>
            <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-2 text-zinc-500">
              <svg class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7" />
              </svg>
            </div>
          </div>
          <button
            v-else-if="field.type === 'boolean'"
            @click="options[field.name] = !options[field.name]"
            class="native-toggle-track shrink-0"
            :class="options[field.name] ? 'native-toggle-track-on' : 'native-toggle-track-off'"
          >
            <span
              class="native-toggle-thumb translate-y-0.5"
              :class="options[field.name] ? 'translate-x-[18px]' : 'translate-x-0.5'"
            />
          </button>
          <input
            v-else
            :id="field.name"
            v-model="options[field.name]"
            type="text"
            class="glass-input w-32 text-right text-[11px] py-1.5"
          />
        </div>
      </div>

      <!-- Generate -->
      <button
        @click="handleGenerate"
        :disabled="loading"
        class="w-full flex items-center justify-center gap-2 py-2.5 rounded-lg bg-blue-500 hover:bg-blue-400 disabled:opacity-50 text-white text-[12px] font-medium transition-all"
      >
        <Loader2 v-if="loading" class="w-4 h-4 animate-spin" />
        <Play v-else class="w-4 h-4" />
        {{ result ? 'Regenerate' : 'Generate' }}
      </button>

      <!-- Result -->
      <div v-if="result" class="rounded-lg border border-white/10 bg-white/5 p-3 relative group">
        <button
          @click="handleCopy"
          class="absolute top-2 right-2 p-1.5 rounded-md bg-white/10 text-zinc-400 hover:text-white opacity-0 group-hover:opacity-100 transition-opacity"
        >
          <Check v-if="copied" class="w-3.5 h-3.5 text-green-400" />
          <Copy v-else class="w-3.5 h-3.5" />
        </button>
        <p class="font-mono text-[11px] text-zinc-200 whitespace-pre-wrap break-all pr-8">
          {{ result }}
        </p>
        <p v-if="copied" class="text-[10px] text-green-400 mt-2">Copied to clipboard</p>
      </div>

      <!-- Error -->
      <div v-if="error" class="rounded-lg border border-red-500/20 bg-red-500/10 p-3">
        <p class="text-[11px] text-red-300">{{ error }}</p>
      </div>
    </div>
  </div>

  <div v-else class="flex flex-col items-center justify-center h-screen text-zinc-500 panel-bg">
    <Loader2 class="w-5 h-5 animate-spin mb-2" />
    <span class="text-[12px]">Loading...</span>
  </div>
</template>

<style scoped>
.panel-bg {
  background: rgba(0, 0, 0, 0.25);
  backdrop-filter: blur(40px);
  -webkit-backdrop-filter: blur(40px);
}
</style>
