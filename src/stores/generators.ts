import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import type { GeneratorDefinition, GenerateRequest, GenerateResponse } from '../types'

export const useGeneratorsStore = defineStore('generators', {
    state: () => ({
        generators: [] as GeneratorDefinition[],
        loading: false,
        error: null as string | null
    }),
    getters: {
        getGeneratorById: (state) => (id: string) => state.generators.find(g => g.id === id),
        categories: (state) => [...new Set(state.generators.map(g => g.category))]
    },
    actions: {
        async fetchGenerators() {
            this.loading = true
            try {
                this.generators = await invoke('list_generators')
            } catch (e: any) {
                this.error = e.toString()
            } finally {
                this.loading = false
            }
        },
        async generate(request: GenerateRequest): Promise<GenerateResponse> {
            try {
                return await invoke('generate', { req: request })
            } catch (e: any) {
                throw new Error(e.message || e.toString())
            }
        }
    }
})
