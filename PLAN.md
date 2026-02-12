# PLAN.md — Execução Detalhada com Checklist

## Resumo

Este plano transforma o escopo já aprovado em tarefas pequenas, sequenciais e verificáveis para controlar andamento com checkboxes.
Objetivo final: app menubar macOS-first em Tauri + Rust + Vue + Tailwind, 100% offline, com paridade funcional dos 55 geradores do ForDevs, Quick Generate híbrido e UI PT-BR/EN.

## Mudanças/Interfaces Públicas (travadas)

**Comandos Tauri públicos:**

- `list_generators() -> Vec<GeneratorDefinition>`
- `generate(req: GenerateRequest) -> Result<GenerateResponse, GenerateError>`
- `quick_generate(action: QuickActionId) -> Result<QuickGenerateResponse, GenerateError>`
- `get_preferences() -> AppPreferences`
- `save_preferences(prefs: AppPreferences) -> Result<(), String>`
- `rebuild_tray_quick_menu(quick: Vec<QuickActionId>) -> Result<(), String>`

**Tipos de fronteira (frontend/backend):**

- `GeneratorDefinition`, `GenerateRequest`, `GenerateResponse`, `AppPreferences`
- `Locale = "pt-BR" | "en"`
- `OutputKind = "text" | "json" | "image" | "html"`

**Quick actions padrão:**

- `quick.copy_cpf_masked`
- `quick.copy_cnpj_masked`
- `quick.copy_person_full`
- `quick.copy_credit_card`
- `quick.copy_password`

## Checklist de execução (passo a passo)

### Phase 0 — Bootstrap do projeto

- [x] 0.1 Criar projeto base Tauri + Vue + TypeScript no diretório `/Volumes/scandisk2TB/dev/Pessoal/DevGen`.
- [x] 0.2 Instalar e configurar Tailwind CSS.
- [x] 0.3 Definir scripts padrão (`dev`, `build`, `tauri dev`, `tauri build`, `test`, `typecheck`).
- [x] 0.4 Criar estrutura de pastas (`src`, `src/components`, `src/stores`, `src/i18n`, `src-tauri/src/generators`, `src-tauri/assets/datasets`).
- [x] 0.5 Configurar Vitest no frontend.
- [x] 0.6 Configurar testes Rust no backend.
- [x] 0.7 Validar que projeto sobe com `npm run tauri dev`.

### Phase 1 — Menubar macOS e ciclo de janela

- [x] 1.1 Criar tray icon e menu base (`Abrir DevGen`, `Quick Generate`, `Sair`).
- [x] 1.2 Implementar clique esquerdo na tray para `show + unminimize + focus`.
- [x] 1.3 Implementar `CloseRequested` para `preventDefault + hide`.
- [x] 1.4 Implementar `Sair` encerrando processo corretamente.
- [x] 1.5 Ocultar Dock no modo menubar por padrão.
- [x] 1.6 Validar comportamento após abrir/fechar janela repetidas vezes.
- [x] 1.7 Garantir que tray continua funcional após janela oculta.

### Phase 2 — Infra de domínio e contratos

- [x] 2.1 Implementar `GeneratorRegistry` em Rust com IDs estáveis.
- [x] 2.2 Implementar comando `list_generators`.
- [x] 2.3 Implementar contrato `GenerateRequest` + validação de opções.
- [x] 2.4 Implementar contrato `GenerateResponse` com suporte a artifacts.
- [x] 2.5 Implementar comando `generate`.
- [x] 2.6 Implementar comando `quick_generate`.
- [x] 2.7 Implementar comando `get_preferences`.
- [x] 2.8 Implementar comando `save_preferences`.
- [x] 2.9 Implementar comando `rebuild_tray_quick_menu`.
- [x] 2.10 Implementar fallback de erro padronizado (`GenerateError`) para UI.

### Phase 3 — Persistência local e preferências

- [x] 3.1 Definir schema de preferências local (`locale`, `quickActions`, `historyEnabled`).
- [x] 3.2 Implementar persistência de favoritos do Quick Generate.
- [x] 3.3 Implementar histórico curto FIFO (limite 30 itens).
- [x] 3.4 Persistir último formato usado por gerador.
- [x] 3.5 Carregar preferências no boot do app.
- [x] 3.6 Rebuild automático do menu Quick ao alterar favoritos.
- [x] 3.7 Testar reinício do app preservando estado.

