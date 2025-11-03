#!/bin/bash

# Script para configurar permissões de acesso aos dispositivos HID
# Necessário para o MiniKeyboard funcionar sem precisar de root

RULE_FILE="/etc/udev/rules.d/99-hidraw-permissions.rules"
RULE_CONTENT='KERNEL=="hidraw*", SUBSYSTEM=="hidraw", MODE="0666"'

echo "=========================================="
echo "MiniKeyboard - Setup HID Permissions"
echo "=========================================="
echo ""

# Verifica se a regra já existe
if [ -f "$RULE_FILE" ]; then
    if grep -q "$RULE_CONTENT" "$RULE_FILE"; then
        echo "✓ Regra udev já está configurada!"
        echo ""
        echo "Verificando dispositivos HID..."
        ls -l /dev/hidraw* 2>/dev/null | head -3
        exit 0
    fi
fi

echo "Este script irá:"
echo "1. Criar uma regra udev para acesso aos dispositivos HID"
echo "2. Recarregar as regras udev"
echo "3. Aplicar as mudanças imediatamente"
echo ""
echo "Permissão de sudo necessária..."
echo ""

# Cria a regra udev
echo "$RULE_CONTENT" | sudo tee "$RULE_FILE" > /dev/null

if [ $? -ne 0 ]; then
    echo "✗ Erro ao criar regra udev"
    exit 1
fi

echo "✓ Regra udev criada: $RULE_FILE"

# Recarrega as regras udev
sudo udevadm control --reload-rules
if [ $? -ne 0 ]; then
    echo "✗ Erro ao recarregar regras udev"
    exit 1
fi

echo "✓ Regras udev recarregadas"

# Aplica as mudanças
sudo udevadm trigger
if [ $? -ne 0 ]; then
    echo "✗ Erro ao aplicar mudanças"
    exit 1
fi

echo "✓ Mudanças aplicadas"
echo ""
echo "Verificando dispositivos HID..."
sleep 1
ls -l /dev/hidraw* 2>/dev/null | head -3
echo ""
echo "=========================================="
echo "✓ Configuração concluída com sucesso!"
echo "=========================================="
echo ""
echo "Agora você pode executar o MiniKeyboard:"
echo "  ./MiniKeyboard-x86_64.AppImage"
echo ""
