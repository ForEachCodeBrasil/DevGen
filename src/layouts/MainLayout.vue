<script setup lang="ts">
import Sidebar from '../components/Sidebar.vue';
</script>

<template>
  <div class="app-layout">
    <Sidebar />
    <main class="content-area">
      <slot></slot>
    </main>
  </div>
</template>

<style scoped>
.app-layout {
  display: flex;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
}

.content-area {
  flex: 1;
  overflow-y: auto;
  padding: 2rem;
  background-color: var(--bg-color, #fff);
}

@media (prefers-color-scheme: dark) {
  .content-area {
    background-color: #121212;
    color: #e0e0e0;
  }
}
</style>