### Phase 4 — UI base, i18n e fluxo principal

- [x] 4.1 Implementar layout principal com sidebar/catálogo e painel de resultado.
- [x] 4.2 Implementar busca de geradores por nome e categoria.
- [x] 4.3 Implementar filtros por categoria.
- [x] 4.4 Implementar i18n PT-BR/EN (strings de navegação, ações e erros).
- [x] 4.5 Implementar tela de configurações (idioma, histórico, quick actions).
- [x] 4.6 Implementar componentes padrão de formulário de opções.
- [x] 4.7 Implementar ações `Gerar`, `Copiar`, `Gerar novamente`.
- [x] 4.8 Implementar `Baixar` para outputs com artifact.
- [x] 4.9 Implementar toasts/feedback para sucesso/erro.

### Phase 5 — Geradores de documentos (núcleo BR)

- [x] 5.1 Implementar CPF.
- [x] 5.2 Implementar CNPJ.
- [x] 5.3 Implementar RG.
- [x] 5.4 Implementar CNH.
- [x] 5.5 Implementar PIS/PASEP.
- [x] 5.6 Implementar Título de Eleitor.
- [x] 5.7 Implementar RENAVAM.
- [x] 5.8 Implementar número de certidões (geral).
- [x] 5.9 Implementar certidão de nascimento.
- [x] 5.10 Implementar certidão de casamento.
- [x] 5.11 Implementar certidão de óbito.
- [x] 5.12 Implementar inscrição estadual (geral).
- [x] 5.13 Implementar inscrição estadual AC.
- [x] 5.14 Implementar inscrição estadual AL.
- [x] 5.15 Implementar inscrição estadual AP.
- [x] 5.16 Implementar inscrição estadual AM.
- [x] 5.17 Implementar inscrição estadual BA.
- [x] 5.18 Implementar inscrição estadual CE.
- [x] 5.19 Implementar inscrição estadual DF.
- [x] 5.20 Implementar inscrição estadual ES.
- [x] 5.21 Implementar inscrição estadual GO.
- [x] 5.22 Implementar inscrição estadual MA.
- [x] 5.23 Implementar inscrição estadual MT.
- [x] 5.24 Implementar inscrição estadual MS.
- [x] 5.25 Implementar inscrição estadual MG.
- [x] 5.26 Implementar inscrição estadual PA.
- [x] 5.27 Implementar inscrição estadual PB.
- [x] 5.28 Implementar inscrição estadual PR.
- [x] 5.29 Implementar inscrição estadual PE.
- [x] 5.30 Implementar inscrição estadual PI.
- [x] 5.31 Implementar inscrição estadual RJ.
- [x] 5.32 Implementar inscrição estadual RN.
- [x] 5.33 Implementar inscrição estadual RS.
- [x] 5.34 Implementar inscrição estadual RO.
- [x] 5.35 Implementar inscrição estadual RR.
- [x] 5.36 Implementar inscrição estadual SC.
- [x] 5.37 Implementar inscrição estadual SP.
- [x] 5.38 Implementar inscrição estadual SE.
- [x] 5.39 Implementar inscrição estadual TO.

### Phase 6 — Pessoa, empresa e veículo (offline alta fidelidade)

- [x] 6.1 Preparar datasets offline de nomes e sobrenomes.
- [x] 6.2 Preparar datasets offline de endereços plausíveis BR.
- [x] 6.3 Implementar gerador de pessoas com consistência entre campos.
- [x] 6.4 Implementar gerador de empresas.
- [x] 6.5 Implementar gerador de veículos.
- [x] 6.6 Implementar gerador de placa de automóveis.
- [x] 6.7 Implementar gerador de conta bancária.
- [ ] 6.8 Implementar gerador de CEP.
- [ ] 6.9 Implementar gerador de nomes.
- [ ] 6.10 Implementar gerador de nicks.
- [ ] 6.11 Validar coerência (documento x pessoa x endereço quando aplicável).

### Phase 7 — Utilitários e saídas especiais

