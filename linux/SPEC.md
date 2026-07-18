# Especificação — Mini Keyboard Programmer nativo para Linux

Status: proposta de implementação clean-room baseada em comportamento observável, artefatos distribuídos pelo fabricante e descritores USB/HID do hardware adquirido.

## 1. Objetivo

Criar uma aplicação Linux nativa para configurar os macro keyboards USB atualmente atendidos pelo aplicativo `MINI_KEYBOARD.exe`, sem Wine, sem empacotar binários Windows e sem reutilizar código do fabricante.

A aplicação deve:

- descobrir o dispositivo por USB VID/PID e selecionar somente sua interface de configuração;
- identificar automaticamente a geometria do teclado;
- ler a configuração armazenada no dispositivo;
- editar teclas, macros, multimídia, mouse, atraso, camadas e LED/RGB;
- gravar somente as posições modificadas;
- verificar a gravação por releitura;
- importar e exportar perfis locais;
- funcionar sem `root`, com uma regra udev restrita ao dispositivo.

## 2. Contrato clean-room

### 2.1 Fontes permitidas

- descritores USB e HID expostos pelo próprio dispositivo;
- tráfego USB/HID produzido ao operar legalmente o hardware e o aplicativo original;
- nomes, textos e comportamento visível da interface;
- documentação pública de USB HID, hidapi, Linux hidraw, GTK e Rust;
- este documento de especificação comportamental.

### 2.2 Restrições

A implementação não deve:

- copiar ou traduzir código decompilado do aplicativo original;
- incorporar `MINI_KEYBOARD.exe`, DLLs, objetos COFF ou o AppImage atual;
- reproduzir imagens, identidade visual ou recursos gráficos do fabricante;
- depender de Wine;
- liberar acesso global a todos os dispositivos `/dev/hidraw*`;
- habilitar escrita em dispositivos desconhecidos apenas porque usam HID.

### 2.3 Separação recomendada

1. **Equipe de protocolo:** coleta capturas, produz vetores de teste e atualiza somente esta especificação.
2. **Equipe de implementação:** recebe a especificação e os vetores, mas não consulta decompilação ou código do fabricante.
3. **Revisão:** confirma que cada implementação pode ser explicada por uma exigência desta especificação ou por documentação pública.

## 3. Evidência observada

### 3.1 Aplicativo distribuído

- Aplicativo principal: PE32/i386, Qt 5.14.2 e MinGW.
- Transporte: hidapi.
- O `MiniKeyboard-x86_64.AppImage` existente não é uma aplicação Linux nativa. Seu SquashFS contém o executável Windows, DLLs Qt para Windows, `hidapi.dll` e os mesmos objetos do pacote Windows.
- O aplicativo consulta conexão a cada aproximadamente 100 ms.

### 3.2 Hardware confirmado

Dispositivo conectado durante a análise:

| Campo | Valor |
|---|---|
| VID | `0x1189` |
| PID | `0x8842` |
| Produto USB | `USB Composite Device` |
| Serial observado | `CD70134330393835` |
| USB | 1.10, Full Speed |

O dispositivo possui duas interfaces HID:

| Interface | Finalidade | Endpoints |
|---|---|---|
| 0 | Configuração proprietária | OUT `0x04`, IN `0x84`, 64 bytes |
| 1 | Teclado, mouse e consumer control | IN `0x82`, 16 bytes |

A interface 0 usa:

- Usage Page `0xFF00`;
- Report ID `0x03`;
- Input de 64 bytes;
- Output de 64 bytes.

A aplicação deve abrir a interface **0**, nunca a interface de teclado 1.

### 3.3 IDs pesquisados pelo software original

VIDs observados:

- `0x1189`;
- `0x514C`.

PIDs observados:

- `0x8842`;
- `0x8840`;
- `0x8830`;
- `0x8831`;
- `0x8832`;
- `0x8833`;
- `0x8850`;
- `0x8851`.

Somente `0x1189:0x8842` foi validado fisicamente nesta análise. Os demais devem permanecer como `experimental` até validação em hardware.

## 4. Modelo funcional

### 4.1 Variantes reconhecidas

A resposta de identificação contém uma tupla de três bytes interpretada como quantidade de teclas, quantidade/variante de controles adicionais e subtipo. Foram observadas rotas de UI para:

