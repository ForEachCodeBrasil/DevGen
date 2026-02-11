<script setup lang="ts">
import { onMounted, computed, ref } from 'vue'
import { useGeneratorsStore } from '../stores/generators'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { Search } from 'lucide-vue-next'

const store = useGeneratorsStore()
const router = useRouter()
const { t } = useI18n()

const searchQuery = ref('')
const selectedCategory = ref<string | null>(null)

onMounted(() => {
  store.fetchGenerators()
})

const filteredGenerators = computed(() => {
  return store.generators.filter(g => {
    const matchesSearch = g.name.toLowerCase().includes(searchQuery.value.toLowerCase()) || 
                          g.description.toLowerCase().includes(searchQuery.value.toLowerCase())
    const matchesCategory = selectedCategory.value ? g.category === selectedCategory.value : true
    return matchesSearch && matchesCategory
  })
})

const categories = computed(() => store.categories)

function openGenerator(id: string) {
  router.push(`/generator/${id}`)
}
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center space-x-4 bg-zinc-800/50 p-4 rounded-xl border border-white/5 backdrop-blur-sm sticky top-0 z-10">
      <Search class="w-5 h-5 text-zinc-400" />
      <input 
        v-model="searchQuery" 
        type="text" 
        :placeholder="t('nav.search_placeholder')"
        class="bg-transparent border-none focus:ring-0 text-white w-full placeholder-zinc-500"
      >
    </div>

    <div class="flex space-x-2 overflow-x-auto pb-2">
      <button 
        @click="selectedCategory = null"
        :class="[
          'px-4 py-1.5 rounded-full text-sm font-medium transition-colors whitespace-nowrap',
          selectedCategory === null 
            ? 'bg-blue-600 text-white' 
            : 'bg-zinc-800 text-zinc-400 hover:bg-zinc-700 hover:text-white'
        ]"
      >
        Todos
      </button>
      <button 
        v-for="cat in categories" 
        :key="cat"
        @click="selectedCategory = cat"
        :class="[
          'px-4 py-1.5 rounded-full text-sm font-medium transition-colors whitespace-nowrap',
          selectedCategory === cat 
            ? 'bg-blue-600 text-white' 
            : 'bg-zinc-800 text-zinc-400 hover:bg-zinc-700 hover:text-white'
        ]"
      >
        {{ cat }}
      </button>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      <div 
        v-for="gen in filteredGenerators" 
        :key="gen.id"
        @click="openGenerator(gen.id)"
        class="group p-5 rounded-xl bg-zinc-900 border border-white/5 hover:border-blue-500/50 hover:bg-zinc-800/50 transition-all cursor-pointer relative overflow-hidden"
      >
        <div class="absolute top-0 left-0 w-1 h-full bg-blue-500 opacity-0 group-hover:opacity-100 transition-opacity"></div>
        <h3 class="font-semibold text-zinc-100 mb-1 group-hover:text-blue-400 transition-colors">{{ gen.name }}</h3>
        <p class="text-sm text-zinc-400 line-clamp-2">{{ gen.description }}</p>
        <div class="mt-4 flex items-center justify-between">
           <span class="text-xs text-zinc-500 font-mono bg-zinc-950 px-2 py-1 rounded">{{ gen.category }}</span>
        </div>
      </div>
    </div>
  </div>
</template>
