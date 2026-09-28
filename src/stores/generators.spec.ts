import { beforeEach, describe, expect, it, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import { useGeneratorsStore } from './generators'
import type { GenerateRequest, GeneratorDefinition } from '../types'

vi.mock('@tauri-apps/api/core', () => ({
    invoke: vi.fn()
}))

const invokeMock = vi.mocked(invoke)

describe('useGeneratorsStore', () => {
    beforeEach(() => {
        setActivePinia(createPinia())
        invokeMock.mockReset()
    })

    it('fetches generators from backend', async () => {
        const defs: GeneratorDefinition[] = [
            {
                id: 'cpf',
                name: 'CPF',
                category: 'documents',
                description: 'Gera CPF'
            },
            {
                id: 'person',
                name: 'Pessoa',
                category: 'person',
                description: 'Gera pessoa'
            }
        ]
        invokeMock.mockResolvedValueOnce(defs)

        const store = useGeneratorsStore()
        await store.fetchGenerators()

        expect(invokeMock).toHaveBeenCalledWith('list_generators')
        expect(store.generators).toEqual(defs)
        expect(store.error).toBeNull()
        expect(store.loading).toBe(false)
    })

    it('returns unique categories preserving appearance order', () => {
        const store = useGeneratorsStore()
        store.generators = [
            { id: '1', name: 'A', category: 'documents', description: '' },
            { id: '2', name: 'B', category: 'person', description: '' },
            { id: '3', name: 'C', category: 'documents', description: '' }
        ]

        expect(store.categories).toEqual(['documents', 'person'])
    })

    it('wraps backend errors in generate action', async () => {
        const store = useGeneratorsStore()
        const request: GenerateRequest = {
            generator_id: 'cpf',
            options: { mask: true }
        }
        invokeMock.mockRejectedValueOnce(new Error('boom'))

        await expect(store.generate(request)).rejects.toThrow('boom')
        expect(invokeMock).toHaveBeenCalledWith('generate', { req: request })
    })
})
