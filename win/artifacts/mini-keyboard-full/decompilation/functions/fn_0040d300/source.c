
void __thiscall FUN_0040d300(int param_1,byte param_2)

{
  char *pcVar1;
  undefined1 *puVar2;
  undefined1 uVar3;
  uint uVar4;
  int iVar5;
  uint uVar6;
  char local_30 [16];
  char *local_20;
  char *local_1c;
  
  uVar4 = (uint)DAT_00620474;
  uVar6 = (uint)DAT_00620475;
  local_30[0] = '\x02';
  local_30[1] = '\0';
  local_30[2] = '\0';
  local_30[3] = '\0';
  local_30[4] = '\0';
  local_30[5] = '\0';
  local_30[6] = '\0';
  local_30[7] = '\0';
  local_30[8] = '\0';
  local_30[9] = '\0';
  local_30[10] = '\0';
  local_30[0xb] = '\0';
  local_30[0xc] = '\0';
  local_30[0xd] = '\0';
  local_30[0xe] = '\0';
  local_30[0xf] = '\0';
  local_20 = "default";
  iVar5 = uVar6 * 3000 + uVar4 * 0x32;
  puVar2 = &DAT_0061e140 + (uint)(byte)(&DAT_0061e140)[iVar5 + 9] * 2 + iVar5 + 10;
  *puVar2 = *(undefined1 *)(param_1 + 0x800 + (uint)param_2);
  puVar2[1] = *(undefined1 *)(param_1 + 0x801 + (uint)param_2);
  uVar3 = DAT_0049e0fe;
  pcVar1 = &DAT_0061e140 + iVar5 + 9;
  *pcVar1 = *pcVar1 + '\x02';
  (&DAT_0061e140)[iVar5 + 3] = uVar3;
  (&DAT_0061e080)[uVar6 * 0x3c + uVar4] = 1;
  iVar5 = 0;
  QMessageLogger::debug(local_30);
  do {
    local_30[4] = '\x02';
    local_30[5] = '\0';
    local_30[6] = '\0';
    local_30[7] = '\0';
    local_30[8] = '\0';
    local_30[9] = '\0';
    local_30[10] = '\0';
    local_30[0xb] = '\0';
    local_30[0xc] = '\0';
    local_30[0xd] = '\0';
    local_30[0xe] = '\0';
    local_30[0xf] = '\0';
    local_20 = (char *)0x0;
    local_1c = "default";
    iVar5 = iVar5 + 1;
    QMessageLogger::debug(local_30);
  } while (iVar5 != 0x32);
  return;
}