- `0+1`, `0+2`, `0+3`;
- `1+0`, `2+0`;
- `3+0`, `3+1`;
- `4+0`, `4+1`, `4+2`, `4+3`;
- `5+0` com mute;
- `6+0`, `6+1`, `6+2`;
- `9+2`, `9+3`;
- `11+3`;
- `12+2`, `12+3`, `12+4`;
- `15+3` como fallback;
- `16+0`;
- `21+1`.

A geometria deve ser descrita por dados, não por telas ou classes específicas para cada modelo.

### 4.2 Camadas e posições

- 3 camadas configuráveis.
- Até 60 posições lógicas por camada.
- Cada posição ocupa um registro de 50 bytes no protocolo observado.
- O layout físico pode expor menos posições que o limite do protocolo.
- Cada posição possui dirty flag independente.

### 4.3 Tipos de ação obrigatórios

1. **Tecla básica:** qualquer usage de teclado suportado pelo firmware.
2. **Combinação:** modificadores Ctrl, Shift, Alt e GUI/Win combinados com uma tecla.
3. **Macro/sequência:** múltiplos eventos ou pares modificador/tecla dentro do limite do registro.
4. **Multimídia:**
   - play/pause;
   - stop;
   - faixa anterior;
   - próxima faixa;
   - mute;
   - volume +;
   - volume -;
   - calculadora.
5. **Mouse:**
   - botão esquerdo;
   - botão central;
   - botão direito;
   - roda + e -;
   - roda com Ctrl, Shift ou Alt.
6. **LED/RGB:** 6 modos observados e pelo menos 8 seleções de cor.
7. **Atraso:** valor associado à ação, representado por dois bytes no registro.
8. **Camada:** selecionar e editar as camadas 1, 2 e 3.
9. **Limpeza:** limpar uma posição ou todas as posições da camada corrente.

## 5. Protocolo HID

### 5.1 Transporte

- Backend Linux: hidapi/hidraw.
- Seleção: VID/PID permitido, Usage Page `0xFF00`, Report ID `0x03` e `interface_number == 0` quando disponível.
- Uma sessão por dispositivo.
- Toda escrita deve ser serializada.
- O worker de I/O deve possuir timeout e cancelamento por desconexão.

O software original escreve buffers de 65 bytes: 1 byte de Report ID mais 64 bytes de relatório. Ele solicita leituras de 64 bytes. Essa assimetria deve ser preservada inicialmente e registrada como uma questão de compatibilidade a confirmar em captura USB.

### 5.2 Consulta de identificação

Transmissão observada:

```text
03 FB FB FB 00 ... 00
```

- tamanho passado a hidapi: 65 bytes;
- leitura: até 64 bytes;
- timeout observado: 10 ms;
- os bytes de resposta nos índices 2, 3 e 4 determinam a variante física.

Requisitos:

- aceitar somente resposta com tamanho suficiente para os índices usados;
- rejeitar resposta vazia, truncada ou de outro Report ID;
- não assumir geometria quando a tupla for desconhecida;
- para tupla desconhecida, abrir em modo diagnóstico/read-only.

### 5.3 Leitura de configuração

Cabeçalho observado:

```text
03 FA <layer-or-count> <range> <block-index> 00 ... 00
```

Comportamento observado:

- a operação percorre as 3 camadas;
- cada comando é seguido por uma sequência de leituras HID;
- 49 bytes úteis de cada resposta são copiados para a área de registros;
- a UI original chama a leitura com parâmetros equivalentes a `(3, 15, 3)`.

A semântica exata dos bytes 2–4 ainda precisa ser fechada com captura USB. A implementação inicial deve encapsular esse comando em `ReadConfigCommand`, sem espalhar offsets pela aplicação.

### 5.4 Gravação de posição

Para cada posição dirty:

```text
byte 0      = 0x03              # Report ID
bytes 1–50  = registro da posição
bytes 51–64 = 0x00
```

- tamanho total: 65 bytes;
- posições limpas não devem ser enviadas;
- a ordem deve ser camada crescente e posição crescente;
- qualquer erro interrompe a transação antes do commit.

### 5.5 Commit/finalização

Após o último conjunto de posições:

```text
03 FD FE FF 00 ... 00
```

- tamanho total: 65 bytes;
- espera observada após o commit: 200 ms;
- depois do commit, a aplicação deve reler e comparar a configuração.

Não repetir automaticamente o commit após timeout: o firmware pode ter aplicado a transação apesar da ausência de confirmação.

### 5.6 Estrutura parcial do registro de 50 bytes

