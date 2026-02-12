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
}

const router = useRouter()
const searchQuery = ref('')
const generators = ref<GeneratorDefinition[]>([])
const selectedTab = ref<string>('All')
const isLoading = ref(true)

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

const filteredGenerators = computed(() => {
  return generators.value.filter(g => {
    const matchesSearch =
      g.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      g.description.toLowerCase().includes(searchQuery.value.toLowerCase())
    const matchesTab =
      selectedTab.value === 'All' || g.category === selectedTab.value
    return matchesSearch && matchesTab
  })
})

async function loadGenerators() {
  try {
    generators.value = await invoke('list_generators')
  } catch (e) {
    console.error('Failed to list generators', e)
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
  <div class="flex flex-col h-screen overflow-hidden panel-bg">
    <!-- Header with title bar drag -->
    <div
      class="flex items-center justify-between px-4 pt-12 pb-2 border-b border-white/5 flex-shrink-0"
      data-tauri-drag-region
    >
      <div class="flex items-center gap-2">
        <div
          class="w-6 h-6 rounded-md bg-blue-500/20 border border-blue-500/30 flex items-center justify-center"
        >
          <svg class="w-3 h-3 text-blue-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="m18 16 4-4-4-4" /><path d="m6 8-4 4 4 4" /><path d="m14.5 4-5 16" />
          </svg>
        </div>
        <span class="text-[13px] font-semibold text-zinc-100">DevGen</span>
      </div>
      <span
        class="text-[9px] px-1.5 py-0.5 rounded bg-green-500/15 text-green-400 font-medium"
      >
        Offline
      </span>
    </div>

    <!-- Tabs (CodexBar-style) -->
    <div
      class="flex gap-0.5 px-3 py-2 border-b border-white/5 overflow-x-auto no-scrollbar flex-shrink-0"
      data-tauri-drag-region
    >
      <button
        v-for="cat in categories"
        :key="cat"
        @click="selectedTab = cat"
        :class="[
          'px-2.5 py-1 rounded-[5px] text-[11px] font-medium whitespace-nowrap transition-all duration-150',
          selectedTab === cat
            ? 'bg-blue-500/30 text-blue-200 border border-blue-500/40'
            : 'text-zinc-500 hover:text-zinc-300 hover:bg-white/5 border border-transparent'
        ]"
      >
        {{ cat }}
      </button>
    </div>

    <!-- Search -->
    <div class="px-3 py-2 flex-shrink-0">
      <div class="relative">
        <Search class="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-zinc-500" />
        <input
          v-model="searchQuery"
          type="text"
          placeholder="Search generators..."
          class="w-full bg-white/5 border border-white/5 rounded-[6px] pl-8 pr-3 py-1.5 text-[11px] text-zinc-300 placeholder-zinc-600 focus:outline-none focus:border-white/10 transition-colors"
        />
      </div>
    </div>

    <!-- Generator list -->
    <div class="flex-1 overflow-y-auto px-3 pb-2 min-h-0">
      <div v-if="isLoading" class="flex justify-center py-8">
        <Loader2 class="w-5 h-5 text-zinc-500 animate-spin" />
      </div>
      <div v-else class="space-y-0.5">
        <button
          v-for="gen in filteredGenerators"
          :key="gen.id"
          @click="selectGenerator(gen.id)"
          class="w-full flex items-center gap-2 px-2.5 py-2 rounded-[6px] text-left transition-all duration-150 group hover:bg-white/5"
        >
          <component
            :is="getCategoryIcon(gen.category)"
            class="w-3.5 h-3.5 text-zinc-500 group-hover:text-zinc-400 shrink-0"
          />
          <span class="text-[12px] text-zinc-300 group-hover:text-zinc-100 flex-1 truncate">
            {{ gen.name }}
          </span>
          <ChevronRight class="w-3.5 h-3.5 text-zinc-600 opacity-0 group-hover:opacity-100 transition-opacity" />
        </button>
      </div>
    </div>

    <!-- Footer menu (CodexBar-style) -->
    <div
      class="border-t border-white/5 px-3 py-2 space-y-0.5 flex-shrink-0"
      data-tauri-drag-region
    >
      <button
        @click="router.push('/settings')"
        class="w-full flex items-center gap-2 px-2.5 py-2 rounded-[6px] text-left text-[12px] text-zinc-400 hover:bg-white/5 hover:text-zinc-200 transition-all duration-150"
      >
        <Settings class="w-3.5 h-3.5" />
        Settings...
      </button>
      <button
        @click="quit"
        class="w-full flex items-center gap-2 px-2.5 py-2 rounded-[6px] text-left text-[12px] text-zinc-400 hover:bg-white/5 hover:text-zinc-200 transition-all duration-150"
      >
        Quit
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
.panel-bg {
  background: rgba(0, 0, 0, 0.25);
  backdrop-filter: blur(40px);
  -webkit-backdrop-filter: blur(40px);
}
</style>
