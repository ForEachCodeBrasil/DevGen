<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import {
  Search,
  Settings,
  FileText,
  User,
  Building2,
  Car,
  Wrench,
  Grid2X2,
  ChevronRight
} from 'lucide-vue-next';

interface GeneratorDefinition {
  id: string;
  name: string;
  category: string;
  description: string;
}

const router = useRouter();
const route = useRoute();
const searchQuery = ref('');
const generators = ref<GeneratorDefinition[]>([]);
const selectedCategory = ref<string | null>(null);

// Map categories to icons
const getCategoryIcon = (category: string) => {
  switch (category.toLowerCase()) {
    case 'documents': return FileText;
    case 'person': return User;
    case 'company': return Building2;
    case 'vehicle': return Car;
    case 'utilities': return Wrench;
    default: return Grid2X2;
  }
};

const categories = computed(() => {
  const cats = new Set(generators.value.map(g => g.category));
  return Array.from(cats).sort();
});

const filteredGenerators = computed(() => {
  return generators.value.filter(g => {
    const matchesSearch = g.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      g.description.toLowerCase().includes(searchQuery.value.toLowerCase());
    const matchesCategory = selectedCategory.value ? g.category === selectedCategory.value : true;
    return matchesSearch && matchesCategory;
  });
});

async function loadGenerators() {
  try {
    generators.value = await invoke('list_generators');
  } catch (e) {
    console.error('Failed to list generators', e);
  }
}

function selectGenerator(id: string) {
  router.push({ name: 'generator', params: { id } });
}

onMounted(() => {
  loadGenerators();
});
</script>

<template>
  <aside class="w-64 flex flex-col h-full border-r border-white/5 bg-zinc-950/30 backdrop-blur-sm"
    data-tauri-drag-region>

    <!-- Header / Drag Region -->
    <div class="px-4 pt-6 pb-4 flex items-center justify-between pointer-events-none" data-tauri-drag-region>
      <div class="flex items-center gap-2">
        <div class="w-3 h-3 rounded-full bg-red-500/80"></div>
        <div class="w-3 h-3 rounded-full bg-yellow-500/80"></div>
        <div class="w-3 h-3 rounded-full bg-green-500/80"></div>
      </div>
    </div>

    <!-- Search -->
    <div class="px-3 mb-4">
      <div class="relative group">
        <Search
          class="absolute left-2.5 top-2 w-4 h-4 text-zinc-500 group-focus-within:text-blue-400 transition-colors" />
        <input v-model="searchQuery" type="text" placeholder="Search..."
          class="w-full bg-black/20 border border-white/5 rounded-lg pl-9 pr-3 py-1.5 text-sm text-zinc-300 placeholder-zinc-600 focus:outline-none focus:ring-1 focus:ring-blue-500/50 focus:border-blue-500/50 transition-all" />
      </div>
    </div>

    <!-- Category Tabs -->
    <div class="px-3 mb-2 flex gap-1 overflow-x-auto no-scrollbar pb-2">
      <button @click="selectedCategory = null" :class="[
        'px-2.5 py-1 rounded text-xs font-medium whitespace-nowrap transition-colors',
        selectedCategory === null
          ? 'bg-blue-600/20 text-blue-400 border border-blue-500/20'
          : 'text-zinc-500 hover:text-zinc-300 hover:bg-white/5'
      ]">
        All
      </button>
      <button v-for="cat in categories" :key="cat" @click="selectedCategory = cat" :class="[
        'px-2.5 py-1 rounded text-xs font-medium whitespace-nowrap transition-colors',
        selectedCategory === cat
          ? 'bg-blue-600/20 text-blue-400 border border-blue-500/20'
          : 'text-zinc-500 hover:text-zinc-300 hover:bg-white/5'
      ]">
        {{ cat }}
      </button>
    </div>

    <!-- Generator List -->
    <div class="flex-1 overflow-y-auto px-2 space-y-0.5">
      <div v-for="gen in filteredGenerators" :key="gen.id" @click="selectGenerator(gen.id)" :class="[
        'group flex items-center justify-between px-3 py-2 rounded-lg cursor-pointer transition-all duration-200',
        route.params.id === gen.id
          ? 'bg-blue-600/10 border border-blue-500/20'
          : 'hover:bg-white/5 border border-transparent hover:border-white/5'
      ]">
        <div class="flex items-center gap-3 overflow-hidden">
          <component :is="getCategoryIcon(gen.category)"
            class="w-4 h-4 text-zinc-500 group-hover:text-blue-400 transition-colors flex-shrink-0" />
          <div class="flex flex-col overflow-hidden">
            <span :class="[
              'text-sm font-medium truncate transition-colors',
              route.params.id === gen.id ? 'text-blue-100' : 'text-zinc-300 group-hover:text-white'
            ]">
              {{ gen.name }}
            </span>
          </div>
        </div>

        <ChevronRight :class="[
          'w-3 h-3 text-zinc-600 transition-transform duration-200',
          route.params.id === gen.id ? 'text-blue-500' : 'group-hover:translate-x-0.5 group-hover:text-zinc-400 opacity-0 group-hover:opacity-100'
        ]" />
      </div>
    </div>

    <!-- Footer -->
    <div class="p-3 border-t border-white/5 mt-auto">
      <button @click="router.push('/settings')"
        class="flex items-center gap-3 w-full px-3 py-2 rounded-lg text-zinc-400 hover:text-white hover:bg-white/5 transition-colors">
        <Settings class="w-4 h-4" />
        <span class="text-sm font-medium">Settings</span>
      </button>
    </div>
  </aside>
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