Offsets abaixo são observados; campos não confirmados permanecem opacos.

| Offset | Tamanho | Interpretação |
|---:|---:|---|
| 0 | 1 | marcador/comando; RGB usa `0xFE` |
| 1 | 1 | subtipo; RGB usa `0xB0` |
| 2 | 1 | camada baseada em 1 no comando RGB |
| 3 | 1 | modo/tipo principal da ação |
| 4–5 | 2 | atraso, ordem little-endian a confirmar |
| 9 | 1 | contador/comprimento da ação |
| 10–49 | 40 | dados da ação, pares e campos específicos |

RGB observado:

- byte 0: `0xFE`;
- byte 1: `0xB0`;
- byte 2: camada + 1;
- byte 9: `1`;
- byte 11: modo e cor compactados em nibbles.

Os demais bytes devem ser preservados quando uma configuração é lida e parcialmente editada. O encoder não pode zerar campos desconhecidos sem uma ação explícita de reset.

## 6. Modelo de domínio

```rust
struct DeviceIdentity {
    vid: u16,
    pid: u16,
    serial: Option<String>,
    path: String,
    interface_number: i32,
    usage_page: u16,
    usage: u16,
}

struct DeviceVariant {
    key_count: u8,
    extra_count: u8,
    subtype: u8,
    geometry_id: String,
    support: SupportLevel,
}

struct DeviceConfig {
    schema_version: u32,
    identity: ProfileIdentity,
    layers: [LayerConfig; 3],
    opaque_device_data: Vec<u8>,
}

struct LayerConfig {
    positions: Vec<PositionConfig>,
}

struct PositionConfig {
    logical_index: u8,
    action: Action,
    delay_ms: Option<u16>,
    raw_record: [u8; 50],
    dirty: bool,
}

enum Action {
    Empty,
    Keyboard { modifiers: Modifiers, usage: u8 },
    Sequence { strokes: Vec<Stroke> },
    Consumer { usage: u16 },
    Mouse { action: MouseAction, modifiers: Modifiers },
    Lighting { mode: u8, color: u8 },
    Opaque { mode: u8 },
}
```

Regras:

- `raw_record` é a fonte para round-trip lossless.
- `Action` é a visão semântica dos campos conhecidos.
- `Opaque` mantém configurações ainda não compreendidas.
- editar um campo conhecido deve alterar somente os bytes correspondentes.
- perfis salvos devem incluir VID, PID, tupla de variante e versão do schema.

## 7. Arquitetura Linux

### 7.1 Stack escolhida

- Linguagem: Rust estável, edition 2024.
- UI: GTK 4 + libadwaita.
- HID: crate `hidapi`, backend hidraw.
- Hotplug: udev/libudev ou refresh periódico do hidapi, isolado no módulo de transporte.
- Persistência: Serde JSON, escrita atômica.
- Logs: `tracing`, sem incluir conteúdo de macros por padrão.
- Empacotamento inicial: Flatpak e pacote nativo; AppImage pode ser adicional, mas não o único formato.

### 7.2 Processo

Um único processo, sem daemon privilegiado:

```text
GTK/libadwaita UI
       │ commands/events
       ▼
Application Controller
       │
       ├── Domain + validation
       ├── Profile persistence
       └── Dedicated HID worker thread
                 │
                 ├── discovery/open/close
                 ├── protocol encoder/decoder
                 └── hidapi/hidraw
```

A thread GTK nunca deve executar I/O HID. O worker recebe comandos por canal e devolve eventos tipados.

### 7.3 Módulos

```text
src/
  main.rs
  app.rs
  domain/
    action.rs
    config.rs
    geometry.rs
  device/
    discovery.rs
    session.rs
    worker.rs
  protocol/
    frame.rs
    identify.rs
    read_config.rs
    write_config.rs
    codec.rs
  profile/
    json.rs
    migration.rs
  ui/
    device_page.rs
    keyboard_editor.rs
    action_editor.rs
    status.rs
```

Começar com um único crate. Separar crates somente se o protocolo passar a ser consumido por CLI ou outra aplicação.

### 7.4 Estados da sessão

```text
Disconnected
  -> Opening
  -> Identifying
  -> ReadyClean | ReadyDirty | ReadOnlyUnknown
  -> Reading | Writing
  -> ReadyClean
  -> ErrorRecoverable | Disconnected
```

Invariantes:

