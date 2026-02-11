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
  <div v-if="generator" class="space-y-6 max-w-3xl mx-auto pt-6">
    <!-- Header -->
    <div class="flex items-center space-x-4">
      <button @click="router.back()"
        class="p-2 rounded-lg text-zinc-400 hover:text-white hover:bg-white/5 transition-colors">
        <ArrowLeft class="w-5 h-5" />
      </button>
      <div>
        <h1 class="text-2xl font-bold text-white flex items-center gap-2">
          {{ generator.name }}
          <span
            class="text-xs font-normal font-mono bg-blue-500/10 text-blue-400 px-2 py-0.5 rounded border border-blue-500/20">
            {{ generator.category }}
          </span>
        </h1>
        <p class="text-zinc-400 text-sm">{{ generator.description }}</p>
      </div>
    </div>

    <!-- Options Panel -->
    <div class="glass-panel rounded-xl p-6">
      <div v-if="!generator.options || !generator.options.fields || generator.options.fields.length === 0"
        class="text-center py-4 text-zinc-500 text-sm">
        No configurable options.
      </div>

      <div v-else class="space-y-4">
        <div v-for="field in generator.options.fields" :key="field.name" class="flex items-center justify-between">
          <label :for="field.name" class="text-sm font-medium text-zinc-300">{{ field.label }}</label>

          <!-- Boolean Toggle -->
          <button v-if="field.type === 'boolean'" @click="options[field.name] = !options[field.name]" :class="[
            'w-10 h-6 rounded-full transition-colors relative focus:outline-none ring-offset-2 ring-offset-zinc-900 focus:ring-2 focus:ring-blue-500/50',
            options[field.name] ? 'bg-blue-600' : 'bg-zinc-700'
          ]">
            <span :class="[
              'absolute top-1 left-1 bg-white w-4 h-4 rounded-full transition-transform',
              options[field.name] ? 'translate-x-4' : 'translate-x-0'
            ]" />
          </button>

          <!-- Text Input (Fallback) -->
          <input v-else v-model="options[field.name]" type="text" class="glass-input w-48">
        </div>
      </div>
    </div>

    <!-- Action Bar -->
    <div class="flex justify-end">
      <button @click="handleGenerate" :disabled="loading"
        class="flex items-center space-x-2 bg-blue-600 hover:bg-blue-500 disabled:bg-blue-600/50 text-white px-6 py-2.5 rounded-lg font-medium transition-all shadow-lg shadow-blue-900/20 active:scale-95">
        <Loader2 v-if="loading" class="w-4 h-4 animate-spin" />
        <Play v-else class="w-4 h-4 fill-current" />
        <span>{{ result ? 'Regenerate' : 'Generate' }}</span>
      </button>
    </div>

    <!-- Result Display -->
    <div v-if="result" class="animate-in fade-in slide-in-from-bottom-4 duration-300">
      <div class="glass-panel rounded-xl p-6 relative group">
        <div class="absolute top-4 right-4 flex space-x-2">
          <button @click="handleCopy"
            class="p-2 rounded-lg bg-white/5 text-zinc-400 hover:text-white hover:bg-white/10 transition-colors border border-white/5"
            :title="copied ? 'Copied!' : 'Copy'">
            <Check v-if="copied" class="w-4 h-4 text-green-500" />
            <Copy v-else class="w-4 h-4" />
          </button>
        </div>

        <div class="font-mono text-lg text-zinc-100 break-all pr-12">
          {{ result }}
        </div>
      </div>
    </div>

    <!-- Error Display -->
    <div v-if="error" class="bg-red-500/10 border border-red-500/20 rounded-xl p-4 text-red-400 text-sm">
      {{ error }}
    </div>
  </div>

  <div v-else class="flex items-center justify-center h-64">
    <Loader2 class="w-8 h-8 text-blue-500 animate-spin" />
  </div>
</template>
