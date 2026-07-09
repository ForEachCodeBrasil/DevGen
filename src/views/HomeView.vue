<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'


import {
  Search,
  Settings,
  ChevronRight,
  Loader2,
  FileText,
  User,
  Building2,
  Car,
  Wrench,
  Grid2X2
} from 'lucide-vue-next'

interface GeneratorDefinition {
  id: string
  name: string
  category: string
  description: string
  requires_pro: boolean
}

const router = useRouter()
const searchQuery = ref('')
const generators = ref<GeneratorDefinition[]>([])
const selectedTab = ref<string>('All')
const isLoading = ref(true)
const loadError = ref<string | null>(null)

const getCategoryIcon = (category: string) => {
  switch (category.toLowerCase()) {
    case 'documents': return FileText
    case 'person': return User
    case 'company': return Building2
    case 'vehicle': return Car
    case 'utilities': return Wrench
    default: return Grid2X2
  }
}

const categories = computed(() => {
  const cats = new Set(generators.value.map(g => g.category))
  return ['All', ...Array.from(cats).sort()]
})

// Free utilities first (habit), then Pro conversion drivers
const PRIORITY_ORDER = [
  'uuid', 'password', 'lorem_ipsum', 'random_number', 'nick', 'qrcode', 'name',
  'cpf', 'cnpj', 'cep', 'person', 'company', 'rg', 'cnh', 'pis',
  'titulo_eleitor', 'bank_account', 'credit_card', 'vehicle', 'vehicle_plate'
]

const filteredGenerators = computed(() => {
  let results = generators.value.filter(g => {
    const matchesSearch =
      g.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      g.description.toLowerCase().includes(searchQuery.value.toLowerCase())
    const matchesTab =
      selectedTab.value === 'All' || g.category === selectedTab.value
    return matchesSearch && matchesTab
  })

  if (selectedTab.value === 'All') {
    results.sort((a, b) => {
      const aIdx = PRIORITY_ORDER.indexOf(a.id)
      const bIdx = PRIORITY_ORDER.indexOf(b.id)
      if (aIdx === -1 && bIdx === -1) return a.name.localeCompare(b.name)
      if (aIdx === -1) return 1
      if (bIdx === -1) return -1
      return aIdx - bIdx
    })
  }

  return results
})

async function loadGenerators() {
  isLoading.value = true
  loadError.value = null

  try {
    generators.value = await invoke('list_generators')
  } catch (e) {
    console.error('Failed to list generators', e)
    loadError.value = 'Não foi possível carregar os geradores.'
  } finally {
    isLoading.value = false
  }
}

function selectGenerator(id: string) {
  router.push({ name: 'generator', params: { id } })
}

async function quit() {
  await invoke('exit_app')
}

onMounted(() => loadGenerators())
</script>

