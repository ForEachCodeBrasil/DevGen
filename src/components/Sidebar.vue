<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';

interface GeneratorDefinition {
  id: string;
  name: string;
  category: string;
  description: string;
}

const router = useRouter();
const searchQuery = ref('');
const generators = ref<GeneratorDefinition[]>([]);
const selectedCategory = ref<string | null>(null);

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
  <aside class="sidebar">
    <div class="sidebar-header">
      <h2>DevGen</h2>
      <input 
        v-model="searchQuery" 
        type="text" 
        placeholder="Buscar geradores..." 
        class="search-input"
      />
    </div>

    <div class="categories">
      <button 
        :class="{ active: selectedCategory === null }"
        @click="selectedCategory = null"
      >
        Todos
      </button>
      <button 
        v-for="cat in categories" 
        :key="cat"
        :class="{ active: selectedCategory === cat }"
        @click="selectedCategory = cat"
      >
        {{ cat }}
      </button>
    </div>

    <div class="generator-list">
      <div 
        v-for="gen in filteredGenerators" 
        :key="gen.id"
        class="generator-item"
        @click="selectGenerator(gen.id)"
      >
        <span class="gen-name">{{ gen.name }}</span>
        <span class="gen-desc">{{ gen.description }}</span>
      </div>
    </div>
    
    <div class="sidebar-footer">
        <button @click="router.push('/settings')">Configurações</button>
    </div>
  </aside>
</template>

<style scoped>
.sidebar {
  width: 250px;
  background-color: #f0f0f0;
  border-right: 1px solid #ddd;
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.sidebar-header {
  padding: 1rem;
  border-bottom: 1px solid #e0e0e0;
}

.search-input {
  width: 100%;
  padding: 0.5rem;
  margin-top: 0.5rem;
  border: 1px solid #ccc;
  border-radius: 4px;
}

.categories {
  padding: 0.5rem;
  display: flex;
  gap: 0.5rem;
  overflow-x: auto;
  border-bottom: 1px solid #e0e0e0;
}

.categories button {
  padding: 0.25rem 0.5rem;
  font-size: 0.8rem;
  background: none;
  border: none;
  cursor: pointer;
  opacity: 0.7;
}

.categories button.active {
  font-weight: bold;
  opacity: 1;
  border-bottom: 2px solid #396cd8;
}

.generator-list {
  flex: 1;
  overflow-y: auto;
  padding: 0.5rem;
}

.generator-item {
  padding: 0.75rem;
  cursor: pointer;
  border-radius: 4px;
  margin-bottom: 0.5rem;
  display: flex;
  flex-direction: column;
}

.generator-item:hover {
  background-color: #e0e0e0;
}

.gen-name {
  font-weight: bold;
  font-size: 0.9rem;
}

.gen-desc {
  font-size: 0.75rem;
  color: #666;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.sidebar-footer {
    padding: 1rem;
    border-top: 1px solid #e0e0e0;
}

/* Dark mode support */
@media (prefers-color-scheme: dark) {
  .sidebar {
    background-color: #1e1e1e;
    border-right-color: #333;
  }
  
  .sidebar-header, .categories, .sidebar-footer {
    border-color: #333;
  }
  
  .search-input {
    background-color: #2d2d2d;
    border-color: #444;
    color: #fff;
  }
  
  .generator-item:hover {
    background-color: #2d2d2d;
  }
  
  .gen-desc {
    color: #aaa;
  }
  
  .categories button {
      color: #eee;
  }
}
</style>
