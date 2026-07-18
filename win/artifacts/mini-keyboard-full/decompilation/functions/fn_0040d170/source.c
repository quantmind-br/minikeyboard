
void __thiscall FUN_0040d170(int param_1,byte param_2)

{
  char *pcVar1;
  byte *pbVar2;
  byte *pbVar3;
  byte bVar4;
  int iVar5;
  uint uVar6;
  uint uVar7;
  int iVar8;
  char local_30 [16];
  char *local_20;
  char *local_1c;
  
  uVar6 = (uint)DAT_00620475;
  uVar7 = (uint)DAT_00620474;
  iVar5 = uVar6 * 3000 + uVar7 * 0x32;
  iVar8 = (uint)(byte)(&DAT_0061e140)[iVar5 + 9] * 2 + 10;
  pbVar2 = &DAT_0061e140 + iVar8 + iVar5;
  bVar4 = *pbVar2;
  if ((param_2 < 0x14) || (*(byte *)(param_1 + 0x45) < param_2)) {
    (&DAT_0061e140)[iVar8 + uVar6 * 3000 + uVar7 * 0x32] =
         bVar4 | *(byte *)(param_1 + 0x800 + (uint)param_2);
  }
  else {
    pbVar3 = &DAT_0061e140 + iVar8 + iVar5 + 1;
    *pbVar2 = bVar4 | 2;
    *pbVar3 = *pbVar3 | *(byte *)(param_1 + 0x800 + (uint)param_2);
    pcVar1 = &DAT_0061e140 + iVar5 + 9;
    *pcVar1 = *pcVar1 + '\x01';
  }
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
  (&DAT_0061e143)[uVar6 * 3000 + uVar7 * 0x32] = DAT_0049e0fe;
  iVar8 = 0;
  (&DAT_0061e080)[uVar6 * 0x3c + uVar7] = 1;
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
    iVar8 = iVar8 + 1;
    QMessageLogger::debug(local_30);
  } while (iVar8 != 0x32);
  return;
}