<template>
  <div class="flex flex-col h-screen overflow-hidden bg-deep-space">
    <!-- Header -->
    <div
      class="flex items-center justify-between px-4 pt-5 pb-3 border-b border-border-dark bg-dark-surface/50"
      data-tauri-drag-region
    >
      <div class="flex items-center gap-2">
        <div
          class="w-6 h-6 rounded-sm bg-neon-green/10 border border-neon-green/30 flex items-center justify-center text-neon-green"
        >
          <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="m18 16 4-4-4-4" /><path d="m6 8-4 4 4 4" /><path d="m14.5 4-5 16" />
          </svg>
        </div>
        <span class="text-sm font-bold tracking-tight text-gray-100">DEV<span class="text-neon-green">GEN</span></span>
      </div>
      <span
        class="badge badge-success"
      >
        Stable v1.0
      </span>
    </div>

    <!-- Tabs -->
    <div
      class="flex gap-2 px-4 py-3 border-b border-border-dark overflow-x-auto no-scrollbar shrink-0"
      data-tauri-drag-region
    >
      <button
        v-for="cat in categories"
        :key="cat"
        @click="selectedTab = cat"
        class="px-3 py-1 text-xs font-medium rounded-sm border transition-all duration-200"
        :class="[
          selectedTab === cat
            ? 'bg-neon-green/10 border-neon-green/50 text-neon-green shadow-[0_0_10px_rgba(34,197,94,0.2)]'
            : 'bg-transparent border-transparent text-gray-400 hover:text-gray-200 hover:bg-white/5'
        ]"
      >
        {{ cat }}
      </button>
    </div>

    <!-- Search -->
    <div class="px-4 py-3 shrink-0">
      <div class="relative group">
        <Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-500 group-focus-within:text-neon-green transition-colors" />
        <input
          v-model="searchQuery"
          type="text"
          placeholder="SEARCH GENERATORS..."
          class="w-full bg-dark-surface border border-gray-800 rounded-sm pl-9 pr-4 py-2 text-sm text-gray-200 placeholder-gray-600 focus:outline-none focus:border-neon-green/50 focus:ring-1 focus:ring-neon-green/50 transition-all font-mono"
        />
      </div>
    </div>

    <!-- Generator list -->
    <div class="flex-1 overflow-y-auto px-4 pb-2 min-h-0 space-y-1">
      <div v-if="isLoading" class="flex justify-center py-12">
        <Loader2 class="w-6 h-6 text-neon-green animate-spin" />
      </div>
      
      <div
        v-else-if="loadError"
        class="p-4 rounded-sm border border-border-dark bg-dark-surface text-center space-y-3"
      >
        <p class="text-sm text-red-400 font-mono">{{ loadError }}</p>
        <button
          @click="loadGenerators"
          class="btn btn-secondary btn-sm"
        >
          Retry Connection
        </button>
      </div>
      
      <div
        v-else-if="filteredGenerators.length === 0"
        class="p-8 text-center text-gray-500 font-mono text-sm border border-dashed border-gray-800 rounded-sm"
      >
        NO_MATCH_FOUND
      </div>
      
      <button
        v-for="gen in filteredGenerators"
        :key="gen.id"
        @click="selectGenerator(gen.id)"
        class="w-full flex items-center justify-between p-3 rounded-sm border border-transparent hover:border-neon-green/30 hover:bg-dark-surface/80 group transition-all duration-200"
      >
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-sm bg-gray-900 group-hover:bg-black/50 text-gray-500 group-hover:text-neon-green transition-colors">
            <component :is="getCategoryIcon(gen.category)" class="w-4 h-4" />
          </div>
          <div class="text-left">
            <div class="text-sm font-medium text-gray-200 group-hover:text-white font-mono">{{ gen.name }}</div>
            <div class="text-xs text-gray-500 group-hover:text-gray-400">{{ gen.description }}</div>
          </div>
        </div>
        
        <div class="flex items-center gap-3">
          <span
            v-if="gen.requires_pro"
            class="text-[10px] uppercase tracking-wider px-1.5 py-0.5 rounded-sm bg-warning-amber/10 text-warning-amber border border-warning-amber/20 font-bold"
          >
            PRO
          </span>
          <ChevronRight class="w-4 h-4 text-gray-700 group-hover:text-neon-green group-hover:translate-x-0.5 transition-all" />
        </div>
      </button>
    </div>

    <!-- Footer menu -->
    <div
      class="border-t border-border-dark px-2 py-2 flex items-center justify-between shrink-0 bg-dark-surface/30"
      data-tauri-drag-region
    >
      <button
        @click="router.push('/settings')"
        class="flex items-center gap-2 px-3 py-2 rounded-sm text-xs font-mono text-gray-500 hover:text-neon-green hover:bg-white/5 transition-colors"
      >
        <Settings class="w-3.5 h-3.5" />
        SYS_CONFIG
      </button>



      <button
        @click="quit"
        class="flex items-center gap-2 px-3 py-2 rounded-sm text-xs font-mono text-gray-500 hover:text-error-red hover:bg-white/5 transition-colors"
      >
        EXIT
      </button>
    </div>
  </div>
</template>

<style scoped>
.no-scrollbar::-webkit-scrollbar {
  display: none;
}
.no-scrollbar {
  -ms-overflow-style: none;
  scrollbar-width: none;
}
</style>
