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

// Dynamic options state (using simple type to avoid build errors)
const options = ref<any>({})

onMounted(async () => {
  if (store.generators.length === 0) {
    await store.fetchGenerators()
  }

  if (!generator.value) {
    router.replace('/')
    return
  }

  // Load defaults from options schema
  if (generator.value?.options?.fields) {
    const defaults: any = {}
    for (const field of generator.value.options.fields) {
      if (field.default !== undefined) {
        defaults[field.name] = field.default
      }
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
    const response = await store.generate({
      generator_id: generatorId,
      options: options.value
    })

    if (response.text) {
      result.value = response.text
    }
  } catch (e: any) {
    error.value = e.message
  } finally {
    loading.value = false
  }
}

function handleCopy() {
  if (result.value) {
    copy(result.value)
  }
}
</script>

<template>
  <div v-if="generator" class="flex flex-col h-full max-w-3xl mx-auto px-6 py-6 animate-in fade-in duration-300">
    <!-- Header -->
    <div class="flex items-center gap-4 mb-8">
      <button @click="router.back()"
        class="group p-2 -ml-2 rounded-md text-zinc-500 hover:text-zinc-200 hover:bg-white/5 transition-all">
        <ArrowLeft class="w-5 h-5 group-hover:-translate-x-0.5 transition-transform" />
      </button>
      <div>
        <h1 class="text-xl font-semibold text-zinc-100 flex items-center gap-3">
          {{ generator.name }}
          <span
            class="text-[10px] uppercase tracking-wider font-bold text-zinc-500 border border-zinc-800 px-1.5 py-0.5 rounded-sm bg-zinc-900/50">
            {{ generator.category }}
          </span>
        </h1>
        <p class="text-zinc-500 text-sm mt-0.5">{{ generator.description }}</p>
      </div>
    </div>

    <!-- Main Content Area -->
    <div class="space-y-6 flex-1 overflow-y-auto pb-10">

      <!-- Configuration Panel -->
      <div v-if="generator.options && generator.options.fields && generator.options.fields.length > 0"
        class="bg-zinc-900/20 border border-white/5 rounded-lg p-5 backdrop-blur-sm">
        <div class="space-y-5">
          <div v-for="field in generator.options.fields" :key="field.name"
            class="flex items-center justify-between group">
            <label :for="field.name"
              class="text-sm font-medium text-zinc-400 group-hover:text-zinc-300 transition-colors cursor-pointer select-none"
              @click="field.type === 'boolean' ? options[field.name] = !options[field.name] : null">
              {{ field.label }}
            </label>

            <!-- Select Input -->
            <div v-if="field.type === 'select'" class="relative">
              <select v-model="options[field.name]"
                class="appearance-none bg-zinc-950/50 border border-white/10 text-zinc-300 text-sm rounded-md py-1.5 pl-3 pr-8 focus:outline-none focus:ring-1 focus:ring-blue-500/30 hover:border-white/20 transition-all w-48 text-right cursor-pointer">
                <option v-for="opt in field.options" :key="opt" :value="opt">
                  {{ opt }}
                </option>
              </select>
              <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-zinc-500">
                <svg class="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                </svg>
              </div>
            </div>

            <!-- Boolean Toggle (Sleek) -->
            <button v-else-if="field.type === 'boolean'" @click="options[field.name] = !options[field.name]"
              class="relative inline-flex h-5 w-9 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500/20 focus:ring-offset-1 focus:ring-offset-zinc-900 cursor-pointer"
              :class="options[field.name] ? 'bg-blue-600' : 'bg-zinc-700'">
              <span
                class="inline-block h-3.5 w-3.5 transform rounded-full bg-white transition-transform duration-200 ease-in-out shadow-sm"
                :class="options[field.name] ? 'translate-x-[18px]' : 'translate-x-0.5'" />
            </button>

            <!-- Text Input -->
            <input v-else v-model="options[field.name]" type="text"
              class="bg-zinc-950/50 border border-white/10 text-zinc-300 text-sm rounded-md py-1.5 px-3 focus:outline-none focus:ring-1 focus:ring-blue-500/30 hover:border-white/20 transition-all w-48 text-right">
          </div>
        </div>
      </div>

      <!-- Action Button -->
      <button @click="handleGenerate" :disabled="loading"
        class="w-full group relative flex items-center justify-center gap-2 bg-gradient-to-b from-blue-600 to-blue-700 hover:from-blue-500 hover:to-blue-600 disabled:opacity-50 disabled:cursor-not-allowed text-white shadow-lg shadow-blue-900/20 border-t border-white/10 rounded-lg py-2.5 font-medium transition-all active:scale-[0.99]">
        <span
          class="absolute inset-0 bg-white/5 opacity-0 group-hover:opacity-100 rounded-lg transition-opacity"></span>
        <Loader2 v-if="loading" class="w-4 h-4 animate-spin" />
        <Play v-else class="w-4 h-4 fill-current opacity-80" />
        <span>{{ result ? 'Regenerate' : 'Generate' }}</span>
      </button>

      <!-- Result Display -->
      <div v-if="result" class="animate-in fade-in slide-in-from-bottom-2 duration-300">
        <div
          class="relative group bg-zinc-950/40 border border-white/10 rounded-xl p-6 backdrop-blur-md shadow-2xl transition-all hover:border-white/20">

          <div
            class="absolute top-3 right-3 flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity duration-200">
            <button @click="handleCopy"
              class="p-2 rounded-md bg-white/10 text-zinc-400 hover:text-white hover:bg-white/20 border border-white/5 transition-all active:scale-95 backdrop-blur-md"
              :title="copied ? 'Copied!' : 'Copy'">
              <Check v-if="copied" class="w-4 h-4 text-green-400" />
              <Copy v-else class="w-4 h-4" />
            </button>
          </div>

          <!-- Content -->
          <div
            class="font-mono text-base md:text-lg text-zinc-200 whitespace-pre-wrap break-all pr-12 leading-relaxed selection:bg-blue-500/40">
            {{ result }}
          </div>

          <!-- Metadata footer if needed, e.g. "Copied to clipboard" toast -->
          <div v-if="copied"
            class="absolute bottom-3 right-3 text-xs text-green-400 font-medium animate-in fade-in duration-200 flex items-center gap-1.5 bg-zinc-900/80 px-2 py-1 rounded backdrop-blur-sm border border-green-500/20">
            <Check class="w-3 h-3" /> Copied
          </div>
        </div>
      </div>

      <!-- Error Display -->
      <div v-if="error" class="bg-red-500/10 border border-red-500/20 rounded-lg p-4 flex items-start gap-3">
        <div class="text-red-400 mt-0.5">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
          </svg>
        </div>
        <span class="text-sm text-red-300">{{ error }}</span>
      </div>

    </div>
  </div>

  <div v-else class="flex flex-col items-center justify-center h-full text-zinc-500 gap-3">
    <Loader2 class="w-6 h-6 animate-spin text-blue-500/80" />
    <span class="text-sm font-medium">Loading generator...</span>
  </div>
</template>