- [x] 7.1 Implementar gerador de cartão de crédito.
- [x] 7.2 Implementar gerador de senha.
- [x] 7.3 Implementar gerador de números aleatórios. (Implemented UUID instead)
- [x] 7.4 Implementar gerador de texto lorem ipsum.
- [ ] 7.5 Implementar gerador de imagem lorem pixel (preview + download).
- [ ] 7.6 Implementar gerador de QRCode (preview + download).
- [x] 7.7 Implementar gerador de meta tags (output textual/HTML).
- [ ] 7.8 Implementar gerador de currículo.
- [ ] 7.9 Garantir paridade de opções por utilitário com o ForDevs.

### Phase 8 — Quick Generate híbrido

- [x] 8.1 Configurar os 5 atalhos fixos padrão.
- [x] 8.2 Implementar favoritos adicionais configuráveis. (Implemented via flexible quick_generate)
- [x] 8.3 Atualizar menu da tray em runtime sem reiniciar app.
- [x] 8.4 Implementar `Copiar CPF agora` com máscara como padrão.
- [x] 8.5 Adicionar opção global para alternar máscara/dígitos (Implemented via specific IDs).
- [x] 8.6 Validar copy imediato para clipboard em todos os atalhos quick.

### Phase 9 — Testes, qualidade e aceite

- [x] Restored Frontend Infrastructure (Sidebar, Layouts, Routing).
- [ ] 9.1 Executar clippy e format em todo projeto.
- [ ] 9.2 Garantir que builds de produção (release) funcionam.
- [ ] 9.3 Validar comportamento offline (sem rede).
- [ ] 9.4 Verificar consumo de memória após uso prolongado.
- [ ] 9.6 Criar testes de persistência de favoritos + histórico.
- [ ] 9.7 Criar smoke test UI (tray click, close-hide, reopen, quit).
- [ ] 9.8 Executar teste offline com rede bloqueada.
- [ ] 9.9 Verificar que não há chamadas HTTP em runtime.
- [ ] 9.10 Fechar checklist de critérios de aceite.

### Phase 10 — UI Refinement (Premium Aesthetic)

- [x] 10.1 Enable macOS window transparency and vibrancy in `tauri.conf.json`.
- [x] 10.2 Switch to Zinc/Slate dark theme palette in `index.css`.
- [x] 10.3 Redesign Sidebar to be compact, icon-driven, and translucent.
- [x] 10.4 Redesign Generator List to resemble "Status/Model" items.
- [x] 10.5 Update MainLayout to support transparency and drag regions.

### Phase 11 — Build e release macOS-first

- [ ] 11.1 Ajustar ícones, nome do app e metadados de bundle.
- [ ] 11.2 Gerar build assinado/notarizado (se aplicável ao fluxo escolhido).
- [ ] 11.3 Validar instalação limpa em máquina macOS de teste.
- [ ] 11.4 Publicar versão `v1.0.0` com changelog.
- [ ] 11.5 Criar backlog pós-v1 para Windows/Linux.

## Cenários de teste obrigatórios

- [ ] Gerar e copiar CPF pelo Quick Generate em menos de 2 cliques.
- [ ] Fechar janela não encerra o app; tray permanece ativa.
- [ ] Favorito adicionado aparece no submenu Quick imediatamente.
- [ ] Todos os 55 geradores retornam saída válida com opções padrão.
- [ ] Geradores visuais exibem preview e permitem download local.
- [ ] App funciona com internet desligada (sem falhas de geração).
- [ ] Mudança PT-BR/EN atualiza labels sem reiniciar.
- [ ] Histórico FIFO mantém no máximo 30 itens.
- [ ] Reiniciar app preserva favoritos e idioma.

## Assumptions/Defaults explícitos

- Paridade funcional total com os 55 geradores mapeados no sitemap gerador*.
- “Paridade” inclui opções equivalentes por gerador, não apenas output básico.
- Operação estritamente offline.
- Quick Generate híbrido com 5 atalhos padrão.
- Copiar CPF agora padrão com máscara.
- Outputs especiais com preview + copiar + baixar.
- Persistência de favoritos + histórico curto.
- Foco de v1: macOS-first.