- somente `ReadyDirty` permite gravar;
- `Writing` bloqueia edição e nova leitura;
- desconexão invalida o handle imediatamente;
- reconexão com outro serial não reutiliza estado dirty;
- dispositivo desconhecido nunca entra em `ReadyDirty` por padrão.

## 8. Interface do usuário

### 8.1 Tela principal

- seletor de dispositivo com VID:PID, serial e estado;
- desenho data-driven da geometria;
- abas ou segmented control para três camadas;
- indicação visual de posições modificadas;
- editor lateral de ação;
- ações `Ler do dispositivo`, `Gravar`, `Reverter alterações`, `Limpar tecla`, `Limpar camada`;
- status persistente da última operação e erro acionável.

### 8.2 Editor de ação

Categorias:

- teclado;
- combinação/macro;
- multimídia;
- mouse;
- iluminação;
- atraso.

A UI deve mostrar o limite real do firmware antes de aceitar uma sequência. Campos não suportados pela variante ficam desabilitados, não ocultamente descartados.

### 8.3 Perfis

- exportar JSON sem dados privados além do serial opcional;
- importar com validação de schema e variante;
- permitir importar perfil de outra unidade da mesma variante após aviso;
- nunca gravar automaticamente após importar;
- manter backup automático da última leitura confirmada.

## 9. Requisitos funcionais

| ID | Requisito |
|---|---|
| FR-001 | Descobrir somente dispositivos da allowlist e selecionar interface 0. |
| FR-002 | Exibir conexão/desconexão sem bloquear a UI. |
| FR-003 | Consultar e mapear a tupla de variante. |
| FR-004 | Ler as três camadas e preservar bytes desconhecidos. |
| FR-005 | Editar todos os tipos de ação da seção 4.3. |
| FR-006 | Marcar dirty por posição e gravar somente registros alterados. |
| FR-007 | Enviar commit uma única vez por transação. |
| FR-008 | Reler e verificar após gravação. |
| FR-009 | Exportar/importar perfis versionados. |
| FR-010 | Operar sem root por udev/uaccess. |
| FR-011 | Oferecer modo diagnóstico read-only para variante desconhecida. |
| FR-012 | Produzir relatório de diagnóstico sem conteúdo sensível das macros. |

## 10. Requisitos não funcionais

| ID | Requisito |
|---|---|
| NFR-001 | Nenhum binário Windows ou Wine no artefato final. |
| NFR-002 | Inicialização sem dispositivo em menos de 2 s em máquina comum. |
| NFR-003 | UI responsiva durante leitura, gravação e hotplug. |
| NFR-004 | Nenhuma escrita sem ação explícita do usuário. |
| NFR-005 | Escrita de perfil local atômica e recuperável. |
| NFR-006 | Logs sem sequências de teclas por padrão. |
| NFR-007 | Código de protocolo coberto por vetores binários determinísticos. |
| NFR-008 | Compatível com Wayland e X11 via GTK. |
| NFR-009 | Suporte inicial a x86_64 e aarch64. |
| NFR-010 | Regra udev limitada a VID/PID conhecidos e `TAG+="uaccess"`. |

## 11. Permissões udev

A regra atual `MODE="0666"` para todo `hidraw*` é insegura e deve ser removida do produto.

Regra proposta para o dispositivo validado:

```udev
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="1189", ATTRS{idProduct}=="8842", TAG+="uaccess"
```

PIDs adicionais somente devem ser acrescentados após validação. Distribuições sem logind/uaccess podem documentar um grupo dedicado como fallback; não usar `0666`.

## 12. Tratamento de erros

- **Permissão negada:** indicar a regra udev exata e como recarregá-la.
- **Dispositivo ocupado:** mostrar processo provável quando detectável; não tentar detach da interface de teclado.
- **Resposta truncada:** cancelar operação e preservar estado anterior.
- **Desconexão durante leitura:** descartar leitura incompleta.
- **Desconexão durante escrita:** marcar resultado como indeterminado e exigir releitura após reconexão.
- **Timeout depois do commit:** não repetir commit; exigir verificação.
- **Tupla desconhecida:** read-only e exportação de diagnóstico.
- **Perfil incompatível:** bloquear gravação até conversão explícita.

## 13. Plano de validação com hardware

### Fase A — passiva, sem escrita

1. Confirmar VID/PID, serial, interfaces, endpoints e descritores.
2. Confirmar que discovery seleciona apenas a interface vendor-defined.
3. Testar conexão, desconexão e reconexão repetidas.
4. Comparar descritor lido pelo backend hidapi com sysfs.

