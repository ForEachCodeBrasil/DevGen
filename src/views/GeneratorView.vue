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
  if (generator.value?.requires_pro) {
    error.value = 'Este gerador está disponível apenas no DevGen Pro. Ative sua licença nas configurações.'
    return
  }
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
  <div v-if="generator" class="flex flex-col h-screen overflow-hidden bg-deep-space text-gray-100 font-mono">
    <!-- Header with back -->
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
      <span class="text-sm font-bold tracking-tight text-gray-100 flex-1 uppercase">
        {{ generator.name }}
      </span>
      <div v-if="loading" class="animate-spin text-neon-green">
        <Loader2 class="w-4 h-4" />
      </div>
    </div>

    <!-- Content -->
    <div class="flex-1 overflow-y-auto px-4 py-4 space-y-4 min-h-0">
      <!-- Options -->
      <div
        v-if="generator.options?.fields?.length"
        class="card space-y-4"
      >
        <div
          v-for="field in generator.options.fields"
          :key="field.name"
          class="flex items-center justify-between gap-4"
        >
          <label
            :for="field.name"
            class="text-xs font-medium text-gray-400 shrink-0 cursor-pointer uppercase tracking-wider"
            @click="field.type === 'boolean' && (options[field.name] = !options[field.name])"
          >
            {{ field.label }}
          </label>
          
          <div v-if="field.type === 'select'" class="relative min-w-0 w-40">
            <select
              :id="field.name"
              v-model="options[field.name]"
              class="appearance-none w-full bg-deep-space border border-gray-700 rounded-sm px-3 py-1.5 text-xs text-gray-200 focus:border-neon-green/50 focus:ring-1 focus:ring-neon-green/50 outline-none transition-all"
            >
              <option v-for="opt in field.options" :key="opt" :value="opt">{{ opt }}</option>
            </select>
            <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-2 text-gray-500">
              <svg class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7" />
              </svg>
            </div>
          </div>
          
          <button
            v-else-if="field.type === 'boolean'"
            @click="options[field.name] = !options[field.name]"
            class="relative inline-flex h-5 w-9 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-neon-green/50 focus:ring-offset-2 focus:ring-offset-gray-900"
            :class="options[field.name] ? 'bg-neon-green' : 'bg-gray-700'"
          >
            <span
              class="inline-block h-3 w-3 transform rounded-full bg-white transition-transform"
              :class="options[field.name] ? 'translate-x-5' : 'translate-x-1'"
            />
          </button>
          
          <input
            v-else-if="field.type === 'number'"
            :id="field.name"
            v-model.number="options[field.name]"
            type="number"
            min="1"
            class="w-24 bg-deep-space border border-gray-700 rounded-sm px-2 py-1.5 text-xs text-right text-gray-200 focus:border-neon-green/50 focus:ring-1 focus:ring-neon-green/50 outline-none transition-all"
          />
          
          <input
            v-else
            :id="field.name"
            v-model="options[field.name]"
            type="text"
            class="w-40 bg-deep-space border border-gray-700 rounded-sm px-2 py-1.5 text-xs text-right text-gray-200 focus:border-neon-green/50 focus:ring-1 focus:ring-neon-green/50 outline-none transition-all"
          />
        </div>
      </div>

      <!-- Action Button -->
      <button
        @click="handleGenerate"
        :disabled="loading || generator.requires_pro"
        class="w-full btn btn-primary py-3 text-sm font-bold tracking-wide uppercase shadow-neon disabled:opacity-50 disabled:shadow-none disabled:cursor-not-allowed group"
      >
        <span class="flex items-center gap-2">
          <Loader2 v-if="loading" class="w-4 h-4 animate-spin" />
          <Play v-else class="w-4 h-4 group-hover:fill-current" />
          {{ generator.requires_pro ? 'PRO_ONLY' : (result ? 'REGENERATE' : 'GENERATE') }}
        </span>
      </button>

      <div
        v-if="generator.requires_pro"
        class="p-3 rounded-sm border border-warning-amber/20 bg-warning-amber/10 flex items-start gap-3"
      >
        <div class="text-warning-amber shrink-0 mt-0.5">
           <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/></svg>
        </div>
        <p class="text-xs text-warning-amber/90">
          This generator requires a <strong>PRO</strong> license. Please upgrade in Settings to access this feature.
        </p>
      </div>

      <!-- Result Area -->
      <div v-if="result" class="card relative p-0 overflow-hidden group border-neon-green/30">
        <div class="absolute top-0 right-0 p-2 flex gap-1 z-10">
          <button
            @click="handleCopy"
            class="p-1.5 rounded-sm bg-dark-surface border border-gray-700 text-gray-400 hover:text-white hover:border-neon-green/50 transition-all shadow-lg"
            title="Copy to clipboard"
          >
            <Check v-if="copied" class="w-3.5 h-3.5 text-neon-green" />
            <Copy v-else class="w-3.5 h-3.5" />
          </button>
        </div>
        
        <div class="p-4 bg-deep-space/50 min-h-[100px] max-h-[300px] overflow-y-auto custom-scrollbar">
          <pre class="font-mono text-sm text-neon-green whitespace-pre-wrap break-all">{{ result }}</pre>
        </div>
        
        <div class="px-3 py-1 bg-dark-surface border-t border-gray-800 flex justify-between items-center text-[10px] text-gray-500 uppercase">
          <span>OUTPUT_LENGTH: {{ result.length }}</span>
          <span v-if="copied" class="text-neon-green font-bold">COPIED_TO_CLIPBOARD</span>
          <span v-else>READY</span>
        </div>
      </div>

      <!-- Error -->
      <div v-if="error" class="p-3 rounded-sm border border-error-red/20 bg-error-red/10 text-error-red text-xs font-mono">
        ERROR: {{ error }}
      </div>
    </div>
  </div>

  <div v-else class="flex flex-col items-center justify-center h-screen bg-deep-space text-gray-500">
    <Loader2 class="w-8 h-8 animate-spin text-neon-green mb-4" />
    <span class="text-xs font-mono tracking-widest uppercase">INITIALIZING_SYSTEM...</span>
  </div>
</template>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: #30363d;
  border-radius: 2px;
}
.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: #4ade80;
}
</style>