Critério: nenhuma alteração no comportamento das teclas.

### Fase B — comandos read-only

1. Capturar a consulta `03 FB FB FB` por usbmon/Wireshark.
2. Registrar a resposta completa e a tupla de variante.
3. Capturar `Read Configuration` no aplicativo original com USBPcap ou ambiente controlado.
4. Determinar exatamente os campos layer/range/block.
5. Ler a configuração com a implementação Linux e comparar byte a byte.

Critério: três leituras consecutivas idênticas com o dispositivo ocioso.

### Fase C — escrita mínima e reversível

1. Salvar snapshot binário completo.
2. Escolher uma tecla não crítica em uma camada não ativa.
3. Alterar para uma tecla básica simples.
4. Gravar um único registro e commit.
5. Reler e confirmar somente o registro esperado.
6. Testar fisicamente a tecla.
7. Restaurar o snapshot e verificar novamente.

Critério: round-trip sem alteração colateral.

### Fase D — matriz de ações

Cobrir pelo menos um caso de:

- tecla simples;
- cada modificador;
- combinação com múltiplos modificadores;
- sequência no limite e acima do limite;
- atraso mínimo, típico e máximo;
- cada ação multimídia;
- cada ação de mouse;
- cada modo e cor RGB;
- três camadas;
- limpeza de tecla e camada.

Critério: bytes gravados iguais aos vetores de referência e comportamento físico correto.

### Fase E — falhas

- remover o cabo durante leitura;
- remover durante escrita antes do commit;
- remover imediatamente após commit;
- negar permissão ao hidraw;
- conectar duas unidades;
- conectar HID não relacionado;
- resposta curta ou arquivo de captura corrompido no simulador.

Critério: nenhuma escrita em dispositivo errado, nenhum travamento e estado final explícito.

### Fase F — distribuição

Validar em:

- Arch Linux;
- Fedora;
- Ubuntu LTS;
- GNOME/Wayland;
- KDE/Wayland;
- sessão X11;
- x86_64;
- aarch64 quando hardware estiver disponível.

## 14. Estratégia de testes

### 14.1 Unitários

- encode/decode de cada frame;
- validação de tamanho e Report ID;
- round-trip de registros com bytes opacos;
- migrations de perfil;
- seleção da interface correta;
- máquina de estados da sessão.

### 14.2 Golden vectors

Manter arquivos binários produzidos por captura, com metadados separados:

```text
tests/vectors/<variant>/<operation>/
  request.bin
  response.bin
  expected.json
```

Os vetores não devem conter código do fabricante. Cada vetor registra hardware, firmware, operação humana, hash e origem da captura.

### 14.3 Simulador

Implementar `MockTransport` que reproduz:

- respostas válidas;
- timeout;
- leitura curta;
- desconexão;
- erro antes/depois de commit.

O simulador não substitui validação física.

## 15. Critérios de aceite da primeira versão

A versão 1.0 está pronta quando:

1. roda como binário Linux nativo, sem Wine;
2. detecta `1189:8842` e abre somente a interface 0;
3. identifica corretamente o hardware validado;
4. lê e mostra as três camadas sem perda de bytes desconhecidos;
5. edita e grava todas as categorias da seção 4.3;
6. grava somente posições dirty, envia commit e verifica por releitura;
7. exporta/importa perfil JSON;
8. funciona com regra udev restrita e sem root;
9. passa pela matriz de hardware da seção 13;
10. não contém código, binários ou recursos do aplicativo original.

## 16. Questões abertas antes da implementação completa

1. Confirmar em captura se a leitura transporta 64 ou 65 bytes no wire.
2. Nomear a semântica exata dos bytes 2–4 do comando `0xFA`.
3. Fechar todos os offsets e limites do registro de 50 bytes.
4. Mapear a tabela completa de códigos de teclado, consumer, mouse e RGB.
5. Confirmar o limite máximo de eventos de macro por variante.
6. Validar cada PID alternativo e sua geometria.
7. Determinar se existe ACK explícito de gravação ou apenas verificação por releitura.
8. Confirmar se o commit é global, por camada ou por lote.
9. Identificar versão de firmware e diferenças de protocolo entre variantes.

Nenhuma dessas questões autoriza um fallback de escrita por tentativa. Funcionalidade não confirmada deve permanecer desabilitada ou read-only.
